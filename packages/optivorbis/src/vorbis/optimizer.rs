//! Contains the [`VorbisOptimizer`] struct, which is responsible for optimizing
//! unencapsulated, raw Vorbis streams.
//!
//! [`Remuxers`](crate::remuxer) are the primary intended consumers of this
//! low-level module.

use std::{
	borrow::Cow,
	io::{self, ErrorKind},
	mem,
	num::{NonZeroU8, NonZeroU32, TryFromIntError}
};

use audio_packet_analyze::AudioPacketAnalyze;
use audio_packet_rewrite::AudioPacketRewrite;
use comment_header_copy::CommentHeaderCopy;
use comment_header_parse::{CommentHeaderParse, VorbisCommentData};
use identification_header_copy::IdentificationHeaderCopy;
use log::{info, trace};
use setup_header_parse::{SetupHeaderParse, VorbisSetupData};
use setup_header_rewrite::SetupHeaderRewrite;
use thiserror::Error;

use super::{
	PacketType, TryPacketTypeFromInt, TryResidueTypeFromInt, TryVectorLookupTypeFromInt,
	codebook::VorbisCodebookError
};

/// Calls the specified `method` on a Vorbis bitpacker struct.
///
/// For bitpacker methods that allow reading integers of variable width, a build-time
/// constant width can be specified with the `const <integer>` syntax, or a run-time
/// width can be specified via `mut <integer>`. When possible it's preferable to use
/// build-time constants, as their correctness is checked at compile time. The read
/// integer will be casted to the specified `type`.
///
/// Any EOF condition found while reading from the stream will be mapped to a "too
/// small packet" error.
macro_rules! bitpack_packet_read {
	($bitpacker:expr, $method:ident, $packet_length:expr, const $width:expr, $type:ty) => {
		$crate::vorbis::optimizer::map_eof_err_to_small_packet_err(
			$bitpacker.$method(::vorbis_bitpack::bitpacked_integer_width!($width)),
			$packet_length
		)
		.map(|v| v as $type)
	};
	($bitpacker:expr, $method:ident, $packet_length:expr, mut $width:expr, $type:ty) => {
		$crate::vorbis::optimizer::map_eof_err_to_small_packet_err(
			$bitpacker.$method(::vorbis_bitpack::BitpackedIntegerWidth::new($width).unwrap()),
			$packet_length
		)
		.map(|v| v as $type)
	};
	($bitpacker:expr, $method:ident, $packet_length:expr) => {
		$crate::vorbis::optimizer::map_eof_err_to_small_packet_err(
			$bitpacker.$method(),
			$packet_length
		)
	};
}

/// Helper macro that evaluates and returns the value of the specified expression on a
/// `TooSmallPacket`, `UnexpectedEof` or `EofWhileDecodingEntry` Vorbis optimizer I/O
/// error. Any other errors and successful results are passed through.
macro_rules! eval_on_eop {
	($result:expr, $value:expr) => {
		match $result {
			Ok(inner) => Ok(inner),
			Err(VorbisOptimizerError::TooSmallPacket(_)) => $value,
			Err(VorbisOptimizerError::Io(err))
				if err.kind() == ::std::io::ErrorKind::UnexpectedEof =>
			{
				$value
			}
			Err(VorbisOptimizerError::CodebookError(
				$crate::vorbis::codebook::VorbisCodebookError::EofWhileDecodingEntry { .. }
			)) => $value,
			Err(err) => Err(err)
		}
	};
}

// Declare submodules after the macros so they can use them
mod audio_packet_analyze;
mod audio_packet_common;
mod audio_packet_rewrite;
mod comment_header_copy;
mod comment_header_parse;
mod identification_header_copy;
mod setup_header_parse;
mod setup_header_rewrite;

/// Holds settings that customize how Vorbis streams are optimized, irrespectively of
/// their container encapsulation.
#[derive(Default)]
#[non_exhaustive]
pub struct VorbisOptimizerSettings {
	/// Describes how the vendor string in the Vorbis comment header will be optimized.
	pub vendor_string_action: VorbisVendorStringAction,
	/// Describes how the vendor string in the Vorbis comment header will be optimized.
	pub comment_fields_action: VorbisCommentFieldsAction
}

