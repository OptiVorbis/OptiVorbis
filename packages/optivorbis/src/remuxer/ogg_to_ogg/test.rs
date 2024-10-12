use std::io::Cursor;

use log::LevelFilter;
use oggvorbismeta::CommentHeader;

use super::*;
use crate::{
	vorbis::optimizer::{VorbisCommentFieldsAction, VorbisVendorStringAction},
	OPTIVORBIS_VERSION_TAG
};

fn init_logging() {
	pretty_env_logger::formatted_timed_builder()
		.is_test(true)
		.filter_level(LevelFilter::Info)
		.try_init()
		.ok();
}

fn remux_with_settings<B: AsRef<[u8]>, M: OggVorbisStreamMangler>(
	ogg_vorbis_data: B,
	mut settings_supplier: impl FnMut() -> Settings<M>,
	mut optimizer_settings_supplier: impl FnMut() -> VorbisOptimizerSettings,
	comment_header_predicate: impl FnOnce(CommentHeader)
) -> Result<(), RemuxError> {
	let mut byte_destination = Vec::new();

	OggToOgg::new(settings_supplier(), optimizer_settings_supplier())
		.remux(Cursor::new(ogg_vorbis_data.as_ref()), &mut byte_destination)?;

	comment_header_predicate(oggvorbismeta::read_comment_header(Cursor::new(
		&byte_destination
	)));

	// Dogfooding asserts the invariant that the result of optimizing an Ogg Vorbis
	// file is another Ogg Vorbis file we can parse
	OggToOgg::new(settings_supplier(), optimizer_settings_supplier())
		.remux(Cursor::new(byte_destination), io::sink())?;

	Ok(())
}

#[test]
fn remuxing_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/44100hz_500ms_stereo_400hz_sine_wave_skeleton.ogg"),
		Default::default,
		|| VorbisOptimizerSettings {
			vendor_string_action: VorbisVendorStringAction::AppendTag,
			..Default::default()
		},
		|comment_header| {
			assert!(
				comment_header.vendor.contains(OPTIVORBIS_VERSION_TAG),
				"The vendor string did not contain the version tag"
			)
		}
	)
	.expect("Unexpected remuxing error")
}

#[test]
fn remuxing_with_chaining_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/8khz_2x500ms_mono_400hz_sine_wave_chained.ogg"),
		Default::default,
		Default::default,
		|_| ()
	)
	.expect("Unexpected remuxing error")
}

#[test]
fn remuxing_with_comment_header_changes_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/8khz_500ms_mono_400hz_sine_wave_comments.ogg"),
		Default::default,
		|| VorbisOptimizerSettings {
			vendor_string_action: VorbisVendorStringAction::Empty,
			comment_fields_action: VorbisCommentFieldsAction::Delete
		},
		|comment_header| {
			assert!(
				comment_header.vendor.is_empty() && comment_header.comment_list.is_empty(),
				"Non-empty vendor and/or comment list"
			)
		}
	)
	.expect("Unexpected remuxing error")
}

#[test]
fn remuxing_and_mangling_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/8khz_500ms_mono_400hz_sine_wave.ogg"),
		|| Settings {
			randomize_stream_serials: false,
			first_stream_serial_offset: 0,
			ignore_start_sample_offset: true,
			error_on_no_vorbis_streams: true,
			verify_ogg_page_checksums: true,
			vorbis_stream_mangler: {
				struct Mangler;

				impl OggVorbisStreamMangler for Mangler {
					fn mangle_packet_stream_serial(
						&mut self,
						stream_serial: u32,
						packet_number: usize,
						_is_last_stream_packet: bool
					) -> u32 {
						// Change the stream serial for the setup header.
						// This should render the generated file undecodeable
						if packet_number == 2 {
							1
						} else {
							stream_serial
						}
					}
				}

				Mangler
			}
		},
		Default::default,
		|_| ()
	)
	.expect_err("Expected remuxing error");
}

#[test]
fn empty_last_audio_packet_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/zero_bytes_last_audio_packet.ogg"),
		Default::default,
		Default::default,
		|_| ()
	)
	.expect("Unexpected remuxing error")
}

#[test]
fn non_vorbis_data_returns_error() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/44100hz_500ms_mono_440hz_sine_wave_ogg_opus.ogg"),
		|| Settings {
			error_on_no_vorbis_streams: true,
			..Default::default()
		},
		Default::default,
		|_| ()
	)
	.expect_err("Expected remuxing error");
}

#[test]
fn ogg_page_crc_verification_works() {
	init_logging();

	remux_with_settings(
		include_bytes!("../../../resources/test/crc_mismatch_8khz_500ms_mono_400hz_sine_wave.ogg"),
		|| Settings {
			verify_ogg_page_checksums: true,
			..Default::default()
		},
		Default::default,
		|_| ()
	)
	.expect_err("Expected remuxing error");

	remux_with_settings(
		include_bytes!("../../../resources/test/crc_mismatch_8khz_500ms_mono_400hz_sine_wave.ogg"),
		|| Settings {
			verify_ogg_page_checksums: false,
			..Default::default()
		},
		Default::default,
		|_| ()
	)
	.expect("Unexpected remuxing error");
}
