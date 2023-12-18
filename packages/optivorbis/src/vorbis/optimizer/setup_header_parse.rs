//! Contains the supporting code for the [`VorbisSetupData`] Vorbis optimizer state.

use std::{cmp, io::Read, mem};

use indexmap::IndexSet;
use log::{debug, info, trace};
use vorbis_bitpack::BitpackReader;

use super::{
	common_header_validation, ilog, AudioPacketAnalyze, VorbisCommentData,
	VorbisIdentificationHeaderData, VorbisOptimizerError
};
use crate::vorbis::{codebook::VorbisCodebook, PacketType, ResidueType, VectorLookupType};

/// A mutable reference to an immutable byte slice, used in [`parse_codebook_configurations`]
/// to instantiate a bitpack reader without any cursor seek position tracking overhead.
/// The caller manages the lifetime of the mutable reference, and thus of the bitpack reader.
type PacketSlice<'pref, 'packet> = &'pref mut &'packet [u8];

/// Contains the whole codec setup information. The Figure 1 in the Vorbis I
/// specification, § 1.2, graphically represents this information and how
/// its components are related to each other.
#[derive(Default)]
pub(super) struct VorbisSetupData {
	pub(super) codebook_configurations: Vec<CodebookConfiguration>,
	/// Contains [1, 64] elements (length stored in offset-1 6-bit integer).
	pub(super) floor_configurations: Vec<Floor1Configuration>,
	/// Contains [1, 64] elements (length stored in offset-1 6-bit integer).
	pub(super) residue_configurations: Vec<ResidueConfiguration>,
	/// Contains [1, 64] elements (length stored in offset-1 6-bit integer).
	pub(super) mapping_configurations: Vec<MappingConfiguration>,
	/// Contains [1, 64] elements (length stored in offset-1 6-bit integer).
	pub(super) modes: Vec<Mode>
}

/// The Vorbis optimizer state reached when decoding a setup header. After
/// decoding the setup header, a state transition is made to analyze audio
/// packets.
pub(super) struct SetupHeaderParse {
	pub(super) comment_data: VorbisCommentData
}

/// A channel mapping configuration, used for coupling.
pub(super) struct ChannelMapping {
	pub(super) magnitude_channel: u8,
	pub(super) angle_channel: u8
}

/// A pair of floor and residue configuration indexes, used in [audio packet
/// mode mappings](MappingConfiguration).
pub(super) struct FloorAndResidueMapping {
	pub(super) floor_number: u8,
	pub(super) residue_number: u8
}

/// A codebook configuration, used for entropy coding and vector quantization
/// of spectrum floor and residue signal components.
pub(super) struct CodebookConfiguration {
	pub(super) codebook: VorbisCodebook,
	/// The value is always in [0, 2^24 - 1] (stored in 24-bit integer).
	pub(super) entry_count: u32,
	/// Number of elements per vector this codebook would yield in a VQ context.
	pub(super) dimensions: u16,
	pub(super) vector_lookup_type: VectorLookupType,
	/// Called `codebook_minimum_value` in the specification.
	pub(super) codebook_vector_minimum_value: f64,
	/// Called `codebook_delta_value` in the specification.
	pub(super) codebook_vector_delta_value: f64,
	/// Called `codebook_multiplicands` in the specification.
	pub(super) codebook_vector_multiplicands: Vec<u16>,
	/// Called `codebook_value_bits` in the specification.
	pub(super) codebook_vector_value_bits: u8,
	/// Called `codebook_sequence_p` in the specification.
	pub(super) codebook_vector_sequence_flag: bool
}

/// A configuration for a type 1 floor encoding.
pub(super) struct Floor1Configuration {
	pub(super) multiplier: u8,
	pub(super) range_bits: u8,
	/// At most 31 4-bit integers each.
	pub(super) partition_class_list: Vec<u8>,
	/// At most 15 3-bit offset-1 integers.
	pub(super) class_dimensions: Vec<u8>,
	/// At most 15 2-bit integers.
	pub(super) class_subclasses: Vec<u8>,
	/// At most 15 8-bit integers. `None` means no codebook for the class.
	pub(super) class_masterbooks: Vec<Option<u8>>,
	/// At most 15 elements. Each element at most has 8 elements. Each codebook
	/// index is 8 bits. `None` means no codebook for the subclass.
	pub(super) subclass_books: Vec<Vec<Option<u8>>>,
	/// At most 65 point X-coordinates, each [0, 15]-bit integers.
	pub(super) x_list: Vec<u16>
}

