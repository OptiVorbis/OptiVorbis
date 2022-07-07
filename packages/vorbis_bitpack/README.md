[![docs.rs status](https://img.shields.io/docsrs/vorbis_bitpack?label=docs.rs)](https://docs.rs/vorbis_bitpack)

This crate implements the bitpacking convention defined in the [Vorbis I specification](https://xiph.org/vorbis/doc/Vorbis_I_spec.pdf), section 2.

# Overview

The Vorbis bitpacking convention is a simple means of efficiently writing and reading sequences of arbitrary-width integers in byte-oriented transports. It can be summarized as consecutively storing the bits of the binary representations of those integers, avoiding padding or aligning them to bytes as much as possible.

Unlike most general-purpose serialization formats, Vorbis bitpacking does not add any header or field delimiting overhead: a bitpacked stream is a sequence of raw, unstructured bits without an intrinsic interpretation. Any sequence of bits is a valid bitpacked stream. Thus, it is the responsibility of encoders and decoders to agree on some protocol for interpreting the data.

Bitpacking is little-endian: the least significant bits are written first to the stream. When dealing with complete bytes only, bitpacking is equivalent to reading and writing them in little-endian order.

# Supported data types

Any data type can be converted to an integer, so the bitpacking convention is generalizable to non-integer data types. However, this crate limits its scope to the types used in Vorbis streams, at most 32-bit wide:

- Unsigned integers, 0 to 32-bit wide. Reading a 0-bit wide integer always succeeds and returns 0.
- Signed integers, in two's complement notation, 0 to 32-bit wide.
- Vorbis `float32`, a custom floating-point number format that may be converted exactly to a [`f64`], but not vice-versa.
- Boolean flags, 1 bit long.

These types can be freely used as a base to implement new data types. For example, a 64-bit integer can be implemented as two 32-bit integers combined.

# `#![no_std]` compatibility

By default, this crate depends on the Rust standard library, but it is compatible with `#![no_std]` environments when the optional `no-std` feature is enabled.

# Example

The following code recreates the Vorbis I specification bitpacking example.

```rust
use std::io::Cursor;
use vorbis_bitpack::{bitpacked_integer_width, BitpackReader, BitpackWriter};

let mut buf = Vec::new();

// Write bitpacked integers
let mut bitpacker = BitpackWriter::new(&mut buf);
bitpacker.write_unsigned_integer(12, bitpacked_integer_width!(4))?;
bitpacker.write_signed_integer(-1, bitpacked_integer_width!(3))?;
bitpacker.write_unsigned_integer(17, bitpacked_integer_width!(7))?;
bitpacker.write_unsigned_integer(6969, bitpacked_integer_width!(13))?;
drop(bitpacker); // Pads and writes the incomplete last byte

// Read them back
let mut bitpacker = BitpackReader::new(Cursor::new(&mut buf));
assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(4))?, 12);
assert_eq!(bitpacker.read_signed_integer(bitpacked_integer_width!(3))?, -1);
assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(7))?, 17);
assert_eq!(bitpacker.read_unsigned_integer(bitpacked_integer_width!(13))?, 6969);

// The buffer should have all the integer bits concatenated together as tightly
// as possible
assert_eq!(buf, [0b1_111_1100, 0b01_001000, 0b11001110, 0b00000_110]);
```
