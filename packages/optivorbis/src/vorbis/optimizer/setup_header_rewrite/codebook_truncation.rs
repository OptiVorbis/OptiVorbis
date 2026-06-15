use crate::vorbis::{
	VectorLookupType, lookup1_values,
	optimizer::{VorbisSetupData, setup_header_parse::CodebookConfiguration}
};

impl VorbisSetupData {
	/// Safely truncates any trailing run of unused entries in every codebook.
	///
	/// This optimization reduces the size of the codebook's entry set while
	/// preserving all decoded entry numbers, which reduces header size and our
	/// execution time.
	pub(super) fn truncate_unused_codebook_entry_suffixes(&mut self) {
		// For codebooks used in residue decoding, it must hold that classifications ^ dimensions
		// <= entries. Codebooks used for other purposes only need the decoded entry numbers to
		// stay as-is, which suffix truncation preserves by construction. Note that the same
		// codebook can theoretically be used by different configurations
		let mut min_residue_classbook_entries = vec![0; self.codebook_configurations.len()];
		for residue in &self.residue_configurations {
			let classbook = residue.classbook as usize;
			let need = (residue.classifications as u32)
				.saturating_pow(self.codebook_configurations[classbook].dimensions as u32);

			min_residue_classbook_entries[classbook] =
				min_residue_classbook_entries[classbook].max(need);
		}

		for (codebook_configuration, &min_residue_classbook_entries) in self
			.codebook_configurations
			.iter_mut()
			.zip(&min_residue_classbook_entries)
		{
			codebook_configuration.truncate_unused_entries_suffix(min_residue_classbook_entries);
		}
	}
}

