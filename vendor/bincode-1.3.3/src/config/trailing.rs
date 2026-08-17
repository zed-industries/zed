use de::read::SliceReader;
use {ErrorKind, Result};

/// A trait for erroring deserialization if not all bytes were read.
pub trait TrailingBytes {
    /// Checks a given slice reader to determine if deserialization used all bytes in the slice.
    fn check_end(reader: &SliceReader) -> Result<()>;
}

/// A TrailingBytes config that will allow trailing bytes in slices after deserialization.
#[derive(Copy, Clone)]
pub struct AllowTrailing;

/// A TrailingBytes config that will cause bincode to produce an error if bytes are left over in the slice when deserialization is complete.

#[derive(Copy, Clone)]
pub struct RejectTrailing;

impl TrailingBytes for AllowTrailing {
    #[inline(always)]
    fn check_end(_reader: &SliceReader) -> Result<()> {
        Ok(())
    }
}

impl TrailingBytes for RejectTrailing {
    #[inline(always)]
    fn check_end(reader: &SliceReader) -> Result<()> {
        if reader.is_finished() {
            Ok(())
        } else {
            Err(Box::new(ErrorKind::Custom(
                "Slice had bytes remaining after deserialization".to_string(),
            )))
        }
    }
}
