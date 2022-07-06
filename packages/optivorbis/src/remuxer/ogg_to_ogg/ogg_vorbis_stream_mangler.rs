//! Contains the [`OggVorbisStreamMangler`] trait definition and a default
//! pass-through mangler implementation.
//!
//! **Manglers are an advanced feature** meant for low-level repairs of Ogg
//! Vorbis streams, development and research purposes. Most users will not
//! need any other mangler than the default
//! [`OggVorbisStreamPassthroughMangler`], which does nothing.

use std::num::NonZeroU32;

use ogg::PacketWriteEndInfo;

/// A mangler for Ogg Vorbis streams, capable of changing some of the Ogg
/// page encapsulation values and parameters that OptiVorbis automatically
/// generates for Vorbis streams just before they are written.
///
/// **Manglers are an advanced feature** meant for low-level repairs of Ogg
/// Vorbis streams, development and research purposes. Most users will not
/// need any other mangler than the default
/// [`OggVorbisStreamPassthroughMangler`], which does nothing.
///
/// Every mangler method has a default implementation that passes through
/// the values computed by OptiVorbis. Therefore, it only is necessary to
/// override the methods where this behavior is not desired.
pub trait OggVorbisStreamMangler {
	/// Returns the sampling frequency to write in the Vorbis identification header.
	/// The passed `sampling_frequency` is taken as-is from the original stream.
	///
	/// As the Vorbis codec is only concerned about audio samples, the sampling
	/// frequency is used by players to convert between sample counts and time.
	/// Obviously, it also is used to decide how many audio samples should be played
	/// per unit of time, dictating the perceived speed.
	fn mangle_sampling_frequency(&mut self, sampling_frequency: NonZeroU32) -> NonZeroU32 {
		sampling_frequency
	}

	/// Returns the values to write in the Vorbis identification header bitrate
	/// fields, in order of minimum, nominal and maximum bitrate. Bitrates may
	/// be used by decoders to size buffers, roughly estimate the total audio
	/// length, and other purposes. The passed values are taken as-is from the
	/// original stream.
	fn mangle_bitrates(
		&mut self,
		minimum_bitrate: i32,
		nominal_bitrate: i32,
		maximum_bitrate: i32
	) -> (i32, i32, i32) {
		(minimum_bitrate, nominal_bitrate, maximum_bitrate)
	}

	/// Returns the stream serial to associate with a packet. The
	/// `stream_serial` parameter contains the serial computed by OptiVorbis,
	/// which is guaranteed to be valid.
	#[allow(unused_variables)]
	fn mangle_packet_stream_serial(
		&mut self,
		stream_serial: u32,
		packet_number: usize,
		is_last_stream_packet: bool
	) -> u32 {
		stream_serial
	}

	/// Returns the page end information to pass along to the `ogg` crate
	/// when writing a packet. The `packet_end_info` parameter contains the
	/// end information computed by OptiVorbis, which is guaranteed to be
	/// valid.
	#[allow(unused_variables)]
	fn mangle_packet_page_end_info(
		&mut self,
		packet_end_info: PacketWriteEndInfo,
		packet_number: usize,
		is_last_stream_packet: bool
	) -> PacketWriteEndInfo {
		packet_end_info
	}

	/// Returns the granule position to associate with a Vorbis audio packet.
	///
	/// Granule positions are used to convey playback position information to players,
	/// and stored per Ogg page. As each Ogg page may contain several complete audio
	/// packets, or even an incomplete audio packet, per-packet would be granule
	/// positions are mapped to per-page granule positions by only taking into account
	/// the granule position of the last completed packet in a page.
	///
	/// The calculated granule position is usable, standards-conformant and thus fine in
	/// the vast majority of cases.
	#[allow(unused_variables)]
	fn mangle_granule_position(
		&mut self,
		calculated_granule_position: i64,
		packet_number: usize,
		is_header_packet: bool,
		is_last_stream_packet: bool
	) -> i64 {
		calculated_granule_position
	}
}

/// A [mangler](OggVorbisStreamMangler) that just passes through OptiVorbis-calculated
/// values without any modification, suitable for most purposes.
pub struct OggVorbisStreamPassthroughMangler;

impl OggVorbisStreamMangler for OggVorbisStreamPassthroughMangler {}
