//! Contains the [`OggToOgg`] remuxer struct and helper data types.

use std::{
	cell::RefCell,
	collections::hash_map::DefaultHasher,
	env,
	hash::Hasher,
	io::{self, Read, Seek, SeekFrom, Write},
	num::ParseIntError,
	sync::Mutex,
	time::UNIX_EPOCH
};

use getrandom::getrandom;
use granulator::granule_position_for_packet;
use indexmap::{map::Entry, IndexMap};
use log::info;
use ogg::{OggReadError, PacketReader, PacketWriteEndInfo, PacketWriter};
#[doc(inline)]
pub use ogg_vorbis_stream_mangler::{OggVorbisStreamMangler, OggVorbisStreamPassthroughMangler};
use rand_xoshiro::{
	rand_core::{RngCore, SeedableRng},
	Xoshiro256PlusPlus
};
use thiserror::Error;

use super::Remuxer;
use crate::vorbis::optimizer::{VorbisOptimizer, VorbisOptimizerError, VorbisOptimizerSettings};

mod granulator;
mod ogg_vorbis_stream_mangler;
#[cfg(test)]
mod test;

/// A [`Remuxer`] that processes Vorbis streams in unmultiplexed Ogg containers, generating
/// another Ogg Vorbis file. Non-Vorbis streams will be ignored. Chained Vorbis streams are
/// supported.
///
/// If the `source-date-epoch` feature is enabled, this remuxer honors the
/// [`SOURCE_DATE_EPOCH` specification]: it reads the `SOURCE_DATE_EPOCH` environment variable
/// and uses it to set a reproducible PRNG state for Ogg stream serial randomization.
///
/// [`SOURCE_DATE_EPOCH` specification]: https://reproducible-builds.org/specs/source-date-epoch
pub struct OggToOgg<M: OggVorbisStreamMangler> {
	remuxer_settings: RefCell<Settings<M>>,
	optimizer_settings: VorbisOptimizerSettings
}