/// A configuration for a residue encoding.
pub(super) struct ResidueConfiguration {
	pub(super) residue_type: ResidueType,
	/// 24-bit integer.
	pub(super) begin: u32,
	/// 24-bit integer.
	pub(super) end: u32,
	/// A 25-bit integer, stored as an offset-1 24-bit integer.
	pub(super) partition_size: u32,
	/// A 7-bit integer, stored as an offset-1 6-bit integer.
	pub(super) classifications: u8,
	pub(super) classbook: u8,
	pub(super) books: Vec<[Option<u8>; 8]>
}

/// A mapping configuration that relates audio channels with their
/// coupling, floor and residue configurations.
pub(super) struct MappingConfiguration {
	/// Contains [1, 256] elements (length stored in offset-1 8-bit integer).
	pub(super) channel_mappings: Vec<ChannelMapping>,
	/// Contains as many elements as audio channels.
	pub(super) mapping_mux: Vec<u8>,
	/// Contains [1, 16] elements (length stored in offset-1 4-bit integer).
	pub(super) floor_and_residue_mappings: Vec<FloorAndResidueMapping>
}

/// An audio packet mode, which brings together the rest of the codec
/// setup configuration to decode actual audio data.
pub(super) struct Mode {
	/// Called `vorbis_mode_blockflag` in the specification.
	pub(super) big_block: bool,
	/// Called `vorbis_mode_mapping` in the specification.
	pub(super) mapping_number: u8
}

impl SetupHeaderParse {
	pub(super) fn analyze_packet(
		&mut self,
		packet: &[u8],
		identification_data: &VorbisIdentificationHeaderData
	) -> Result<(Option<u16>, Option<AudioPacketAnalyze>), VorbisOptimizerError> {
		trace!("Decoding setup header Vorbis packet");

		let mut setup_header = common_header_validation(packet, PacketType::SetupHeader)?;

		// Vorbis I spec, § 4.2.4, step 1: read codebook configurations
		let (codebook_configurations, mut bitpacker, header_length) =
			parse_codebook_configurations(&mut setup_header)?;

		// Vorbis I spec, § 4.2.4, step 2: discard time domain transforms, which are placeholders
		// in Vorbis I, to keep sync.
		// The specification mandates that they are zero, but we can mostly ignore that
		let time_domain_transform_count =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
		info!(
			"Time domain transforms (unused): {}",
			time_domain_transform_count
		);

		for _ in 0..time_domain_transform_count {
			let transform = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;
			if cfg!(debug_assertions) && transform != 0 {
				return Err(VorbisOptimizerError::InvalidPattern);
			}
		}

		// Vorbis I spec, § 4.2.4, step 3: now read the floor configurations that may be used to
		// encode Vorbis audio frames and encoded with codebooks
		let floor_configurations = parse_floor1_configurations(
			&mut bitpacker,
			header_length,
			codebook_configurations.len()
		)?;

		// Vorbis I spec, § 4.2.4, step 4: decode residue vectors that may be used to encode Vorbis audio frames
		let residue_configurations =
			parse_residue_configurations(&mut bitpacker, header_length, &codebook_configurations)?;

		// Vorbis I spec, § 4.2.4, step 5: decode audio packet mode -> (floors, residues) mappings
		let mapping_configurations = parse_mapping_configurations(
			&mut bitpacker,
			header_length,
			identification_data.channels.get(),
			floor_configurations.len(),
			residue_configurations.len()
		)?;

		// Vorbis I spec, 4.2.4, step 6: finally, read the audio packet modes in use
		let modes = parse_modes(&mut bitpacker, header_length, mapping_configurations.len())?;

		Ok((
			None,
			Some(AudioPacketAnalyze {
				comment_data: mem::take(&mut self.comment_data),
				codec_setup: VorbisSetupData {
					residue_configurations,
					codebook_configurations,
					floor_configurations,
					mapping_configurations,
					modes
				}
			})
		))
	}
}

