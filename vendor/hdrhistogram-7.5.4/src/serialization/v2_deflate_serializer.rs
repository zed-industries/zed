use super::v2_serializer::{V2SerializeError, V2Serializer};
use super::{Serializer, V2_COMPRESSED_COOKIE};
use crate::core::counter::Counter;
use crate::Histogram;
use byteorder::{BigEndian, WriteBytesExt};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{self, Write};
use std::{self, error, fmt};

/// Errors that occur during serialization.
#[derive(Debug)]
pub enum V2DeflateSerializeError {
    /// The underlying serialization failed
    InternalSerializationError(V2SerializeError),
    /// An i/o operation failed.
    IoError(io::Error),
}

impl std::convert::From<std::io::Error> for V2DeflateSerializeError {
    fn from(e: std::io::Error) -> Self {
        V2DeflateSerializeError::IoError(e)
    }
}

impl fmt::Display for V2DeflateSerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V2DeflateSerializeError::InternalSerializationError(e) => {
                write!(f, "The underlying serialization failed: {}", e)
            }
            V2DeflateSerializeError::IoError(e) => {
                write!(f, "The underlying serialization failed: {}", e)
            }
        }
    }
}

impl error::Error for V2DeflateSerializeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            V2DeflateSerializeError::InternalSerializationError(e) => Some(e),
            V2DeflateSerializeError::IoError(e) => Some(e),
        }
    }
}

/// Serializer for the V2 + DEFLATE binary format.
///
/// It's called "deflate" to stay consistent with the naming used in the Java implementation, but
/// it actually uses zlib's wrapper format around plain DEFLATE.
pub struct V2DeflateSerializer {
    uncompressed_buf: Vec<u8>,
    compressed_buf: Vec<u8>,
    v2_serializer: V2Serializer,
}

impl Default for V2DeflateSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl V2DeflateSerializer {
    /// Create a new serializer.
    pub fn new() -> V2DeflateSerializer {
        V2DeflateSerializer {
            uncompressed_buf: Vec::new(),
            compressed_buf: Vec::new(),
            v2_serializer: V2Serializer::new(),
        }
    }
}

impl Serializer for V2DeflateSerializer {
    type SerializeError = V2DeflateSerializeError;

    fn serialize<T: Counter, W: Write>(
        &mut self,
        h: &Histogram<T>,
        writer: &mut W,
    ) -> Result<usize, V2DeflateSerializeError> {
        // TODO benchmark serializing in chunks rather than all at once: each uncompressed v2 chunk
        // could be compressed and written to the compressed buf, possibly using an approach like
        // that of https://github.com/HdrHistogram/HdrHistogram_rust/issues/32#issuecomment-287583055.
        // This would reduce the overall buffer size needed for plain v2 serialization, and be
        // more cache friendly.

        self.uncompressed_buf.clear();
        self.compressed_buf.clear();
        // TODO serialize directly into uncompressed_buf without the buffering inside v2_serializer
        let uncompressed_len = self
            .v2_serializer
            .serialize(h, &mut self.uncompressed_buf)
            .map_err(V2DeflateSerializeError::InternalSerializationError)?;

        debug_assert_eq!(self.uncompressed_buf.len(), uncompressed_len);
        // On randomized test histograms we get about 10% compression, but of course random data
        // doesn't compress well. Real-world data may compress better, so let's assume a more
        // optimistic 50% compression as a baseline to reserve. If we're overly optimistic that's
        // still only one more allocation the first time it's needed.
        self.compressed_buf.reserve(self.uncompressed_buf.len() / 2);

        self.compressed_buf
            .write_u32::<BigEndian>(V2_COMPRESSED_COOKIE)?;
        // placeholder for length
        self.compressed_buf.write_u32::<BigEndian>(0)?;

        // TODO pluggable compressors? configurable compression levels?
        // TODO benchmark https://github.com/sile/libflate
        // TODO if uncompressed_len is near the limit of 16-bit usize, and compression grows the
        // data instead of shrinking it (which we cannot really predict), writing to compressed_buf
        // could panic as Vec overflows its internal `usize`.

        {
            // TODO reuse deflate buf, or switch to lower-level flate2::Compress
            let mut compressor = ZlibEncoder::new(&mut self.compressed_buf, Compression::default());
            compressor.write_all(&self.uncompressed_buf[0..uncompressed_len])?;
            let _ = compressor.finish()?;
        }

        // fill in length placeholder. Won't underflow since length is always at least 8, and won't
        // overflow u32 as the largest array is about 6 million entries, so about 54MiB encoded (if
        // counter is u64).
        let total_compressed_len = self.compressed_buf.len();
        (&mut self.compressed_buf[4..8])
            .write_u32::<BigEndian>((total_compressed_len as u32) - 8)?;

        writer.write_all(&self.compressed_buf)?;

        Ok(total_compressed_len)
    }
}
