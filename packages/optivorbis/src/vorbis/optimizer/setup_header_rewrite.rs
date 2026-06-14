//! Contains the supporting code for the [`SetupHeaderRewrite`] Vorbis optimizer state.

use std::{borrow::Cow, io, io::Write};

use log::trace;
use slice_group_by::GroupBy;
use vorbis_bitpack::{BitpackWriter, BitpackedIntegerWidth, bitpacked_integer_width};

use super::{
	VorbisOptimizerError, audio_packet_rewrite::AudioPacketRewrite,
	setup_header_parse::VorbisSetupData
};
use crate::vorbis::{VectorLookupType, ilog, lookup1_values};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CodebookTruncation {
	entry_count: u32,
	multiplicand_count: usize
}

/// The Vorbis optimizer state reached when rewriting an optimized setup header.
/// A state transition is made to the audio packet optimizing state.
pub(super) struct SetupHeaderRewrite {
	pub(super) codec_setup: Option<VorbisSetupData>
}

impl SetupHeaderRewrite {
	#[allow(clippy::type_complexity)]
	pub(super) fn optimize_packet<'packet>(
		&mut self,
		mut packet: Cow<'packet, [u8]>
	) -> Result<
		(
			Option<(Cow<'packet, [u8]>, Option<u16>)>,
			Option<AudioPacketRewrite>
		),
		VorbisOptimizerError
	> {
		trace!("Optimizing setup header Vorbis packet");

		// Unwrap is safe because the stream is assumed to have passed the analysis phase:
		// if we optimize a comment header packet, we've analyzed it before, and thus we
		// have that data available
		let mut codec_setup = self.codec_setup.take().unwrap();
		let packet_data = packet.to_mut();

		packet_data.clear();

		// Common header packet fields
		packet_data.push(5); // Packet type
		packet_data.extend_from_slice(b"vorbis"); // Header signature

		// Codebooks
		let mut bitpacker = optimize_and_write_codebooks(&mut codec_setup, packet_data)?;

		// Time domain transforms placeholder data. Write the minimum data possible:
		// a single time domain transform value, set to zero
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(6))?;
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(16))?;

		// Floor configurations
		write_floor_configurations(&codec_setup, &mut bitpacker)?;

		// Residue configurations
		write_residue_configurations(&codec_setup, &mut bitpacker)?;

		// Mapping configurations
		write_mapping_configurations(&codec_setup, &mut bitpacker)?;

		// Modes
		write_modes(&codec_setup, &mut bitpacker)?;

		// Framing flag
		bitpacker.write_flag(true)?;

		drop(bitpacker);

		Ok((
			Some((packet, None)),
			Some(AudioPacketRewrite::new(codec_setup))
		))
	}
}

