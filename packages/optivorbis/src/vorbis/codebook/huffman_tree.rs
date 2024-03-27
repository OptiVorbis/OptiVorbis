//! Contains the [`VorbisHuffmanTree`] definition and implementation.

// Workaround for Ouroboros issue: https://github.com/joshua-maros/ouroboros/issues/91
#![allow(clippy::useless_transmute)]

use std::fmt::{Debug, Formatter};

use bumpalo::Bump;
use ouroboros::self_referencing;
use thiserror::Error;

/// Represents an error that may happen while dealing with a codewords list.
#[derive(Debug, Error)]
pub enum TryFromCodewordLengthsListError {
	/// The codeword lengths overspecify a Huffman tree (i.e., indicate that it
	/// has a leaf in an illegal position).
	#[error("The codeword lengths list described an overspecified Huffman tree")]
	OverspecifiedTree
}

/// Represents an error that may happen while walking down a [`VorbisHuffmanTree`].
#[derive(Debug, Error)]
pub enum VorbisHuffmanTreeWalkerError {
	/// An unassigned leaf was reached while walking down the tree.
	#[error("An attempt to use an underspecified region of the codebook Huffman tree was made")]
	UnderspecifiedTree
}

/// A Vorbis binary prefix code tree, used to provide lossless entropy coding of
/// entry numbers (the symbols) that may be used as-is in scalar contexts, or
/// interpreted as a vector quantization table index in vector contexts.
///
/// Although nothing in the stream data guarantees that such trees were constructed
/// following the Huffman algorithm, the codeword assigning process is very
/// Huffman-like and in practice it only makes sense to use Huffman codes, as they
/// are optimal.
///
/// By convention, branching left is assigned the bit 0, while branching right is
/// assigned the bit 1.
///
/// The current implementation of this tree builds its nodes explicitly in an arena
/// backed by a resizable array. This should be faster and more cache-friendly than
/// performing one allocation per node, but it's still slower than a more optimized
/// approach. It has the benefit of being easier to read and reason about, though.
#[self_referencing]
pub(super) struct VorbisHuffmanTree {
	arena: Bump,
	#[borrows(arena)]
	#[not_covariant] // Mutable references are not covariant
	root: VorbisHuffmanTreeNode<'this, VorbisHuffmanTreeEntry>
}

impl VorbisHuffmanTree {
	/// Builds a Vorbis Huffman tree from the provided codeword lengths.
	///
	/// # Preconditions
	/// The length of `codeword_lengths` fits in a 32-bit integer. Due to
	/// the setup header construction, this is always the case.
	pub(super) fn try_from_codeword_lengths<T: AsRef<[u8]>>(
		codeword_lengths: T
	) -> Result<Self, TryFromCodewordLengthsListError> {
		let codeword_lengths = codeword_lengths.as_ref();

		// Handle technically erroneous single-entry codebooks as defined
		// in the specification, by decoding any single bit to the the only
		// possible entry number. Note that single-entry codebooks with other
		// codeword lengths are not special-cased, and will have a codeword
		// with that length assigned to them as usual, which will work if the
		// stream does not actually use underspecified parts of the tree.
		// This is a deviation from the specification, but provides for a
		// simpler implementation and maybe some corrupt stream recovery
		// capabilities
		if codeword_lengths == [1] {
			return Ok(VorbisHuffmanTreeBuilder {
				arena: Bump::new(),
				root_builder: |arena| VorbisHuffmanTreeNode {
					left_child: Some(arena.alloc(VorbisHuffmanTreeNode {
						entry: Some(VorbisHuffmanTreeEntry { number: 0 }),
						..Default::default()
					})),
					right_child: Some(arena.alloc(VorbisHuffmanTreeNode {
						entry: Some(VorbisHuffmanTreeEntry { number: 0 }),
						..Default::default()
					})),
					entry: None
				}
			}
			.build());
		}

		VorbisHuffmanTreeTryBuilder {
			arena: Bump::new(),
			root_builder: |arena| {
				let mut root = VorbisHuffmanTreeNode::default();

				for (entry_number, codeword_length) in codeword_lengths.iter().copied().enumerate()
				{
					// Ignore unused entries for sparse codebooks
					if codeword_length == 0 {
						continue;
					}

					let (entry, _) = root
						.leftmost_free_leaf_at_depth(codeword_length, arena)
						.ok_or(TryFromCodewordLengthsListError::OverspecifiedTree)?;

					entry.entry = Some(VorbisHuffmanTreeEntry {
						number: entry_number as u32
					});
				}

				Ok(root)
			}
		}
		.try_build()
	}