/// Settings that influence how the remuxing from an Ogg file to another Ogg file is done.
pub struct Settings<M: OggVorbisStreamMangler> {
	/// If `true`, each Vorbis stream will be assigned a different serial derived from a
	/// randomly-generated 32-bit number. Otherwise, streams will be assigned a sequential
	/// number starting from [`first_stream_serial_offset`](Self::first_stream_serial_offset).
	///
	/// Assigning randomized serials better adheres to [RFC 3533 "The Ogg Encapsulation
	/// Format Version 0"](https://www.xiph.org/ogg/doc/rfc3533.txt) and makes it very
	/// likely that concatenating two different Ogg files together with general-purpose file
	/// manipulation tools, such as the POSIX `cat` command, generates a valid, chained
	/// Ogg stream. Setting this to `false` does not prevent two different streams in two
	/// different files from having the same serial, so concatenating those files would
	/// generate an invalid Ogg stream with non-unique serials.
	///
	/// Therefore, the recommended value for this option is `true`. Reasonable motives for
	/// setting it to `false` include the necessary random number generation being a no-go
	/// or manually taking care of the randomization by varying
	/// [`first_stream_serial_offset`](Self::first_stream_serial_offset).
	///
	/// If this option is set to `true`, the [`SOURCE_DATE_EPOCH` environment variable]
	/// is set, and the `source-date-epoch` feature is enabled, OptiVorbis will use a
	/// reproducible randomization algorithm. The increased reproducibility may come at the
	/// cost of increased serial number collisions, however.
	///
	/// **Default value**: `true`
	///
	/// [`SOURCE_DATE_EPOCH` environment variable]: https://reproducible-builds.org/specs/source-date-epoch/
	pub randomize_stream_serials: bool,
	/// Sets the offset that will be added to the serial of the first stream, in turn
	/// influencing the sequence of serials that will be assigned to further streams.
	///
	/// When randomizing serials this number will be added to the randomly-generated serial
	/// for the first stream. When not, the offset will effectively become the serial for
	/// the first stream.
	///
	/// Changing this offset is mainly useful for avoiding serial clashes and, when not
	/// randomizing stream serials, avoiding assigning a zero serial to the first stream,
	/// which makes tools such as `ogginfo` complain.
	///
	/// **Default value**: `0`
	pub first_stream_serial_offset: u32,
	/// Sets whether a non-zero calculated granule position for the first audio sample will
	/// be honored when recomputing granule positions in the generated Ogg file or not.
	///
	/// The granule position of Vorbis stream pages in Ogg files is defined to be the count
	/// of complete audio samples that a decoder would return so far. Therefore, the granule
	/// position is monotonically increasing in conforming streams, so the playback timestamp
	/// of that page can be calculated easily, which is needed by players for seeking and display
	/// purposes. However, it is legal for the first sample to have an induced granule position
	/// lower than zero (if decoding the first packets yields more samples than the granule
	/// position of the first audio page indicates) or greater than zero (in the opposite case
	/// of decoding less samples than the granule position suggests).
	///
	/// Lower than zero granule positions are used for lossless sample truncation in the
	/// beginning of Vorbis streams. Conversely, higher than zero positions are characteristic of
	/// partial livestream recordings (the transmitter had already been broadcasting for some time
	/// before we tuned in) or chained streams composed of several temporally consecutive streams.
	/// When ignoring the start sample offset, OptiVorbis will behave as if the initial granule
	/// position was zero: no samples were truncated, and no other audio has been encoded so far.
	///
	/// OptiVorbis needs to recalculate granule positions, so it usually is a good idea to let
	/// it take into account these non-zero initial granule positions, in order to carry over
	/// the original timestamping information, by leaving this option set to `false`. Good reasons
	/// for setting it to `true` include repairing Ogg Vorbis files with bad granule position data
	/// (i.e., seeking problems), purposefully wanting to get rid of truncated samples or the time
	/// a livestream has been going so far, and increasing compatibility with players that do not
	/// handle non-zero granule positions gracefully.
	///
	/// **Default value**: `false`
	pub ignore_start_sample_offset: bool,
	/// Sets whether not finding any Vorbis stream within the Ogg container will be considered an
	/// error condition. Returning an error when this happens usually is a good thing because
	/// running OptiVorbis in such cases tends to be a usage mistake, and the otherwise silent
	/// behavior of generating empty Ogg files is counter-intuitive for end-users. However, not
	/// returning an error may be helpful for advanced use cases where returning an empty file is
	/// desirable.
	///
	/// **Default value**: `true`
	pub error_on_no_vorbis_streams: bool,
	/// Sets the [mangler](OggVorbisStreamMangler) that will have a final say on some values
	/// generated for the Ogg page and packet encapsulations. OptiVorbis almost always does the
	/// right thing by itself, so **using manglers others than the
	/// [built-in passthrough](OggVorbisStreamPassthroughMangler) is not recommended**.
	pub vorbis_stream_mangler: M
}

impl Default for Settings<OggVorbisStreamPassthroughMangler> {
	fn default() -> Self {
		Self {
			randomize_stream_serials: true,
			first_stream_serial_offset: 0,
			ignore_start_sample_offset: false,
			error_on_no_vorbis_streams: true,
			vorbis_stream_mangler: OggVorbisStreamPassthroughMangler
		}
	}
}

/// Holds the state needed for an optimizing remux of an Ogg Vorbis stream.
struct VorbisStreamState<'settings> {
	optimizer: VorbisOptimizer<'settings>,
	original_last_audio_packet_in_first_audio_page_granule_position: Option<(i64, usize)>,
	last_written_packet_granule_position: Option<i64>,
	last_written_packet_sample_block_size: Option<u16>,
	start_granule_position_offset: Option<i64>,
	analyzed_packet_count: usize,
	optimized_packet_count: usize,
	checksum: u32
}

