//! Contains the supporting code for the [`AudioPacketRewrite`] Vorbis optimizer state.

use std::borrow::Cow;

use log::trace;
use vorbis_bitpack::{
	BitpackReader, BitpackWriter, BitpackedIntegerWidth, bitpacked_integer_width
};

use super::{
	VorbisIdentificationHeaderData, VorbisOptimizerError,
	audio_packet_common::process_audio_packet, setup_header_parse::VorbisSetupData
};

/// Rewrites Huffman codewords contained in audio packets with their optimal ones.
/// This is the terminal state of the optimization phase.
pub(super) struct AudioPacketRewrite {
	pub(super) codec_setup: VorbisSetupData,
	codebook_optimal_codewords: Vec<Vec<Option<(u32, u8)>>>
}

impl AudioPacketRewrite {
	/// Creates a new instance of this optimizer state. This is relatively expensive,
	/// as it will ask each codebook to generate its optimized codewords.
	pub(super) fn new(mut codec_setup: VorbisSetupData) -> Self {
		// Compute the optimal codeword for each codebook entry. Unused entries have None
		let mut codebook_optimal_codewords =
			Vec::with_capacity(codec_setup.codebook_configurations.len());
		for codebook_configuration in &mut codec_setup.codebook_configurations {
			codebook_optimal_codewords.push(codebook_configuration.codebook.optimal_codewords());
		}

		Self {
			codec_setup,
			codebook_optimal_codewords
		}
	}

	#[allow(clippy::type_complexity)]
	pub(super) fn optimize_packet<'packet>(
		&mut self,
		packet: Cow<'packet, [u8]>,
		identification_data: &VorbisIdentificationHeaderData
	) -> Result<(Option<(Cow<'packet, [u8]>, Option<u16>)>, Option<Self>), VorbisOptimizerError> {
		trace!("Optimizing Vorbis audio packet");

		let mut packet = &*packet;
		let packet_length = packet.len();
		let mut previous_packet_bitpacker = BitpackReader::new(&mut packet);

		// We need to read from the unoptimized packet, and then write a potentially very different
		// version of it. Thus, we need a whole new buffer. Usually, each packet is just slightly
		// shorter than its unoptimized version, so the extra capacity is not that wasteful
		let mut new_packet = Vec::with_capacity(packet_length);
		let mut new_packet_bitpacker = BitpackWriter::new(&mut new_packet);

		// Packet type. As we've validated the stream previously, we can assume it's an audio packet.
		// It is important to return early successfully if the read we do to keep sync fails due to
		// EOP. See process_audio_packet for more details
		new_packet_bitpacker.write_unsigned_integer(
			eval_on_eop!(
				bitpack_packet_read!(previous_packet_bitpacker, read_unsigned_integer, packet_length, const 1, u32),
				return Ok((None, None))
			)?,
			bitpacked_integer_width!(1)
		)?;

		let (keep_packet, decode_blocksize) = process_audio_packet(
			identification_data,
			&self.codec_setup,
			packet_length,
			&mut previous_packet_bitpacker,
			|unsigned_integer, width, bitpacker| {
				// Any bitpacked data we read is necessary for decode, so pass it through
				Ok(bitpacker.write_unsigned_integer(
					unsigned_integer,
					BitpackedIntegerWidth::new(width).unwrap()
				)?)
			},
			|codebook_number, entry_number, bitpacker| {
				// Replace codebook codewords by their optimal versions, already in the new setup header
				let (optimal_codeword, optimal_codeword_length) = self.codebook_optimal_codewords
					[codebook_number as usize][entry_number as usize]
					.unwrap();

				Ok(bitpacker.write_unsigned_integer(
					optimal_codeword,
					BitpackedIntegerWidth::new(optimal_codeword_length).unwrap()
				)?)
			},
			new_packet_bitpacker
		)?;

		Ok((
			keep_packet.then(|| (new_packet.into(), decode_blocksize)),
			None
		))
	}
}
