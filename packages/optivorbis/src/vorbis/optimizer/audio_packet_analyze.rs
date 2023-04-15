//! Contains the supporting code for the [`AudioPacketAnalyze`] Vorbis optimizer state.

use log::trace;
use vorbis_bitpack::BitpackReader;

use super::{
	audio_packet_common::process_audio_packet, VorbisCommentData, VorbisIdentificationHeaderData,
	VorbisOptimizerError, VorbisSetupData
};
use crate::vorbis::PacketType;

/// Analyzes the Huffman codeword frequencies of audio packets. This is the terminal
/// state of the analyzing phase.
pub(super) struct AudioPacketAnalyze {
	pub(super) comment_data: VorbisCommentData,
	pub(super) codec_setup: VorbisSetupData
}

impl AudioPacketAnalyze {
	pub(super) fn analyze_packet(
		&mut self,
		mut packet: &[u8],
		identification_data: &VorbisIdentificationHeaderData
	) -> Result<(Option<u16>, Option<Self>), VorbisOptimizerError> {
		trace!("Analyzing Vorbis audio packet");

		let packet_length = packet.len();
		let mut bitpacker = BitpackReader::new(&mut packet);

		// § 4.3.1, step 1: packet type decode. As mentioned in the end of that section of
		// the spec, it is important to return early successfully if the read fails due to
		// EOP. See process_audio_packet for more details
		let packet_type = PacketType::try_from(eval_on_eop!(
			bitpack_packet_read!(bitpacker, read_unsigned_integer, packet_length, const 1, u8),
			return Ok((None, None))
		)?)?;
		if packet_type != PacketType::Audio {
			return Err(VorbisOptimizerError::UnexpectedPacketType {
				expected_type: PacketType::Audio,
				actual_type: packet_type
			});
		}

		let (_, decode_blocksize) = process_audio_packet(
			identification_data,
			&self.codec_setup,
			packet_length,
			&mut bitpacker,
			// We just let the codebook collect frequencies for now, so do nothing in the callbacks
			|_, _, _| Ok(()),
			|_, _, _| Ok(()),
			()
		)?;

		// The specification does not require this, but in practice it makes little sense for
		// encoders to write bytes that will not be read by decoders, unless a too high minimum
		// bitrate is enforced. Exploit that as a sanity check for debugging purposes.
		// Comment this out when dealing with files that do not have a minimum bitrate set and do not
		// have padded packets
		/*debug_assert!(
			identification_data.minimum_bitrate < 0 && bitpacker.into_inner().is_empty(),
			"Trailing bytes at end of audio packet"
		);*/

		Ok((decode_blocksize, None))
	}
}
