use std::io;

use super::{Decoder, Encoder};

/// Decompress from the given source as if using a `Decoder`.
///
/// The input data must be in the zstd frame format.
pub fn decode_all<R: io::Read>(source: R) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    copy_decode(source, &mut result)?;
    Ok(result)
}

/// Decompress from the given source as if using a `Decoder`.
///
/// Decompressed data will be appended to `destination`.
pub fn copy_decode<R, W>(source: R, mut destination: W) -> io::Result<()>
where
    R: io::Read,
    W: io::Write,
{
    let mut decoder = Decoder::new(source)?;
    io::copy(&mut decoder, &mut destination)?;
    Ok(())
}

/// Compress all data from the given source as if using an `Encoder`.
///
/// Result will be in the zstd frame format.
///
/// A level of `0` uses zstd's default (currently `3`).
pub fn encode_all<R: io::Read>(source: R, level: i32) -> io::Result<Vec<u8>> {
    let mut result = Vec::<u8>::new();
    copy_encode(source, &mut result, level)?;
    Ok(result)
}

/// Compress all data from the given source as if using an `Encoder`.
///
/// Compressed data will be appended to `destination`.
///
/// A level of `0` uses zstd's default (currently `3`).
pub fn copy_encode<R, W>(
    mut source: R,
    destination: W,
    level: i32,
) -> io::Result<()>
where
    R: io::Read,
    W: io::Write,
{
    let mut encoder = Encoder::new(destination, level)?;
    io::copy(&mut source, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

#[cfg(tests)]
mod tests {}
