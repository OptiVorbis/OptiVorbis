//! Contains the helper [`process_audio_packet`] function to parse audio packets and
//! execute callbacks when some interesting piece of data is read.

use std::cmp;
use std::io::Read;

use log::trace;
use tinyvec::TinyVec;

use vorbis_bitpack::BitpackReader;

use crate::vorbis::codebook::VorbisCodebook;
use crate::vorbis::optimizer::setup_header_parse::Mode;
use crate::vorbis::{ResidueType, VectorLookupType};

use super::{
	ilog,
	setup_header_parse::{
		CodebookConfiguration, Floor1Configuration, ResidueConfiguration, VorbisSetupData
	},
	VorbisIdentificationHeaderData, VorbisOptimizerError
};

/// Parses the specified audio packet, whose source is already wrapped in a
/// [bitpacker](BitpackReader), and invokes the specified callbacks when a
/// packed integer is read or a codebook entry is decoded, respectively.
/// A successful `false` return value indicates that the packet was parsed
/// without major errors, but that it should be discarded from the stream.
/// A successful `true` return value means that the packet was parsed
/// without major errors and it should be kept in the stream.
///
/// The parsing is done according to the Vorbis I specification, § 4.3.1,
/// step 2, up to § 4.3.4, as synthesizing the actual audio samples is not
/// necessary.
pub(super) fn process_audio_packet<
	R: Read,
	T,
	F: FnMut(u32, u8, &mut T) -> Result<(), VorbisOptimizerError>,
	G: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>
>(
	identification_data: &VorbisIdentificationHeaderData,
	codec_setup: &VorbisSetupData,
	packet_length: usize,
	bitpacker: &mut BitpackReader<R>,
	mut bitpack_read_callback: F,
	codebook_entry_decode_callback: G,
	mut shared_callback_data: T
) -> Result<(bool, Option<u16>), VorbisOptimizerError> {
	// The Vorbis specification says, § 4.3.1, that "an end-of-packet condition up to this point
	// should be considered an error that discards this packet from the stream". However, this
	// wording is confusing, because the reference implementation does not really treat the
	// situation as an error, just discarding the packet. Similarly to what we do in the comment
	// header, warn about this, but do not fail and let callers know that the packet can be removed
	// from the stream
	let (mode_configuration, decode_blocksize) = eval_on_eop!(
		process_audio_packet_first_part(
			identification_data,
			codec_setup,
			packet_length,
			bitpacker,
			&mut bitpack_read_callback,
			&mut shared_callback_data
		),
		{
			trace!("Discarding audio packet due to premature end of packet");
			return Ok((false, None));
		}
	)?;

	// The Vorbis specification says, § 4.3.1, that "an end of packet condition past this point
	// is to be considered a possible nominal occurrence". § 8.6.2 adds that in the last case
	// "decode returns the result of vector decode up to that point", but we actually don't
	// decode vectors. Therefore, the proper thing to do on an unexpected end of packet is to
	// ignore the packet. That is a normal condition for bitrate limited streams, for example
	eval_on_eop!(
		process_audio_packet_second_part(
			identification_data,
			codec_setup,
			packet_length,
			bitpacker,
			mode_configuration,
			decode_blocksize,
			bitpack_read_callback,
			codebook_entry_decode_callback,
			shared_callback_data
		)
		.map(|_| (true, Some(decode_blocksize))),
		Ok((true, Some(decode_blocksize)))
	)
}

/// Implements the step 2 and onwards of the audio packet parsing algorithm
/// described in section § 4.3.1. Any end of packet error while performing
/// these steps should discard the packet from the stream.
fn process_audio_packet_first_part<
	'setup,
	R: Read,
	T,
	F: FnMut(u32, u8, &mut T) -> Result<(), VorbisOptimizerError>
