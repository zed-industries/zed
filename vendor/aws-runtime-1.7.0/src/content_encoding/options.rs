/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_types::config_bag::{Storable, StoreReplace};

use super::{
    CHUNK_SIGNATURE_BEGIN, CHUNK_TERMINATOR, CRLF, DEFAULT_CHUNK_SIZE_BYTE, SIGNATURE_LENGTH,
};

/// Options used when constructing an [`AwsChunkedBody`](super::AwsChunkedBody).
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct AwsChunkedBodyOptions {
    /// The total size of the stream.
    pub(crate) stream_length: u64,
    /// The length of each trailer sent within an `AwsChunkedBody`. Necessary in
    /// order to correctly calculate the total size of the body accurately.
    pub(crate) trailer_lengths: Vec<u64>,
    /// Whether the aws-chunked encoding is disabled. This could occur, for instance,
    /// if a user specifies a custom checksum, rendering aws-chunked encoding unnecessary.
    pub(crate) disabled: bool,
    /// Whether chunks and trailer are signed.
    pub(crate) is_signed: bool,
    /// The size of each chunk in bytes.
    /// None means use default (64 KiB)
    pub(crate) chunk_size: Option<usize>,
}

impl Storable for AwsChunkedBodyOptions {
    type Storer = StoreReplace<Self>;
}

impl AwsChunkedBodyOptions {
    /// Create a new [`AwsChunkedBodyOptions`].
    pub fn new(stream_length: u64, trailer_lengths: Vec<u64>) -> Self {
        Self {
            stream_length,
            trailer_lengths,
            disabled: false,
            is_signed: false,
            chunk_size: None,
        }
    }

    /// Set the chunk size for aws-chunked encoding.
    ///
    /// This allows customizing the size of each chunk when using aws-chunked encoding.
    /// The chunk size is validated by the interceptor (minimum 8 KiB).
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = Some(chunk_size);
        self
    }

    /// Get the chunk size that will be used for aws-chunked encoding.
    ///
    /// Returns the configured chunk size, or the default if not set.
    pub fn chunk_size(&self) -> usize {
        self.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE_BYTE)
    }

    pub(super) fn total_trailer_length(&self) -> u64 {
        self.trailer_lengths.iter().sum::<u64>()
            // We need to account for a CRLF after each trailer name/value pair
            + (self.trailer_lengths.len() * CRLF.len()) as u64
    }

    /// Set the stream length in the options
    pub fn with_stream_length(mut self, stream_length: u64) -> Self {
        self.stream_length = stream_length;
        self
    }

    /// Append a trailer length to the options
    pub fn with_trailer_len(mut self, trailer_len: u64) -> Self {
        self.trailer_lengths.push(trailer_len);
        self
    }

    /// Return whether there are no trailers
    pub fn is_trailer_empty(&self) -> bool {
        self.trailer_lengths.is_empty()
    }

    /// Create a new [`AwsChunkedBodyOptions`] with aws-chunked encoding disabled.
    ///
    /// When the option is disabled, the body must not be wrapped in an `AwsChunkedBody`.
    pub fn disable_chunked_encoding() -> Self {
        Self {
            disabled: true,
            ..Default::default()
        }
    }

    /// Return whether aws-chunked encoding is disabled.
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Set whether to use signed chunked encoding
    pub fn signed_chunked_encoding(mut self, is_signed: bool) -> Self {
        self.is_signed = is_signed;
        self
    }

    /// Return the length of the body after `aws-chunked` encoding is applied
    pub fn encoded_length(&self) -> u64 {
        if self.is_signed {
            self.signed_encoded_length()
        } else {
            self.unsigned_encoded_length()
        }
    }

    fn signed_encoded_length(&self) -> u64 {
        let number_of_data_chunks = self.stream_length / self.chunk_size() as u64;
        let remaining_data_chunk = self.stream_length % self.chunk_size() as u64;

        let mut length = number_of_data_chunks
            * get_signed_chunk_bytes_length(self.chunk_size() as u64)
            + if remaining_data_chunk > 0 {
                get_signed_chunk_bytes_length(remaining_data_chunk)
            } else {
                0
            };

        // End chunk
        length += get_signed_chunk_bytes_length(0);

        length -= CRLF.len() as u64; // The last CRLF is not needed for 0-sized signed chunk

        // Trailers
        for len in self.trailer_lengths.iter() {
            length += len + CRLF.len() as u64;
        }

        // Encoding terminator
        length += CRLF.len() as u64;

        length
    }

    fn unsigned_encoded_length(&self) -> u64 {
        let number_of_data_chunks = self.stream_length / self.chunk_size() as u64;
        let remaining_data_chunk = self.stream_length % self.chunk_size() as u64;

        let mut length = number_of_data_chunks
            * get_unsigned_chunk_bytes_length(self.chunk_size() as u64)
            + if remaining_data_chunk > 0 {
                get_unsigned_chunk_bytes_length(remaining_data_chunk)
            } else {
                0
            };

        // End chunk
        length += CHUNK_TERMINATOR.len() as u64;

        // Trailers
        for len in self.trailer_lengths.iter() {
            length += len + CRLF.len() as u64;
        }

        // Encoding terminator
        length += CRLF.len() as u64;

        length
    }
}

