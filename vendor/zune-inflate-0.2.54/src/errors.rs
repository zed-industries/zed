//! Errors possible when decoding deflate/zlib/gzip
//! streams

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Display, Formatter};

/// A struct returned when decompression fails
///
/// This struct contains two fields,
///
/// - `error`:Tells you the error that actually occured.
/// - `data`: Gives you decoded data up until that point when
/// the error was encountered.
///
/// One can recover data up to the error if they so wish but
/// guarantees about data state is not given
pub struct InflateDecodeErrors
{
    /// reason why decompression fails
    pub error: DecodeErrorStatus,
    /// Decoded data up until that decompression error
    pub data:  Vec<u8>
}

impl InflateDecodeErrors
{
    /// Create a new decode wrapper with data being
    /// how many bytes we actually decoded before hitting an error
    ///
    /// # Arguments
    /// - `error`: Error encountered during decoding
    /// - `data`:  Data up to that point of decoding
    ///
    /// # Returns
    /// Itself
    pub fn new(error: DecodeErrorStatus, data: Vec<u8>) -> InflateDecodeErrors
    {
        InflateDecodeErrors { error, data }
    }
    /// Create a new decode wrapper with an empty vector
    ///
    /// # Arguments
    /// - `error`: Error encountered during decoding.
    pub fn new_with_error(error: DecodeErrorStatus) -> InflateDecodeErrors
    {
        InflateDecodeErrors::new(error, vec![])
    }
}

impl Debug for InflateDecodeErrors
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
    {
        writeln!(f, "{:?}", self.error)
    }
}

pub enum DecodeErrorStatus
{
    /// Input data is not enough to construct
    /// a full output
    InsufficientData,
    /// Anything that isn't significant
    Generic(&'static str),
    /// Anything that isn't significant but we need to
    /// pass back information to the user as to what went wrong
    GenericStr(String),
    ///Input data was malformed.
    CorruptData,
    /// Limit set by the user was exceeded by
    /// decompressed output
    OutputLimitExceeded(usize, usize),
    /// Output CRC does not match stored CRC.
    ///
    /// Only present for zlib
    MismatchedCRC(u32, u32),
    /// Output Adler does not match stored adler
    ///
    /// Only present for gzip
    MismatchedAdler(u32, u32)
}

impl Debug for DecodeErrorStatus
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
    {
        match self
        {
            Self::InsufficientData => writeln!(f, "Insufficient data"),
            Self::Generic(reason) => writeln!(f, "{reason}"),
            Self::GenericStr(reason) => writeln!(f, "{reason}"),
            Self::CorruptData => writeln!(f, "Corrupt data"),
            Self::OutputLimitExceeded(limit, current) => writeln!(
                f,
                "Output limit exceeded, set limit was {limit} and output size is {current}"
            ),
            Self::MismatchedCRC(expected, found) =>
            {
                writeln!(f, "Mismatched CRC, expected {expected} but found {found}")
            }
            Self::MismatchedAdler(expected, found) =>
            {
                writeln!(f, "Mismatched Adler, expected {expected} but found {found}")
            }
        }
    }
}

impl Display for InflateDecodeErrors
{
    #[allow(clippy::uninlined_format_args)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result
    {
        writeln!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InflateDecodeErrors {}
