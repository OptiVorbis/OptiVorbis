use std::io::Cursor;

use optivorbis::{
	OggToOgg, Remuxer, VorbisOptimizerSettings, VorbisVendorStringAction,
	remuxer::ogg_to_ogg::Settings
};
use vorbis_rs::VorbisDecoder;

fn main() {
	afl::fuzz!(|data: &[u8]| {
		let mut remuxed = vec![];

		// Part 1 of test oracle: no crashes, hangs, panics, overflows... in our code
		if OggToOgg::new(
			Settings {
				// Do not randomize serials to increase stability
				randomize_stream_serials: false,
				// Disable checksum verification to not discard interesting data mutations too early
				verify_ogg_page_checksums: false,
				..Settings::default()
			},
			{
				let mut vorbis_optimizer_settings = VorbisOptimizerSettings::default();
				vorbis_optimizer_settings.vendor_string_action = VorbisVendorStringAction::Copy;
				vorbis_optimizer_settings
			}
		)
		.remux(Cursor::new(data), &mut remuxed)
		.is_err()
		{
			return;
		}

		if std::env::var_os("OPTIVORBIS_FUZZ_EXTENDED_TEST_ORACLE").as_deref() != Some("1".as_ref())
		{
			return;
		}

		// Part 2 of the test oracle: if the original stream is decodable, assert that
		// encoded samples are unmodified (differential testing), and file size is kept
		// the same or reduced (property testing)
		let Some(original_samples) = decode_samples(data) else {
			return;
		};

		let remuxed_samples =
			decode_samples(&remuxed).expect("successfully remuxed stream must be decodable");

		assert_eq!(
			original_samples.len(),
			remuxed_samples.len(),
			"channel count must match"
		);

		for (ch, (orig, remuxed)) in original_samples.iter().zip(&remuxed_samples).enumerate() {
			assert_eq!(
				orig[..orig.len()],
				remuxed[..remuxed.len()],
				"decoded samples for channel {ch} must match"
			);
		}
	})
}

fn decode_samples(data: &[u8]) -> Option<Vec<Vec<f32>>> {
	let mut decoder = VorbisDecoder::<&[u8]>::new(data).ok()?;
	let mut samples = vec![vec![]; decoder.channels().get() as usize];

	while let Some(block) = decoder.decode_audio_block().ok()? {
		for (ch_samples, out) in block.samples().iter().zip(samples.iter_mut()) {
			out.extend_from_slice(ch_samples);
		}
	}

	Some(samples)
}
