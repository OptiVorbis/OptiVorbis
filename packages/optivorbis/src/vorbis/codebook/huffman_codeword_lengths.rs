//! Contains helper functions for computing optimal codeword lengths from symbol frequencies.
//!
//! The functions are based on the Algorithm 2 described in [A. Moffat, ‘Huffman Coding’,
//! ACM Comput. Surv., volume 52, issue 4, August 2019].
//!
//! [A. Moffat, ‘Huffman Coding’, ACM Comput. Surv., volume 52, issue 4, August 2019]:
//! https://dl.acm.org/doi/10.1145/3342555

use std::ops::{Index, IndexMut};

/// Decorates a slice of codebook number frequencies to be suitable for computing codeword
/// lengths.
pub(super) struct VorbisCodebookNumberFrequenciesDecorator<T: AsMut<[u64]> + AsRef<[u64]>> {
	number_frequencies: T,
	number_index_map: Vec<usize>
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> VorbisCodebookNumberFrequenciesDecorator<T> {
	/// Creates this decorator. This operation performs a comparison sort, so it takes
	/// O(n log n) time.
	///
	/// # Preconditions
	/// The length of the `number_frequencies` array fits in a signed 32-bit integer.
	/// This is not a concern due to codec setup header construction.
	pub(super) fn new(number_frequencies: T) -> Self {
		let mut number_index_map;
		{
			let number_frequencies = number_frequencies.as_ref();

			// The codeword length calculation algorithm we use is efficient, but has two limitations:
			// - It does not ignore unused entries, with frequency zero. They are assigned a codeword
			//   too.
			// - It assumes that codebook entry numbers are sorted by decreasing frequency, which
			//   may not be the case (i.e., codebook entry number 0 may be more frequent than entry 1).
			// To address these limitations, create a temporary index mapping array, as hinted in
			// section 5 of the "In-Place Calculation of Minimum-Redundancy Codes" paper, "Other
			// Considerations". This array maps virtual indexes to indexes in the proper array, so
			// that the element at position 0 is the entry number that was most frequent in the
			// original frequencies array, and so on, skipping unused entries. The result is that the
			// codeword length computation algorithm gets a randomly-indexable and sorted view of the
			// original array with the unused entries removed
			number_index_map = Vec::with_capacity(number_frequencies.len());
			for (index, frequency) in number_frequencies.iter().copied().enumerate() {
				if frequency != 0 {
					number_index_map.push(index);
				}
			}
			number_index_map
				.sort_unstable_by(|i, j| number_frequencies[*j].cmp(&number_frequencies[*i]));
			number_index_map.shrink_to_fit(); // Save memory in case there are lots of unused entries
		}

		Self {
			number_frequencies,
			number_index_map
		}
	}

	/// Consumes this decorator and stores the optimal codeword lengths in the
	/// decorated slice, which was holding symbol frequencies before. Thus,
	/// this operation is in place. It executes in O(n) time.
	pub(super) fn into_huffman_codeword_lengths(mut self) -> T {
		let used_codeword_count = self.number_index_map.len();

		match used_codeword_count {
			0 => {
				// The codeword length is not well-defined for an empty set of codewords.
				// We won't use these codeword lengths down the line, but erroring out
				// would be a non-conformance (or so we think). Therefore, do nothing.
				// This is arguably an annoying inelegance in the Vorbis I specification
				self.number_frequencies
			}
			1 => {
				// A single used entry is also a degenerate case, and our general algorithm
				// can't handle that. Special-case it to use a codeword length of 1, as
				// mandated by the Vorbis I specification
				self[0] = 1;
				self.number_frequencies
			}
			used_codeword_count => {
				// Our general algorithm can handle the rest of possible used entry counts
				compute_huffman_codeword_lengths(self, used_codeword_count).number_frequencies
			}
		}
	}
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> Index<usize> for VorbisCodebookNumberFrequenciesDecorator<T> {
	type Output = u64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.number_frequencies.as_ref()[self.number_index_map[index]]
	}
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> IndexMut<usize>
	for VorbisCodebookNumberFrequenciesDecorator<T>
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.number_frequencies.as_mut()[self.number_index_map[index]]
	}
}

