//! Contains code for calculating valid, sensible granule positions for Ogg Vorbis packets.

use super::{Settings, VorbisStreamState, ogg_vorbis_stream_mangler::OggVorbisStreamMangler};

/// Calculates a valid granule position for an Ogg Vorbis packet, assuming that:
///
/// - The Vorbis stream structure to which the packet belongs was validated to have three
///   header packets first, and then only audio packets.
/// - The packets are temporally consecutive (i.e., there are no "gaps" between one packet
///   and the next one handed over to this function).
///
/// If the information in `stream_state` for the original stream contains a feasible granule
/// granule position for the last audio packet, that granule position will be used. This allows
/// decoders to truncate the last audio packet decoded sample count to achieve sample-exact
/// decode. Otherwise, it will be assumed that the packet is to not be truncated, which will
/// likely yield more samples than the stream author intended, but provides an usable result.
///
/// The information in `stream_state` is also used to take into account the offset of the
/// granule position of the original first audio page from the expected one, which is assumed
/// to start at time zero. This offset is used to adjust the granule positions of any following
/// packets, so any lossless beginning sample truncation or livestream running time information
/// is also kept. This offset can be ignored via the appropriate `remuxer_settings` configuration.
pub(super) fn granule_position_for_packet<M: OggVorbisStreamMangler>(
	packet_sample_block_size: Option<u16>,
	packet_number: usize,
	packet_page_granule_position: u64,
	is_last_stream_packet: bool,
	remuxer_settings: &Settings<M>,
	stream_state: &mut VorbisStreamState
) -> i64 {
	match (
		stream_state.last_written_packet_granule_position,
		stream_state.last_written_packet_sample_block_size,
		stream_state.original_last_audio_packet_in_first_audio_page_granule_position,
		packet_number,
		is_last_stream_packet
	) {
		(_, _, _, 0..=2, _) => {
			// Vorbis I specification, § A.2: "the granule position of these first
			// pages containing only [the three] headers is zero"

			0
		}
		(None, None, None, 3, _) => {
			// Vorbis I specification, § 4.3.8: "data is not returned from the first
			// frame; it must be used to 'prime' the decode engine. The encoder accounts
			// for this priming when calculating PCM offsets; after the first frame,
			// the proper PCM output offset is '0' (as no data has been returned yet)".
			// § A.2 further states that the granule position should be incremented by the
			// number of actual samples returned by decode, after any windowing, lapping and
			// the like, so the proper granule position of the first audio packet (a.k.a.
			// frame) is zero

			stream_state.last_written_packet_granule_position = Some(0);
			stream_state.last_written_packet_sample_block_size =
				Some(packet_sample_block_size.unwrap());

			0
		}
		(
			None,
			None,
			Some((_, final_packet_in_first_audio_page_number)),
			processed_packet_count @ 3,
			_
		) if final_packet_in_first_audio_page_number != processed_packet_count => {
			// Same case as above, this packet is not special in any other way

			stream_state.last_written_packet_granule_position = Some(0);
			stream_state.last_written_packet_sample_block_size =
				Some(packet_sample_block_size.unwrap());

			0
		}
		(
			None,
			None,
			Some((first_audio_page_granule_position, final_packet_in_first_audio_page_number)),
			processed_packet_count @ 3,
			_
		) if final_packet_in_first_audio_page_number == processed_packet_count => {
			// Same case as above, but compare the calculated granule position for
			// this packet, 0, with the actual granule position. Its difference
			// is the initial sample offset we should add to any future granule
			// position to not discard sample truncation at the beginning, or
			// livestream running time up to the encoded start point

			// No-op subtraction for human comprehension, optimized out by the compiler
			#[allow(clippy::identity_op)]
			let start_granule_position_offset = first_audio_page_granule_position - 0;

			let actual_granule_position =
				0i64.wrapping_add(if remuxer_settings.ignore_start_sample_offset {
					0
				} else {
					start_granule_position_offset
				});

			stream_state.start_granule_position_offset = Some(start_granule_position_offset);
			stream_state.last_written_packet_granule_position = Some(actual_granule_position);
			stream_state.last_written_packet_sample_block_size =
				Some(packet_sample_block_size.unwrap());

			actual_granule_position
		}
		(
			Some(last_written_packet_granule_position),
			Some(last_written_packet_sample_block_size),
			None,
			4..,
			false
		) => {
			// Non-first and non-last audio packet. These packets always are decoded
			// and outputted completely, so we can just pass on our calculated granule
			// position, maybe taking into account the start granule position offset.
			// This can repair files with bad granule position data

			let packet_sample_block_size = packet_sample_block_size.unwrap();

			let calculated_granule_position = calculate_granule_position(
				last_written_packet_granule_position,
				last_written_packet_sample_block_size,
				packet_sample_block_size
			);

			stream_state.last_written_packet_granule_position = Some(calculated_granule_position);
			stream_state.last_written_packet_sample_block_size = Some(packet_sample_block_size);

			calculated_granule_position
		}
		(
			Some(last_written_packet_granule_position),
			Some(last_written_packet_sample_block_size),
			Some((_, final_packet_in_first_audio_page_number)),
			processed_packet_count @ 4..,
			false
		) if final_packet_in_first_audio_page_number != processed_packet_count => {
			// Same case as above, this packet is not special in any other way

			let packet_sample_block_size = packet_sample_block_size.unwrap();

			let calculated_granule_position = calculate_granule_position(
				last_written_packet_granule_position,
				last_written_packet_sample_block_size,
				packet_sample_block_size
			);

			stream_state.last_written_packet_granule_position = Some(calculated_granule_position);
			stream_state.last_written_packet_sample_block_size = Some(packet_sample_block_size);

			calculated_granule_position
		}
		(
			Some(last_written_packet_granule_position),
			Some(last_written_packet_sample_block_size),
			Some((first_audio_page_granule_position, final_packet_in_first_audio_page_number)),
			processed_packet_count @ 4..,
			false
		) if final_packet_in_first_audio_page_number == processed_packet_count => {
			// Same case as above, but compare the calculated granule position for
			// this packet with the actual granule position. Its difference
			// is the initial sample offset we should add to any future granule
			// position to not discard sample truncation at the beginning, or
			// livestream running time up to the encoded start point.
			//
			// Note that only packets returned from now on will have its granule
			// adjusted. This is not a concern for our purposes, as the original
			// Ogg Vorbis stream was checked to be valid, and the original stream
			// either finishes the first page sooner or at the same time than us,
			// so the non-adjusted granule position for previous packets will not
			// be written to the stream. User-defined stream manglers can challenge
			// this and other assumptions, though, but that's not a concern either.
			//
			// A concerning edge case is when enough packets in the first page of
			// the original stream are discarded. In that case, the first last
			// non-ignored audio packet may be placed in the second page by us, and
			// thus the granule position of the first page we write will not account
			// for any offset. This is extremely unlikely in practice, so we can solve
			// it with the sophisticated ostrich algorithm. If it really matters to
			// anyone, they can probably ignore the start sample offset anyway

			let packet_sample_block_size = packet_sample_block_size.unwrap();

			let calculated_granule_position = calculate_granule_position(
				last_written_packet_granule_position,
				last_written_packet_sample_block_size,
				packet_sample_block_size
			);

			// Fuzzing discovered that the granule position may be very negative, and thus the
			// subtraction would overflow. Saturate it to keep the numerically closest value.
			// Such extremely negative granule positions are invalid according to the specification
			// anyway
			let start_granule_position_offset =
				first_audio_page_granule_position.saturating_sub(calculated_granule_position);

			let actual_granule_position = calculated_granule_position.wrapping_add(
				if remuxer_settings.ignore_start_sample_offset {
					0
				} else {
					start_granule_position_offset
				}
			);

			stream_state.start_granule_position_offset = Some(start_granule_position_offset);
			stream_state.last_written_packet_granule_position = Some(actual_granule_position);
			stream_state.last_written_packet_sample_block_size = Some(packet_sample_block_size);

			actual_granule_position
		}
		(
			Some(last_written_packet_granule_position),
			Some(last_written_packet_sample_block_size),
			_,
			_,
			true
		) => {
			// Same case as the non-first audio packet case, barring one complication:
			// the granule position is used to truncate the block of samples returned
			// by the decoder, so that the total amount of played samples is the same
			// as in the original audio stream (see the Vorbis I specification, last
			// point of § A.2). It is very unlikely that the source audio has exactly
			// x * (small_blocksize / 4) + y * (big_blocksize / 4) samples when
			// x and y are restricted to integers, so handling this is important for
			// later sample-exact decode. The special cases of only headers or four
			// packet Vorbis streams are handled in the cases above

			let packet_sample_block_size = packet_sample_block_size.unwrap();

			// This granule position is calculated assuming that the last packet is fully
			// decoded, which is unlikely
			let calculated_granule_position = calculate_granule_position(
				last_written_packet_granule_position,
				last_written_packet_sample_block_size,
				packet_sample_block_size
			);

			// The last packet of the stream finishes a page too, so the granule position
			// of the page matches the one of the packet
			let original_granule_position = packet_page_granule_position as i64;
			let start_granule_position_offset =
				stream_state.start_granule_position_offset.unwrap_or(0);

			let minimum_expected_granule_position = if remuxer_settings.ignore_start_sample_offset {
				// Undo original stream offset ignore for comparison
				last_written_packet_granule_position
					.wrapping_add(start_granule_position_offset)
					.wrapping_add(1)
			} else {
				last_written_packet_granule_position.wrapping_add(1)
			};
			let maximum_expected_granule_position = if remuxer_settings.ignore_start_sample_offset {
				// Undo original stream offset ignore for comparison
				calculated_granule_position.wrapping_add(start_granule_position_offset)
			} else {
				calculated_granule_position
			};

			// Use the original granule position if it looks sensible: it decodes at least
			// a sample, but not more samples than possible. If not, to deal with granule
			// position corruption in the most seamless way, use our calculated position
			// that would output the entire sample block. Blocks are small enough and
			// sampling frequencies high enough for this to not matter that much to humans:
			// this yields at most ~186 ms of spurious audio data at 44.1 kHz
			if (minimum_expected_granule_position..=maximum_expected_granule_position)
				.contains(&original_granule_position)
			{
				if remuxer_settings.ignore_start_sample_offset {
					original_granule_position.wrapping_sub(start_granule_position_offset)
				} else {
					original_granule_position
				}
			} else {
				calculated_granule_position
			}
		}
		_ => unreachable!()
	}
}

/// Calculates a valid granule position for an Ogg Vorbis packet from the granule position of
/// the last written packet, its block size, and the block size of the current packet. Effectively,
/// the granule position of the last packet will be incremented by the amount of samples that an
/// Ogg Vorbis decoder can return after decoding this packet.
const fn calculate_granule_position(
	last_written_packet_granule_position: i64,
	last_written_packet_sample_block_size: u16,
	packet_sample_block_size: u16
) -> i64 {
	// Formula from Vorbis I spec, § 4.3.8, to calculate the amount of samples
	// that would be returned by a decoder. Used by liboggz and revorb too.
	// On overflow, wrap around to get the least upsetting granule position for
	// decoders: due to two's complement arithmetic things break less this way.
	// liboggz relevant lines:
	// https://github.com/kfish/liboggz/blob/bde065d0415a94e92bb9f9cacdbc70276d3932c2/src/liboggz/oggz_auto.c#L940-L946
	last_written_packet_granule_position.wrapping_add(
		(last_written_packet_sample_block_size as i64 + packet_sample_block_size as i64) / 4
	)
}
