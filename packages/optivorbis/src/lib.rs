//! OptiVorbis is a Rust library for lossless optimization and repair of complete,
//! seekable Vorbis I audio streams (files), as defined in the [Vorbis I
//! specification]. It leverages the great flexibility that the Vorbis I stream
//! setup configuration header provides to achieve its goals.
//!
//! The optimization is _lossless_: streams processed by this library are guaranteed
//! to be decoded to the same audio samples as before by any compliant decoder. The
//! internal file structure may differ substantially, but these differences are
//! transparent for end-users. At the user's discretion, it is possible to save even
//! more space by removing comments (track title, author, etc.) and encoder version
//! information.
//!
//! In addition, OptiVorbis' understanding of the Vorbis format and container
//! encapsulations, combined with the somewhat more lenient, purpose-built parsers
//! it uses, provide it with some repair capabilities. It also tends to output more
//! detailed and actionable error messages on failure than other tools.
//!
//! # Remuxers
//!
//! Typically, Vorbis streams are neither stored nor transmitted raw, as the Vorbis
//! format does not provide any support for features such as seeking or packet loss
//! handling. Nevertheless, to make this library as flexible as possible, its
//! optimization logic is coupled only with the Vorbis streams and does not assume
//! any particular container. _Remuxers_ handle extracting Vorbis streams and
//! encapsulating their optimized representations from and to containers,
//! respectively.
//!
//! The available remuxers can be found in the [`remuxer`] module. Remuxers are the
//! recommended entry point for the API offered by this library.
//!
//! The lower-level, container-agnostic [`VorbisOptimizer`], which deals with the
//! optimization of raw Vorbis streams, is another entry point for the API. However,
//! the main reason this type is exposed is to enable the implementation of
//! third-party remuxers or other advanced use cases. Most end-users should try
//! the simpler remuxing interfaces first.
//!
//! ## Ogg Vorbis to Ogg Vorbis remuxer
//!
//! OptiVorbis currently ships with an Ogg Vorbis to Ogg Vorbis remuxer, which deals
//! with Vorbis packets on unmultiplexed Ogg containers, usually stored in files
//! with a `.ogg` extension. These Ogg Vorbis streams must not contain other
//! interleaved streams, such as a video stream, but may contain chained
//! (concatenated) Ogg Vorbis streams. Non-Vorbis streams (for example, Ogg Skeleton
//! metadata streams) are ignored and not copied. Granule positions (timestamps) are
//! recomputed, fixing any incorrect information that may be present in the original
//! stream. Non-zero initial timestamps, chiefly used in live recordings and for
//! lossless sample truncation of Vorbis streams, are supported. Stream serials may
//! be randomized, making it easier to concatenate (chain) several generated files
//! together. This encapsulation is the most common means of storage and distribution
//! of Vorbis audio streams.
//!
//! # Implemented optimizations
//!
//! For technically-inclined people curious about what kind of low-level magic
//! OptiVorbis does, here lies a brief overview of its optimization techniques.
//!
//! - **Codebook [Huffman codes](https://en.wikipedia.org/wiki/Huffman_coding)
//!   recoding**. Vorbis is designed to support live audio streaming applications,
//!   so virtually every encoder assumes a non-optimal, fixed symbol probability
//!   distribution based on the bitrate, the number of channels, sampling frequency
//!   and target bitrate to avoid imposing buffering requirements. OptiVorbis
//!   disregards this requirement, which allows it to know the real symbol probability
//!   distribution and generate optimal Huffman codes.
//! - _(Optional)_ **Comment header emptying**.
//! - _(For the Ogg Vorbis encapsulation)_ **Muxing overhead reduction**, achieved
//!   by putting as many Vorbis packets per Ogg page as possible. This might slow
//!   down seeking operations and negate the error detection capabilities of smaller
//!   pages but, in turn, speeds up sequential decoding, which is a better tradeoff
//!   for files.
//!
//! # Repair capabilities
//!
//! OptiVorbis is not meant to be a thorough repair tool that turns any corrupt
//! stream into something usable. However, it does not enforce some checks mandated
//! by the specification on purpose, with the expectation of making it able to
//! accept and fix streams that were accidentally slightly broken. It also handles
//! some technically legal but practically troublesome edge cases carefully, making
//! the generated files more interoperable:
//!
//! - Unnecessary packet padding is removed, which also saves space.
//! - Window, mapping, time-domain transform types, and some reserved fields which
//!   should always be set to zero are not checked. These fields are meant to allow
//!   future expansion of the Vorbis codec, but in practice, it seems extremely
//!   unlikely that these will ever be used without bumping the Vorbis version.
//! - The codebook sync pattern is read but not checked.
//! - Underspecified Huffman decode trees are accepted as long as the audio packets
//!   do not contain codewords that belong to the underspecified sections of the
//!   tree.
//! - Codebooks with a single used entry whose codeword length is not one are
//!   accepted. This assigns an inefficient all-zeros codeword to that single entry
//!   but follows the apparent intent of the codeword assignment algorithm described
//!   in the specification.
//! - Truncated comment headers are not considered a fatal error, as mandated by the
//!   specification, and are de-truncated.
//! - Strings in the comment header with invalid UTF-8 characters are not considered
//!   errors to keep their original values intact. It is possible to replace these
//!   with valid UTF-8 strings with the appropriate settings.
//! - Audio packets that should be completely discarded by decoders according to the
//!   specification are removed, yielding identical results for the reference
//!   implementation, while saving more troublesome implementations the hassle of
//!   dealing with such a case.
//!
//! Please note that these repair capabilities are not meant to be an excuse for
//! encoders to be buggy or non-conformant. Encoders should generate correct and
//! interoperable output no matter what, and end-users will appreciate it. If you
//! are a Vorbis encoder developer and rely on OptiVorbis to generate proper
//! streams, your encoder is either incomplete or doing something wrong.
//!
//! These repair capabilities may change as the library matures and more information
//! about how Vorbis streams are corrupted and used in obscure scenarios is gathered.
//!
//! # Known limitations
//!
//! Vorbis streams that report being encoded with floor format 0 are not supported.
//! While such format is not deprecated according to the standard, it has been
//! effectively superseded by the floor format 1 for all intents, purposes, and
//! known encoders for more than 20 years, so this limitation should not matter in
//! practice. Some decoders do not support this format either, rendering it less
//! interoperable in practice.
//!
//! # Logging
//!
//! This crate uses the [`log`](https://crates.io/crates/log) crate for logging status
//! and diagnostic messages to any interested consumers. Executables can customize the
//! verbosity of this logging, and even compile it out, [as explained in the `log`
//! crate documentation](https://docs.rs/log/0.4.16/log).
//!
//! # Examples
//!
//! The following example shows how to optimize and perform basic repairs on an Ogg
//! Vorbis file, saving the result to another file. Every setting is set to a default
//! value.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::fs::File;
//! use std::io::{Seek, SeekFrom};
//! use optivorbis::{Remuxer, OggToOgg};
//!
//! # #[cfg(__example_only)]
//! let mut original_file = File::open("original_audio.ogg")?;
//! # let mut original_file = std::io::Cursor::new(include_bytes!("../resources/test/8khz_500ms_mono_400hz_sine_wave.ogg"));
//! # #[cfg(__example_only)]
//! let mut optimized_file = File::create("optimized_audio.ogg")?;
//! # let mut optimized_file = std::io::sink();
//!
//! OggToOgg::new_with_defaults().remux(&mut original_file, &mut optimized_file)?;
//!
//! println!("Original file size: {} bytes", original_file.seek(SeekFrom::End(0))?);
//! # #[cfg(__example_only)]
//! println!("Optimized file size: {} bytes", optimized_file.seek(SeekFrom::End(0))?);
//! # Ok(())
//! # }
//! ```
//!
//! # Acknowledgements
//!
//! The ideas for the optimization techniques implemented in this library were
//! inspired by Segher Boessenkool's work on the incomplete, source-available
//! [`rehuff`] tool published in 2002. No source code was taken from this program.
//!
//! The Ogg page granule position computation code was deduced from a careful
//! reading of the Vorbis I specification and then confirmed correct by examining
//! the source of both [`revorb`] and [`liboggz`].
//!
//! [Vorbis I specification]: https://xiph.org/vorbis/doc/Vorbis_I_spec.pdf
//! [`rehuff`]: https://wiki.xiph.org/Rehuff
//! [`revorb`]: https://github.com/jonboydell/revorb-nix
//! [`liboggz`]: https://gitlab.xiph.org/xiph/liboggz

