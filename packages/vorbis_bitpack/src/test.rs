#![allow(clippy::unusual_byte_groupings)]

use super::*;

/// This Vorbis float word is constructed such that:
/// - Its sign bit is 1 (negative).
/// - It has the highest and lowest mantissa bits set.
/// - Its encoded exponent is 788, which should read back to zero,
///   so the mantissa is multiplied by one.
const VORBIS_FLOAT_WORD: u32 = 0b1_1100010100_100000000000000000001;
/// The value of [`VORBIS_FLOAT_WORD`], as a native double-precision float.
#[allow(clippy::unnecessary_cast)] // There are no binary f64 literals
const VORBIS_FLOAT_VALUE: f64 = -0b100000000000000000001 as f64;

/// These values were extracted from an audio file encoded with
/// libvorbis, and the unpacked float values obtained by modifying
/// a known-good, open source Vorbis decoder to output them.
const VORBIS_FLOAT_WORD_VALUE_PAIRS: &[(u32, f64)] = &[
	(0b1_1100000000_100000000000000000000, -1.0),
	(0b1_1100000011_101100000000000000000, -11.0),
	(0b1_1100001010_111011000100000000000, -1890.0)
];

#[test]
fn reading_spec_bitpacking_example_works() {
	let mut bitpacked_data = &[0b1111_1100, 0b0100_1000, 0b1100_1110, 0b0000_0110][..];
	let mut bitpacker = BitpackReader::new(&mut bitpacked_data);

	macro_rules! read_value {
		($method:ident, $width:expr, $expected:expr) => {
			#[cfg(not(feature = "no-std"))]
			eprintln!(
				"Bitpacker state before calling {}: {:?}",
				stringify!($method),
				bitpacker
			);

			let actual = bitpacker
				.$method(BitpackedIntegerWidth::new($width).unwrap())
				.expect("No EOF expected");

			#[cfg(not(feature = "no-std"))]
			eprintln!(
				"Bitpacker state after calling {}: {:?}",
				stringify!($method),
				bitpacker
			);

			assert_eq!(actual, $expected);
		};
	}

	read_value!(read_unsigned_integer, 4, 12);
	read_value!(read_signed_integer, 3, -1);
	read_value!(read_unsigned_integer, 7, 17);
	read_value!(read_unsigned_integer, 13, 6969);

	// Extra operations not in the spec example
	read_value!(read_unsigned_integer, 4, 0);
	assert!(
		bitpacked_data.is_empty(),
		"All the bytes should have been read"
	);
}

#[test]
fn reading_zero_length_integer_works() {
	// Initialize the bitpacker to a state where lots of non-zero
	// bits are available. This should help catching non-conformances
	let mut bitpacker = BitpackReader {
		last_read_byte: 0xFF,
		remaining_bits: 8,
		source: {
			#[cfg(not(feature = "no-std"))]
			{
				std::io::empty()
			}
			#[cfg(feature = "no-std")]
			{
				acid_io::empty()
			}
		}
	};

	assert_eq!(
		bitpacker
			.read_unsigned_integer(BitpackedIntegerWidth(0))
			.expect("No I/O error expected"),
		0
	);

	// Also assert that the internal bitpacker state is the same
	assert_eq!(bitpacker.last_read_byte, 0xFF, "Unexpected internal state");
	assert_eq!(bitpacker.remaining_bits, 8, "Unexpected internal state");
}

#[test]
#[cfg(not(feature = "no-std"))]
fn writing_spec_bitpacking_example_works() {
	let mut bitpacked_data = Vec::with_capacity(4);
	let mut bitpacker = BitpackWriter::new(&mut bitpacked_data);

	macro_rules! write_value {
		($method:ident, $integer:expr, $width:expr) => {
			eprintln!(
				"Bitpacker state before calling {}: {:?}",
				stringify!($method),
				bitpacker
			);

			bitpacker
				.$method($integer, BitpackedIntegerWidth::new($width).unwrap())
				.expect("No I/O error expected");

			eprintln!(
				"Bitpacker state after calling {}: {:?}",
				stringify!($method),
				bitpacker
			);
		};
	}

	write_value!(write_unsigned_integer, 12, 4);
	write_value!(write_signed_integer, -1, 3);
	write_value!(write_unsigned_integer, 17, 7);
	write_value!(write_unsigned_integer, 6969, 13);
	// Extra operations not in the spec example
	write_value!(write_signed_integer, -15, 5);

	drop(bitpacker);

	assert_eq!(
		bitpacked_data,
		&[0b1111_1100, 0b0100_1000, 0b1100_1110, 0b1000_1110],
		"Unexpected bitpack write result"
	);
}

#[test]
#[cfg(not(feature = "no-std"))]
fn writing_zero_width_integers_does_nothing() {
	let mut dummy = Vec::new();
	let mut bitpacker = BitpackWriter::new(&mut dummy);

	bitpacker
		.write_unsigned_integer(u32::MAX, bitpacked_integer_width!(0))
		.expect("No I/O error expected");
	bitpacker
		.write_signed_integer(i32::MAX, bitpacked_integer_width!(0))
		.expect("No I/O error expected");

	drop(bitpacker);

	assert!(
		dummy.is_empty(),
		"Some bytes were written to the sink when packing zero width integers"
	);
}

#[test]
fn float32_unpack_works() {
	assert_eq!(float32_unpack(VORBIS_FLOAT_WORD), VORBIS_FLOAT_VALUE);
}

#[test]
fn float32_unpack_real_values_works() {
	for (word, unpacked_float) in VORBIS_FLOAT_WORD_VALUE_PAIRS.iter().copied() {
		assert_eq!(float32_unpack(word), unpacked_float);
	}
}

#[test]
fn float32_pack_works() {
	assert_eq!(float32_pack(VORBIS_FLOAT_VALUE), VORBIS_FLOAT_WORD);
}

#[test]
fn float32_pack_real_values_works() {
	for (word, unpacked_float) in VORBIS_FLOAT_WORD_VALUE_PAIRS.iter().copied() {
		assert_eq!(float32_pack(unpacked_float), word);
	}
}