/// Represents an error that may occur while optimizing a Vorbis stream. This error can
/// be returned by [`VorbisOptimizer`] methods.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum VorbisOptimizerError {
	/// A Vorbis packet is of an invalid type.
	#[error("Invalid Vorbis I packet type: {0}")]
	InvalidPacketType(#[from] TryPacketTypeFromInt),
	/// A Vorbis packet was not of the expected type within a context.
	#[error("Unexpected Vorbis I packet type: got {expected_type}, expected {actual_type}")]
	UnexpectedPacketType {
		/// The expected packet type.
		expected_type: PacketType,
		/// The actual packet type.
		actual_type: PacketType
	},
	/// A Vorbis packet is too small to contain valid data.
	#[error("Too small Vorbis I packet of {0} bytes")]
	TooSmallPacket(usize),
	/// A Vorbis data pattern, such as a fixed signature or sync pattern, was expected
	/// but not found in the stream.
	#[error("Invalid Vorbis I packet signature, sync or framing pattern")]
	InvalidPattern,
	/// A Vorbis header packet does not have the expected length for it to contain valid
	/// data.
	#[error(
		"Unexpected Vorbis I {header_type} length: got {actual_length} bytes, expected {expected_length}"
	)]
	UnexpectedHeaderPacketLength {
		/// The type of the header packet.
		header_type: PacketType,
		/// The expected header packet length.
		expected_length: usize,
		/// The actual packet length.
		actual_length: usize
	},
	/// The Vorbis stream reports to follow an unsupported version of the Vorbis
	/// specification.
	#[error("Vorbis codec version {0} is not supported")]
	IncompatibleVorbisVersion(u32),
	/// The reported number of channels in the Vorbis identification header is not valid.
	#[error("Invalid number of audio channels: {0}")]
	InvalidChannelCount(u8),
	/// The reported sampling frequency in the Vorbis identification header is not valid.
	#[error("Invalid sampling frequency: {0}")]
	InvalidSamplingFrequency(u32),
	/// The reported block sizes in the Vorbis identification header are not valid.
	#[error("Invalid Vorbis I block sizes: {0}, {1}")]
	InvalidBlocksizes(u16, u16),
	/// A integer conversion failed because the word size of this platform is not wide
	/// enough to hold its value. Optimizing on a platform with a bigger word size (i.e.,
	/// `usize` size) may help.
	#[error("Too big value for this platform: {0}")]
	TooBigInteger(#[from] TryFromIntError),
	/// A value in the Vorbis I setup header was found to be invalid or incoherent.
	#[error("Invalid value in setup header")]
	InvalidSetupValue,
	/// A codebook codeword in the Vorbis I setup header is too long.
	#[error("A codeword length exceeded the 32 bits limit")]
	TooBigCodewordLength,
	/// The Vorbis I setup header referenced a codebook lookup type that is reserved
	/// by the Vorbis I specification.
	#[error("Reserved codebook lookup type: {0}")]
	ReservedLookupType(#[from] TryVectorLookupTypeFromInt),
	/// A Vorbis I audio packet tried to decode a vector using a codebook that yields
	/// vectors of dimension zero, which is invalid, or of a dimension that does not
	/// fit in the residue vector.
	#[error(
		"Codebook {codebook} has a vector dimension of {dimensions} \
		(expected greater than zero and multiple of {expected_dimensions_multiple_of})"
	)]
	InvalidCodebookDimension {
		/// The codebook whose dimension was invalid.
		codebook: u8,
		/// The actual dimension of the codebook vectors.
		dimensions: u16,
		/// The number `dimensions` was expected to be a multiple of.
		expected_dimensions_multiple_of: u32
	},
	/// The Vorbis I setup header referenced an unsupported floor type.
	#[error("Unsupported floor type: {0}")]
	UnsupportedFloorType(u16),
	/// The Vorbis I setup header referenced a codebook that is undefined.
	#[error("Invalid codebook number referenced: {0}")]
	InvalidCodebookNumber(u8),
	/// The Vorbis I setup header contained a floor configuration of type 1 that
	/// referenced duplicated curve X points.
	#[error("The floor {0} setup data has repeated X points")]
	RepeatedFloor1Point(u8),
	/// The Vorbis I setup header contained a floor configuration of type 1 that
	/// contains curve X points than allowed by the specification.
	#[error("The floor {0} setup data has more than 65 X points")]
	TooManyFloor1Points(u8),
	/// The Vorbis I setup header referenced a residue type that is reserved by the
	/// Vorbis I specification.
	#[error("Reserved residue type: {0}")]
	ReservedResidueType(#[from] TryResidueTypeFromInt),
	/// The Vorbis I setup header referenced a channel mapping type that is reserved
	/// by the Vorbis I specification.
	#[error("Reserved mapping type: {0}")]
	ReservedMappingType(u16),
	/// The Vorbis I setup header referenced a channel mapping that was found to be
	/// invalid for the stream.
	#[error(
		"The channel mapping with magnitude channel {magnitude_channel} \
		and angle channel {angle_channel} is invalid for {audio_channels} audio channel(s)"
	)]
	InvalidChannelMapping {
		/// The magnitude channel index.
		magnitude_channel: u8,
		/// The angle channel index.
		angle_channel: u8,
		/// The number of audio channels, never zero.
		audio_channels: u8
	},
	/// The Vorbis I setup header referenced a channel multiplexing that was found to
	/// be invalid for the stream.
	#[error(
		"Invalid channel multiplexing submap: {mux_submap} \
		(must be equal or less than {mapping_submap_count})"
	)]
	InvalidChannelMultiplexing {
		/// The submap index.
		mux_submap: u8,
		/// The number of available submaps.
		mapping_submap_count: u8
	},
	/// The Vorbis I setup header referenced a floor that is invalid.
	#[error("Referenced invalid floor number: {0}")]
	InvalidFloorNumber(u8),
	/// The Vorbis I setup header referenced a residue that is invalid.
	#[error("Referenced invalid residue number: {0}")]
	InvalidResidueNumber(u8),
	/// The Vorbis I setup header referenced a mapping that is invalid.
	#[error("Referenced invalid mapping number: {0}")]
	InvalidMappingNumber(u8),
	/// A Vorbis I audio packet tried to decode a vector using a codebook that can
	/// only yield scalar values, due to its vector lookup type being "no lookup".
	#[error("Codebook {0} is not suitable for vector lookup, but an audio packet tried to do so")]
	ScalarCodebookUsedInVectorContext(u8),
	/// A Vorbis I audio packet tried to decode a residue vector by using a codebook
	/// that was not defined.
	#[error("An audio packet referred to the VQ classbook {0}, which was not defined")]
	InvalidVectorQuantizationClassbook(usize),
	/// A Vorbis I audio packet was encoded with a mode that was not defined in the
	/// setup header.
	#[error("An audio packet is encoded with mode {0}, which was not defined")]
	InvalidModeNumber(u8),
	/// An error occurred while performing an operation with a Vorbis
	/// codebook.
	#[error("Codebook error: {0}")]
	CodebookError(#[from] VorbisCodebookError),
	/// An I/O error occurred while handling a Vorbis packet.
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

/// Identifies which strategy to use to optimize the Vorbis vendor string
/// in the Vorbis comment header.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
#[derive(Default)]
pub enum VorbisVendorStringAction {
	/// The vendor string contained in the comment header will be preserved
	/// as-is.
	Copy,
	/// The vendor string contained in the comment header will be replaced
	/// by a short one that identifies OptiVorbis.
	///
	/// This will improve interoperability if the original vendor string
	/// contains invalid UTF-8 characters, as such characters violate the
	/// Vorbis specification.
	Replace,
	/// A string that identifies OptiVorbis will be appended to the vendor
	/// string contained in the comment header. The additional information
	/// takes little space and can be useful for traceability and
	/// troubleshooting purposes, so this is the recommended action in most
	/// cases.
	#[default]
	AppendTag,
	/// Like [`AppendTag`](Self::AppendTag), but appends a shorter identifying
	/// string.
	AppendShortTag,
	/// The vendor string contained in the comment header will be cleared
	/// out, to save as much space as possible.
	///
	/// This will improve interoperability if the original vendor string
	/// contains invalid UTF-8 characters, as such characters violate the
	/// Vorbis specification.
	Empty
}

/// Identifies which strategy to use to optimize the Vorbis user comment
/// string pairs in the Vorbis comment header.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
#[derive(Default)]
pub enum VorbisCommentFieldsAction {
	/// The comment pairs will be copied as-is to the optimized stream.
	#[default]
	Copy,
	/// Every comment will be removed, to save space on the optimized stream.
	///
	/// This may delete comment strings which contain invalid UTF-8 characters
	/// and are against the specification, improving interoperability.
	Delete
}

/// Implementation detail that represents all the possible states a Vorbis
/// optimizer can be in. The Vorbis optimizer implementation design is thus
/// inspired by the state pattern.
enum VorbisOptimizerState {
	CommentHeaderParse(CommentHeaderParse),
	SetupHeaderParse(SetupHeaderParse),
	AudioPacketAnalyze(AudioPacketAnalyze),
	IdentificationHeaderCopy(IdentificationHeaderCopy),
	CommentHeaderCopy(CommentHeaderCopy),
	SetupHeaderRewrite(SetupHeaderRewrite),
	AudioPacketRewrite(AudioPacketRewrite)
}

/// Implements the `From` trait to convert from a data type holding a
/// [`VorbisOptimizerState`] to its homonym variant.
macro_rules! optimizer_state_from_inner_impl {
	($($inner_state:ident),+) => {
		$(impl From<$inner_state> for VorbisOptimizerState {
			fn from(state: $inner_state) -> Self {
				Self::$inner_state(state)
			}
		})+
	}
}

optimizer_state_from_inner_impl!(
	CommentHeaderParse,
	SetupHeaderParse,
	AudioPacketAnalyze,
	IdentificationHeaderCopy,
	CommentHeaderCopy,
	SetupHeaderRewrite,
	AudioPacketRewrite
);

/// Matches the state of the current [`VorbisOptimizer`], executing a method on
/// the struct that holds the matched state data. The method is assumed to return
/// an [`Option`] or [`Result`] wrapping a `(return_value, Option<NewState>)` pair.
/// If there is a new state in that wrapped tuple, a state transition to that new
/// state will be made.
///
/// If the current state is not matched by any arm, the macro will panic.
macro_rules! match_and_delegate {
	( $self:ident { $( $variant:ident => $method:ident ( $($arg:expr),* ) ),+ } ) => {
		match &mut $self.state {
			$(VorbisOptimizerState::$variant(state) => {
				state.$method($($arg),*).map(|(return_value, new_state)| {
					// Transition to a new state if the method returned a new state
					if let Some(new_state) = new_state {
						$self.state = new_state.into();
					}

					return_value
				})
			}),+
			_ => panic!(
				"Unexpected optimizer state. Was the optimizer method called according to its contract?"
			)
		}
	};
}

/// A raw Vorbis stream optimizer, which tries to reduce the size of an already
/// encoded stream according to the specified settings, without any changes to
/// the decoded audio samples. In addition, some basic repairs may be done.
///
/// **This struct is fairly low-level and most end-users will not need to use it**.
/// [Remuxers](crate::remuxer) provide a simpler and powerful API suitable for
/// most cases.
///
/// From a detailed software design standpoint, this optimizer is an incarnation
/// of the state pattern: feeding packets to it makes it transition between
/// different optimization states. The concrete analysis and optimization operations
/// depend on the current state. Roughly, the internal states can be classified in
/// _analysis states_, which only consume packets for analysis, and _optimization
/// states_, which leverage the data obtained from the analysis states to optimize
/// packets.
pub struct VorbisOptimizer<'settings> {
	settings: &'settings VorbisOptimizerSettings,
	pub(crate) identification_data: VorbisIdentificationHeaderData,
	state: VorbisOptimizerState
}

