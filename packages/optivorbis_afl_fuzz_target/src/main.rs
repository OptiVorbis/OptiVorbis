use std::{io, io::Cursor};

use optivorbis::{OggToOgg, Remuxer, remuxer::ogg_to_ogg::Settings};

fn main() {
	afl::fuzz!(|data: &[u8]| {
		// Each fuzz run has to be pretty fast. Just check that no crashes happen
		OggToOgg::new(
			Settings {
				// Do not randomize serials to increase stability
				randomize_stream_serials: false,
				// Disable checksum verification to not discard interesting data mutations too early
				verify_ogg_page_checksums: false,
				..Default::default()
			},
			Default::default()
		)
		.remux(Cursor::new(data), io::sink())
		.ok();
	})
}
