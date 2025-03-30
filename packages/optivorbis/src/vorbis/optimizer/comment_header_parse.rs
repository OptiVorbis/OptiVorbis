//! Contains the supporting code for the [`CommentHeaderParse`] Vorbis optimizer state.

use std::borrow::Cow;

use log::{info, trace, warn};
use thiserror::Error;

use super::{
	SetupHeaderParse, VorbisCommentFieldsAction, VorbisOptimizerError, VorbisOptimizerSettings,
	VorbisVendorStringAction, common_header_validation
};
use crate::{OPTIVORBIS_SHORT_VERSION_TAG, OPTIVORBIS_VERSION_TAG, vorbis::PacketType};

/// The Vorbis optimizer state reached when decoding a comment header. After decoding
/// the comment header, the next state is decoding the setup header.
pub(super) struct CommentHeaderParse;

/// The Vorbis comment data to carry over from the comment header.
#[derive(Default)]
pub(super) struct VorbisCommentData {
	pub(super) vendor_string: Option<Vec<u8>>,
	pub(super) user_comments: Vec<Vec<u8>> // At most 2^32 - 1 comments of at most 2^32 - 1 length each
}

/// Represents an error that may happen while reading or parsing the
/// comment header.
#[derive(Debug, Error)]
enum CommentReadError {
	#[error("{0}")]
	OptimizerError(VorbisOptimizerError),
	#[error("End of packet while reading comment header packet")]
	EndOfPacket
}

impl<T: Into<VorbisOptimizerError>> From<T> for CommentReadError {
	fn from(into_err: T) -> Self {
		Self::OptimizerError(into_err.into())
	}
}

impl CommentHeaderParse {
	pub(super) fn analyze_packet(
		&mut self,
		packet: &[u8],
		settings: &VorbisOptimizerSettings
	) -> Result<(Option<u16>, Option<SetupHeaderParse>), VorbisOptimizerError> {
		trace!("Decoding comment header Vorbis packet");

		let comment_header = match common_header_validation(packet, PacketType::CommentHeader) {
			Err(
				VorbisOptimizerError::TooSmallPacket(_)
				| VorbisOptimizerError::UnexpectedHeaderPacketLength { .. }
			) => {
				// Vorbis I spec, section 4.2, says:
				// "End-of-packet decoding the comment header is a non-fatal error condition".
				// We interpret "non-fatal" as meaning "the stream may still be decoded further".
				// Therefore, even though most Vorbis encoders and decoders we know deal with this
				// situation by erroring out, it is legal for a stream to be decoded as normal even
				// if the packet that would contain the comment header is truncated. Treat this
				// situation as if no error happened, and the comment header was just
				// empty

				&[]
			}
			Ok(comment_header) => comment_header,
			Err(error) => {
				// Unexpected packet type, or other fatal error condition
				return Err(error);
			}
		};

		let mut user_comments = vec![];
		let mut vendor_string = None;

		// As stated, the Vorbis specification allows for an end of packet to happen while
		// reading the comment header. Therefore, we need to handle that condition specifically
		match parse(
			comment_header,
			settings,
			&mut vendor_string,
			&mut user_comments
		) {
			Err(CommentReadError::OptimizerError(err)) => return Err(err),
			Err(CommentReadError::EndOfPacket) => {
				// Most programs won't accept this kind of Vorbis streams, so let the user
				// know. As we just parse this header to copy it around, there's no problem
				// if we "fix" it in the process
				warn!(
					"End of Vorbis packet while decoding the comment header. \
					The comment header is likely corrupt, but optimization can continue"
				);
			}
			_ => ()
		}

		// The parsing went as expected, or we found the end of packet. In any case,
		// we've parsed what we could, and we can consider the job with this packet
		// done
		Ok((
			None,
			Some(SetupHeaderParse {
				comment_data: VorbisCommentData {
					vendor_string,
					user_comments
				}
			})
		))
	}
}