/// Relevant data stored in the Vorbis identification header, which is the first
/// packet of any Vorbis stream.
pub(crate) struct VorbisIdentificationHeaderData {
	channels: NonZeroU8,
	/// The sampling frequency of the encoded audio samples, used by players to
	/// convert between sample counts and time.
	pub(crate) sampling_frequency: NonZeroU32,
	/// The hard maximum bitrate of the Vorbis stream that the encoder reports
	/// being told to heed.
	///
	/// The reference Vorbis encoder, `libvorbisenc`, selects how many bits to
	/// use to encode each packet and what value to use for this field based on
	/// the following bitrate control schemes:
	///
	/// - Quality mode: the encoder will try to keep the specified perceived
	///   audio quality constant, even if the audio data takes more or less bits
	///   to represent to a certain quality level (true VBR). Only the nominal
	///   bitrate is set in this case.
	/// - Quality mode selected by average bitrate: alternatively, the encoder
	///   can try match a priori a specified average bitrate to some perceived
	///   audio quality, disabling the reservoir-based bitrate management algorithm.
	///   This is also true VBR. Only the nominal bitrate is set in this case.
	/// - Average bitrate (ABR): the encoder builds a psychoacoustic profile of the
	///   audio data and then removes or adds fine detail to the audio packets
	///   as needed to meet the target bitrate. The bitrate of easy to
	///   encode audio sections will be increased towards this average, while the
	///   quality of the harder to encode ones will be decreased to match the
	///   average. The reservoir-based bitrate management algorithm is enabled.
	///   Only the nominal bitrate is set on this case.
	/// - Constrained ABR: like ABR, but setting additional hard minimum and/or
	///   maximum bitrate constraints. If the bitrate drops below the minimum, audio
	///   packets will be encoded with more quality, padded if it is not possible to
	///   add more quality. If the bitrate exceeds the maximum, the quality of the
	///   packets will be reduced, or if it is not possible to reduce it further,
	///   the packet will be truncated. The reservoir-based bitrate management
	///   algorithm is enabled. The nominal bitrate is set in this case, and
	///   so are the minimum and maximum bitrates if applicable.
	/// - Constant bitrate (CBR): if the minimum, average and maximum bitrates
	///   are equal, the stream will have a constant bitrate. The reservoir-based
	///   bitrate management algorithm is enabled. The minimum, nominal and maximum
	///   bitrates are set.
	///
	/// Different encoders may provide more or less precise bitrate estimations, or
	/// have different bitrate management modes. The description above is a summary
	/// of the relevant parts of the
	/// [`libvorbisenc` documentation](https://xiph.org/vorbis/doc/vorbisenc/overview.html).
	pub(crate) maximum_bitrate: i32,
	/// The average bitrate of the Vorbis stream that the encoder reports being told
	/// to target.
	///
	/// For more information about the usual meaning of this and other bitrate fields,
	/// see the documentation for [`maximum_bitrate`](Self::maximum_bitrate).
	pub(crate) nominal_bitrate: i32,
	/// The minimum bitrate of the Vorbis stream that the encoder reports being told
	/// to target.
	///
	/// For more information about the usual meaning of this and other bitrate fields,
	/// see the documentation for [`maximum_bitrate`](Self::maximum_bitrate).
	pub(crate) minimum_bitrate: i32,
	blocksizes: (u16, u16)
}