/// Generates the optimal codeword lengths for all the codebooks, and then writes their
/// configurations as dictated by the Vorbis stream format.
fn optimize_and_write_codebooks<W: Write>(
	codec_setup: &mut VorbisSetupData,
	mut packet_data: W
) -> Result<BitpackWriter<W>, io::Error> {
	// Codebook count. Guaranteed to be in [1, 256] by construction
	packet_data.write_all(&[(codec_setup.codebook_configurations.len() - 1) as u8])?;

	// Residue classbooks must still satisfy classifications^dimensions <= entries.
	// Other books only need the decoded entry numbers, which suffix truncation preserves.
	let mut min_entry_count = vec![0u32; codec_setup.codebook_configurations.len()];
	for residue in &codec_setup.residue_configurations {
		let classbook = residue.classbook as usize;
		let classbook_configuration = &codec_setup.codebook_configurations[classbook];
		let need = (residue.classifications as u32)
			.saturating_pow(classbook_configuration.dimensions as u32);
		min_entry_count[classbook] = min_entry_count[classbook].max(need);
	}

	// From now on, byte-wise writes are not necessarily equivalent to bitpacked writes
	let mut bitpacker = BitpackWriter::new(packet_data);

	for (codebook_index, codebook_configuration) in
		codec_setup.codebook_configurations.iter_mut().enumerate()
	{
		// Codebook sync pattern
		bitpacker.write_unsigned_integer(0x564342, bitpacked_integer_width!(24))?;

		bitpacker.write_unsigned_integer(
			codebook_configuration.dimensions as u32,
			bitpacked_integer_width!(16)
		)?;

		// Truncate only an unused suffix. This preserves every decoded entry number and
		// VQ vector; type-1 lookup books also keep lookup1_values unchanged.
		let original_entry_count = codebook_configuration.entry_count;
		let lookup_type = codebook_configuration.vector_lookup_type;
		let dimensions = codebook_configuration.dimensions;
		let codebook_truncation = codebook_truncation(
			original_entry_count,
			dimensions,
			lookup_type,
			codebook_configuration.codebook.optimal_codeword_lengths(),
			min_entry_count[codebook_index]
		);
		let new_entry_count = codebook_truncation.entry_count;

		bitpacker.write_unsigned_integer(new_entry_count, bitpacked_integer_width!(24))?;
		codebook_configuration.entry_count = new_entry_count;

		codebook_configuration
			.codebook_vector_multiplicands
			.truncate(codebook_truncation.multiplicand_count);

		let optimal_codeword_lengths =
			&codebook_configuration.codebook.optimal_codeword_lengths()[..new_entry_count as usize];

		// Single-entry codebooks are not iterated in windows below, so has_unused_entries could end
		// up with an incorrect value. Handle that by checking the first entry now.
		// Zero-entry codebooks can be considered to not have any unused entries.
		//
		// TODO we should remove empty codebooks. Encoders are not broken enough to generate them,
		// though, so that would not translate to real-world space savings. It also would require
		// replacing any references to the old codebook numbers
		let mut has_unused_entries = optimal_codeword_lengths
			.first()
			.is_some_and(|&first_codeword_length| first_codeword_length == 0);

		let codeword_lengths_are_sorted = optimal_codeword_lengths.windows(2).fold(
			true,
			|cw_lengths_are_sorted, cw_length_window| {
				has_unused_entries =
					has_unused_entries || cw_length_window[0] == 0 || cw_length_window[1] == 0;

				cw_lengths_are_sorted && cw_length_window[0] > cw_length_window[1]
			}
		);

		// The ordered codeword lengths format has an O(log entries) bit cost, while the unordered
		// format has an O(entries) bit cost. The constant overhead of the ordered format vs. the
		// unordered, non-sparse format is 4 bits larger, but some mathematical analysis shows that
		// for single-entry codebooks the ordered format is as space efficient, and for many-entry
		// codebooks the ordered format is more space efficient, so it's the better format overall.
		// The only exception are zero-entry codebooks, where only the constant overhead matters.
		// Unordered format must also be used if we have unused entries.
		//
		// TODO it may be possible to reorder the codebook entries so that their frequencies are
		// always sorted. However, doing so at least requires modifying the codebook VQ lookup data
		// for VQ lookup enabled codebooks, which needs some care and attention to get right, and
		// replacing any references to the old entry numbers in the setup and audio packets.
		//
		// Interleaved unused entries stay as holes. Compacting them would renumber later
		// entries, which are observed as scalar values or VQ lookup indexes.
		let use_ordered_format = codeword_lengths_are_sorted
			&& !has_unused_entries
			&& !optimal_codeword_lengths.is_empty();

		bitpacker.write_flag(use_ordered_format)?;

		if use_ordered_format {
			// By construction, the original codeword lengths are never greater than 32, so when
			// optimizing them they can't get bigger: the "worst case" is that codeword lengths are
			// redistributed among entries according to the actual frequencies.
			// Codeword lengths for used entries never are smaller than 1, too.
			// We also have at least one entry when using this format
			bitpacker.write_unsigned_integer(
				(optimal_codeword_lengths[0] - 1) as u32,
				bitpacked_integer_width!(5)
			)?;

			// Write the number of codewords per length, starting at the length of the codeword
			// for entry 0. This is easy and efficient thanks to a good library :)
			// The decoder will assume that the next run of codewords have + 1 length. We can
			// assume that here too due to how optimal Huffman codeword lengths are constructed:
			// it does not make sense to go one level down in the tree when there are free
			// leaves in the previous level
			let mut processed_entries = 0;
			for codeword_length_run in optimal_codeword_lengths.exponential_group() {
				let entries_per_codeword_length = codeword_length_run.len();

				bitpacker.write_unsigned_integer(
					entries_per_codeword_length as u32,
					// This can be assumed to be successful because entry_count is at most
					// 2^24 and processed_entries <= entry_count, so ilog returns at most 24
					BitpackedIntegerWidth::new(ilog(new_entry_count as i32 - processed_entries))
						.unwrap()
				)?;

				processed_entries += entries_per_codeword_length as i32;
			}
		} else {
			bitpacker.write_flag(has_unused_entries)?;

			if has_unused_entries {
				// Unordered, sparse format. Supports unused entries

				for codeword_length in optimal_codeword_lengths.iter().copied() {
					let used_entry = codeword_length != 0;

					bitpacker.write_flag(used_entry)?;

					if used_entry {
						bitpacker.write_unsigned_integer(
							(codeword_length - 1) as u32,
							bitpacked_integer_width!(5)
						)?;
					}
				}
			} else {
				// Unordered, non-sparse format. Does not support unused entries

				for codeword_length in optimal_codeword_lengths.iter().copied() {
					bitpacker.write_unsigned_integer(
						(codeword_length - 1) as u32,
						bitpacked_integer_width!(5)
					)?;
				}
			}
		}

		// Write VQ setup information
		bitpacker.write_unsigned_integer(
			codebook_configuration.vector_lookup_type as u32,
			bitpacked_integer_width!(4)
		)?;
		if codebook_configuration.vector_lookup_type != VectorLookupType::NoLookup {
			bitpacker.write_float32(codebook_configuration.codebook_vector_minimum_value)?;
			bitpacker.write_float32(codebook_configuration.codebook_vector_delta_value)?;
			// By construction, this is at least 1, so overflow does not happen
			bitpacker.write_unsigned_integer(
				codebook_configuration.codebook_vector_value_bits as u32 - 1,
				bitpacked_integer_width!(4)
			)?;
			bitpacker.write_flag(codebook_configuration.codebook_vector_sequence_flag)?;

			// This width is valid by construction, so we can unwrap it
			let multiplicand_width =
				BitpackedIntegerWidth::new(codebook_configuration.codebook_vector_value_bits)
					.unwrap();

			// The multiplicand suffix orphaned by entry truncation was already removed above.
			for multiplicand in codebook_configuration
				.codebook_vector_multiplicands
				.iter()
				.copied()
			{
				bitpacker.write_unsigned_integer(multiplicand as u32, multiplicand_width)?;
			}
		}
	}

	Ok(bitpacker)
}