/// Implementation of the "Huffman coding" paper algorithm to compute codeword
/// lengths efficiently and in-place.
fn compute_huffman_codeword_lengths<T: IndexMut<usize, Output = u64>>(
	number_frequencies: T,
	frequency_count: usize
) -> T {
	let mut w = number_frequencies;

	// The rest of this function is a Rust translation of the paper pseudocode

	// First phase
	let mut leaf = (frequency_count - 1) as i32;
	let mut root = (frequency_count - 1) as i32;
	for next in (1..frequency_count as i32).rev() {
		// Find first child
		if leaf < 0 || (root > next && w[root as usize] < w[leaf as usize]) {
			// Use internal node
			w[next as usize] = w[root as usize];
			w[root as usize] = next as u64;
			root -= 1;
		} else {
			// Use leaf node
			w[next as usize] = w[leaf as usize];
			leaf -= 1;
		}

		// Find second child
		if leaf < 0 || (root > next && w[root as usize] < w[leaf as usize]) {
			// Use internal node
			w[next as usize] += w[root as usize];
			w[root as usize] = next as u64;
			root -= 1;
		} else {
			// Use leaf node
			w[next as usize] += w[leaf as usize];
			leaf -= 1;
		}
	}

	// Second phase
	w[1] = 0;
	for next in 2..frequency_count {
		w[next] = w[w[next] as usize] + 1;
	}

	// Third phase
	let mut available = 1;
	let mut used = 0;
	let mut depth = 0;
	root = 1;
	let mut next = 0;
	while available > 0 {
		// Count internal nodes used at depth
		while root < frequency_count as i32 && w[root as usize] == depth {
			used += 1;
			root += 1;
		}

		// Assign as leaves any nodes that are not internal
		while available > used {
			w[next] = depth;
			next += 1;
			available -= 1;
		}

		// Move to next depth
		available = 2 * used;
		depth += 1;
		used = 0;
	}

	w
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::vorbis::codebook::VorbisCodebook;

	#[test]
	fn compute_huffman_codeword_lengths_works() {
		// Example frequencies array from the "Huffman Coding" paper by A. Moffat, 2019.
		const PAPER_EXAMPLE_FREQUENCIES_ARRAY: [u64; 10] = [20, 17, 6, 3, 2, 2, 2, 1, 1, 1];
		const PAPER_EXAMPLE_CODELENGTHS_RESULT: [u64; 10] = [1, 2, 4, 5, 5, 5, 5, 5, 6, 6];
		const PAPER_EXAMPLE_CODELENGTHS_RESULT_U8: [u8; 10] = [1, 2, 4, 5, 5, 5, 5, 5, 6, 6];

		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new(PAPER_EXAMPLE_FREQUENCIES_ARRAY)
				.into_huffman_codeword_lengths(),
			PAPER_EXAMPLE_CODELENGTHS_RESULT
		);

		VorbisCodebook::new(0, PAPER_EXAMPLE_CODELENGTHS_RESULT_U8).expect(
			"It should be possible to construct a Huffman tree with the computed codeword lengths"
		);
	}

	#[test]
	fn compute_huffman_codeword_lengths_works_for_unsorted_and_unused_entries() {
		// Should yield the same codelengths than the verbatim paper example, but taking into
		// account the different ordering and the presence of unused entries, which should stay
		// with a zero codeword length
		const TWEAKED_PAPER_EXAMPLE_FREQUENCIES_ARRAY: [u64; 11] =
			[1, 20, 2, 1, 6, 0, 2, 2, 3, 1, 17];
		const TWEAKED_PAPER_EXAMPLE_CODELENGTHS_RESULT: [u64; 11] =
			[5, 1, 5, 6, 4, 0, 5, 5, 5, 6, 2];
		const TWEAKED_PAPER_EXAMPLE_CODELENGTHS_RESULT_U8: [u8; 11] =
			[5, 1, 5, 6, 4, 0, 5, 5, 5, 6, 2];

		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new(TWEAKED_PAPER_EXAMPLE_FREQUENCIES_ARRAY)
				.into_huffman_codeword_lengths(),
			TWEAKED_PAPER_EXAMPLE_CODELENGTHS_RESULT
		);

		VorbisCodebook::new(0, TWEAKED_PAPER_EXAMPLE_CODELENGTHS_RESULT_U8).expect(
			"It should be possible to construct a Huffman tree with the computed codeword lengths"
		);
	}

	#[test]
	fn compute_huffman_codeword_lengths_works_for_no_used_entries() {
		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new([]).into_huffman_codeword_lengths(),
			[]
		);

		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new([0, 0, 0])
				.into_huffman_codeword_lengths(),
			[0, 0, 0]
		);
	}

	#[test]
	fn compute_huffman_codeword_lengths_works_for_single_used_entry() {
		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new([22]).into_huffman_codeword_lengths(),
			[1],
			"Single-used entry codebooks should have a codeword length of 1 for that entry"
		);

		assert_eq!(
			VorbisCodebookNumberFrequenciesDecorator::new([0, 22, 0, 0])
				.into_huffman_codeword_lengths(),
			[0, 1, 0, 0],
			"Single-used entry codebooks should have a codeword length of 1 for that entry"
		);
	}
}