impl<'settings> VorbisOptimizer<'settings> {
	/// Creates a new [`VorbisOptimizer`] that uses the provided optimization
	/// settings, for the Vorbis stream that begins with the specified
	/// identification header. An error will be returned if the identification
	/// header is not valid for a Vorbis stream.
	pub fn new<B: AsRef<[u8]>>(
		settings: &'settings VorbisOptimizerSettings,
		identification_header: B
	) -> Result<Self, VorbisOptimizerError> {
		const IDENTIFICATION_HEADER_LENGTH: usize = 23 + 7;

		trace!("Decoding identification header Vorbis packet");

		let identification_header = common_header_validation(
			identification_header.as_ref(),
			PacketType::IdentificationHeader
		)?;

		// Validate the specific identification header fields, which always take 23 bytes
		let header_length = identification_header.len() + 7;
		if header_length < IDENTIFICATION_HEADER_LENGTH {
			return Err(VorbisOptimizerError::UnexpectedHeaderPacketLength {
				header_type: PacketType::IdentificationHeader,
				expected_length: IDENTIFICATION_HEADER_LENGTH,
				actual_length: header_length
			});
		}

		let vorbis_version = u32::from_le_bytes(identification_header[..4].try_into().unwrap());
		if vorbis_version != 0 {
			return Err(VorbisOptimizerError::IncompatibleVorbisVersion(
				vorbis_version
			));
		}

		let channels = {
			let channels = u8::from_le_bytes(identification_header[4..5].try_into().unwrap());

			NonZeroU8::new(channels).ok_or(VorbisOptimizerError::InvalidChannelCount(channels))?
		};

		let sampling_frequency = {
			let sampling_frequency =
				u32::from_le_bytes(identification_header[5..9].try_into().unwrap());

			NonZeroU32::new(sampling_frequency).ok_or(
				VorbisOptimizerError::InvalidSamplingFrequency(sampling_frequency)
			)?
		};

		// The bitrate fields are not relevant for our optimization purposes, but we may want to
		// copy them later
		let maximum_bitrate = i32::from_le_bytes(identification_header[9..13].try_into().unwrap());
		let nominal_bitrate = i32::from_le_bytes(identification_header[13..17].try_into().unwrap());
		let minimum_bitrate = i32::from_le_bytes(identification_header[17..21].try_into().unwrap());

		let blocksizes = (
			1u16 << (identification_header[21] & 0x0F),
			1u16 << (identification_header[21] >> 4)
		);

		const fn is_blocksize_in_range(blocksize: u16) -> bool {
			// From Vorbis I spec, section 4.2.2:
			// "Allowed final blocksize values are 64, 128, 256, 512, 1024, 2048, 4096 and 8192 in Vorbis I."
			// Blocksizes are always powers of two due to how we initialized them above, so we only need to
			// check that they are within the allowed range
			blocksize >= 64 && blocksize <= 8192
		}

		if !is_blocksize_in_range(blocksizes.0)
			|| !is_blocksize_in_range(blocksizes.1)
			|| blocksizes.0 > blocksizes.1
		{
			return Err(VorbisOptimizerError::InvalidBlocksizes(
				blocksizes.0,
				blocksizes.1
			));
		}

		// Ignore framing byte content, even though the specification mandates that it is zero.
		// We don't use its value for anything, and we trust that lower layers do their error
		// detection and correction. This is useful to "repair" streams too

		info!(
			"Vorbis identification header: {channels} channel(s), \
			{sampling_frequency} Hz sampling frequency, \
			minimum, nominal and maximum bitrates: {minimum_bitrate}, {nominal_bitrate} and {maximum_bitrate}, \
			blocksizes {} and {}",
			blocksizes.0, blocksizes.1
		);

		Ok(VorbisOptimizer {
			settings,
			identification_data: VorbisIdentificationHeaderData {
				channels,
				sampling_frequency,
				maximum_bitrate,
				nominal_bitrate,
				minimum_bitrate,
				blocksizes
			},
			state: CommentHeaderParse.into()
		})
	}