fn codebook_truncation(
	original_entry_count: u32,
	dimensions: u16,
	lookup_type: VectorLookupType,
	optimal_codeword_lengths: &[u64],
	min_entry_count: u32
) -> CodebookTruncation {
	let used_entry_prefix_len = optimal_codeword_lengths
		.iter()
		.rposition(|&codeword_length| codeword_length != 0)
		.map_or(0, |last_used_entry| last_used_entry as u32 + 1);

	let mut entry_count = if used_entry_prefix_len == 0 {
		// Keep all-unused codebooks exactly as they were.
		original_entry_count
	} else {
		used_entry_prefix_len.max(min_entry_count)
	};

	if lookup_type == VectorLookupType::ImplicitlyPopulated
		&& entry_count < original_entry_count
		&& dimensions != 0
	{
		// Clamp to the start of the original lookup1_values bucket.
		let target = lookup1_values(original_entry_count, dimensions);
		let lookup_value_floor = target
			.saturating_pow(dimensions as u32)
			.min(original_entry_count);
		entry_count = entry_count.max(lookup_value_floor);
	}

	let multiplicand_count = match lookup_type {
		VectorLookupType::ImplicitlyPopulated => lookup1_values(entry_count, dimensions) as usize,
		VectorLookupType::ExplicitlyPopulated => entry_count as usize * dimensions as usize,
		VectorLookupType::NoLookup => 0
	};

	CodebookTruncation {
		entry_count,
		multiplicand_count
	}
}

