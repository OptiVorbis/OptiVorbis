use std::io;
use std::io::Cursor;

use optivorbis::remuxer::ogg_to_ogg::Settings;
use optivorbis::{OggToOgg, Remuxer};

fn main() {
	afl::fuzz!(|data: &[u8]| {
		// Each fuzz run has to be pretty fast. Just check that no crashes happen
		OggToOgg::new(
			Settings {
				// Do not randomize serials to increase stability
				randomize_stream_serials: false,
				..Default::default()
			},
			Default::default()
		)
		.remux(Cursor::new(data), io::sink())
		.ok();
	})
}