	/// Consumes the specified Vorbis packet to analyze the Vorbis stream for a
	/// future second optimization pass, and returns the size of the block of
	/// samples that decoding this packet would yield.
	///
	/// Note that, due to the inter-packet windowing performed by a Vorbis decoder,
	/// not every sample in a block would be output for an audio packet: some samples
	/// are overlapped with the previous and next packets. Read the Vorbis I
	/// specification, § 4.3.8, for more details.
	///
	/// If no error happens, and the specified packet is not an audio packet, or a
	/// decoder would discard it from the stream, `Ok(None)` is returned.
	///
	/// # Panics
	/// If [`optimize_packet`](Self::optimize_packet) was called once for this
	/// optimizer, implicitly moving it into the second optimization pass.
	pub fn analyze_packet<B: AsRef<[u8]>>(
		&mut self,
		packet: B
	) -> Result<Option<u16>, VorbisOptimizerError> {
		let packet = packet.as_ref();

		match_and_delegate!(self {
			CommentHeaderParse => analyze_packet(packet, self.settings),
			SetupHeaderParse => analyze_packet(packet, &self.identification_data),
			AudioPacketAnalyze => analyze_packet(packet, &self.identification_data)
		})
	}

	/// Consumes the specified Vorbis packet, returning its optimized representation
	/// and the size of the block of samples that decoding this packet would yield.
	///
	/// The optimizer will implicitly transition to the second optimization pass if it
	/// was not already in that pass, meaning that no further packets can be analyzed.
	/// When passing a packet in an owned buffer, this method will write to the buffer
	/// it already allocated for optimum performance.
	///
	/// `Ok(None)` is be returned for audio packets that may be entirely dropped from
	/// the stream without any side effects (e.g., 0 byte audio packets).
	/// `Ok(Some(..., None))` is returned on success for non-audio packets.
	/// `Ok(Some(..., Some(...)))` is returned on success for audio packets that a decoder
	/// would attempt to decode.
	///
	/// Note that, due to the inter-packet windowing performed by a Vorbis decoder,
	/// not every sample in a block would be output for an audio packet: some samples
	/// are overlapped with the previous and next packets. Read the Vorbis I
	/// specification, § 4.3.8, for more details.
	///
	/// # Preconditions
	/// It is assumed that packets are passed to this method in the same sequence they
	/// were passed to [`analyze_packet`](Self::analyze_packet). Failure to do so may
	/// cause panics and corrupt streams to be generated.
	// This type complexity is intrinsic to Vorbis complexity. Moving it around does not
	// help that much
	#[allow(clippy::type_complexity)]
	pub fn optimize_packet<'packet, B: Into<Cow<'packet, [u8]>>>(
		&mut self,
		packet: B
	) -> Result<Option<(Cow<'packet, [u8]>, Option<u16>)>, VorbisOptimizerError> {
		// Transition into optimizing states from analyzing states if necessary
		match &mut self.state {
			VorbisOptimizerState::CommentHeaderParse(_) => {
				self.state = IdentificationHeaderCopy {
					comment_data: None,
					codec_setup: None
				}
				.into();
			}
			VorbisOptimizerState::SetupHeaderParse(setup_header_parser) => {
				self.state = IdentificationHeaderCopy {
					comment_data: Some(mem::take(&mut setup_header_parser.comment_data)),
					codec_setup: None
				}
				.into();
			}
			VorbisOptimizerState::AudioPacketAnalyze(audio_packet_analyzer) => {
				self.state = IdentificationHeaderCopy {
					comment_data: Some(mem::take(&mut audio_packet_analyzer.comment_data)),
					codec_setup: Some(mem::take(&mut audio_packet_analyzer.codec_setup))
				}
				.into();
			}
			_ => ()
		}

		let packet = packet.into();

		match_and_delegate!(self {
			IdentificationHeaderCopy => optimize_packet(packet, &self.identification_data),
			CommentHeaderCopy => optimize_packet(packet),
			SetupHeaderRewrite => optimize_packet(packet),
			AudioPacketRewrite => optimize_packet(packet, &self.identification_data)
		})
	}
}