/// Parses the specified comment header, extracting the vendor string and user comments according
/// to the settings.
fn parse(
	comment_header: &[u8],
	settings: &VorbisOptimizerSettings,
	vendor_string: &mut Option<Vec<u8>>,
	user_comments: &mut Vec<Vec<u8>>
) -> Result<(), CommentReadError> {
	macro_rules! get_packet_checked {
		($index:expr) => {
			comment_header
				.get($index)
				.ok_or(CommentReadError::EndOfPacket)?
		};
	}

	// Read the vendor string
	let vendor_string_length = usize::try_from(u32::from_le_bytes(
		get_packet_checked!(..4).try_into().unwrap()
	))?;
	// The Vorbis specification mandates this string to be encoded in UTF-8, but
	// we don't enforce that here because some encoders and Vorbis manipulation
	// tools do not respect that. We could do a lossy UTF-8 conversion here to
	// ensure we always output valid UTF-8, but that would lose information about
	// the offending characters, so no program would be able to read the original
	// string in any encoding. Moreover, the most broken decoders probably won't
	// handle UTF-8 properly and choke with the Unicode replacement character. So
	// treat this as an opaque byte string and let broken decoders deal with their
	// own brokenness. Good decoders are very likely to be made by programmers who
	// paid attention to these matters like us, and handle the error by doing a
	// lossy conversion for display, or signalling the appropriate warnings or
	// errors. In any case, we only append valid UTF-8 bytes to this string, so it
	// won't happen that valid vendor strings are made non-conforming by us.
	//
	// As a side benefit, skipping UTF-8 validation is a bit faster
	let raw_vendor_string = get_packet_checked!(4..4 + vendor_string_length);

	info!(
		"Encoder vendor string: {}",
		String::from_utf8_lossy(raw_vendor_string)
	);

	*vendor_string = Some(match settings.vendor_string_action {
		VorbisVendorStringAction::Copy => raw_vendor_string.into(),
		VorbisVendorStringAction::Replace => OPTIVORBIS_VERSION_TAG.into(),
		VorbisVendorStringAction::AppendTag => {
			append_tag_if_needed(raw_vendor_string, OPTIVORBIS_VERSION_TAG)
		}
		VorbisVendorStringAction::AppendShortTag => {
			append_tag_if_needed(raw_vendor_string, OPTIVORBIS_SHORT_VERSION_TAG)
		}
		VorbisVendorStringAction::Empty => "".into()
	});

	let mut user_comment_count = u32::from_le_bytes(
		get_packet_checked!(4 + vendor_string_length..4 + vendor_string_length + 4)
			.try_into()
			.unwrap()
	);

	info!("User comment count: {user_comment_count}");

	// Now read the user comment fields if they should be copied
	if settings.comment_fields_action == VorbisCommentFieldsAction::Copy {
		trace!("Copying user comments");

		let mut user_comment_length_start_index = 4 + vendor_string_length + 4;

		while user_comment_count > 0 {
			let user_comment_length_end_index = user_comment_length_start_index + 4;
			let user_comment_length = usize::try_from(u32::from_le_bytes(
				get_packet_checked!(user_comment_length_start_index..user_comment_length_end_index)
					.try_into()
					.unwrap()
			))?;
			let user_comment_end_index = user_comment_length_end_index + user_comment_length;

			// We don't do any validation on the actual contents of the comments because not
			// all encoders follow the specification, and we don't care about their contents
			// anyway. Even UTF-8 validity can't be taken for granted, as explained above
			// with the vendor strings
			let user_comment =
				get_packet_checked!(user_comment_length_end_index..user_comment_end_index);

			info!("User comment: {}", String::from_utf8_lossy(user_comment));

			user_comments.push(user_comment.into());

			user_comment_length_start_index = user_comment_end_index;
			user_comment_count -= 1;
		}
	} else {
		trace!("Skipping user comments");
	}

	Ok(())
}

/// Appends the given tag to the given binary string if it doesn't already contain it.
fn append_tag_if_needed<'str, S: Into<Cow<'str, [u8]>>>(vendor_string: S, tag: &str) -> Vec<u8> {
	let mut vendor_string = vendor_string.into().into_owned();

	if !ends_with_optivorbis_version_tag_and_separator(&vendor_string) {
		vendor_string.extend_from_slice(b"; ");
		vendor_string.extend_from_slice(tag.as_bytes());
	}

	vendor_string
}

/// Checks whether the given binary string ends with a separator followed by an OptiVorbis version tag.
fn ends_with_optivorbis_version_tag_and_separator<S: AsRef<[u8]>>(vendor_string: S) -> bool {
	let vendor_string = vendor_string.as_ref();

	vendor_string
		.strip_suffix(OPTIVORBIS_SHORT_VERSION_TAG.as_bytes())
		.map(|vendor_string_without_tag| vendor_string_without_tag.ends_with(b"; "))
		.or_else(|| {
			vendor_string
				.strip_suffix(OPTIVORBIS_VERSION_TAG.as_bytes())
				.map(|vendor_string_without_tag| vendor_string_without_tag.ends_with(b"; "))
		})
		.unwrap_or(false)
}
