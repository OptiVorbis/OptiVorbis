//! Container-agnostic data types for parsing and optimizing Vorbis streams.

use std::{
	error::Error,
	fmt::{Display, Formatter}
};

pub(crate) mod optimizer;

pub(crate) mod codebook;

/// Helper macro that implements a custom error type for a failed [`TryFrom`] conversion
/// from an enum representation to its variant.
macro_rules! try_from_impl {
	{ type Enum = $enum_type:ident($repr_type:ty) { $( $variant:ident ),+ }; type Error = $error_type:ident } => {
		#[doc = "The error type for fallible conversions from integers to a `"]
		#[doc = stringify!($enum_type)]
		#[doc = "`."]
		#[derive(Debug)]
		#[repr(transparent)]
		pub struct $error_type($repr_type);

		impl Display for $error_type {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		impl Error for $error_type {}

		impl TryFrom<$repr_type> for $enum_type {
			type Error = $error_type;

			fn try_from(value: $repr_type) -> Result<Self, Self::Error> {
				match value {
					$( value if Self::$variant as $repr_type == value => Ok(Self::$variant) ),+,
					_ => Err($error_type(value))
				}
			}
		}

		impl $error_type {
			/// Returns the integer whose conversion failed.
			pub const fn integer(&self) -> $repr_type {
				self.0
			}
		}
	}
}

/// Represents a Vorbis packet type, defined in the Vorbis I specification, § 4.2.1.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PacketType {
	/// An audio packet, which encodes an audio frame composed of samples.
	Audio = 0,
	/// The Vorbis identification header packet, that marks a stream as Vorbis and contains
	/// some basic metadata about it.
	IdentificationHeader = 1,
	/// The Vorbis comment header packet, which contains a list of comment key-value pairs meant
	/// for tagging and stream metadata that does not affect decoding.
	CommentHeader = 3,
	/// The Vorbis setup header packet, whose data sets up the codec setup data structures used
	/// for decoding audio packets.
	SetupHeader = 5
}

try_from_impl! {
	type Enum = PacketType(u8) { Audio, IdentificationHeader, CommentHeader, SetupHeader };
	type Error = TryPacketTypeFromInt
}

impl Display for PacketType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::Audio => "audio packet",
			Self::IdentificationHeader => "identification header packet",
			Self::CommentHeader => "comment header packet",
			Self::SetupHeader => "setup header packet"
		})
	}
}

/// Represents a codebook vector quantization lookup type, defined in the Vorbis I
/// specification, § 3.2.1.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
enum VectorLookupType {
	NoLookup = 0,
	ImplicitlyPopulated = 1,
	ExplicitlyPopulated = 2
}

try_from_impl! {
	type Enum = VectorLookupType(u8) { NoLookup, ImplicitlyPopulated, ExplicitlyPopulated };
	type Error = TryVectorLookupTypeFromInt
}

/// Represents a residue vector type, defined in the Vorbis I specification, § 8.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u16)]
enum ResidueType {
	Interleaved = 0,
	Ordered = 1,
	InterleavedVectors = 2
}

try_from_impl! {
	type Enum = ResidueType(u16) { Interleaved, Ordered, InterleavedVectors };
	type Error = TryResidueTypeFromInt
}

/// The Vorbis I `ilog` function, as defined in section 9.2.1 of the Vorbis I
/// specification. Mathematically, it returns the floor of the base-2 logarithm
/// of the specified number plus one, except for 0 and negative numbers, where it
/// returns zero. For zero and positive numbers, this is equivalent to the minimum
/// number of bits required to represent integers in [0, n].
const fn ilog(n: i32) -> u8 {
	// Surprisingly, branching in the source code translates to better machine code
	if n > 0 {
		32 - n.leading_zeros() as u8
	} else {
		0
	}
}

/// The Vorbis I `lookup1_values` function, as defined in section 9.2.3 of the
/// Vorbis I specification. Mathematically, it returns the
/// `codebook_dimensions`-root of `codebook_entries`, rounded down to an integer.
fn lookup1_values(codebook_entries: u32, codebook_dimensions: u16) -> u32 {
	// codebook_entries is at most 2^24 - 1, so it fits in a f32.
	// codebook_dimensions of zero does not make sense for codebooks used for vector
	// lookup, but the specification does not say they're illegal otherwise. Therefore,
	// let's handle that edge case to avoid division by zero
	if codebook_dimensions == 0 {
		u32::MAX
	} else {
		(codebook_entries as f32).powf(1.0 / codebook_dimensions as f32) as u32
	}
}

#[cfg(test)]
mod tests {
	use super::{ilog, lookup1_values};

	#[test]
	fn ilog_works() {
		// Values from Vorbis I specification, section 9.2.1
		assert_eq!(ilog(0), 0);
		assert_eq!(ilog(1), 1);
		assert_eq!(ilog(2), 2);
		assert_eq!(ilog(3), 2);
		assert_eq!(ilog(4), 3);
		assert_eq!(ilog(7), 3);

		// Additional checks
		assert_eq!(ilog(i32::MAX), 31);
		assert_eq!(ilog(i32::MIN), 0);
	}

	#[test]
	fn lookup1_values_works() {
		assert_eq!(lookup1_values(100, 5), 2);
		assert_eq!(lookup1_values(1, 5), 1);

		assert_eq!(lookup1_values(0, u16::MAX), 0);
		assert_eq!(lookup1_values(0xFFFFFF, 0), u32::MAX);
		assert_eq!(lookup1_values(0xFFFFFF, u16::MAX), 1);
	}
}