impl CodebookConfiguration {
	/// Truncates any trailing run of unused entries in this codebook configuration, taking
	/// into account whether minimum number of entries is required due to this codebook
	/// being used for residue classifications decoding.
	///
	/// If the underlying codebook is not already in _optimizing mode_, the recorded entry
	/// frequencies must be in their final state for a call to this method to yield correct
	/// results.
	fn truncate_unused_entries_suffix(&mut self, min_residue_classbook_entries: u32) {
		// Start by computing the optimal entry count value, taking into account residue
		// classbook constraints
		let mut new_entry_count = self
			.codebook
			.entry_decode_frequencies_or_lengths
			.get_mut()
			.iter()
			.rposition(|&codeword_frequency_or_length| codeword_frequency_or_length != 0)
			.map_or(0, |last_used_entry_index| last_used_entry_index as u32 + 1)
			.max(min_residue_classbook_entries);

		// Type-1 vector lookup codebooks read lookup1_values(entries, dimensions) multiplicands
		// during decode, so the minimum new entry count for those is also constrained by keeping
		// the result of that function equal for multiplicand preservation
		if self.vector_lookup_type == VectorLookupType::ImplicitlyPopulated
			&& new_entry_count < self.entry_count
		{
			// lookup1_values = floor(dim-root of entries), so min entries = lookup1_values ^ dim
			let min_lookup1_value_entries = lookup1_values(self.entry_count, self.dimensions)
				.saturating_pow(self.dimensions as u32);
			new_entry_count = new_entry_count.max(min_lookup1_value_entries);
		}

		let new_multiplicand_count = match self.vector_lookup_type {
			VectorLookupType::ImplicitlyPopulated => {
				lookup1_values(new_entry_count, self.dimensions) as usize
			}
			VectorLookupType::ExplicitlyPopulated => {
				new_entry_count as usize * self.dimensions as usize
			}
			VectorLookupType::NoLookup => 0
		};

		self.entry_count = new_entry_count;
		// Dropping unused entries (symbols) does not affect decoding with our Huffman tree, and
		// they won't participate in the code construction for any new, optimized tree either
		self.codebook
			.entry_decode_frequencies_or_lengths
			.get_mut()
			.truncate(new_entry_count as usize);

		self.codebook_vector_multiplicands
			.truncate(new_multiplicand_count);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::vorbis::codebook::VorbisCodebook;

	/// Creates a [`CodebookConfiguration`] whose internal entry frequencies are set to
	/// `entry_frequencies`, with zero-filled multiplicands and otherwise arbitrary-but-valid
	/// field values. Equal-length codewords are used to form a valid although inefficient
	/// prefix code tree of the required amount of leaf nodes.
	fn config_with_entry_frequencies(
		entry_frequencies: &[u64],
		dimensions: u16,
		vector_lookup_type: VectorLookupType
	) -> CodebookConfiguration {
		let entry_count = entry_frequencies.len();
		let prefix_tree_height = (entry_count as u32).next_power_of_two().ilog2().max(1) as u8;
		let mut codebook = VorbisCodebook::new(0, vec![prefix_tree_height; entry_count]).unwrap();
		*codebook.entry_decode_frequencies_or_lengths.get_mut() = entry_frequencies.to_vec();

		let multiplicand_count = match vector_lookup_type {
			VectorLookupType::ImplicitlyPopulated => {
				lookup1_values(entry_count as u32, dimensions) as usize
			}
			VectorLookupType::ExplicitlyPopulated => entry_count * dimensions as usize,
			VectorLookupType::NoLookup => 0
		};

		CodebookConfiguration {
			codebook,
			entry_count: entry_count as u32,
			dimensions,
			vector_lookup_type,
			codebook_vector_minimum_value: 0.0,
			codebook_vector_delta_value: 0.0,
			codebook_vector_multiplicands: vec![0; multiplicand_count],
			codebook_vector_value_bits: 1,
			codebook_vector_sequence_flag: false
		}
	}

	#[test]
	fn drops_trailing_unused_entries() {
		let mut config =
			config_with_entry_frequencies(&[1, 2, 0, 0], 2, VectorLookupType::ExplicitlyPopulated);

		config.truncate_unused_entries_suffix(0);

		assert_eq!(config.entry_count, 2);
		assert_eq!(config.codebook_vector_multiplicands.len(), 4); // 2 entries x 2 dimensions
	}

	#[test]
	fn keeps_interleaved_unused_entries() {
		let mut config =
			config_with_entry_frequencies(&[1, 0, 2, 0, 0], 2, VectorLookupType::NoLookup);

		config.truncate_unused_entries_suffix(0);

		assert_eq!(config.entry_count, 3);
		assert_eq!(config.codebook_vector_multiplicands.len(), 0);
	}

	#[test]
	fn all_unused_entries_leaves_empty_codebook() {
		let mut config = config_with_entry_frequencies(&[0, 0, 0], 2, VectorLookupType::NoLookup);

		config.truncate_unused_entries_suffix(0);

		assert_eq!(config.entry_count, 0);
	}

	#[test]
	fn preserves_residue_classbook_minimum() {
		let mut config =
			config_with_entry_frequencies(&[1, 0, 0, 0], 2, VectorLookupType::NoLookup);

		config.truncate_unused_entries_suffix(3);

		assert_eq!(config.entry_count, 3);
		assert_eq!(config.codebook_vector_multiplicands.len(), 0);
	}

	#[test]
	fn preserves_type1_lookup_value_count() {
		let mut frequencies = vec![0; 100];
		frequencies[0] = 1;
		let mut config =
			config_with_entry_frequencies(&frequencies, 5, VectorLookupType::ImplicitlyPopulated);

		config.truncate_unused_entries_suffix(0);
		assert_eq!(config.entry_count, 32);
		assert_eq!(config.codebook_vector_multiplicands.len(), 2);
	}

	#[test]
	fn type1_bucket_floor_preserves_lookup_value_count() {
		const MAX_CODEBOOK_ENTRIES: u32 = 0xFF_FFFF;

		for dimensions in 1..=u16::MAX {
			for target in 1..=lookup1_values(MAX_CODEBOOK_ENTRIES, dimensions) {
				let bucket_floor = target
					.saturating_pow(dimensions as u32)
					.min(MAX_CODEBOOK_ENTRIES);

				assert_eq!(
					lookup1_values(bucket_floor, dimensions),
					target,
					"lookup1_values bucket floor {bucket_floor} for target {target}, \
					dimensions {dimensions} decodes as {}",
					lookup1_values(bucket_floor, dimensions)
				);
			}
		}
	}
}
