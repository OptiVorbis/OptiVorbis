//! Contains the Vorbis codebook abstraction and related entropy coding functions.

use std::{
	cell::Cell,
	io,
	io::{ErrorKind, Read}
};

use huffman_codeword_lengths::VorbisCodebookNumberFrequenciesDecorator;
use huffman_tree::{
	TryFromCodewordLengthsListError, VorbisHuffmanTree, VorbisHuffmanTreeWalkerError
};
use log::trace;
use thiserror::Error;
use vorbis_bitpack::BitpackReader;

mod huffman_codeword_lengths;
mod huffman_tree;

/// An error that may happen when instantiating a Vorbis codebook.
#[derive(Debug, Error)]
#[allow(variant_size_differences)] // io::Error is 14 bytes larger than other variants
pub enum VorbisCodebookError {
	/// An attempt to build a codebook with invalid codewords was made.
	#[error("Codebook {codebook_number} is invalid: {error}")]
	InvalidCodebookCodewords {
		/// The number of the involved codebook.
		codebook_number: u16,
		/// The internal error information. Any change to this field is considered non-breaking.
		#[doc(hidden)]
		error: TryFromCodewordLengthsListError
	},
	/// The tree could not be walked while decoding an entry number.
	#[error("Codebook {codebook_number} entry decode error: {error}")]
	CodebookTreeWalkError {
		/// The number of the involved codebook.
		codebook_number: u16,
		/// The internal error information. Any change to this field is considered non-breaking.
		#[doc(hidden)]
		error: VorbisHuffmanTreeWalkerError
	},
	/// EOF was reached while decoding an entry number.
	#[error("Codebook {codebook_number} entry decode error: end of packet while decoding entry")]
	EofWhileDecodingEntry {
		/// The number of the involved codebook.
		codebook_number: u16
	},
	/// An I/O error happened while decoding an entry number.
	#[error("I/O error decoding codebook entry: {0}")]
	IoError(#[from] io::Error)
}

/// A Vorbis codebook, used for lossless entropy coding of entry numbers that may be used
/// as-is in scalar contexts, or interpreted as a vector quantization table index in
/// vector contexts.
///
/// Due to the needs of this crate, this struct can work in two modes:
///
/// - _Decode frequency recording mode_, where the times that
///   [`decode_entry_number`](VorbisCodebook::decode_entry_number) returns every entry
///   are recorded. In other words, this mode records the entry number frequencies.
/// - _Optimizing mode_, after
///   [`optimal_codeword_lengths`](VorbisCodebook::optimal_codeword_lengths) is called.
///   In this mode the decoding frequencies are no longer updated on read. It is not
///   possible to go back to the previous mode once this one is reached.
///
/// When created, a codebook is in _decode frequency recording mode_.
pub(super) struct VorbisCodebook {
	pub(super) codebook_number: u16,
	huffman_tree: VorbisHuffmanTree,
	entry_decode_frequencies_or_lengths: Cell<Vec<u64>>,
	recording_decode_frequencies: bool
}

impl VorbisCodebook {
	/// Creates a new codebook from the specified list of codeword lengths.
	///
	/// The codebook number is used on error messages only, and it is not
	/// relevant for the operation of the struct.
	pub(super) fn new<T: AsRef<[u8]>>(
		codebook_number: u16,
		codeword_lengths: T
	) -> Result<Self, VorbisCodebookError> {
		Ok(Self {
			codebook_number,
			entry_decode_frequencies_or_lengths: Cell::new(vec![
				0;
				codeword_lengths.as_ref().len()
			]),
			huffman_tree: VorbisHuffmanTree::try_from_codeword_lengths(codeword_lengths).map_err(
				|error| VorbisCodebookError::InvalidCodebookCodewords {
					codebook_number,
					error
				}
			)?,
			recording_decode_frequencies: true
		})
	}

	/// Decodes an entry number using this codebook, reading bits from the
	/// specified bitpack reader as needed.
	///
	/// This method will also update an internal list of decoding frequencies
	/// in the _decode frequency recording mode_.
	// Decoding entry numbers is a very hot function, with well over a half of
	// the execution time being spent here, as indicated by the perf profiler.
	// This is caused due to audio packet residue decode. Any performance
	// improvement here will be great
	pub(super) fn decode_entry_number<R: Read>(
		&self,
		bitpack_reader: &mut BitpackReader<R>
	) -> Result<u32, VorbisCodebookError> {
		self.huffman_tree.with_walker(|mut walker| {
			// Read a single bit from the bitstream until the word read so far
			// can be decoded to an entry number. This loop is guaranteed to
			// terminate by definition: either we reach a leaf with no children
			// and know that either the tree is underspecified or the stream is
			// corrupt, or the end of packet is reached
			loop {
				if let Some(entry) = walker
					.walk(
						// Due to the tree construction, branch left = 0, and branch right = 1
						// (codewords are assigned left to right)
						bitpack_reader.read_flag().map_err(|err| {
							if err.kind() == ErrorKind::UnexpectedEof {
								// Dedicating a variant to this fairly common case improves
								// error messages quite a bit
								VorbisCodebookError::EofWhileDecodingEntry {
									codebook_number: self.codebook_number
								}
							} else {
								err.into()
							}
						})?
					)
					.map_err(|error| VorbisCodebookError::CodebookTreeWalkError {
						codebook_number: self.codebook_number,
						error
					})? {
					if self.recording_decode_frequencies {
						let mut entry_decode_frequencies =
							self.entry_decode_frequencies_or_lengths.take();

						entry_decode_frequencies[entry.number as usize] =
							entry_decode_frequencies[entry.number as usize].saturating_add(1);

						self.entry_decode_frequencies_or_lengths
							.set(entry_decode_frequencies);
					}

					trace!(
						"Reading entry {} using codebook {}",
						entry.number,
						self.codebook_number
					);

					return Ok(entry.number);
				}
			}
		})
	}

	/// Computes the optimal codeword length for every entry, transitioning this
	/// codebook into _optimizing mode_. The element in position `n` of the slice
	/// represents the number of times the entry number `n` has been decoded so far.
	///
	/// This is an in-place operation that does not consume any additional memory once
	/// it finishes. It executes in O(n log n) the first time it is called, but the
	/// result is memoized, so the next invocations are virtually free.
	pub(super) fn optimal_codeword_lengths(&mut self) -> &[u64] {
		if self.recording_decode_frequencies {
			self.recording_decode_frequencies = false;
			VorbisCodebookNumberFrequenciesDecorator::new(
				self.entry_decode_frequencies_or_lengths.get_mut()
			)
			.into_huffman_codeword_lengths()
		} else {
			self.entry_decode_frequencies_or_lengths.get_mut()
		}
	}

	/// Computes the optimal codewords for every entry, implicitly transitioning
	/// this codebook into _optimizing mode_ if necessary. The element in position
	/// `n` of the returned `Vec` is a `(codeword, codeword_length)` pair for the
	/// entry number `n`. Unused entries are marked with a `None` value.
	///
	/// This is a relatively expensive operation. Callers are encouraged to use
	/// its result for as long as possible.
	pub(super) fn optimal_codewords(&mut self) -> Vec<Option<(u32, u8)>> {
		// Unwrap is safe: we trust that our codeword length computation code works
		VorbisHuffmanTree::try_codewords_from_codeword_lengths(self.optimal_codeword_lengths())
			.unwrap()
	}
}
