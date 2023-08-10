//! This crate implements the bitpacking convention defined in the [Vorbis I specification], section 2.
//!
//! # Overview
//!
//! The Vorbis bitpacking convention is a simple means of efficiently writing and reading sequences of
//! arbitrary-width integers in byte-oriented transports. It can be summarized as consecutively storing
//! the bits of the binary representations of those integers, avoiding padding or aligning them to bytes
//! as much as possible.
//!
//! Unlike most general-purpose serialization formats, Vorbis bitpacking does not add any header or
//! field delimiting overhead: a bitpacked stream is a sequence of raw, unstructured bits without an
//! intrinsic interpretation. Any sequence of bits is a valid bitpacked stream. Thus, it is the
//! responsibility of encoders and decoders to agree on some protocol for interpreting the data.
//!
//! Bitpacking is little-endian: the least significant bits are written first to the stream. When
//! dealing with complete bytes only, bitpacking is equivalent to reading and writing them in
//! little-endian order.
//!
//! # Supported data types
//!
//! Any data type can be converted to an integer, so the bitpacking convention is generalizable to
//! non-integer data types. However, this crate limits its scope to the types used in Vorbis streams, at
//! most 32-bit wide:
//!
//! - Unsigned integers, 0 to 32-bit wide. Reading a 0-bit wide integer always succeeds and returns 0.
//! - Signed integers, in two's complement notation, 0 to 32-bit wide.
//! - Vorbis `float32`, a custom floating-point number format that may be converted exactly to a
//!   [`f64`], but not vice-versa.
//! - Boolean flags, 1 bit long.
//!
//! These types can be freely used as a base to implement new data types. For example, a 64-bit integer
//! can be implemented as two 32-bit integers combined.
//!
//! # `#![no_std]` compatibility
//!
//! By default, this crate depends on the Rust standard library, but it is compatible with `#![no_std]`
//! environments when the optional `no-std` feature is enabled.
//!
//! # Example
//!
//! The following code recreates the Vorbis I specification bitpacking example.
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use std::io::Cursor;
//! use vorbis_bitpack::{bitpacked_integer_width, BitpackReader, BitpackWriter};
//!
//! let mut buf = Vec::new();
//!
//! // Write bitpacked integers
//! let mut bitpacker = BitpackWriter::new(&mut buf);
//! bitpacker.write_unsigned_integer(12, bitpacked_integer_width!(4))?;
//! bitpacker.write_signed_integer(-1, bitpacked_integer_width!(3))?;
//! bitpacker.write_unsigned_integer(17, bitpacked_integer_width!(7))?;
//! bitpacker.write_unsigned_integer(6969, bitpacked_integer_width!(13))?;
//! drop(bitpacker); // Pads and writes the incomplete last byte
//!
//! // Read them back
//! let mut bitpacker = BitpackReader::new(Cursor::new(&mut buf));
//! assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(4))?, 12);
//! assert_eq!(bitpacker.read_signed_integer(bitpacked_integer_width!(3))?, -1);
//! assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(7))?, 17);
//! assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(13))?, 6969);
//!
//! // The buffer should have all the integer bits concatenated together as tightly
//! // as possible
//! assert_eq!(buf, [0b1_111_1100, 0b01_001000, 0b11001110, 0b00000_110]);
//! # Ok(())
//! # }
//! ```
//!
//! [Vorbis I specification]: https://xiph.org/vorbis/doc/Vorbis_I_spec.pdf

#![cfg_attr(feature = "no-std", no_std)]
#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(rustdoc::broken_intra_doc_links)]

use core::cmp;
#[cfg(not(feature = "no-std"))]
use std::io::{Read, Result, Write};

#[cfg(feature = "no-std")]
use acid_io::{Read, Result, Write};

#[cfg(test)]
mod test;

/// A newtype that holds the width of an integer that can be read or written in the
/// Vorbis I bitpack format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BitpackedIntegerWidth(u8);

impl BitpackedIntegerWidth {
	/// Wraps `width` in this newtype, returning `None` if `width` exceeds the
	/// maximum value of 32.
	pub const fn new(width: u8) -> Option<Self> {
		if width <= 32 {
			Some(Self(width))
		} else {
			None
		}
	}

	/// Unwraps the integer width contained by this newtype.
	pub const fn get(self) -> u8 {
		self.0
	}