/// Parses the codebook configurations contained in the Vorbis setup header as described in
/// the Vorbis I specification, § 3.2.1. Because this is the first thing parsed in a setup
/// header, the [`BitpackReader`] and setup header packet size are also returned.
fn parse_codebook_configurations<'pref, 'packet>(
	setup_header: PacketSlice<'pref, 'packet>
) -> Result<
	(
		Vec<CodebookConfiguration>,
		BitpackReader<PacketSlice<'pref, 'packet>>,
		usize
	),
	VorbisOptimizerError
> {
	// A valid setup header must contain a codebook count and more things
	let header_length = setup_header.len() + 7; // + 7 to take into account prelude
	if setup_header.is_empty() {
		return Err(VorbisOptimizerError::TooSmallPacket(header_length));
	}

	// The specification is not very clear on how to handle overflow in this sum, or
	// how much precision bits are required to perform this operation.
	//
	// We interpret that it is meant to indicate that the count of codebooks should be
	// numerically one unit larger than before, and that even though we read a 8 bit
	// integer from the stream, the precision bits used to do this computation can be
	// greater than 8.
	//
	// This interpretation is supported by what stb_vorbis does, which stores this count
	// in a C int. Overflowing to zero allows no codebooks, which would be a more
	// space-efficient encoding for degenerate Vorbis streams with no audio packets.
	//
	// In any case, the maximum value for codebook_count is 256, so 0-based codebook
	// numbers are in range [0, 255] and fit in an 8-bit unsigned integer
	let codebook_count = setup_header
		.split_first()
		.map(|(codebook_count, setup_header_tail)| {
			*setup_header = setup_header_tail;
			*codebook_count as u16 + 1
		})
		.unwrap();
	info!("Codebook count: {}", codebook_count);

	// From now on reads must be done via the Vorbis bitpack format.
	// Skip the consumed codebook count
	let mut bitpacker = BitpackReader::new(setup_header);

	let mut codebook_configurations = Vec::with_capacity(codebook_count as usize);
	for i in 0..codebook_count {
		// For development, it's good to check the sync pattern, which definitely marks
		// this as a packed codebook, to catch logic errors. For release, however, we can
		// ignore it to increase our stream repair capabilities
		let sync_pattern =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 24, u32)?;
		if cfg!(debug_assertions) && sync_pattern != 0x564342 {
			return Err(VorbisOptimizerError::InvalidPattern);
		}

		let codebook_dimensions =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;

		let codebook_entries =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 24, u32)?;
		let codebook_entries_usize = codebook_entries.try_into()?;

		let ordered = bitpack_packet_read!(bitpacker, read_flag, header_length)?;

		debug!(
			"Codebook {}: {} dimensions, {} entries, ordered: {}",
			i, codebook_dimensions, codebook_entries, ordered
		);

		let mut codeword_lengths = vec![0; codebook_entries_usize];

		if ordered {
			// Codewords are ordered in ascending length, and the number of codewords
			// per length is read. Due to how the edges of the Huffman tree would be
			// traversed to compute codeword values, this would match a canonical
			// Huffman code (codewords would have the numerical sequence property),
			// barring any optimality considerations

			let mut start_entry = 0;
			let mut codeword_length = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 5, u8)?
				+ 1;

			// Using a while-loop to check this invariant is more idiomatic and results in
			// the same behavior as the specification algorithm for empty codebooks (in such
			// degenerate case, entries_with_this_cw_length would read zero)
			while start_entry < codebook_entries_usize {
				// Fuzzing discovered this edge case. Protect ourselves against too much
				// looping and codewords so long that do not hold our invariants
				if codeword_length > 32 {
					return Err(VorbisOptimizerError::TooBigCodewordLength);
				}

				let entries_with_this_cw_length = usize::try_from(bitpack_packet_read!(
					bitpacker,
					read_unsigned_integer,
					header_length,
					// This "as" numeric cast is guaranteed to always work, because
					// start_entry < codebook_entries, and codebook_entries < 2^24
					mut ilog((codebook_entries_usize - start_entry) as i32),
					u32
				)?)?;

				let next_start_entry = start_entry + entries_with_this_cw_length;
				if next_start_entry > codebook_entries_usize {
					return Err(VorbisOptimizerError::InvalidSetupValue);
				}

				codeword_lengths[start_entry..next_start_entry].fill(codeword_length);

				start_entry = next_start_entry;
				codeword_length += 1;
			}
		} else {
			// Codewords are not necessarily ordered, so this codebook may not represent
			// a canonical Huffman code, even if we try to sort the codewords by length,
			// due to the codeword assignation algorithm described in the specification.
			// Maybe the Vorbis authors didn't want to impose the additional "sort and map"
			// overhead computing the canonical code would require, or it didn't play that
			// nice with VQ or sparse codebooks.
			//
			// In general, this defines a prefix code that is assigned codewords using a
			// Huffman tree, from left to right. We must not assume any properties other
			// than it is a prefix code and that codewords of a given length are
			// lexicographically sorted, but not necessarily consecutive

			// Sparse codebooks may have unused entries. Unused entries are ignored in the
			// codeword assignment process and do not appear in the stream
			let sparse = bitpack_packet_read!(bitpacker, read_flag, header_length)?;
			trace!("Codebook {} is sparse: {}", i, sparse);

			for codeword_length in codeword_lengths.iter_mut() {
				// Non-sparse codebooks always read the codeword length from the stream.
				// For sparse codebooks, we read the "used" flag for this entry, which
				// is either unset (0) or set (1). Unused entries are marked by having
				// a codeword length of 0 and are not read from the stream
				if !sparse || bitpack_packet_read!(bitpacker, read_flag, header_length)? {
					*codeword_length = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 5, u8)?
						+ 1;
				}
			}
		}

		// Proceed with codebook vector lookup decode
		let lookup_type = VectorLookupType::try_from(
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)?
		)?;
		let codebook_vector_minimum_value;
		let codebook_vector_delta_value;
		let codebook_vector_value_bits;
		let codebook_vector_sequence_flag;
		let mut codebook_vector_multiplicands;

		match lookup_type {
			VectorLookupType::NoLookup => {
				// This codebook is not used for vector lookup. Skip
				debug!("Codebook {} is not used for vector decoding", i);
				codebook_vector_minimum_value = 0.0;
				codebook_vector_delta_value = 0.0;
				codebook_vector_value_bits = 0;
				codebook_vector_sequence_flag = false;
				codebook_vector_multiplicands = Vec::new();
			}
			_ => {
				// A zero-dimension codebook would not make sense for VQ and vector
				// lookup in residue decode later, but don't error out yet to
				// ignore that if the codebook is not used for that purpose by any
				// audio packet

				codebook_vector_minimum_value =
					bitpack_packet_read!(bitpacker, read_float32, header_length)?;
				codebook_vector_delta_value =
					bitpack_packet_read!(bitpacker, read_float32, header_length)?;
				codebook_vector_value_bits = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)?
					+ 1;
				codebook_vector_sequence_flag =
					bitpack_packet_read!(bitpacker, read_flag, header_length)?;

				let codebook_lookup_value_count =
					if lookup_type == VectorLookupType::ImplicitlyPopulated {
						lookup1_values(codebook_entries, codebook_dimensions) as u64
					} else {
						// A 24-bit number multiplied by a 16-bit number is guaranteed
						// to fit in a 40-bit number, so make sure we use 64-bit integers
						codebook_entries as u64 * codebook_dimensions as u64
					};

				debug!(
					"Codebook {} vector lookup: type {} ({:?}), minimum value {}, delta value {}, \
						value bits {}, value count {}",
					i,
					lookup_type as u8,
					lookup_type,
					codebook_vector_minimum_value,
					codebook_vector_delta_value,
					codebook_vector_value_bits,
					codebook_lookup_value_count
				);

				// We don't need to actually do vector quantization, so any VQ-related data we
				// are reading here is useless for us. But we should store it for copying it later.
				// Fuzzing revealed that it is relatively easy for a crafted small file to make us
				// allocate too much memory here. 4096 entries of 16 dimensions yields 65536 values
				// here, and those high counts are not seen in valid Ogg Vorbis files in the wild.
				// So just allocate space for 65535 multiplicands and let the vector grow if it
				// really needs to
				codebook_vector_multiplicands =
					Vec::with_capacity(cmp::min(codebook_lookup_value_count.try_into()?, 65535));
				for _ in 0..codebook_lookup_value_count {
					codebook_vector_multiplicands.push(bitpack_packet_read!(
						bitpacker,
						read_unsigned_integer,
						header_length,
						mut codebook_vector_value_bits,
						u16
					)?);
				}
			}
		}

		codebook_configurations.push(CodebookConfiguration {
			codebook: VorbisCodebook::new(i, &codeword_lengths)?,
			entry_count: codebook_entries,
			vector_lookup_type: lookup_type,
			codebook_vector_minimum_value,
			codebook_vector_delta_value,
			codebook_vector_multiplicands,
			codebook_vector_value_bits,
			codebook_vector_sequence_flag,
			dimensions: codebook_dimensions
		});
	}

	Ok((codebook_configurations, bitpacker, header_length))
}