/// Represents an error that may happen while remuxing with the [`OggToOgg`] remuxer.
#[derive(Debug, Error)]
pub enum RemuxError {
	/// Represents an Ogg container decoding error, which may be an I/O error.
	#[error("Ogg read error: {0}")]
	OggError(#[from] OggReadError),
	/// Represents a Vorbis stream optimizer error. This may happen in corrupt Vorbis
	/// streams, or streams that use unsupported features.
	#[error("Vorbis optimization error: {0}")]
	OptimizerError(#[from] VorbisOptimizerError),
	/// Represents an unsupported Ogg container multiplexing error, which happens if
	/// several streams are concurrently multiplexed.
	#[error("Remuxing Ogg bitstreams with grouped logical bitstreams is not supported")]
	UnsupportedStreamMultiplexing,
	/// Represents a missing Vorbis stream error, which signals that no Vorbis audio
	/// data was found in the Ogg container.
	#[error("No Vorbis bitstream found. Is this Ogg Vorbis data?")]
	NoVorbisStreamFound,
	/// The value of the `SOURCE_DATE_EPOCH` environment variable does not conform to
	/// the [`SOURCE_DATE_EPOCH` specification].
	///
	/// [`SOURCE_DATE_EPOCH` specification]: https://reproducible-builds.org/specs/source-date-epoch
	#[error("The SOURCE_DATE_EPOCH environment variable is set, but its value is invalid")]
	#[cfg(any(doc, feature = "source-date-epoch"))]
	InvalidSourceDateEpoch,
	/// An I/O error outside of any of the previously mentioned error contexts happened.
	#[error("I/O error: {0}")]
	IoError(#[from] io::Error)
}

impl<M: OggVorbisStreamMangler> Remuxer for OggToOgg<M> {
	type RemuxError = RemuxError;
	type RemuxerSettings = Settings<M>;

	fn new(remuxer_settings: Settings<M>, optimizer_settings: VorbisOptimizerSettings) -> Self {
		Self {
			remuxer_settings: RefCell::new(remuxer_settings),
			optimizer_settings
		}
	}

	fn remux<R: Read + Seek, W: Write>(
		&self,
		mut source: R,
		mut sink: W
	) -> Result<W, Self::RemuxError> {
		// Remember the source stream position to rewind to it later
		let initial_source_pos = source.stream_position()?;
		let remuxer_settings = &mut *self.remuxer_settings.borrow_mut();

		// First pass: validate and gather stream data for optimization
		info!("Starting first Ogg to Ogg remux pass");
		let mut vorbis_streams =
			first_pass(&mut source, &self.optimizer_settings, remuxer_settings)?;
		info!("First Ogg to Ogg remux pass completed");

		// Get the serial for the first stream, and the increment to add for the next streams.
		// It's important to randomize the serials per remux operation, if applicable; otherwise,
		// any physical bitstreams remuxed in this session would share serials
		let (first_stream_serial, stream_serial_increment) =
			if remuxer_settings.randomize_stream_serials {
				random_stream_serial_and_increment(
					remuxer_settings.first_stream_serial_offset,
					// Calculate a PRNG seed tweak by XORing the checksums of every stream
					vorbis_streams
						.values()
						.fold(0, |checksum, state| checksum ^ state.checksum)
				)?
			} else {
				(remuxer_settings.first_stream_serial_offset, 1)
			};

		// Rewind for the second pass
		source.seek(SeekFrom::Start(initial_source_pos))?;

		// Second pass: optimizing Vorbis packet rewrite
		info!("Starting second Ogg to Ogg remux pass");
		second_pass(
			source,
			&mut sink,
			&mut vorbis_streams,
			remuxer_settings,
			first_stream_serial,
			stream_serial_increment
		)?;
		info!("Second Ogg to Ogg remux pass completed");

		Ok(sink)
	}
}

/// Executes the first remuxing pass, where the Vorbis streams within the source Ogg physical
/// bitstream are read and analyzed for optimization.
fn first_pass<'settings, R: Read + Seek, M: OggVorbisStreamMangler>(
	source: R,
	optimizer_settings: &'settings VorbisOptimizerSettings,
	remuxer_settings: &mut Settings<M>
) -> Result<IndexMap<u32, VorbisStreamState<'settings>>, RemuxError> {
	let mut packet_reader = PacketReader::new(source);

	let mut vorbis_streams = IndexMap::with_capacity(1);
	let mut reading_vorbis_stream = false;

	while let Some(packet) = packet_reader.read_packet()? {
		let stream_serial = packet.stream_serial();
		let page_checksum = packet.checksum_page();

		if packet.first_in_stream() {
			match VorbisOptimizer::new(optimizer_settings, packet.data) {
				Ok(mut stream_optimizer) => {
					// The just-started logical bitstream looks like Vorbis

					if reading_vorbis_stream {
						// A logical Vorbis bitstream starts while we're already optimizing another.
						// This means that streams are grouped (concurrently multiplexed), and we
						// don't support that: we won't know how to interleave their pages properly
						// later on
						return Err(RemuxError::UnsupportedStreamMultiplexing);
					}

					info!(
						"Analyzing Ogg Vorbis bitstream with serial {}",
						stream_serial
					);

					// Mangle the sampling frequency and bitrates read from the header packet.
					// We can do this anytime, as we don't use them for anything
					let sampling_frequency = remuxer_settings
						.vorbis_stream_mangler
						.mangle_sampling_frequency(
							stream_optimizer.identification_data.sampling_frequency
						);
					stream_optimizer.identification_data.sampling_frequency = sampling_frequency;

					let (minimum_bitrate, nominal_bitrate, maximum_bitrate) =
						remuxer_settings.vorbis_stream_mangler.mangle_bitrates(
							stream_optimizer.identification_data.minimum_bitrate,
							stream_optimizer.identification_data.nominal_bitrate,
							stream_optimizer.identification_data.maximum_bitrate
						);
					stream_optimizer.identification_data.minimum_bitrate = minimum_bitrate;
					stream_optimizer.identification_data.nominal_bitrate = nominal_bitrate;
					stream_optimizer.identification_data.maximum_bitrate = maximum_bitrate;

					vorbis_streams.insert(
						stream_serial,
						VorbisStreamState {
							optimizer: stream_optimizer,
							original_last_audio_packet_in_first_audio_page_granule_position: None,
							last_written_packet_granule_position: None,
							last_written_packet_sample_block_size: None,
							start_granule_position_offset: None,
							analyzed_packet_count: 1, // Just processed the identification header packet
							optimized_packet_count: 0,
							checksum: page_checksum
						}
					);
					reading_vorbis_stream = true;
				}
				Err(
					VorbisOptimizerError::TooSmallPacket(_)
					| VorbisOptimizerError::UnexpectedPacketType { .. }
					| VorbisOptimizerError::InvalidPacketType(_)
					| VorbisOptimizerError::InvalidPattern
				) => {
					// These errors signal that the basic Vorbis header packet validation did
					// not pass. This signals non-Vorbis data
					info!(
						"Ignoring non-Vorbis logical bitstream with serial {}",
						stream_serial
					);
				}
				Err(error) => {
					// The stream has an identification header that looks like Vorbis, but is corrupt
					return Err(error.into());
				}
			}
		} else if let Some(stream_state) = vorbis_streams.get_mut(&stream_serial) {
			// The second and next Vorbis packets of a Vorbis logical bitstream

			// last_in_stream() may return false for the last packet of a bitstream
			// if its page does not set the EOS flag, but that's not a concern if
			// no other packets follow. If they do, and are not for this stream,
			// we will rightfully return an error
			reading_vorbis_stream = !packet.last_in_stream();

			// Hand over the packet to the optimizer for analysis
			let packet_sample_block_size = stream_state.optimizer.analyze_packet(&packet.data)?;
			let is_not_discarded_audio_packet = packet_sample_block_size.is_some();

			// When optimizing, audio packets that are discarded from the stream won't be
			// written out. It's convenient to pretend they weren't analyzed either, to
			// handle them with almost the same code path on the second pass
			if stream_state.analyzed_packet_count <= 2 || is_not_discarded_audio_packet {
				// Remember the granule position of the last audio packet in the first audio page.
				// This is needed to compute granule positions properly for streams that originally
				// did not start at time zero (livestream recordings, truncating some samples in the
				// beginning)
				if let (None, true, true) = (
					stream_state.original_last_audio_packet_in_first_audio_page_granule_position,
					packet.last_in_page(),
					is_not_discarded_audio_packet
				) {
					stream_state.original_last_audio_packet_in_first_audio_page_granule_position =
						Some((
							packet.absgp_page() as i64,
							stream_state.analyzed_packet_count
						));
				}

				stream_state.analyzed_packet_count =
					stream_state.analyzed_packet_count.saturating_add(1);

				stream_state.checksum ^= page_checksum;
			}
		}
	}

	if vorbis_streams.is_empty() && remuxer_settings.error_on_no_vorbis_streams {
		Err(RemuxError::NoVorbisStreamFound)
	} else {
		Ok(vorbis_streams)
	}
}

/// Executes the second remuxing pass, where Vorbis streams within the source Ogg physical
/// bitstream are read again, and their optimized versions written out to new Vorbis streams
/// in a new Ogg physical bitstream.
fn second_pass<R: Read + Seek, W: Write, M: OggVorbisStreamMangler>(
	source: R,
	sink: W,
	vorbis_streams: &mut IndexMap<u32, VorbisStreamState<'_>>,
	remuxer_settings: &mut Settings<M>,
	first_stream_serial: u32,
	stream_serial_increment: u32
) -> Result<(), RemuxError> {
	let mut packet_reader = PacketReader::new(source);
	let mut packet_writer = PacketWriter::new(sink);

	let mut last_seen_vorbis_stream_serial = None;

	while let Some(packet) = packet_reader.read_packet()? {
		let stream_serial = packet.stream_serial();

		// Ignore non-Vorbis streams we skipped in the first pass
		if let Entry::Occupied(mut entry) = vorbis_streams.entry(stream_serial) {
			if last_seen_vorbis_stream_serial != Some(stream_serial) {
				info!(
					"Optimizing Ogg Vorbis bitstream with serial {}",
					stream_serial
				);
			}
			last_seen_vorbis_stream_serial = Some(stream_serial);

			let stream_index = entry.index() as u32;
			let stream_state = entry.get_mut();

			// Optimize the packet
			let packet_page_granule_position = packet.absgp_page();
			let (optimized_packet, packet_sample_block_size) = if let Some(optimized_packet_data) =
				stream_state.optimizer.optimize_packet(packet.data)?
			{
				optimized_packet_data
			} else {
				// Discard the packet. Pretend it never existed by not writing it and
				// not incrementing the optimized packet count
				continue;
			};

			let packet_number = stream_state.optimized_packet_count;
			let is_header_packet = packet_number < 3;
			// The last_in_stream() method relies on the physical Ogg bitstream to set
			// the EOS flag on the last page to return a proper result. However, it
			// happens in practice that some physical bitstreams end with a page that
			// does not have EOS set. Handle that by not relying on what the stream
			// says, using the packet count we computed during the first pass
			let is_last_stream_packet = packet_number == stream_state.analyzed_packet_count - 1;

			// Vorbis stream encapsulation in Ogg is documented in the Vorbis I specification,
			// § A.2. Putting Vorbis packets in Ogg pages is pretty straightforward: the
			// identification and setup headers must end the page they are in, but the rest
			// of packets may be stuffed in pages as desired, according to the ease of seeking,
			// container overhead and maximum livestream recapture time requirements. In our
			// case, we only care about minimizing container overhead (we are dealing with
			// seekable sources in any case), so just put as many packets per page as possible
			let page_end_info = if is_last_stream_packet {
				PacketWriteEndInfo::EndStream
			} else if packet_number == 0 || packet_number == 2 {
				PacketWriteEndInfo::EndPage
			} else {
				PacketWriteEndInfo::NormalPacket
			};

			let calculated_granule_position = granule_position_for_packet(
				packet_sample_block_size,
				packet_number,
				packet_page_granule_position,
				is_last_stream_packet,
				remuxer_settings,
				stream_state
			);

			// Letting the stream serial addition to overflow is the most sensible thing:
			// the Ogg specification just requires serials to be unique per stream, so by
			// wrapping we make a good use of the available bit space
			let packet_stream_serial = first_stream_serial
				.wrapping_add(stream_serial_increment.wrapping_mul(stream_index));

			// Mangle some Ogg page data. The mangler usually is a no-op
			let packet_stream_serial = remuxer_settings
				.vorbis_stream_mangler
				.mangle_packet_stream_serial(
					packet_stream_serial,
					packet_number,
					is_last_stream_packet
				);
			let page_end_info = remuxer_settings
				.vorbis_stream_mangler
				.mangle_packet_page_end_info(page_end_info, packet_number, is_last_stream_packet);
			let granule_position = remuxer_settings
				.vorbis_stream_mangler
				.mangle_granule_position(
					calculated_granule_position,
					packet_number,
					is_header_packet,
					is_last_stream_packet
				);

			packet_writer.write_packet(
				optimized_packet,
				packet_stream_serial,
				page_end_info,
				// Ogg does not care about the signedness of the granule position, but in Vorbis
				// we may interpret it as a signed integer, and doing so is convenient for us
				granule_position as u64
			)?;

			stream_state.optimized_packet_count =
				stream_state.optimized_packet_count.saturating_add(1);
		}
	}

	Ok(())
}

/// Computes a random serial for the first Vorbis logical bitstream in an Ogg physical
/// bitstream, and the increment to add to that serial with wrapping arithmetic to
/// cheaply generate fairly unique serials for other bitstreams. This should be done
/// at least once per Ogg physical bitstream.
fn random_stream_serial_and_increment(
	first_stream_serial_offset: u32,
	stream_serial_prng_seed_tweak: u32
) -> Result<(u32, u32), RemuxError> {
	let mut random_bytes = [0; 5];
	let source_date_epoch = cfg!(feature = "source-date-epoch")
		.then(|| env::var_os("SOURCE_DATE_EPOCH"))
		.flatten();

	// When a source date epoch is not provided or ignored, try to use OS-provided
	// cryptographically-secure random data for the serial, to avoid the possibility of
	// brute-forcing state data from the serial under certain assumptions. If a source
	// date epoch is available, use a known PRNG with a fixed seed to guarantee
	// reproducibility
	if source_date_epoch.is_some() || getrandom(&mut random_bytes[..]).is_err() {
		/// The PRNG to use when reproducible results are requested via environment variables,
		/// or the system CSPRNG fails.
		static STREAM_SERIAL_PRNG: Mutex<Option<Result<Xoshiro256PlusPlus, ParseIntError>>> =
			Mutex::new(None);

		let mut stream_serial_prng = STREAM_SERIAL_PRNG.lock().unwrap();
		let stream_serial_prng = stream_serial_prng
			.get_or_insert_with(|| {
				source_date_epoch
					.map_or_else(
						|| {
							Ok(UNIX_EPOCH
								.elapsed()
								.unwrap_or_else(|err| err.duration())
								.as_nanos() as u64)
						},
						|timestamp| {
							timestamp
								.to_str()
								.unwrap_or_default()
								// SOURCE_DATE_EPOCH spec: "the value MUST be an ASCII representation
								// of an integer with no fractional component, identical to the output
								// format of date +%s."
								// GNU "date +%s" can output negative numbers (try e.g. faketime
								// '1960-01-01 00:00:00' /bin/date +%s), so accept signed integers here
								.parse::<i128>()
								.map(|timestamp| timestamp as u64)
						}
					)
					.map(|seed| {
						// Expand the 32-bit tweak to 64-bit
						let tweak = {
							let mut hasher = DefaultHasher::new();
							hasher.write_u32(stream_serial_prng_seed_tweak);
							hasher.finish()
						};
						Xoshiro256PlusPlus::seed_from_u64(seed ^ tweak)
					})
			})
			.as_mut();

		#[cfg(feature = "source-date-epoch")]
		let stream_serial_prng = stream_serial_prng.map_err(|_| RemuxError::InvalidSourceDateEpoch)?;
		#[cfg(not(feature = "source-date-epoch"))]
		// Seeding a PRNG can't fail when not parsing env vars
		let stream_serial_prng = stream_serial_prng.unwrap();

		stream_serial_prng.fill_bytes(&mut random_bytes);
	}

	Ok((
		u32::from_ne_bytes(random_bytes[..4].try_into().unwrap())
			.wrapping_add(first_stream_serial_offset),
		// Random odd number, to guarantee that the every possible stream serial will be used
		// when wrapping around on modular arithmetic
		1 + 2 * random_bytes[4] as u32 % 32
	))
}