>(
	identification_data: &VorbisIdentificationHeaderData,
	codec_setup: &'setup VorbisSetupData,
	packet_length: usize,
	bitpacker: &mut BitpackReader<R>,
	mut bitpack_read_callback: F,
	shared_callback_data: &mut T
) -> Result<(&'setup Mode, u16), VorbisOptimizerError> {
	// § 4.3.1, step 2 onwards: packet mode and window decode
	let mode_bits = ilog(codec_setup.modes.len() as i32 - 1);
	let mode = bitpack_packet_read!(
		bitpacker,
		read_unsigned_integer,
		packet_length,
		mut mode_bits,
		u8
	)?;

	let mode_configuration = codec_setup
		.modes
		.get(mode as usize)
		.ok_or(VorbisOptimizerError::InvalidModeNumber(mode))?;

	trace!("Audio packet mode {}", mode);
	bitpack_read_callback(mode as u32, mode_bits, shared_callback_data)?;

	let decode_blocksize = if mode_configuration.big_block {
		identification_data.blocksizes.1
	} else {
		identification_data.blocksizes.0
	};

	if mode_configuration.big_block {
		// We don't need to build the window, so discard window flags, after callbacks
		bitpack_read_callback(
			bitpack_packet_read!(bitpacker, read_flag, packet_length)? as u32,
			1,
			shared_callback_data
		)?;
		bitpack_read_callback(
			bitpack_packet_read!(bitpacker, read_flag, packet_length)? as u32,
			1,
			shared_callback_data
		)?;
	}

	Ok((mode_configuration, decode_blocksize))
}

/// Implements the audio packet parsing algorithm described sections § 4.3.2
/// and next ones in the Vorbis specification. Any end of packet error while
/// performing these steps should return the audio data decoded so far.
#[allow(clippy::too_many_arguments)]
fn process_audio_packet_second_part<
	R: Read,
	T,
	F: FnMut(u32, u8, &mut T) -> Result<(), VorbisOptimizerError>,
	G: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>
>(
	identification_data: &VorbisIdentificationHeaderData,
	codec_setup: &VorbisSetupData,
	packet_length: usize,
	bitpacker: &mut BitpackReader<R>,
	mode_configuration: &Mode,
	decode_blocksize: u16,
	mut bitpack_read_callback: F,
	mut codebook_entry_decode_callback: G,
	mut shared_callback_data: T
) -> Result<(), VorbisOptimizerError> {
	// § 4.3.2. Floor curve decode
	let mapping_configuration =
		&codec_setup.mapping_configurations[mode_configuration.mapping_number as usize];

	let mut no_residue =
		TinyVec::<[bool; 8]>::with_capacity(identification_data.channels.get() as usize);

	// By construction, mapping_mux has as many elements as audio channels
	for submap_number in mapping_configuration.mapping_mux.iter().copied() {
		let floor_number =
			mapping_configuration.floor_and_residue_mappings[submap_number as usize].floor_number;
		let floor_configuration = &codec_setup.floor_configurations[floor_number as usize];

		trace!("Processing floor vector, submap {}", submap_number);

		// The specification mandates at § 4.3.2 that end-of-packet while decoding floor data
		// means that the packet should be directly synthesized, with null channel audio data.
		// We don't synthesize audio, so bail out
		let has_audio_energy = process_floor1(
			bitpacker,
			packet_length,
			floor_configuration,
			&codec_setup.codebook_configurations,
			&mut bitpack_read_callback,
			&mut codebook_entry_decode_callback,
			&mut shared_callback_data
		)?;

		no_residue.push(!has_audio_energy);
	}

	// § 4.3.3. Nonzero vector propagate
	for channel_mapping in &mapping_configuration.channel_mappings {
		let magnitude_channel = channel_mapping.magnitude_channel as usize;
		let angle_channel = channel_mapping.angle_channel as usize;

		let propagated_no_residue = no_residue[magnitude_channel] && no_residue[angle_channel];
		no_residue[magnitude_channel] = propagated_no_residue;
		no_residue[angle_channel] = propagated_no_residue;
	}

	// § 4.3.4. Residue decode
	for (i, floor_and_residue_mapping) in mapping_configuration
		.floor_and_residue_mappings
		.iter()
		.enumerate()
	{
		let mut residue_vectors_masks =
			TinyVec::<[bool; 8]>::with_capacity(identification_data.channels.get() as usize);
		let mut no_residue_vector_to_decode = true;

		for (j, mapping_mux) in mapping_configuration
			.mapping_mux
			.iter()
			.copied()
			.enumerate()
		{
			if mapping_mux as usize == i {
				let do_not_decode = no_residue[j];
				residue_vectors_masks.push(do_not_decode);
				no_residue_vector_to_decode &= do_not_decode;
			}
		}

		// Optimization for the case when there are no vectors to decode
		if no_residue_vector_to_decode {
			trace!("All residue vectors are marked do not decode, skipping residue processing");
			continue;
		}

		trace!("Processing residue vectors, submap {}", i);

		process_residue(
			bitpacker,
			&codec_setup.residue_configurations[floor_and_residue_mapping.residue_number as usize],
			&codec_setup.codebook_configurations,
			&residue_vectors_masks,
			decode_blocksize,
			&mut codebook_entry_decode_callback,
			&mut shared_callback_data
		)?;
	}

	Ok(())
}