/// Parses the floor configurations contained in the Vorbis setup header as described in
/// the Vorbis I specification, § 4.2.4 and § 7.2.2.
fn parse_floor1_configurations<R: Read>(
	bitpacker: &mut BitpackReader<R>,
	header_length: usize,
	codebook_count: usize
) -> Result<Vec<Floor1Configuration>, VorbisOptimizerError> {
	let floor_count =
		bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
	info!("Floor configurations count: {}", floor_count);

	let mut floor_configurations = Vec::with_capacity(floor_count as usize);
	for i in 0..floor_count {
		let floor_type =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;

		// Floor type 0 is described in the specification, but almost no encoder uses it
		// in practice. Therefore, limit ourselves to floor type 1
		if floor_type != 1 {
			return Err(VorbisOptimizerError::UnsupportedFloorType(floor_type));
		}

		let partitions =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 5, u8)?;

		let mut partition_class_list = Vec::with_capacity(partitions as usize);
		let mut maximum_class = -1;
		for _ in 0..partitions {
			let class =
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)?;
			partition_class_list.push(class);

			maximum_class = cmp::max(class as i8, maximum_class);
		}

		debug!(
			"Floor {}: type {}, {} partitions, {} classes",
			i,
			floor_type,
			partitions,
			maximum_class + 1
		);

		// This will not do any iterations in the event there are no classes
		let mut class_dimensions = Vec::with_capacity((maximum_class + 1) as usize);
		let mut class_subclasses = Vec::with_capacity((maximum_class + 1) as usize);
		let mut class_masterbooks = Vec::with_capacity((maximum_class + 1) as usize);
		let mut subclass_books = Vec::with_capacity((maximum_class + 1) as usize * 8);
		let mut maximum_class_dimension = 1;
		for _ in 0..=maximum_class {
			let class_dimension = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 3, u8)?
				+ 1;
			class_dimensions.push(class_dimension);
			maximum_class_dimension = cmp::max(class_dimension, maximum_class_dimension);

			let current_subclass =
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 2, u8)?;
			class_subclasses.push(current_subclass);

			if current_subclass != 0 {
				let codebook_number = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;

				// The codebook must exist
				if codebook_number as usize >= codebook_count {
					return Err(VorbisOptimizerError::InvalidCodebookNumber(codebook_number));
				}

				debug!(
					"Floor {}, subclass {} codebook: {}",
					i, current_subclass, codebook_number
				);

				class_masterbooks.push(Some(codebook_number));
			} else {
				// This subclass does not have a codebook. It'd be an error to decode a packet
				// with this subclass' codebook later
				debug!("Floor {}, subclass {} has no codebook", i, current_subclass);

				class_masterbooks.push(None);
			}

			let current_subclass_books_count = 1 << current_subclass;
			let mut current_subclass_books = Vec::with_capacity(current_subclass_books_count);
			for _ in 0..current_subclass_books_count {
				// The codebook number 0 - 1 = -1 may be encoded on the stream. This is used to set
				// floor values to zero during packet decode later
				let codebook_number =
					bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?
						.checked_sub(1);

				// If we succeed finding an invalid codebook number return error
				if let Some(codebook_number) = codebook_number
					.iter()
					.filter(|n| **n as usize >= codebook_count)
					.last()
				{
					return Err(VorbisOptimizerError::InvalidCodebookNumber(
						*codebook_number
					));
				}

				debug!(
					"Floor {} vector partition codebook: {:?}",
					i, codebook_number
				);

				current_subclass_books.push(codebook_number);
			}

			subclass_books.push(current_subclass_books);
		}

		// Read data necessary to synthesize the floor curve. We don't care about most
		// of it, as we don't need to synthesize actual audio frames, so just store the
		// minimum we need for optimization
		let multiplier =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 2, u8)? + 1;
		let range_bits =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)?;

		let mut x_list =
			IndexSet::with_capacity(partition_class_list.len() * maximum_class_dimension as usize);
		for current_class in partition_class_list.iter().copied().map(|c| c as usize) {
			for _ in 0..class_dimensions[current_class] {
				if !x_list.insert(bitpack_packet_read!(
					bitpacker,
					read_unsigned_integer,
					header_length,
					mut range_bits,
					u16
				)?) {
					// The specification does not allow repeated values
					return Err(VorbisOptimizerError::RepeatedFloor1Point(i));
				}
			}
		}

		// Limit imposed by the specification
		if x_list.len() > 65 {
			return Err(VorbisOptimizerError::TooManyFloor1Points(i));
		}

		floor_configurations.push(Floor1Configuration {
			multiplier,
			range_bits,
			partition_class_list,
			class_dimensions,
			class_subclasses,
			class_masterbooks,
			subclass_books,
			x_list: x_list.into_iter().collect()
		});
	}

	Ok(floor_configurations)
}