	/// Converts the specified list of entry codeword lengths to a list of
	/// entry codeword and length pairs.
	///
	/// # Preconditions
	/// Each codeword length is less than or equal to 32.
	pub(super) fn try_codewords_from_codeword_lengths<T: AsRef<[u64]>>(
		codeword_lengths: T
	) -> Result<Vec<Option<(u32, u8)>>, TryFromCodewordLengthsListError> {
		let codeword_lengths = codeword_lengths.as_ref();

		// Build and populate the tree, taking note of the codewords, and then
		// tear it down. Elegant and concise, but not very efficient
		let mut codewords = vec![None; codeword_lengths.len()];
		let mut root = VorbisHuffmanTreeNode::default();
		let arena = Bump::new();

		for (entry_number, codeword_length) in codeword_lengths.iter().copied().enumerate() {
			let codeword_length = codeword_length as u8;

			// Ignore unused entries for sparse codebooks
			if codeword_length == 0 {
				continue;
			}

			let (entry, codeword) = root
				.leftmost_free_leaf_at_depth(codeword_length, &arena)
				.ok_or(TryFromCodewordLengthsListError::OverspecifiedTree)?;

			entry.entry = Some(VorbisHuffmanTreeEntry {
				number: entry_number as u32
			});

			codewords[entry_number] = Some((codeword, codeword_length));
		}

		Ok(codewords)
	}

	/// Executes the provided callback, passing a [walker][VorbisHuffmanTreeWalker]
	/// that can be used to inspect the tree.
	pub(super) fn with_walker<R>(
		&self,
		f: impl FnOnce(VorbisHuffmanTreeWalker<'_, '_, VorbisHuffmanTreeEntry>) -> R
	) -> R {
		self.with_root(|root| f(VorbisHuffmanTreeWalker { current_node: root }))
	}
}

impl Debug for VorbisHuffmanTree {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.with_root(|root| Debug::fmt(root, f))
	}
}

/// A Vorbis Huffman tree entry. Currently, it only holds its number.
#[derive(Debug)]
#[repr(transparent)]
pub(super) struct VorbisHuffmanTreeEntry {
	/// The number of this entry.
	pub(super) number: u32
}

/// Helper struct to walk down a Huffman binary tree, iterator style.
#[derive(Debug)]
pub(super) struct VorbisHuffmanTreeWalker<'this, 'tree, V> {
	current_node: &'this VorbisHuffmanTreeNode<'tree, V>
}

impl<V> VorbisHuffmanTreeWalker<'_, '_, V> {
	/// Walks down the binary tree, deciding whether to branch left or
	/// right depending on the specified bit.
	pub(super) fn walk(
		&mut self,
		branch_right: bool
	) -> Result<Option<&V>, VorbisHuffmanTreeWalkerError> {
		self.current_node = if branch_right {
			self.current_node.right_child.as_ref()
		} else {
			self.current_node.left_child.as_ref()
		}
		.ok_or(VorbisHuffmanTreeWalkerError::UnderspecifiedTree)?;

		Ok(self.current_node.entry.as_ref())
	}
}

/// A node in a Vorbis Huffman tree, holding an entry.
#[derive(Debug)]
struct VorbisHuffmanTreeNode<'tree, V> {
	left_child: Option<&'tree mut VorbisHuffmanTreeNode<'tree, V>>,
	right_child: Option<&'tree mut VorbisHuffmanTreeNode<'tree, V>>,
	entry: Option<V>
}

