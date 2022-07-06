//! Contains the [`Remuxer`] trait definition and other optimizing remuxing related code. Start here.

use std::{
	error::Error,
	io::{Read, Seek, Write}
};

use crate::vorbis::optimizer::VorbisOptimizerSettings;

pub mod ogg_to_ogg;

/// Defines the contract for any remuxer, responsible for reading Vorbis streams from a container,
/// optimizing them and encapsulating their optimized representation to a container.
pub trait Remuxer {
	/// The error type that the remuxer might return if something goes wrong during its operation.
	type RemuxError: Error;

	/// The type of the settings that may be used to customize the how the remuxer reads and
	/// encapsulates packets to a container.
	type RemuxerSettings;

	/// Creates a remuxer for a container format that will optimize Vorbis streams according to
	/// the provided settings.
	fn new(
		remuxer_settings: Self::RemuxerSettings,
		optimizer_settings: VorbisOptimizerSettings
	) -> Self
	where
		Self: Sized;

	/// Creates a remuxer for a container format that will optimize Vorbis streams according
	/// to its default options.
	fn new_with_defaults() -> Self
	where
		Self: Sized,
		Self::RemuxerSettings: Default
	{
		Self::new(
			Self::RemuxerSettings::default(),
			VorbisOptimizerSettings::default()
		)
	}

	/// Demuxes the container from `source`, optimizing its Vorbis streams, and then muxes their
	/// optimized representations to maybe another container to `sink`. In addition to being
	/// seekable, `source` is required to have a fixed size (i.e., it must not never return more
	/// data after EOF is reached).
	///
	/// Implementations of this method are required to not assume that the passed `source` stream
	/// position is initially zero. In other words, if they need to rewind `source`, they must set
	/// its stream position to the position it had when this method was called, not zero. No
	/// guarantees are made about the stream position of either `source` or `sink` when this method
	/// returns.
	fn remux<R: Read + Seek, W: Write>(&self, source: R, sink: W) -> Result<W, Self::RemuxError>;
}