/// Implements the algorithm described in the Vorbis I specification, § 4.3.2, step 4.
fn process_floor1<
	R: Read,
	T,
	F: FnMut(u32, u8, &mut T) -> Result<(), VorbisOptimizerError>,
	G: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>
>(
	bitpacker: &mut BitpackReader<R>,
	packet_length: usize,
	floor_configuration: &Floor1Configuration,
	codebook_configurations: &[CodebookConfiguration],
	mut bitpack_read_callback: F,
	mut codebook_entry_decode_callback: G,
	shared_callback_data: &mut T
) -> Result<bool, VorbisOptimizerError> {
	// Floor type is always 1 because we reject type 0 on setup decode,
	// so there's no need to check type
	let has_audio_energy = bitpack_packet_read!(bitpacker, read_flag, packet_length)?;
	trace!("Audio energy this frame: {}", has_audio_energy);
	bitpack_read_callback(has_audio_energy as u32, 1, shared_callback_data)?;

	if has_audio_energy {
		/// Array of range bits to read, straight from section 7.2.3 of the Vorbis I
		/// specification.
		const RANGE_BITS_ARRAY: [u8; 4] =
			[ilog(256 - 1), ilog(128 - 1), ilog(86 - 1), ilog(64 - 1)];

		let range_bits = RANGE_BITS_ARRAY[(floor_configuration.multiplier - 1) as usize];

		// Read and discard the first 2 floor vector elements
		for _ in 0..2 {
			let value = bitpack_packet_read!(
				bitpacker,
				read_unsigned_integer,
				packet_length,
				mut range_bits,
				u8
			)?;
			bitpack_read_callback(value as u32, range_bits, shared_callback_data)?;
		}

		for class in &floor_configuration.partition_class_list {
			let class = *class as usize;
			let class_dimension = floor_configuration.class_dimensions[class];
			let class_bits = floor_configuration.class_subclasses[class]; // In [0, 3] range

			let csub = (1 << class_bits) - 1;
			let mut cval = if class_bits > 0 {
				// Unwrap is safe because this is always Some when class_bits != 0
				let class_masterbook =
					floor_configuration.class_masterbooks[class].unwrap() as usize;

				// Scalar read
				decode_codebook_entry_number(
					&codebook_configurations[class_masterbook].codebook,
					bitpacker,
					&mut codebook_entry_decode_callback,
					shared_callback_data
				)?
			} else {
				0
			};

			for _ in 0..class_dimension {
				let book = floor_configuration.subclass_books[class][usize::try_from(cval & csub)?];
				cval >>= class_bits;

				if let Some(book) = book {
					// Scalar read
					decode_codebook_entry_number(
						&codebook_configurations[book as usize].codebook,
						bitpacker,
						&mut codebook_entry_decode_callback,
						shared_callback_data
					)?;
				}
			}
		}
	}

	Ok(has_audio_energy)
}