impl<'tree, V> VorbisHuffmanTreeNode<'tree, V> {
	/// Finds the leftmost free leaf in this tree at a certain depth,
	/// returning the would-be codeword for it and allocating any new
	/// nodes in the specified arena.
	///
	/// # Preconditions
	/// `depth` is less than or equal to 32.
	fn leftmost_free_leaf_at_depth<'this>(
		&'this mut self,
		depth: u8,
		arena: &'tree Bump
	) -> Option<(&'this mut Self, u32)> {
		self.leftmost_free_leaf_at_depth_internal(depth, 0, arena)
			.map(|(leaf, codeword)| {
				// The codeword returned by this method is meant to be interpreted in
				// MSB -> LSB order, skipping the 32 - depth most significant bits, but
				// codewords are read from and written to Vorbis streams in LSB -> MSB
				// order. Reverse the order before returning. The wrapping left shift
				// handles the theoretical case of a zero-length codeword by returning a
				// numerically appropriate result, even though conceptually a zero-length
				// codeword does not make sense, and this function is not called with
				// such a depth parameter because zero-length codewords are used to
				// encode unused entries
				let offset = u32::BITS - depth as u32;
				(leaf, codeword.wrapping_shl(offset).reverse_bits())
			})
	}

	fn leftmost_free_leaf_at_depth_internal<'this>(
		&'this mut self,
		depth: u8,
		codeword_so_far: u32,
		arena: &'tree Bump
	) -> Option<(&'this mut Self, u32)> {
		// Base case 1: occupied nodes are not free and can't have free nodes beneath them
		if self.entry.is_some() {
			return None;
		}

		// Base case 2: this node is a leftmost free leaf candidate
		if depth == 0 {
			return if self.left_child.is_none() && self.right_child.is_none() {
				Some((self, codeword_so_far))
			} else {
				None
			};
		}

		// Recursive cases: expand and try finding a free leaf in the left child.
		// If that does not yield a leaf, try the right child
		self.left_child
			.get_or_insert_with(|| arena.alloc(Default::default()))
			.leftmost_free_leaf_at_depth_internal(depth - 1, codeword_so_far, arena)
			.or_else(|| {
				self.right_child
					.get_or_insert_with(|| arena.alloc(Default::default()))
					.leftmost_free_leaf_at_depth_internal(
						depth - 1,
						codeword_so_far | 1 << (depth - 1),
						arena
					)
			})
	}
}

