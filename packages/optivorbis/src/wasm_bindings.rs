#![allow(unreachable_pub)]
// The wasm_bindgen macro generates these warnings
#![allow(clippy::use_self)]

use std::{io::Cursor, panic};

use wasm_bindgen::prelude::*;

use crate::remuxer::{
	ogg_to_ogg::{OggToOgg as OggToOggRemuxer, OggVorbisStreamPassthroughMangler},
	Remuxer
};

#[wasm_bindgen(start)]
#[allow(dead_code)] // Used by the WASM module entrypoint
pub fn main() {
	// Make logging and error handling use the console on web browsers
	#[cfg(feature = "wasm-web-bindings")]
	{
		panic::set_hook(Box::new(console_error_panic_hook::hook));
		console_log::init().ok();
	}
}

#[wasm_bindgen]
pub struct OggToOgg {
	inner: OggToOggRemuxer<OggVorbisStreamPassthroughMangler>
}

#[wasm_bindgen]
impl OggToOgg {
	/// Creates an Ogg to Ogg remuxer with the default options.
	///
	/// Equivalent to `OggToOgg::new_with_defaults()`.
	#[wasm_bindgen(constructor)]
	pub fn new_with_defaults() -> Self {
		Self {
			inner: OggToOggRemuxer::new_with_defaults()
		}
	}

	/// Remuxes the specified Ogg Vorbis data to a new, optimized representation
	/// in another buffer, using the default settings. Any error that may occur
	/// is converted to a string and thrown in an exception.
	///
	/// Equivalent to `OggToOgg::remux(&self, ...)`.
	pub fn remux(&self, buf: &[u8]) -> Result<Box<[u8]>, String> {
		let mut sink = Vec::with_capacity(buf.len() / 2);

		self.inner
			.remux(Cursor::new(buf), &mut sink)
			.map_err(|err| err.to_string())?;

		Ok(sink.into_boxed_slice())
	}
}