/// Implements the algorithm described in the Vorbis I specification, § 4.3.4, step 5.
///
/// # Preconditions
/// At least one residue vector is to be decoded.
fn process_residue<R: Read, T, F: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>>(
	bitpacker: &mut BitpackReader<R>,
	residue_configuration: &ResidueConfiguration,
	codebook_configurations: &[CodebookConfiguration],
	original_residue_vectors_masks: &[bool],
	current_blocksize: u16,
	mut codebook_entry_decode_callback: F,
	shared_callback_data: &mut T
) -> Result<(), VorbisOptimizerError> {
	// Vorbis I spec, § 8.6.5. Format 2 decode can be implemented as format 1 decoding
	// with different parameters
	let residue_vectors_masks;
	let vector_size; // At most 4096 (8192 / 2) * 255 (audio channels) = 2^13 * (2^8 - 1)
	if residue_configuration.residue_type == ResidueType::InterleavedVectors {
		// "If at least one vector is to be decoded, all the vectors are decoded"
		// Because format 2 decode is reducible to a single vector format 1 decode,
		// that is equivalent to stating that we just decode a single big vector in
		// that case as usual. If this method is invoked, at least one residue vector
		// is to be decoded
		residue_vectors_masks = &[false][..];
		vector_size = current_blocksize as u32 / 2 * original_residue_vectors_masks.len() as u32;
	} else {
		residue_vectors_masks = original_residue_vectors_masks;
		vector_size = current_blocksize as u32 / 2;
	};

	let residue_vector_count = residue_vectors_masks.len();

	// Vorbis I spec, § 8.6.2, common residue packet decode

	let residue_begin = cmp::min(residue_configuration.begin, vector_size);
	let residue_end = cmp::min(residue_configuration.end, vector_size);

	let n_to_read = residue_end - residue_begin;

	if n_to_read == 0 {
		trace!("No residue to decode, skipping");
		return Ok(());
	}

	let classbook_configuration =
		&codebook_configurations[residue_configuration.classbook as usize];
	let classwords_per_codeword = classbook_configuration.dimensions;
	// At most 2^13 * (2^8 - 1)
	let partitions_to_read = n_to_read / residue_configuration.partition_size;

	// A random stereo Vorbis stream had 114 classifications. Assume that's typical to
	// optimize out heap allocations. Codebooks are not that unique to their streams
	let classifications_stride = classwords_per_codeword as u32 + partitions_to_read;
	let classification_count = (residue_vector_count as u32 * classifications_stride).try_into()?;
	let mut classifications = if classification_count <= 128 {
		TinyVec::from_array_len([0; 128], classification_count)
	} else {
		TinyVec::Heap(vec![0; classification_count])
	};

	// These casts are safe because the classifications capacity fits in a valid usize
	let partitions_to_read = partitions_to_read as usize;
	let classifications_stride = classifications_stride as usize;
	let classwords_per_codeword = classwords_per_codeword as usize;

	for pass in 0..8 {
		let mut partition_count = 0;

		while partition_count < partitions_to_read {
			if pass == 0 {
				for (j, do_not_decode) in residue_vectors_masks.iter().enumerate() {
					if *do_not_decode {
						continue;
					}

					let mut temp = decode_codebook_entry_number(
						&classbook_configuration.codebook,
						bitpacker,
						&mut codebook_entry_decode_callback,
						shared_callback_data
					)?;

					for i in (0..classwords_per_codeword).rev() {
						classifications[j * classifications_stride + i + partition_count] =
							temp % residue_configuration.classifications as u32;

						temp /= residue_configuration.classifications as u32;
					}
				}
			}

			for _ in 0..classwords_per_codeword {
				for (j, do_not_decode) in residue_vectors_masks.iter().enumerate() {
					if *do_not_decode {
						continue;
					}

					let vq_class = usize::try_from(
						classifications[j * classifications_stride + partition_count]
					)?;

					if let Some(vq_book) = residue_configuration.books.get(vq_class).ok_or(
						VorbisOptimizerError::InvalidVectorQuantizationClassbook(vq_class)
					)?[pass]
					{
						let vq_book_configuration = &codebook_configurations[vq_book as usize];

						// Even though we don't actually decode vectors, check that the stream is
						// consistent, as a real decoder would do so
						if vq_book_configuration.vector_lookup_type == VectorLookupType::NoLookup {
							return Err(VorbisOptimizerError::ScalarCodebookUsedInVectorContext(
								vq_book
							));
						}

						residue_configuration
							.partition_size
							// Dividing by zero is an arithmetic error, and conceptually zero-dimension
							// vectors make no sense. The specification does not explicitly address this.
							// Even though a value of zero for the codebook dimension can be legally encoded,
							// the partition size is always greater than zero, and it never is possible
							// that it is a multiple of zero, so the specification gives sufficient information
							// to conclude that it is implicitly invalid. See § 8.3 and § 8.4
							.checked_rem(vq_book_configuration.dimensions as u32)
							// Vorbis I spec, § 8.3: "[...] the partition size must be an even multiple
							// of the codebook dimension"
							.filter(|remainder| *remainder == 0)
							.ok_or(VorbisOptimizerError::InvalidCodebookDimension {
								codebook: vq_book,
								dimensions: vq_book_configuration.dimensions,
								expected_dimensions_multiple_of: residue_configuration
									.partition_size
							})?;

						process_residue_partition_vector(
							bitpacker,
							residue_configuration,
							vq_book_configuration,
							&mut codebook_entry_decode_callback,
							shared_callback_data
						)?;
					}
				}

				partition_count += 1;
				if partition_count >= partitions_to_read {
					break;
				}
			}
		}
	}

	Ok(())
}