impl<V> Default for VorbisHuffmanTreeNode<'_, V> {
	fn default() -> Self {
		Self {
			left_child: None,
			right_child: None,
			entry: None
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn huffman_tree_from_codeword_lengths_works() {
		// Example tree from the Vorbis I specification § 3.2.1
		let tree = VorbisHuffmanTree::try_from_codeword_lengths([2, 4, 4, 4, 4, 2, 3, 3])
			.expect("The Huffman tree was assumed to not be overspecified");

		tree.with_root(|root| eprintln!("Tree: {root:#?}"));
		eprintln!(
			"Tree nodes arena allocated bytes: {}",
			tree.borrow_arena().allocated_bytes()
		);

		for (entry_number, (codeword, codeword_length)) in [
			(0b00, 2),
			(0b0100, 4),
			(0b0101, 4),
			(0b0110, 4),
			(0b0111, 4),
			(0b10, 2),
			(0b110, 3),
			(0b111, 3)
		]
		.into_iter()
		.enumerate()
		{
			eprintln!("Testing decode of codeword {codeword:0codeword_length$b}");

			tree.with_walker(|mut walker| {
				let mut read_entry = None;

				for i in (0..codeword_length).rev() {
					let bit = codeword >> i & 1;
					read_entry = walker
						.walk(bit != 0)
						.expect("The Huffman tree was assumed to not be underspecified");
				}

				assert_eq!(
					read_entry
						.expect("The Huffman tree could not decode an assumed valid codeword")
						.number,
					entry_number as u32
				);
			});
		}
	}

	#[test]
	fn single_entry_huffman_tree_works() {
		let tree = VorbisHuffmanTree::try_from_codeword_lengths([1])
			.expect("The Huffman tree was assumed to not be overspecified");

		tree.with_root(|root| eprintln!("Tree: {root:#?}"));
		eprintln!(
			"Tree nodes arena allocated bytes: {}",
			tree.borrow_arena().allocated_bytes()
		);

		// Reading any bit should return the entry zero
		for bit in [false, true] {
			assert_eq!(
				tree.with_walker(|mut walker| walker
					.walk(bit)
					.expect("The Huffman tree was assumed to not be underspecified")
					.expect("A single-bit codeword was expected to be decoded")
					.number),
				0
			);
		}
	}

	#[test]
	fn overspecified_huffman_tree_is_rejected() {
		// Example tree from the Vorbis I specification § 3.2.1,
		// but with an additional codeword length
		VorbisHuffmanTree::try_from_codeword_lengths([2, 4, 4, 4, 4, 2, 3, 3, 32])
			.expect_err("The Huffman tree was assumed to be overspecified");
	}

	#[test]
	fn underspecified_huffman_tree_codewords_are_rejected() {
		// Example tree from the Vorbis I specification § 3.2.1,
		// but without the codeword length for entry 4
		let tree = VorbisHuffmanTree::try_from_codeword_lengths([2, 4, 4, 4, 2, 3, 3])
			.expect("The Huffman tree was assumed to not be overspecified");

		tree.with_root(|root| eprintln!("Tree: {root:#?}"));
		eprintln!(
			"Tree nodes arena allocated bytes: {}",
			tree.borrow_arena().allocated_bytes()
		);

		for codeword in [
			// Read codeword 0111, which would correspond to entry 4, but was removed from the tree
			&[false, true, true, true][..],
			// Read codeword 01000, which would be a child of entry 1, but entry nodes, which always
			// are leaves, don't have children
			&[false, true, false, false, false][..]
		] {
			tree.with_walker(|mut walker| {
				for codeword_bit in codeword.iter().take(codeword.len() - 1) {
					walker
						.walk(*codeword_bit)
						.expect("Unexpected underspecified Huffman tree error");
				}

				walker
					.walk(*codeword.last().unwrap())
					.expect_err("Expected underspecified Huffman tree error");
			});
		}
	}

	#[test]
	fn empty_huffman_tree_is_underspecified() {
		// Observe that this tests uses a tree similar to the one that would result
		// from converting a codeword length list with a single zero
		let tree = VorbisHuffmanTreeBuilder {
			arena: Bump::new(),
			root_builder: |_| Default::default()
		}
		.build();

		tree.with_walker(|mut walker| {
			walker
				.walk(false) // Any bit
				.expect_err("Expected underspecified Huffman tree error")
		});
	}

	#[test]
	#[ignore = "Takes a long time to run"]
	fn monstrous_codeword_lengths_list_has_reasonable_resource_consumption() {
		const MONSTROUS_CODEWORD_LENGTH: u8 = 16;
		let tree = VorbisHuffmanTree::try_from_codeword_lengths(
			[MONSTROUS_CODEWORD_LENGTH; 2_usize.pow(MONSTROUS_CODEWORD_LENGTH as u32)]
		)
		.expect("The Huffman tree was assumed to not be overspecified");

		let allocated_bytes = tree.borrow_arena().allocated_bytes();
		eprintln!("Tree nodes arena allocated bytes: {allocated_bytes}");

		if allocated_bytes > 8 * 1024 * 1024 {
			panic!("More than 8 MiB of RAM were allocated for the Huffman tree");
		}
	}

	#[test]
	fn codeword_lengths_list_is_assigned_expected_codewords() {
		// Example codeword length list from the Vorbis I specification, § 3.2.1,
		// but adding an unused codebook entry in the middle
		let codewords =
			VorbisHuffmanTree::try_codewords_from_codeword_lengths([2, 4, 4, 4, 0, 4, 2, 3, 3])
				.expect("The Huffman tree was assumed to not be overspecified");

		// Codeword assignment from the Vorbis I specification, § 3.2.1, but reversing
		// the bit order, as expected for the Vorbis bitpack convention, and taking into
		// account the unused entry we've added
		const EXPECTED_CODEWORDS: &[Option<(u32, u8)>] = &[
			Some((0b00, 2)),
			Some((0b0010, 4)),
			Some((0b1010, 4)),
			Some((0b0110, 4)),
			None,
			Some((0b1110, 4)),
			Some((0b01, 2)),
			Some((0b011, 3)),
			Some((0b111, 3))
		];

		assert_eq!(
			codewords, EXPECTED_CODEWORDS,
			"Unexpected codeword assignments for codeword lengths"
		);
	}
}