/// Writes all the floor configurations as dictated by the Vorbis stream format.
fn write_floor_configurations<W: Write>(
	codec_setup: &VorbisSetupData,
	bitpacker: &mut BitpackWriter<W>
) -> Result<(), io::Error> {
	// Floor configuration count
	bitpacker.write_unsigned_integer(
		codec_setup.floor_configurations.len() as u32 - 1,
		bitpacked_integer_width!(6)
	)?;

	for floor_configuration in &codec_setup.floor_configurations {
		// Floor type. We only support type 1
		bitpacker.write_unsigned_integer(1, bitpacked_integer_width!(16))?;

		// Partition classes
		bitpacker.write_unsigned_integer(
			floor_configuration.partition_class_list.len() as u32,
			bitpacked_integer_width!(5)
		)?;
		for partition_class in floor_configuration
			.partition_class_list
			.iter()
			.map(|class| *class as u32)
		{
			bitpacker.write_unsigned_integer(partition_class, bitpacked_integer_width!(4))?;
		}

		let class_configuration = floor_configuration
			.class_dimensions
			.iter()
			.copied()
			.zip(floor_configuration.class_subclasses.iter().copied())
			.zip(floor_configuration.class_masterbooks.iter())
			.zip(floor_configuration.subclass_books.iter());

		for (((class_dimensions, class_subclasses), class_masterbooks), subclass_books) in
			class_configuration
		{
			bitpacker
				.write_unsigned_integer(class_dimensions as u32 - 1, bitpacked_integer_width!(3))?;
			bitpacker
				.write_unsigned_integer(class_subclasses as u32, bitpacked_integer_width!(2))?;
			if let Some(codebook_number) = class_masterbooks {
				bitpacker
					.write_unsigned_integer(*codebook_number as u32, bitpacked_integer_width!(8))?;
			}

			for subclass_book in subclass_books {
				bitpacker.write_unsigned_integer(
					subclass_book.map_or(0, |book| book as u32 + 1),
					bitpacked_integer_width!(8)
				)?;
			}
		}

		// Spectrum point data
		bitpacker.write_unsigned_integer(
			floor_configuration.multiplier as u32 - 1,
			bitpacked_integer_width!(2)
		)?;
		bitpacker.write_unsigned_integer(
			floor_configuration.range_bits as u32,
			bitpacked_integer_width!(4)
		)?;

		// The width is valid by construction, so unwrapping is safe
		let range_bits_width = BitpackedIntegerWidth::new(floor_configuration.range_bits).unwrap();
		for x_value in floor_configuration.x_list.iter().copied() {
			bitpacker.write_unsigned_integer(x_value as u32, range_bits_width)?;
		}
	}

	Ok(())
}

/// Writes all the residue configurations as dictated by the Vorbis stream format.
fn write_residue_configurations<W: Write>(
	codec_setup: &VorbisSetupData,
	bitpacker: &mut BitpackWriter<W>
) -> Result<(), io::Error> {
	// Residue configuration count
	bitpacker.write_unsigned_integer(
		codec_setup.residue_configurations.len() as u32 - 1,
		bitpacked_integer_width!(6)
	)?;

	for residue_configuration in &codec_setup.residue_configurations {
		// Residue type
		bitpacker.write_unsigned_integer(
			residue_configuration.residue_type as u32,
			bitpacked_integer_width!(16)
		)?;

		// Residue vector decode parameters
		bitpacker
			.write_unsigned_integer(residue_configuration.begin, bitpacked_integer_width!(24))?;
		bitpacker
			.write_unsigned_integer(residue_configuration.end, bitpacked_integer_width!(24))?;
		bitpacker.write_unsigned_integer(
			residue_configuration.partition_size - 1,
			bitpacked_integer_width!(24)
		)?;
		bitpacker.write_unsigned_integer(
			residue_configuration.classifications as u32 - 1,
			bitpacked_integer_width!(6)
		)?;
		bitpacker.write_unsigned_integer(
			residue_configuration.classbook as u32,
			bitpacked_integer_width!(8)
		)?;

		// Residue cascade (codebooks mask)
		for books in &residue_configuration.books {
			let mut mask = 0;
			for (i, book) in books.iter().enumerate() {
				mask |= (book.is_some() as u32) << i;
			}
			let high_mask_bits = (mask & 0xF8) >> 3;
			// The Vorbis I spec allows skipping writing 5 bits if they are all zeroes
			let any_high_mask_bit_set = high_mask_bits != 0;

			bitpacker.write_unsigned_integer(mask, bitpacked_integer_width!(3))?;
			bitpacker.write_flag(any_high_mask_bit_set)?;
			if any_high_mask_bit_set {
				bitpacker.write_unsigned_integer(high_mask_bits, bitpacked_integer_width!(5))?;
			}
		}

		// Residue vector codebooks
		for books in &residue_configuration.books {
			for book in books.iter().filter_map(|book| *book) {
				bitpacker.write_unsigned_integer(book as u32, bitpacked_integer_width!(8))?;
			}
		}
	}

	Ok(())
}

