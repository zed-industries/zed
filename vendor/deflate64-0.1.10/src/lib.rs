//! Deflate64 implementation based on [.NET System.IO.Compression][dotnet].
//!
//! This is made to unzip zip file with deflate64 made with windows 11.
//!
//! [dotnet]: https://github.com/dotnet/runtime/tree/e5efd8010e19593298dc2c3ee15106d5aec5a924/src/libraries/System.IO.Compression/src/System/IO/Compression/DeflateManaged

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, nonstandard_style, future_incompatible)]

mod buffer;
mod huffman_tree;
mod inflater_managed;
mod input_buffer;
mod output_window;
mod stream;

pub use inflater_managed::InflaterManaged;
pub use stream::Deflate64Decoder;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockType {
    Uncompressed = 0,
    Static = 1,
    Dynamic = 2,
}

impl BlockType {
    pub fn from_int(int: u16) -> Option<BlockType> {
        match int {
            0 => Some(Self::Uncompressed),
            1 => Some(Self::Static),
            2 => Some(Self::Dynamic),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum InflaterState {
    //ReadingHeader = 0,           // Only applies to GZIP
    ReadingBFinal = 2,
    // About to read bfinal bit
    ReadingBType = 3, // About to read blockType bits

    ReadingNumLitCodes = 4,
    // About to read # literal codes
    ReadingNumDistCodes = 5,
    // About to read # dist codes
    ReadingNumCodeLengthCodes = 6,
    // About to read # code length codes
    ReadingCodeLengthCodes = 7,
    // In the middle of reading the code length codes
    ReadingTreeCodesBefore = 8,
    // In the middle of reading tree codes (loop top)
    ReadingTreeCodesAfter = 9, // In the middle of reading tree codes (extension; code > 15)

    DecodeTop = 10,
    // About to decode a literal (char/match) in a compressed block
    HaveInitialLength = 11,
    // Decoding a match, have the literal code (base length)
    HaveFullLength = 12,
    // Ditto, now have the full match length (incl. extra length bits)
    HaveDistCode = 13, // Ditto, now have the distance code also, need extra dist bits

    /* uncompressed blocks */
    UncompressedAligning = 15,
    UncompressedByte1 = 16,
    UncompressedByte2 = 17,
    UncompressedByte3 = 18,
    UncompressedByte4 = 19,
    DecodingUncompressed = 20,

    // These three apply only to GZIP
    //StartReadingFooter = 21,
    // (Initialisation for reading footer)
    //ReadingFooter = 22,
    //VerifyingFooter = 23,
    Done = 24, // Finished

    DataErrored = 100,
}

impl std::ops::Sub for InflaterState {
    type Output = u8;

    fn sub(self, rhs: Self) -> Self::Output {
        self as u8 - rhs as u8
    }
}

fn array_copy<T: Copy>(source: &[T], dst: &mut [T], length: usize) {
    dst[..length].copy_from_slice(&source[..length]);
}

fn array_copy1<T: Copy>(
    source: &[T],
    source_index: usize,
    dst: &mut [T],
    dst_index: usize,
    length: usize,
) {
    dst[dst_index..][..length].copy_from_slice(&source[source_index..][..length]);
}

/// A structure containing result of streaming inflate.
#[derive(Debug)]
pub struct InflateResult {
    /// The number of bytes consumed from the input slice.
    pub bytes_consumed: usize,
    /// The number of bytes written to the output slice.
    pub bytes_written: usize,
    /// true if there is error in input buffer
    pub data_error: bool,
}

impl InflateResult {
    /// Creates `InflateResult` with zero bytes consumed and written, and no error.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            bytes_consumed: 0,
            bytes_written: 0,
            data_error: false,
        }
    }
}

#[derive(Debug)]
enum InternalErr {
    DataNeeded,
    DataError,
}