fn int_log16<T>(mut i: T) -> u64
where
    T: std::ops::DivAssign + PartialOrd + From<u8> + Copy,
{
    let mut len = 0;
    let zero = T::from(0);
    let sixteen = T::from(16);

    // Handle an edge case where 0 is passed in, which still requires 1 hex digit to represent
    if i == zero {
        return 1;
    }

    while i > zero {
        i /= sixteen;
        len += 1;
    }

    len
}

// Return the length of a signed chunk:
//
// A signed chunk looks like:
// 10000;chunk-signature=b474d8862b1487a5145d686f57f013e54db672cee1c953b3010fb58501ef5aa2\r\n
// <65536-bytes>\r\n
fn get_signed_chunk_bytes_length(payload_length: u64) -> u64 {
    let hex_repr_len = int_log16(payload_length);
    hex_repr_len
        + CHUNK_SIGNATURE_BEGIN.len() as u64
        + SIGNATURE_LENGTH as u64
        + CRLF.len() as u64
        + payload_length
        + CRLF.len() as u64
}

// Return the length of an unsigned chunk:
//
// An unsigned chunk looks like:
// 10000\r\n
// <65536-bytes>\r\n
fn get_unsigned_chunk_bytes_length(payload_length: u64) -> u64 {
    let hex_repr_len = int_log16(payload_length);
    hex_repr_len + CRLF.len() as u64 + payload_length + CRLF.len() as u64
}

#[cfg(test)]
mod tests {
    use super::int_log16;

    #[test]
    fn test_int_log16() {
        assert_eq!(int_log16(0u64), 1); // 0x0
        assert_eq!(int_log16(1u64), 1); // 0x1
        assert_eq!(int_log16(15u64), 1); // 0xF
        assert_eq!(int_log16(16u64), 2); // 0x10
        assert_eq!(int_log16(255u64), 2); // 0xFF
        assert_eq!(int_log16(256u64), 3); // 0x100
        assert_eq!(int_log16(4095u64), 3); // 0xFFF
        assert_eq!(int_log16(4096u64), 4); // 0x1000
        assert_eq!(int_log16(65535u64), 4); // 0xFFFF
        assert_eq!(int_log16(65536u64), 5); // 0x10000
        assert_eq!(int_log16(1048575u64), 5); // 0xFFFFF
        assert_eq!(int_log16(1048576u64), 6); // 0x100000
        assert_eq!(int_log16(u64::MAX), 16); // 0xFFFFFFFFFFFFFFFF
    }
}
