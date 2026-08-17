// Copyright (c) 2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! <https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316>
//!
//! As with other ZIP libraries, we face the predicament that the end of central directory record may contain a
//! variable-length file comment. As a result, we cannot just make the assumption that the start of this record is
//! 18 bytes (the length of the EOCDR) offset from the end of the data - we must locate it ourselves.
//!
//! The `zip-rs` crate handles this by reading in reverse from the end of the data. This involves seeking backwards
//! by a single byte each iteration and reading 4 bytes into a u32. Whether this is performant/acceptable within a
//! a non-async context, I'm unsure, but it isn't desirable within an async context. Especially since we cannot just
//! place a [`BufReader`] infront of the upstream reader (as its internal buffer is invalidated on each seek).
//!
//! Reading in reverse is still desirable as the use of file comments is limited and they're unlikely to be large.
//!
//! The below method is one that compromises on these two contention points. Please submit an issue or PR if you know
//! of a better algorithm for this (and have tested/verified its performance).

#[cfg(doc)]
use futures_lite::io::BufReader;

use crate::error::{Result as ZipResult, ZipError};
use crate::spec::consts::{EOCDR_LENGTH, EOCDR_SIGNATURE, SIGNATURE_LENGTH};

use futures_lite::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, SeekFrom};

/// The buffer size used when locating the EOCDR, equal to 2KiB.
const BUFFER_SIZE: usize = 2048;

/// The upper bound of where the EOCDR signature cannot be located.
const EOCDR_UPPER_BOUND: u64 = EOCDR_LENGTH as u64;

/// The lower bound of where the EOCDR signature cannot be located.
const EOCDR_LOWER_BOUND: u64 = EOCDR_UPPER_BOUND + SIGNATURE_LENGTH as u64 + u16::MAX as u64;

/// Locate the `end of central directory record` offset, if one exists.
/// The returned offset excludes the signature (4 bytes)
///
/// This method involves buffered reading in reverse and reverse linear searching along those buffers for the EOCDR
/// signature. As a result of this buffered approach, we reduce seeks when compared to `zip-rs`'s method by a factor
/// of the buffer size. We also then don't have to do individual u32 reads against the upstream reader.
///
/// Whilst I haven't done any in-depth benchmarks, when reading a ZIP file with the maximum length comment, this method
/// saw a reduction in location time by a factor of 500 when compared with the `zip-rs` method.
pub async fn eocdr<R>(mut reader: R) -> ZipResult<u64>
where
    R: AsyncRead + AsyncSeek + Unpin,
{
    let length = reader.seek(SeekFrom::End(0)).await?;
    let signature = &EOCDR_SIGNATURE.to_le_bytes();
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let mut position = length.saturating_sub(BUFFER_SIZE as u64);
    reader.seek(SeekFrom::Start(position)).await?;

    loop {
        let mut read = 0;

        // Read repeatedly until eof or the buffer is full.
        while read < BUFFER_SIZE {
            let bytes_read = reader.read(&mut buffer[read..]).await?;
            if bytes_read == 0 {
                break;
            }
            read += bytes_read;
        }

        if let Some(match_index) = reverse_search_buffer(&buffer[..read], signature) {
            return Ok(position + (match_index + 1) as u64);
        }

        // If we hit the start of the data or the lower bound, we're unable to locate the EOCDR.
        if position == 0 || position <= length.saturating_sub(EOCDR_LOWER_BOUND) {
            return Err(ZipError::UnableToLocateEOCDR);
        }

        // To handle the case where the EOCDR signature crosses buffer boundaries, we simply overlap reads by the
        // signature length. This significantly reduces the complexity of handling partial matches with very little
        // overhead.
        position = position.saturating_sub((BUFFER_SIZE - SIGNATURE_LENGTH) as u64);
        reader.seek(SeekFrom::Start(position)).await?;
    }
}

/// A naive reverse linear search along the buffer for the specified signature bytes.
///
/// This is already surprisingly performant. For instance, using memchr::memchr() to match for the first byte of the
/// signature, and then manual byte comparisons for the remaining signature bytes was actually slower by a factor of
/// 2.25. This method was explored as tokio's `read_until()` implementation uses memchr::memchr().
pub(crate) fn reverse_search_buffer(buffer: &[u8], signature: &[u8]) -> Option<usize> {
    'outer: for index in (0..buffer.len()).rev() {
        for (signature_index, signature_byte) in signature.iter().rev().enumerate() {
            if let Some(next_index) = index.checked_sub(signature_index) {
                if buffer[next_index] != *signature_byte {
                    continue 'outer;
                }
            } else {
                break 'outer;
            }
        }
        return Some(index);
    }
    None
}