/// Parses the residue configurations contained in the Vorbis setup header as described in
/// the Vorbis I specification, § 4.2.4 and § 8.6.1.
fn parse_residue_configurations<R: Read>(
	bitpacker: &mut BitpackReader<R>,
	header_length: usize,
	codebook_configurations: &[CodebookConfiguration]
) -> Result<Vec<ResidueConfiguration>, VorbisOptimizerError> {
	let residue_count =
		bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
	info!("Residue count: {}", residue_count);

	let mut residue_configurations = Vec::with_capacity(residue_count as usize);
	for i in 0..residue_count {
		let residue_type = ResidueType::try_from(
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?
		)?;
		let residue_begin =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 24, u32)?;
		// Fuzzing discovered that it can happen that residue_begin > residue_end, which would provoke
		// a subtraction with overflow when computing the residue vector size later. There's no right
		// thing to do here: the specification does not say how to handle this case, so interoperability
		// is at stake. Try to fix the situation by using a zero-length residue, which should give
		// an usable result.
		// Upstream Vorbis issue: https://github.com/xiph/vorbis/issues/87
		let residue_end = cmp::max(
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 24, u32)?,
			residue_begin
		);
		let residue_partition_size = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 24, u32)?
			+ 1;
		let residue_classifications =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
		let residue_classbook =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;

		debug!(
			"Residue {}: type {} ({:?}), begin {}, end {}, partition size {}, classifications {}, classbook {}",
			i,
			residue_type as u8,
			residue_type,
			residue_begin,
			residue_end,
			residue_partition_size,
			residue_classifications,
			residue_classbook
		);

		// Check that the referenced codebook exists, and has enough entries
		#[allow(clippy::blocks_in_conditions)]
		if codebook_configurations
			.get(residue_classbook as usize)
			.filter(|classbook_configuration| {
				let minimum_entry_count = (residue_classifications as u32)
					.saturating_pow(classbook_configuration.dimensions as u32);

				minimum_entry_count <= classbook_configuration.entry_count
			})
			.is_none()
		{
			return Err(VorbisOptimizerError::InvalidCodebookNumber(
				residue_classbook
			));
		}

		// Read bitmap of partition classes and passes
		let mut residue_cascade = Vec::with_capacity(residue_classifications as usize);
		for _ in 0..residue_classifications {
			let low_bits =
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 3, u8)?;
			let high_bits = if bitpack_packet_read!(bitpacker, read_flag, header_length)? {
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 5, u8)?
			} else {
				0
			};

			residue_cascade.push((high_bits << 3) | low_bits);
		}

		// Read codebooks to be used to decode a classification in a pass
		let mut residue_books = Vec::with_capacity(residue_classifications as usize);
		for (i, residue_cascade) in residue_cascade.iter().copied().enumerate() {
			residue_books.push([None; 8]);

			for j in 0..8 {
				if residue_cascade & (1 << j) != 0 {
					let residue_book = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;

					// Check that the codebook exists. We skip checking whether it can be
					// used for VQ because we do that later on audio packet decode, when the
					// codebook is actually used for that
					if codebook_configurations.get(residue_book as usize).is_none() {
						return Err(VorbisOptimizerError::InvalidCodebookNumber(residue_book));
					}

					trace!(
						"Read residue book {} for classification {}",
						residue_book,
						i
					);

					residue_books[i][j] = Some(residue_book);
				} else {
					// Unused. It's an error to try to decode this classification
				}
			}
		}

		residue_configurations.push(ResidueConfiguration {
			residue_type,
			begin: residue_begin,
			end: residue_end,
			partition_size: residue_partition_size,
			classifications: residue_classifications,
			classbook: residue_classbook,
			books: residue_books
		});
	}

	Ok(residue_configurations)
}

