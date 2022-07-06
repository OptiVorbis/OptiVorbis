//! Container-agnostic data types for parsing and optimizing Vorbis streams.

use std::error::Error;
use std::fmt::{Display, Formatter};

use strum_macros::FromRepr;

pub(crate) mod optimizer;

pub(crate) mod codebook;

/// Helper macro that implements the [`TryFrom`] trait for converting from an enum
/// representation to a variant, with a custom error type.
macro_rules! try_from_impl {
	{ type Enum = $enum_type:ident($repr_type:ty); type Error = $error_type:ident } => {
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
				$enum_type::from_repr(value).ok_or($error_type(value))
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
#[derive(Debug, Eq, PartialEq, Clone, Copy, FromRepr)]
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

try_from_impl! {
	type Enum = PacketType(u8);
	type Error = TryPacketTypeFromInt
}

/// Represents a codebook vector quantization lookup type, defined in the Vorbis I
/// specification, § 3.2.1.
#[derive(Debug, Eq, PartialEq, Clone, Copy, FromRepr)]
#[repr(u8)]
enum VectorLookupType {
	NoLookup = 0,
	ImplicitlyPopulated = 1,
	ExplicitlyPopulated = 2
}

try_from_impl! {
	type Enum = VectorLookupType(u8);
	type Error = TryVectorLookupTypeFromInt
}

/// Represents a residue vector type, defined in the Vorbis I specification, § 8.
#[derive(Debug, Eq, PartialEq, Clone, Copy, FromRepr)]
#[repr(u16)]
enum ResidueType {
	Interleaved = 0,
	Ordered = 1,
	InterleavedVectors = 2
}

try_from_impl! {
	type Enum = ResidueType(u16);
	type Error = TryResidueTypeFromInt
}