/// Writes all the mapping configurations as dictated by the Vorbis stream format.
fn write_mapping_configurations<W: Write>(
	codec_setup: &VorbisSetupData,
	bitpacker: &mut BitpackWriter<W>
) -> Result<(), io::Error> {
	// Mapping configuration count
	bitpacker.write_unsigned_integer(
		codec_setup.mapping_configurations.len() as u32 - 1,
		bitpacked_integer_width!(6)
	)?;

	for mapping_configuration in &codec_setup.mapping_configurations {
		// Mapping type. Always 0
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(16))?;

		// Submap count. The spec allows writing less bits if it's 1 (which is written to
		// the stream as 0)
		let mapping_submap_count =
			mapping_configuration.floor_and_residue_mappings.len() as u32 - 1;
		let mapping_submap_count_is_nonzero = mapping_submap_count != 0;
		bitpacker.write_flag(mapping_submap_count_is_nonzero)?;
		if mapping_submap_count_is_nonzero {
			bitpacker.write_unsigned_integer(mapping_submap_count, bitpacked_integer_width!(4))?;
		}

		// Channel mappings
		let channel_mappings_used = !mapping_configuration.channel_mappings.is_empty();
		bitpacker.write_flag(channel_mappings_used)?;
		if channel_mappings_used {
			// Coupling step count
			bitpacker.write_unsigned_integer(
				mapping_configuration.channel_mappings.len() as u32 - 1,
				bitpacked_integer_width!(8)
			)?;

			let audio_channels = mapping_configuration.mapping_mux.len();
			// The width is guaranteed to be valid by construction
			let channel_index_width =
				BitpackedIntegerWidth::new(ilog(audio_channels as i32 - 1)).unwrap();

			for coupling_step in &mapping_configuration.channel_mappings {
				bitpacker.write_unsigned_integer(
					coupling_step.magnitude_channel as u32,
					channel_index_width
				)?;
				bitpacker.write_unsigned_integer(
					coupling_step.angle_channel as u32,
					channel_index_width
				)?;
			}
		}

		// Reserved field. Must be set to zero
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(2))?;

		// Multiplexing submaps
		if mapping_submap_count > 0 {
			for mux_submap in &mapping_configuration.mapping_mux {
				bitpacker
					.write_unsigned_integer(*mux_submap as u32, bitpacked_integer_width!(4))?;
			}
		}

		// Floor and residue mappings
		for floor_and_residue_map in &mapping_configuration.floor_and_residue_mappings {
			// Unused time configuration. Any value goes according to the spec, but 0 would
			// point to a valid placeholder index and tends to be more compressible
			bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(8))?;

			bitpacker.write_unsigned_integer(
				floor_and_residue_map.floor_number as u32,
				bitpacked_integer_width!(8)
			)?;
			bitpacker.write_unsigned_integer(
				floor_and_residue_map.residue_number as u32,
				bitpacked_integer_width!(8)
			)?;
		}
	}

	Ok(())
}