/// Processes a residue partition vector. Because residue types 0 (interleaved) and 1
/// (ordered) read the same amount of vectors of equal dimensions, just differing on
/// how the partition vectors are copied to the final residue vector, we can reuse
/// the same code for both.
fn process_residue_partition_vector<
	R: Read,
	T,
	F: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>
>(
	bitpacker: &mut BitpackReader<R>,
	residue_configuration: &ResidueConfiguration,
	vq_book_configuration: &CodebookConfiguration,
	mut codebook_entry_decode_callback: F,
	shared_callback_data: &mut T
) -> Result<(), VorbisOptimizerError> {
	let partition_count =
		residue_configuration.partition_size / vq_book_configuration.dimensions as u32;

	// Note that the residue 1 algorithm does the same number of codebook vector
	// reads than residue 0
	for _ in 0..partition_count {
		// VQ read. Would return a vector of dimension scalars
		decode_codebook_entry_number(
			&vq_book_configuration.codebook,
			bitpacker,
			&mut codebook_entry_decode_callback,
			shared_callback_data
		)?;
	}

	Ok(())
}

/// Helper function to decode an entry number from a codebook, invoking the
/// specified callback on success.
fn decode_codebook_entry_number<
	R: Read,
	T,
	F: FnMut(u16, u32, &mut T) -> Result<(), VorbisOptimizerError>
>(
	codebook: &VorbisCodebook,
	bitpacker: &mut BitpackReader<R>,
	mut codebook_entry_decode_callback: F,
	shared_callback_data: &mut T
) -> Result<u32, VorbisOptimizerError> {
	let entry_number = codebook.decode_entry_number(bitpacker)?;
	codebook_entry_decode_callback(codebook.codebook_number, entry_number, shared_callback_data)?;
	Ok(entry_number)
}