	/// Creates a newtype that wraps the specified `width`, without checking
	/// that it is in the expected range.
	///
	/// **This function is a macro implementation detail and end-users will never
	/// need to invoke it directly**. It is not considered part of the public API,
	/// so any changes to it, including its removal, will not be considered breaking
	/// changes, and may not be mentioned in changelogs or other documentation.
	#[doc(hidden)]
	pub const fn __internal_unchecked_new(width: u8) -> Self {
		Self(width)
	}
}

/// Creates a [`BitpackedIntegerWidth`] from a width that is statically checked at
/// build time to be valid.
///
/// This macro is a temporary solution to construct widths in constant contexts until the
/// [`const_option`] language feature is stabilized. When that feature gets stabilized, if
/// ever, this macro will be deprecated and removed.
///
/// If using a nightly toolchain with that language feature enabled, it is better to use
/// `BitpackedIntegerWidth::new(width).unwrap()` instead, which shows better error
/// messages and is more idiomatic.
///
/// [`const_option`]: https://doc.rust-lang.org/nightly/unstable-book/library-features/const-option.html
#[macro_export]
macro_rules! bitpacked_integer_width {
	($width:expr) => {{
		const _: [(); 32 - $width as usize] = [(); 32 - $width as usize];
		$crate::BitpackedIntegerWidth::__internal_unchecked_new($width)
	}};
}

/// Wraps a byte source to read variable-length primitive types from it,
/// packed according to the Vorbis I bitpack convention.
#[derive(Debug)]
pub struct BitpackReader<R: Read> {
	last_read_byte: u8,
	remaining_bits: u8,
	source: R
}

impl<R: Read> BitpackReader<R> {
	/// Constructs a Vorbis I bitpack reader that will read variable-length primitive
	/// types from the specified byte source.
	///
	/// The bitpack reader may end up reading small amounts of bytes fairly frequently.
	/// Therefore, for top performance it is recommended to use buffered byte sources.
	pub fn new(source: R) -> Self {
		Self {
			last_read_byte: 0,
			remaining_bits: 0,
			source
		}
	}

	/// Reads a single bitpacked unsigned integer of the specified width from the
	/// source associated to this bitpack reader.
	pub fn read_unsigned_integer(&mut self, width: BitpackedIntegerWidth) -> Result<u32> {
		// This value is in the [0, 8) range
		let remaining_bits = self.remaining_bits;
		let result;

		if remaining_bits >= width.get() {
			// We can satisfy this read request by just extracting bits from the last byte
			// we've already read. Advance the bit position cursor accordingly
			result = self.last_read_byte as u32 & ones_mask(width);

			self.remaining_bits -= width.get();
			self.last_read_byte >>= width.get();
		} else {
			// We need to read up to 4 bytes to fulfill this read request, in case
			// this read request wants to read 32 bits. width > remaining_bits at
			// this point
			let mut read_buf = [0u8; 4];

			// Now read the fewest amount of bytes needed to satisfy this request.
			// Contrary to intuition, reading bytes one by one is faster for the buffered
			// sources we should be using anyway, as that way we can leverage small-copy
			// optimizations in Rust's standard library to avoid emitting a call to memcpy,
			// which has notoriously detrimental performance effects, especially for musl
			// targets (a perf report showed that roughly ~12.12% of execution time for
			// a test file was spent calling memcpy for musl, but this also benefited
			// glibc due to the lesser call overhead). Unbuffered byte sources where a
			// read_exact call translates to a syscall will likely perform significantly
			// worse, but most application code should not be using such sources anyway.
			// Related read:
			// https://github.com/rust-lang/rust/pull/37573
			let bits_to_read = width.get() - remaining_bits;
			let bytes_to_read = (1 + (bits_to_read - 1) / 8) as usize;
			for byte_to_read in &mut read_buf[..bytes_to_read] {
				self.source
					.read_exact(core::slice::from_mut(byte_to_read))?;
			}

			// Put the remaining bits in the least significant positions of the result integer.
			// Due to the rotate_right call below we can't guarantee that upper bits are always
			// set to zero
			let mut partial_result = self.last_read_byte as u32
				& ones_mask(BitpackedIntegerWidth::__internal_unchecked_new(
					remaining_bits
				));

			// Now concat the bits we've read from the source to the result, in increasingly
			// significant positions
			for (i, byte) in read_buf.iter().enumerate().take(bytes_to_read) {
				partial_result |= (*byte as u32) << (remaining_bits + 8 * i as u8);
			}

			// It may happen that we should not fully read the last byte, because we only
			// wanted to extract some bits from it, not all. If that's the case, clear those
			// extra bits in the most significant positions and store the remainder in
			// last_read_byte, so future reads will use those in the first place
			result = partial_result & ones_mask(width);

			// Take into account that the read might have satisfied the request entirely and
			// thus there might be no remaining bits
			self.remaining_bits = bytes_to_read as u8 * 8 - bits_to_read;
			self.last_read_byte =
				read_buf[bytes_to_read - 1].rotate_right(8 - self.remaining_bits as u32);
		}

		Ok(result)
	}