/// Parses the mapping configurations contained in the Vorbis setup header as described in
/// the Vorbis I specification, § 4.2.4.
fn parse_mapping_configurations<R: Read>(
	bitpacker: &mut BitpackReader<R>,
	header_length: usize,
	audio_channels: u8,
	floor_count: usize,
	residue_count: usize
) -> Result<Vec<MappingConfiguration>, VorbisOptimizerError> {
	let mapping_count =
		bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
	info!("Mapping count: {}", mapping_count);

	let mut mapping_configurations = Vec::with_capacity(mapping_count as usize);
	for i in 0..mapping_count {
		let mapping_type =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;

		// Only mapping type 0 is defined in the specification. Checking this is important because
		// the specification says at § 4.3.9 that "applications using Vorbis for dedicated purposes
		// may define channel mapping as seen fit", so interoperability is at stake here
		if mapping_type != 0 {
			return Err(VorbisOptimizerError::ReservedMappingType(mapping_type));
		}

		// This variable is called vorbis_mapping_submaps in the specification
		let mapping_submap_count = if bitpack_packet_read!(bitpacker, read_flag, header_length)? {
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)? + 1
		} else {
			1
		};

		debug!(
			"Mapping {} floor and residue submap count: {}",
			i, mapping_submap_count
		);

		// Read square polar channel mappings, if they are used
		let mut channel_mappings;
		if bitpack_packet_read!(bitpacker, read_flag, header_length)? {
			let mapping_coupling_steps = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u16)?
				+ 1;
			let mapping_component_bits = ilog(audio_channels as i32 - 1); // At most 8: ilog(254) = 8

			channel_mappings = Vec::with_capacity(mapping_coupling_steps as usize);

			for j in 0..mapping_coupling_steps {
				let magnitude_channel = bitpack_packet_read!(
					bitpacker,
					read_unsigned_integer,
					header_length,
					mut mapping_component_bits,
					u8
				)?;
				let angle_channel = bitpack_packet_read!(
					bitpacker,
					read_unsigned_integer,
					header_length,
					mut mapping_component_bits,
					u8
				)?;

				if magnitude_channel == angle_channel
					|| magnitude_channel >= audio_channels
					|| angle_channel >= audio_channels
				{
					return Err(VorbisOptimizerError::InvalidChannelMapping {
						magnitude_channel,
						angle_channel,
						audio_channels
					});
				}

				trace!(
					"Mapping {}, channel mapping {}: \
						magnitude channel {}, angle channel {}",
					i,
					j,
					magnitude_channel,
					angle_channel
				);

				channel_mappings.push(ChannelMapping {
					magnitude_channel,
					angle_channel
				});
			}
		} else {
			trace!("Not using channel mappings for mapping {}", i);

			channel_mappings = vec![];
		}

		// Discard reserved field. The specification mandates that it is nonzero,
		// so when debugging it's a great opportunity to check we're keeping stream
		// sync
		let reserved_field =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 2, u8)?;
		if cfg!(debug_assertions) && reserved_field != 0 {
			return Err(VorbisOptimizerError::InvalidPattern);
		}

		// Read channel multiplexing settings
		let mut mapping_mux;
		if mapping_submap_count > 1 {
			mapping_mux = Vec::with_capacity(audio_channels as usize);

			for _ in 0..audio_channels {
				let mux_submap = bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 4, u8)?;

				if mux_submap >= mapping_submap_count {
					return Err(VorbisOptimizerError::InvalidChannelMultiplexing {
						mux_submap,
						mapping_submap_count
					});
				}

				mapping_mux.push(mux_submap);
			}
		} else {
			// The specification does not say how to handle populating mapping_mux when there is
			// only a single submap, as in that case it is not read from the stream. What makes
			// more sense, and stb_vorbis does, is considering that mapping_mux would be full
			// of zeroes (i.e., each channel points to the only possible submap)
			mapping_mux = vec![0; audio_channels as usize];
		}

		trace!(
			"Read mapping {} channel floor and residue submapping settings: {:?}",
			i,
			mapping_mux
		);

		// Read the floor and residue used to decode submaps
		let mut floor_and_residue_mappings = Vec::with_capacity(mapping_submap_count as usize);
		for j in 0..mapping_submap_count {
			// Discard unused time configuration placeholder. Surprisingly, the spec does not
			// require validating it in any way
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;

			let floor_number =
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;
			if floor_number as usize >= floor_count {
				return Err(VorbisOptimizerError::InvalidFloorNumber(floor_number));
			}

			let residue_number =
				bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;
			if residue_number as usize >= residue_count {
				return Err(VorbisOptimizerError::InvalidResidueNumber(residue_number));
			}

			info!(
				"Mapping {}, submap {}: floor {}, residue {}",
				i, j, floor_number, residue_number
			);

			floor_and_residue_mappings.push(FloorAndResidueMapping {
				floor_number,
				residue_number
			});
		}

		mapping_configurations.push(MappingConfiguration {
			channel_mappings,
			mapping_mux,
			floor_and_residue_mappings
		});
	}

	Ok(mapping_configurations)
}