/// Writes all the audio packet modes as dictated by the Vorbis stream format.
fn write_modes<W: Write>(
	codec_setup: &VorbisSetupData,
	bitpacker: &mut BitpackWriter<W>
) -> Result<(), io::Error> {
	// Mode count
	bitpacker.write_unsigned_integer(
		codec_setup.modes.len() as u32 - 1,
		bitpacked_integer_width!(6)
	)?;

	// TODO unused mode removal. Most audio files use every mode, and there tend to be few modes, but
	// it's possible to find reasonable counterexamples. This would cascade to removing mappings that
	// were only referred by deleted modes, floors and residues that were only referred by deleted
	// mappings, and codebooks that were only referred by deleted floors and residues. It would be
	// necessary to map mode, mapping, floor, residue and codebook numbers accordingly.
	//
	// TODO also remove unused codec configuration elements even if the removal is not cascaded by mode
	// removal. Only broken or adversarial encoders would generate such setup headers, however, so in
	// practice it does not matter
	for mode in &codec_setup.modes {
		bitpacker.write_flag(mode.big_block)?;

		// Window and time transform types. They must be 0
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(16))?;
		bitpacker.write_unsigned_integer(0, bitpacked_integer_width!(16))?;
		bitpacker
			.write_unsigned_integer(mode.mapping_number as u32, bitpacked_integer_width!(8))?;
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::{CodebookTruncation, codebook_truncation, lookup1_values};
	use crate::vorbis::VectorLookupType;

	#[test]
	fn codebook_truncation_drops_trailing_unused_entries() {
		assert_eq!(
			codebook_truncation(
				4,
				2,
				VectorLookupType::ExplicitlyPopulated,
				&[1, 2, 0, 0],
				0
			),
			CodebookTruncation {
				entry_count: 2,
				multiplicand_count: 4
			}
		);
	}

	#[test]
	fn codebook_truncation_keeps_interleaved_unused_entries() {
		assert_eq!(
			codebook_truncation(5, 2, VectorLookupType::NoLookup, &[1, 0, 2, 0, 0], 0),
			CodebookTruncation {
				entry_count: 3,
				multiplicand_count: 0
			}
		);
	}

	#[test]
	fn codebook_truncation_keeps_all_unused_codebooks() {
		assert_eq!(
			codebook_truncation(3, 2, VectorLookupType::NoLookup, &[0, 0, 0], 0),
			CodebookTruncation {
				entry_count: 3,
				multiplicand_count: 0
			}
		);
	}

	#[test]
	fn codebook_truncation_preserves_residue_classbook_minimum() {
		assert_eq!(
			codebook_truncation(4, 2, VectorLookupType::NoLookup, &[1, 0, 0, 0], 3),
			CodebookTruncation {
				entry_count: 3,
				multiplicand_count: 0
			}
		);
	}

	#[test]
	fn codebook_truncation_preserves_type1_lookup_value_count() {
		let mut codeword_lengths = [0; 100];
		codeword_lengths[0] = 1;

		let truncation = codebook_truncation(
			100,
			5,
			VectorLookupType::ImplicitlyPopulated,
			&codeword_lengths,
			0
		);

		assert_eq!(
			truncation,
			CodebookTruncation {
				entry_count: 32,
				multiplicand_count: 2
			}
		);
	}

	#[test]
	fn type1_bucket_floor_preserves_lookup_value_count() {
		const MAX_CODEBOOK_ENTRIES: u32 = 0xFF_FFFF;

		for dimensions in 1..=u16::MAX {
			for target in 1..=lookup1_values(MAX_CODEBOOK_ENTRIES, dimensions) {
				let bucket_floor = target
					.saturating_pow(dimensions as u32)
					.min(MAX_CODEBOOK_ENTRIES);

				assert_eq!(
					lookup1_values(bucket_floor, dimensions),
					target,
					"lookup1_values bucket floor {bucket_floor} for target {target}, \
					dimensions {dimensions} decodes as {}",
					lookup1_values(bucket_floor, dimensions)
				);
			}
		}
	}
}