	/// Reads a single bitpacked signed integer of the specified width from the source
	/// associated to this bitpack reader.
	pub fn read_signed_integer(&mut self, width: BitpackedIntegerWidth) -> Result<i32> {
		Ok(sign_extend(self.read_unsigned_integer(width)?, width))
	}

	/// Reads a single bitpacked Vorbis `float32` value from the source associated
	/// to this bitpack reader, and losslessly converts it to a `f64`.
	pub fn read_float32(&mut self) -> Result<f64> {
		Ok(float32_unpack(
			self.read_unsigned_integer(bitpacked_integer_width!(32))?
		))
	}

	/// Reads a single bitpacked flag (i.e., boolean) value from the source associated
	/// to this bitpack reader.
	///
	/// Reading a flag consumes a single bit from the source. `true` is returned if
	/// that bit is equal to 1; otherwise, 0 is returned.
	pub fn read_flag(&mut self) -> Result<bool> {
		Ok(self.read_unsigned_integer(bitpacked_integer_width!(1))? != 0)
	}

	/// Consumes and tears down this bitpack reader, returning the underlying byte source.
	///
	/// This is an one-way operation: any information about what particular bit this bitpack
	/// reader is pointing to within the last byte read from the source will be lost.
	/// In other words, constructing another [`BitpackReader`] with the returned byte
	/// source is not guaranteed to keep the bitstream sync.
	pub fn into_inner(self) -> R {
		self.source
	}
}

/// Wraps a byte sink to write variable-length primitive types to it,
/// packed according to the Vorbis I bitpack convention.
#[derive(Debug)]
pub struct BitpackWriter<W: Write> {
	byte_to_be_written: u8,
	bits_to_be_written: u8,
	sink: W
}

impl<W: Write> BitpackWriter<W> {
	/// Constructs a Vorbis I bitpack writer that write read variable-length primitive
	/// types to the specified byte sink.
	///
	/// The bitpack writer may end up writing small amounts of bytes fairly frequently.
	/// Therefore, for top performance it is recommended to use buffered byte sinks.
	pub fn new(sink: W) -> Self {
		Self {
			byte_to_be_written: 0,
			bits_to_be_written: 0,
			sink
		}
	}

	/// Writes the `width` least significant bits of the specified unsigned integer to the sink
	/// associated to this bitpack writer.
	///
	/// It may happen that the bitpacker does not immediately write every bit to the sink,
	/// in order to complete the last byte with further data. The [`finalize`](Self::finalize)
	/// method forces any pending bits to be written immediately, but beware that the reader
	/// will have to skip any padding bits in the potentially incomplete last byte to keep
	/// bitstream sync.
	pub fn write_unsigned_integer(
		&mut self,
		mut integer: u32,
		width: BitpackedIntegerWidth
	) -> Result<()> {
		let mut remaining_bits = width.get();

		// First, try to complete the pending byte with bits from this integer
		let free_bits_in_byte_to_be_written = 8 - self.bits_to_be_written;
		let bits_to_write_in_byte_to_be_written =
			cmp::min(remaining_bits, free_bits_in_byte_to_be_written);
		let bits_to_write_in_byte_to_be_written_width =
			BitpackedIntegerWidth::__internal_unchecked_new(bits_to_write_in_byte_to_be_written);

		self.byte_to_be_written |= ((integer & ones_mask(bits_to_write_in_byte_to_be_written_width))
			as u8) << self.bits_to_be_written;

		remaining_bits -= bits_to_write_in_byte_to_be_written;
		self.bits_to_be_written += bits_to_write_in_byte_to_be_written;

		// If the pending byte is now complete, write it to the stream
		if self.bits_to_be_written == 8 {
			self.sink.write_all(&[self.byte_to_be_written])?;
			self.byte_to_be_written = 0;
			self.bits_to_be_written = 0;
		}

		// If all the bits made it to the pending byte, there is nothing else to do
		if remaining_bits == 0 {
			return Ok(());
		}

		// Handle some bits of the integer not making it to the pending byte
		// (remaining_bits > 0 and remaining_bits > free_bits_in_byte_to_be_written).
		// If the remaining bits span several bytes, write those to the stream directly.
		// Any remaining bits are stored in the pending byte

		let bytes_to_write = remaining_bits / 8;
		let remainder_bits = remaining_bits % 8;

		// Consume the bits that were written to the pending byte
		integer >>= bits_to_write_in_byte_to_be_written;

		// Write the remaining whole bytes directly to the stream.
		// We write bytes one by one because that generates significantly more efficient
		// machine code for the buffered sinks we should be using. Read the similar comment
		// at BitpackReader::read_unsigned_integer for more details
		for byte_to_write in &integer.to_le_bytes()[..bytes_to_write as usize] {
			self.sink.write_all(&[*byte_to_write])?;
		}

		// Consume the bytes we've just written to the stream. We always write
		// at most 3 bytes, so the shift never overflows
		integer >>= 8 * bytes_to_write;

		self.byte_to_be_written = integer as u8;
		self.bits_to_be_written = remainder_bits;

		Ok(())
	}

