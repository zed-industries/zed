//! Traits for interpreting font data

#![deny(clippy::arithmetic_side_effects)]

use types::{FixedSize, Scalar, Tag};

use crate::font_data::FontData;

/// A type that can be read from raw table data.
///
/// This trait is implemented for all font tables that are self-describing: that
/// is, tables that do not require any external state in order to interpret their
/// underlying bytes. (Tables that require external state implement
/// [`FontReadWithArgs`] instead)
pub trait FontRead<'a>: Sized {
    /// Read an instance of `Self` from the provided data, performing validation.
    ///
    /// In the case of a table, this method is responsible for ensuring the input
    /// data is consistent: this means ensuring that any versioned fields are
    /// present as required by the version, and that any array lengths are not
    /// out-of-bounds.
    fn read(data: FontData<'a>) -> Result<Self, ReadError>;
}

//NOTE: this is separate so that it can be a super trait of FontReadWithArgs and
//ComputeSize, without them needing to know about each other? I'm not sure this
//is necessary, but I don't know the full hierarchy of traits I'm going to need
//yet, so this seems... okay?

/// A trait for a type that needs additional arguments to be read.
pub trait ReadArgs {
    type Args: Copy;
}

/// A trait for types that require external data in order to be constructed.
///
/// You should not need to use this directly; it is intended to be used from
/// generated code. Any type that requires external arguments also has a custom
/// `read` constructor where you can pass those arguments like normal.
pub trait FontReadWithArgs<'a>: Sized + ReadArgs {
    /// read an item, using the provided args.
    ///
    /// If successful, returns a new item of this type, and the number of bytes
    /// used to construct it.
    ///
    /// If a type requires multiple arguments, they will be passed as a tuple.
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError>;
}

// a blanket impl of ReadArgs/FontReadWithArgs for general FontRead types.
//
// This is used by ArrayOfOffsets/ArrayOfNullableOffsets to provide a common
// interface for regardless of whether a type has args.
impl<'a, T: FontRead<'a>> ReadArgs for T {
    type Args = ();
}

impl<'a, T: FontRead<'a>> FontReadWithArgs<'a> for T {
    fn read_with_args(data: FontData<'a>, _: &Self::Args) -> Result<Self, ReadError> {
        Self::read(data)
    }
}

/// A trait for tables that have multiple possible formats.
pub trait Format<T> {
    /// The format value for this table.
    const FORMAT: T;
}

/// A type that can compute its size at runtime, based on some input.
///
/// For types with a constant size, see [`FixedSize`] and
/// for types which store their size inline, see [`VarSize`].
pub trait ComputeSize: ReadArgs {
    /// Compute the number of bytes required to represent this type.
    fn compute_size(args: &Self::Args) -> Result<usize, ReadError>;
}

/// A trait for types that have variable length.
///
/// As a rule, these types have an initial length field.
///
/// For types with a constant size, see [`FixedSize`] and
/// for types which can pre-compute their size, see [`ComputeSize`].
pub trait VarSize {
    /// The type of the first (length) field of the item.
    ///
    /// When reading this type, we will read this value first, and use it to
    /// determine the total length.
    type Size: Scalar + Into<u32>;

    #[doc(hidden)]
    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        let asu32 = data.read_at::<Self::Size>(pos).ok()?.into();
        (asu32 as usize).checked_add(Self::Size::RAW_BYTE_LEN)
    }

    /// Determine the total length required to store `count` items of `Self` in
    /// `data` starting from `start`.
    #[doc(hidden)]
    fn total_len_for_count(data: FontData, count: usize) -> Result<usize, ReadError> {
        let mut current_pos = 0;
        for _ in 0..count {
            let len = Self::read_len_at(data, current_pos).ok_or(ReadError::OutOfBounds)?;
            // If length is 0 then this will spin until we've completed
            // `count` iterations so just bail out early.
            // See <https://github.com/harfbuzz/harfrust/issues/203>
            if len == 0 {
                return Ok(current_pos);
            }
            current_pos = current_pos.checked_add(len).ok_or(ReadError::OutOfBounds)?;
        }
        Ok(current_pos)
    }
}

/// An error that occurs when reading font data
#[derive(Debug, Clone, PartialEq)]
pub enum ReadError {
    OutOfBounds,
    // i64 is flexible enough to store any value we might encounter
    InvalidFormat(i64),
    InvalidSfnt(u32),
    InvalidTtc(Tag),
    InvalidCollectionIndex(u32),
    InvalidArrayLen,
    ValidationError,
    NullOffset,
    TableIsMissing(Tag),
    MetricIsMissing(Tag),
    MalformedData(&'static str),
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ReadError::OutOfBounds => write!(f, "An offset was out of bounds"),
            ReadError::InvalidFormat(x) => write!(f, "Invalid format '{x}'"),
            ReadError::InvalidSfnt(ver) => write!(f, "Invalid sfnt version 0x{ver:08X}"),
            ReadError::InvalidTtc(tag) => write!(f, "Invalid ttc tag {tag}"),
            ReadError::InvalidCollectionIndex(ix) => {
                write!(f, "Invalid index {ix} for font collection")
            }
            ReadError::InvalidArrayLen => {
                write!(f, "Specified array length not a multiple of item size")
            }
            ReadError::ValidationError => write!(f, "A validation error occurred"),
            ReadError::NullOffset => write!(f, "An offset was unexpectedly null"),
            ReadError::TableIsMissing(tag) => write!(f, "the {tag} table is missing"),
            ReadError::MetricIsMissing(tag) => write!(f, "the {tag} metric is missing"),
            ReadError::MalformedData(msg) => write!(f, "Malformed data: '{msg}'"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ReadError {}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    struct DummyVarSize {}

    impl VarSize for DummyVarSize {
        type Size = u16;

        fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
            data.read_at::<u16>(pos).map(|v| v as usize).ok()
        }
    }

    // Avoid fuzzer timeout when we have a VarSizeArray with a large count
    // that contains a 0 length element.
    // See <https://github.com/harfbuzz/harfrust/issues/203>
    #[test]
    fn total_var_size_with_zero_length_element() {
        // Array that appears to have 4 var size elements totalling
        // 26 bytes in length but the zero length 3rd element makes the
        // final one inaccessible.
        const PAYLOAD_NOT_SIZE: u16 = 1;
        let buf = BeBuffer::new().extend([2u16, 4u16, PAYLOAD_NOT_SIZE, 0u16, 20u16]);
        let total_len =
            DummyVarSize::total_len_for_count(FontData::new(buf.data()), usize::MAX).unwrap();
        assert_eq!(total_len, 6);
    }
}
