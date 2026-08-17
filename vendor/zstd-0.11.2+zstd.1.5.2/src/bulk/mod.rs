//! Compress and decompress data in bulk.
//!
//! These methods process all the input data at once.
//! It is therefore best used with relatively small blocks
//! (like small network packets).

mod compressor;
mod decompressor;

#[cfg(test)]
mod tests;

pub use self::compressor::Compressor;
pub use self::decompressor::Decompressor;

use std::io;

/// Compresses a single block of data to the given destination buffer.
///
/// Returns the number of bytes written, or an error if something happened
/// (for instance if the destination buffer was too small).
///
/// A level of `0` uses zstd's default (currently `3`).
pub fn compress_to_buffer(
    source: &[u8],
    destination: &mut [u8],
    level: i32,
) -> io::Result<usize> {
    Compressor::new(level)?.compress_to_buffer(source, destination)
}

/// Compresses a block of data and returns the compressed result.
///
/// A level of `0` uses zstd's default (currently `3`).
pub fn compress(data: &[u8], level: i32) -> io::Result<Vec<u8>> {
    Compressor::new(level)?.compress(data)
}

/// Deompress a single block of data to the given destination buffer.
///
/// Returns the number of bytes written, or an error if something happened
/// (for instance if the destination buffer was too small).
pub fn decompress_to_buffer(
    source: &[u8],
    destination: &mut [u8],
) -> io::Result<usize> {
    Decompressor::new()?.decompress_to_buffer(source, destination)
}

/// Decompresses a block of data and returns the decompressed result.
///
/// The decompressed data should be less than `capacity` bytes,
/// or an error will be returned.
pub fn decompress(data: &[u8], capacity: usize) -> io::Result<Vec<u8>> {
    Decompressor::new()?.decompress(data, capacity)
}