	/// Writes the the specified signed integer to the sink associated to this bitpack writer,
	/// doing a narrowing wrapping conversion to the specified width.
	///
	/// It may happen that the bitpacker does not immediately write every bit to the sink,
	/// in order to complete the last byte with further data. The [`finalize`](Self::finalize)
	/// method forces any pending bits to be written immediately, but beware that the reader
	/// will have to skip any padding bits in that incomplete byte to keep bitstream sync.
	pub fn write_signed_integer(
		&mut self,
		integer: i32,
		width: BitpackedIntegerWidth
	) -> Result<()> {
		// Due to two's complement representation, narrowing conversions keep their numeric value when
		// just truncating MSBs: a negative value is the negation of the bits of the positive value
		// representation plus one, so when not overflowing the same numerical value is kept by just
		// ignoring the all-ones upper bits. We don't care about overflow here
		self.write_unsigned_integer(integer as u32, width)
	}

	/// Writes the specified double to the sink associated to this bitpack writer, converting
	/// it to the Vorbis `float32` format.
	///
	/// This conversion is lossy for numbers that cannot be exactly represented in the `float32`
	/// format, and is only well-defined for normal floating point numbers. If infinity, NaN or
	/// subnormal numbers are a concern, client code should guard against them by checking the
	/// result of [`float.classify()`](f64::classify) beforehand.
	pub fn write_float32(&mut self, float: f64) -> Result<()> {
		self.write_unsigned_integer(float32_pack(float), bitpacked_integer_width!(32))
	}

	/// Writes a single bitpacked flag (i.e., boolean) value to the sink associated
	/// to this bitpack writer.
	///
	/// Flags are represented by single bits: `true` is converted to 1, and `false` to 0.
	pub fn write_flag(&mut self, flag: bool) -> Result<()> {
		self.write_unsigned_integer(flag as u32, bitpacked_integer_width!(1))
	}

	/// Immediately writes any bits that did not yet complete a byte, padding that byte
	/// with zeroes in the most significant positions.
	///
	/// The bitpack writer is automatically finalized when it is dropped, so this method
	/// usually does not need to be called, unless you need to know whether the finalization
	/// is successful.
	///
	/// Note that finalizing the bitpack writer does not flush the wrapped byte sink.
	/// Any bytes written by this method are only guaranteed to have reached their
	/// destination after a call to [`flush`](Self::flush).
	pub fn finalize(&mut self) -> Result<()> {
		if self.bits_to_be_written > 0 {
			self.bits_to_be_written = 0;
			self.sink.write_all(&[self.byte_to_be_written])
		} else {
			Ok(())
		}
	}

	/// Flushes the wrapped byte sink.
	///
	/// This method will not force writing out any bits that did not yet made it to
	/// a completed byte. To do that, use [`finalize`](Self::finalize).
	pub fn flush(&mut self) -> Result<()> {
		self.sink.flush()
	}
}

impl<W: Write> Drop for BitpackWriter<W> {
	fn drop(&mut self) {
		self.finalize().ok();
	}
}