/// Checks that the common Vorbis header packet prelude is valid, according to section
/// 4.2.1 of the Vorbis I specification. The returned slice is a subslice of the passed
/// slice with the common header fields in the beginning removed.
fn common_header_validation(
	header_packet: &[u8],
	expected_type: PacketType
) -> Result<&[u8], VorbisOptimizerError> {
	trace!("Performing common Vorbis header packet validation");

	// The Vorbis header packets are defined in sections 4 and 5 of the Vorbis I
	// specification. Rust's smallest integer type is 8 bits, so Vorbis bitpacking
	// bytes are 8 bits wide too by definition, which allows some simplifications
	// to be done

	// Validate header packet length
	let header_length = header_packet.len();
	if header_length < 7 {
		return Err(VorbisOptimizerError::TooSmallPacket(header_length));
	}

	// Validate packet type
	let packet_type = PacketType::try_from(header_packet[0])?;
	if packet_type != expected_type {
		return Err(VorbisOptimizerError::UnexpectedPacketType {
			expected_type,
			actual_type: packet_type
		});
	}

	// Validate signature bytes that declare this header packet as Vorbis
	if &header_packet[1..7] != b"vorbis" {
		return Err(VorbisOptimizerError::InvalidPattern);
	}

	// Skip common header bytes
	Ok(&header_packet[7..])
}

