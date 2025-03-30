//! Contains the supporting code for the [`CommentHeaderCopy`] Vorbis optimizer state.

use std::borrow::Cow;

use log::trace;

use super::{
	VorbisOptimizerError, comment_header_parse::VorbisCommentData,
	setup_header_parse::VorbisSetupData, setup_header_rewrite::SetupHeaderRewrite
};

/// Copies the comment header according to the analyzed comment data, and then
/// transitions to the codec setup header rewrite state.
pub(super) struct CommentHeaderCopy {
	pub(super) comment_data: Option<VorbisCommentData>,
	pub(super) codec_setup: Option<VorbisSetupData>
}

impl CommentHeaderCopy {
	#[allow(clippy::type_complexity)]
	pub(super) fn optimize_packet<'packet>(
		&mut self,
		mut packet: Cow<'packet, [u8]>
	) -> Result<
		(
			Option<(Cow<'packet, [u8]>, Option<u16>)>,
			Option<SetupHeaderRewrite>
		),
		VorbisOptimizerError
	> {
		trace!("Optimizing comment header Vorbis packet");

		// Unwrap is safe because the stream is assumed to have passed the analysis phase:
		// if we optimize a comment header packet, we've analyzed it before, and thus we
		// have that data available
		let comment_data = self.comment_data.as_ref().unwrap();
		let packet_data = packet.to_mut();

		// It's very likely we will need to change the comment header contents in some way.
		// To simplify the code let's just regenerate the comment header contents in any case.
		// This is very cheap to do in practice, as we deal with already owned data
		packet_data.clear();

		// Common header packet fields
		packet_data.push(3); // Packet type
		packet_data.extend_from_slice(b"vorbis"); // Header signature

		// Vendor string. During analysis we might have reached EOP reading the comments header
		// packet, and thus not have any vendor string available; in that case, just use an empty
		// vendor string
		let empty_vec = vec![];
		let vendor_string = comment_data.vendor_string.as_ref().unwrap_or(&empty_vec);
		packet_data.extend_from_slice(&u32::try_from(vendor_string.len())?.to_le_bytes()[..]);
		packet_data.extend_from_slice(vendor_string);

		// Pass through the user comments we decided to keep during analysis
		packet_data.extend_from_slice(&(comment_data.user_comments.len() as u32).to_le_bytes()[..]);
		for comment in &comment_data.user_comments {
			packet_data.extend_from_slice(&(comment.len() as u32).to_le_bytes()[..]);
			packet_data.extend_from_slice(comment);
		}

		// Framing flag
		packet_data.push(1);

		Ok((
			Some((packet, None)),
			Some(SetupHeaderRewrite {
				codec_setup: self.codec_setup.take()
			})
		))
	}
}