#![doc(
	html_logo_url = "https://github.com/OptiVorbis/OptiVorbis/raw/master/web/src/static/optivorbis_logo.png"
)]
#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![deny(non_ascii_idents)]
#![deny(clippy::print_stdout)]
#![deny(clippy::unimplemented)]
#![warn(explicit_outlives_requirements)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::empty_enum)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::invalid_upcast_comparisons)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::use_self)]
#![warn(clippy::used_underscore_binding)]
#![warn(clippy::redundant_feature_names)]

pub use remuxer::ogg_to_ogg::OggToOgg;
pub use remuxer::Remuxer;
#[doc(inline)]
pub use vorbis::codebook::VorbisCodebookError;
#[doc(inline)]
pub use vorbis::optimizer::{
	VorbisCommentFieldsAction, VorbisOptimizer, VorbisOptimizerError, VorbisOptimizerSettings,
	VorbisVendorStringAction
};
#[doc(inline)]
pub use vorbis::{
	PacketType, TryPacketTypeFromInt, TryResidueTypeFromInt, TryVectorLookupTypeFromInt
};

/// A text tag that precisely identifies this OptiVorbis build.
pub static OPTIVORBIS_VERSION_TAG: &str = concat!(
	"OptiVorbis ",
	env!("OPTIVORBIS_VERSION"),
	" (",
	env!("OPTIVORBIS_BUILD_DATE"),
	")"
);
/// A shorter text tag that identifies this OptiVorbis build.
pub static OPTIVORBIS_SHORT_VERSION_TAG: &str = concat!("OV ", env!("OPTIVORBIS_VERSION"));

pub mod remuxer;
mod vorbis;

#[cfg(feature = "wasm-bindings")]
mod wasm_bindings;