/// Auxiliary function that maps an unexpected EOF I/O error to a too small packet
/// Vorbis optimizer error. This function is meant to be used by the
/// [`bitpack_packet_read`] macro only.
fn map_eof_err_to_small_packet_err<T>(
	result: Result<T, io::Error>,
	packet_length: usize
) -> Result<T, VorbisOptimizerError> {
	result.map_err(|error| match error.kind() {
		ErrorKind::UnexpectedEof => VorbisOptimizerError::TooSmallPacket(packet_length),
		_ => error.into()
	})
}

/// The Vorbis I `ilog` function, as defined in section 9.2.1 of the Vorbis I
/// specification. Mathematically, it returns the floor of the base-2 logarithm
/// of the specified number plus one, except for 0 and negative numbers, where it
/// returns zero. For zero and positive numbers, this is equivalent to the minimum
/// number of bits required to represent integers in [0, n].
const fn ilog(n: i32) -> u8 {
	// Surprisingly, branching in the source code translates to better machine code
	if n > 0 {
		32 - n.leading_zeros() as u8
	} else {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::ilog;

	#[test]
	fn ilog_works() {
		// Values from Vorbis I specification, section 9.2.1
		assert_eq!(ilog(0), 0);
		assert_eq!(ilog(1), 1);
		assert_eq!(ilog(2), 2);
		assert_eq!(ilog(3), 2);
		assert_eq!(ilog(4), 3);
		assert_eq!(ilog(7), 3);

		// Additional checks
		assert_eq!(ilog(i32::MAX), 31);
		assert_eq!(ilog(i32::MIN), 0);
	}
}
