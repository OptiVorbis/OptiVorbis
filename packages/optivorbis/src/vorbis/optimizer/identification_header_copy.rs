//! Contains the supporting code for the [`IdentificationHeaderCopy`] Vorbis optimizer state.

use std::{borrow::Cow, ops::Range};

use log::trace;

use super::{
	VorbisIdentificationHeaderData, VorbisOptimizerError, comment_header_copy::CommentHeaderCopy,
	comment_header_parse::VorbisCommentData, setup_header_parse::VorbisSetupData
};

/// Copies the provided identification header, fixing or changing some fields as intended.
/// The next state is rewriting the setup header.
pub(super) struct IdentificationHeaderCopy {
	pub(super) comment_data: Option<VorbisCommentData>,
	pub(super) codec_setup: Option<VorbisSetupData>
}

impl IdentificationHeaderCopy {
	#[allow(clippy::type_complexity)]
	pub(super) fn optimize_packet<'packet>(
		&mut self,
		mut packet: Cow<'packet, [u8]>,
		identification_data: &VorbisIdentificationHeaderData
	) -> Result<
		(
			Option<(Cow<'packet, [u8]>, Option<u16>)>,
			Option<CommentHeaderCopy>
		),
		VorbisOptimizerError
	> {
		const SAMPLING_FREQUENCY_BYTE_RANGE: Range<usize> = 7 + 5..7 + 9;
		const MAXIMUM_BITRATE_BYTE_RANGE: Range<usize> = 7 + 9..7 + 13;
		const NOMINAL_BITRATE_BYTE_RANGE: Range<usize> = 7 + 13..7 + 17;
		const MINIMUM_BITRATE_BYTE_RANGE: Range<usize> = 7 + 17..7 + 21;
		const FRAMING_BYTE_OFFSET: usize = 7 + 22;
		const EXPECTED_PACKET_LENGTH: usize = 7 + 23;

		trace!("Optimizing identification header Vorbis packet");

		// Due to the previous analysis states in the optimizer, we can assume that the input
		// packet is well-formed, and contains the data we expect
		let sampling_frequency =
			u32::from_le_bytes(packet[SAMPLING_FREQUENCY_BYTE_RANGE].try_into().unwrap());
		let maximum_bitrate =
			i32::from_le_bytes(packet[MAXIMUM_BITRATE_BYTE_RANGE].try_into().unwrap());
		let nominal_bitrate =
			i32::from_le_bytes(packet[NOMINAL_BITRATE_BYTE_RANGE].try_into().unwrap());
		let minimum_bitrate =
			i32::from_le_bytes(packet[MINIMUM_BITRATE_BYTE_RANGE].try_into().unwrap());

		let target_sampling_frequency = identification_data.sampling_frequency.get();
		let target_maximum_bitrate = identification_data.maximum_bitrate;
		let target_nominal_bitrate = identification_data.nominal_bitrate;
		let target_minimum_bitrate = identification_data.minimum_bitrate;

		// The user might want to mangle some of these fields. If that was done, then modify
		// the packet data
		if target_sampling_frequency != sampling_frequency
			|| target_maximum_bitrate != maximum_bitrate
			|| target_nominal_bitrate != nominal_bitrate
			|| target_minimum_bitrate != minimum_bitrate
		{
			packet.to_mut()[SAMPLING_FREQUENCY_BYTE_RANGE]
				.copy_from_slice(&target_sampling_frequency.to_le_bytes());
			packet.to_mut()[MAXIMUM_BITRATE_BYTE_RANGE]
				.copy_from_slice(&target_maximum_bitrate.to_le_bytes());
			packet.to_mut()[NOMINAL_BITRATE_BYTE_RANGE]
				.copy_from_slice(&target_nominal_bitrate.to_le_bytes());
			packet.to_mut()[MINIMUM_BITRATE_BYTE_RANGE]
				.copy_from_slice(&target_minimum_bitrate.to_le_bytes());
		}

		// The Vorbis I specification mandates that the framing bit is nonzero, but
		// we accept files with a zero framing bit. Change the framing bit if necessary
		if packet[FRAMING_BYTE_OFFSET] != 1 {
			trace!("Fixing framing bit");
			packet.to_mut()[FRAMING_BYTE_OFFSET] = 1;
		}

		// We accept files with trailing bytes in the identification header packet, but
		// make sure we discard them. The Vorbis I specification, § 1.1.3, explicitly
		// mentions that padded packets are valid too
		if packet.len() > EXPECTED_PACKET_LENGTH {
			trace!("Removing trailing bytes at end of packet");
			packet.to_mut().truncate(EXPECTED_PACKET_LENGTH);
		}

		Ok((
			Some((packet, None)),
			Some(CommentHeaderCopy {
				comment_data: self.comment_data.take(),
				codec_setup: self.codec_setup.take()
			})
		))
	}
}