/// Parses the mode configurations contained in the Vorbis setup header as described in
/// the Vorbis I specification, § 4.2.4.
fn parse_modes<R: Read>(
	bitpacker: &mut BitpackReader<R>,
	header_length: usize,
	mapping_count: usize
) -> Result<Vec<Mode>, VorbisOptimizerError> {
	let mode_count =
		bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 6, u8)? + 1;
	info!("Mode count: {}", mode_count);

	let mut modes = Vec::with_capacity(mode_count as usize);
	for i in 0..mode_count {
		let big_block = bitpack_packet_read!(bitpacker, read_flag, header_length)?;

		// Window type. Vorbis I only defines a single window type, so discard.
		// Validate only if debugging
		let window_type =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;
		if cfg!(debug_assertions) && window_type != 0 {
			return Err(VorbisOptimizerError::InvalidPattern);
		}

		// Time transform type. Vorbis I assumes a single time transform type; discard
		// unless debugging
		let time_transform_type =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 16, u16)?;
		if cfg!(debug_assertions) && time_transform_type != 0 {
			return Err(VorbisOptimizerError::InvalidPattern);
		}

		let mapping_number =
			bitpack_packet_read!(bitpacker, read_unsigned_integer, header_length, const 8, u8)?;
		if mapping_number as usize >= mapping_count {
			return Err(VorbisOptimizerError::InvalidMappingNumber(mapping_number));
		}

		debug!(
			"Mode {}: uses big block size: {}, mapping number {}",
			i, big_block, mapping_number
		);

		modes.push(Mode {
			big_block,
			mapping_number
		});
	}

	// In debug builds, check framing bit to aid in detecting logic errors.
	// We can ignore this on release builds to increase repair capabilities
	if cfg!(debug_assertions) && !bitpack_packet_read!(bitpacker, read_flag, header_length)? {
		return Err(VorbisOptimizerError::InvalidPattern);
	}

	Ok(modes)
}

/// The Vorbis I `lookup1_values` function, as defined in section 9.2.3 of the
/// Vorbis I specification. Mathematically, it returns the
/// `codebook_dimensions`-root of `codebook_entries`, rounded down to an integer.
fn lookup1_values(codebook_entries: u32, codebook_dimensions: u16) -> u32 {
	// codebook_entries is at most 2^24 - 1, so it fits in a f32.
	// codebook_dimensions of zero does not make sense for codebooks used for vector
	// lookup, but the specification does not say they're illegal otherwise. Therefore,
	// let's handle that edge case to avoid division by zero
	if codebook_dimensions == 0 {
		u32::MAX
	} else {
		(codebook_entries as f32).powf(1.0 / codebook_dimensions as f32) as u32
	}
}

#[cfg(test)]
mod tests {
	use super::lookup1_values;

	#[test]
	fn lookup1_values_works() {
		assert_eq!(lookup1_values(100, 5), 2);
		assert_eq!(lookup1_values(1, 5), 1);

		assert_eq!(lookup1_values(0, u16::MAX), 0);
		assert_eq!(lookup1_values(0xFFFFFF, 0), u32::MAX);
		assert_eq!(lookup1_values(0xFFFFFF, u16::MAX), 1);
	}
}