/// Returns a 32-bit binary mask that has its `width` least significant bits set to 1,
/// and the remaining bits set to 0. This mask is useful to extract a subset of bits
/// in an unsigned 32-bit word to a native integer.
const fn ones_mask(width: BitpackedIntegerWidth) -> u32 {
	((1u64 << width.get() as u64) - 1) as u32
}

/// Converts the specified variable-width signed integer in an unsigned constant-size
/// word to a native 32-bit signed integer.
const fn sign_extend(integer: u32, width: BitpackedIntegerWidth) -> i32 {
	let extended_bits = 32 - width.get() as u32;
	(integer as i32) << extended_bits >> extended_bits
}

/// Unpacks the specified 32-bit word containing a Vorbis codebook float value, as
/// defined in section 9.2.2 of the Vorbis I specification, to a native floating
/// point number.
fn float32_unpack(word: u32) -> f64 {
	// The Vorbis codebook float value differs from the IEEE-754 binary32 floating
	// point number type in the following aspects:
	//
	// - 21-bit mantissa (significand) vs. 23-bit in IEEE-754.
	// - 10-bit exponent vs. 8-bit in IEEE-754.
	// - 788 exponent bias vs. 127 in IEEE-754.
	// - No implicit precision bit in significand (not normalized).
	//   The initial point position is just after the significand's LSB.
	// - No special values (subnormal numbers, infinity, NaN...).
	//
	// Therefore, Vorbis codebook floats have a higher range, but less
	// granularity than IEEE-754 single-precision floats. The layout is similar.
	// The integer to float conversions done exploit the fact that 21 and
	// 10 bit wide integers can be represented exactly in a single precision
	// float. The layout of floats is also used to make the code branchless.
	// A f64 is returned to preserve the numerical value of a float for higher
	// exponents and significands, as the maximum Vorbis float value is higher
	// than a f32 value
	let mantissa = f32::from_bits(((word & 0x1FFFFF) as f32).to_bits() | word & 0x80000000);
	let exponent = ((word & 0x7FE00000) >> 21) as f64 - 788.0;
	#[cfg(not(feature = "no-std"))]
	{
		mantissa as f64 * exponent.exp2()
	}
	#[cfg(feature = "no-std")]
	{
		mantissa as f64 * libm::exp2(exponent)
	}
}

/// Converts the specified double-precision floating point number to a Vorbis
/// codebook float value.
///
/// As the Vorbis codebook float values are only 32 bits wide, not every double-precision
/// floating point number can be represented exactly in this format. When an exact
/// conversion is not possible, the nearest representable value is returned.
///
/// The Vorbis codebook float format does not support infinity, NaN or subnormal values, but
/// a IEEE-754 floating point number can have those values. As a consequence of the
/// implementation strategy of this method, infinities are converted to numbers with very
/// high, but not the maximum representable, absolute values. The conversion for subnormal
/// and NaN values is undefined, and reading those back with [`float32_unpack`] will yield
/// garbage numbers. Concerned client code should check for those values before calling
/// this method to raise an error as appropriate.
fn float32_pack(float: f64) -> u32 {
	const VORBIS_FLOAT32_MAX_EXPONENT: u32 = (1 << 10) - 1; // 2^10 - 1

	// Copy the sign bit
	let sign_component = ((float.to_bits() & 0x8000_0000_0000_0000) >> 32) as u32;

	// Adjust the exponent, with saturating arithmetic to preserve the scale as much as possible.
	// Subtract 235 to account for the different bias (1023 - 788 = 235).
	// Subtract 20 to account for the different initial point position:
	// - IEEE-754 double: 1.xxxx_xxxx_xxxx_xxxx_xxxx|t...t, where t are the truncated mantissa bits,
	//   which are ignored => the initial point position is 20.
	// - Vorbis float:    xxxx_xxxx_xxxx_xxxx_xxxx_x. => the initial point position is 0
	let exponent = ((float.to_bits() & 0x7FF0_0000_0000_0000) >> 52) as u32;
	let adjusted_exponent = cmp::min(
		exponent.saturating_sub(235 + 20),
		VORBIS_FLOAT32_MAX_EXPONENT
	);
	let exponent_component = adjusted_exponent << 21;

	// Copy the mantissa, ignoring any least significant digits we cannot store.
	// Add the implicit 1 bit, required by IEEE-754 for normal floats
	let mantissa_component = ((float.to_bits() & 0x000F_FFFF_0000_0000) >> 32) as u32 | 0x10_00_00;

	sign_component | exponent_component | mantissa_component
}
