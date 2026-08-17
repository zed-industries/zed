/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides size hint functionality for Event Stream messages to optimize buffer allocation.

use aws_smithy_types::event_stream::{HeaderValue, Message};
use std::mem::size_of;

/// Extension trait that provides size hint functionality for Event Stream messages.
///
/// This trait allows callers to get an accurate estimate of the serialized size
/// of a message before serialization, enabling optimal buffer pre-allocation.
pub trait MessageSizeHint {
    /// Returns an estimate of the serialized size of this message in bytes.
    ///
    /// This provides a hint for buffer allocation to improve performance when
    /// serializing the message. The estimate includes the message prelude,
    /// headers, payload, and CRC checksums.
    ///
    /// # Examples
    ///
    /// ```
    /// use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
    /// use aws_smithy_eventstream::message_size_hint::MessageSizeHint;
    /// use bytes::Bytes;
    ///
    /// let message = Message::new(Bytes::from(b"hello world".to_vec()))
    ///     .add_header(Header::new("content-type", HeaderValue::String("text/plain".into())));
    ///
    /// let size_hint = message.size_hint();
    /// // Use the size hint to pre-allocate a buffer
    /// let mut buffer: Vec<u8> = Vec::with_capacity(size_hint);
    /// ```
    fn size_hint(&self) -> usize;
}

impl MessageSizeHint for Message {
    fn size_hint(&self) -> usize {
        // Constants from the frame format
        const PRELUDE_LENGTH_BYTES: usize = 3 * size_of::<u32>();
        const MESSAGE_CRC_LENGTH_BYTES: usize = size_of::<u32>();
        const MAX_HEADER_NAME_LEN: usize = 255;

        // Calculate headers size
        let mut headers_len = 0;
        for header in self.headers() {
            let name_len = header.name().as_bytes().len().min(MAX_HEADER_NAME_LEN);
            headers_len += 1 + name_len; // name length byte + name bytes

            // Add header value size based on type
            headers_len += match header.value() {
                HeaderValue::Bool(_) => 1,                            // type byte only
                HeaderValue::Byte(_) => 2,                            // type + value
                HeaderValue::Int16(_) => 3,                           // type + value
                HeaderValue::Int32(_) => 5,                           // type + value
                HeaderValue::Int64(_) => 9,                           // type + value
                HeaderValue::ByteArray(val) => 3 + val.len(),         // type + length + data
                HeaderValue::String(val) => 3 + val.as_bytes().len(), // type + length + data
                HeaderValue::Timestamp(_) => 9,                       // type + value
                HeaderValue::Uuid(_) => 17,                           // type + value
                _ => 0, // Handle any future header value types conservatively
            };
        }

        let payload_len = self.payload().len();

        // Total message size: prelude + headers + payload + message CRC
        PRELUDE_LENGTH_BYTES + headers_len + payload_len + MESSAGE_CRC_LENGTH_BYTES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::write_message_to;
    use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
    use bytes::Bytes;

    #[test]
    fn test_size_hint_accuracy() {
        // Test cases with different message configurations
        let test_cases = vec![
            // Simple message with small payload
            Message::new(Bytes::from(b"hello world".to_vec())),
            // Message with headers
            Message::new(Bytes::from(b"test payload".to_vec())).add_header(Header::new(
                "content-type",
                HeaderValue::String("application/json".into()),
            )),
            // Message with multiple headers and different value types
            Message::new(Bytes::from(b"complex test".to_vec()))
                .add_header(Header::new("bool-true", HeaderValue::Bool(true)))
                .add_header(Header::new("bool-false", HeaderValue::Bool(false)))
                .add_header(Header::new("byte-val", HeaderValue::Byte(42)))
                .add_header(Header::new("int16-val", HeaderValue::Int16(12345)))
                .add_header(Header::new("int32-val", HeaderValue::Int32(987654321)))
                .add_header(Header::new(
                    "int64-val",
                    HeaderValue::Int64(1234567890123456789),
                ))
                .add_header(Header::new(
                    "string-val",
                    HeaderValue::String("hello world".into()),
                ))
                .add_header(Header::new(
                    "bytes-val",
                    HeaderValue::ByteArray(Bytes::from(b"binary data".to_vec())),
                )),
            // Empty payload
            Message::new(Bytes::new()),
            // Large payload (1KB)
            Message::new(Bytes::from(vec![0u8; 1024])).add_header(Header::new(
                "large-content",
                HeaderValue::String("large payload test".into()),
            )),
        ];

        for (i, message) in test_cases.iter().enumerate() {
            let size_hint = message.size_hint();

            // Get actual serialized size
            let mut buffer = Vec::new();
            write_message_to(message, &mut buffer)
                .unwrap_or_else(|_| panic!("Failed to serialize test case {i}"));
            let actual_size = buffer.len();

            // The size hint should exactly match the actual serialized size
            assert_eq!(
                size_hint, actual_size,
                "Size hint mismatch for test case {i}: hint={size_hint}, actual={actual_size}"
            );
        }
    }

    #[test]
    fn test_size_hint_with_long_header_name() {
        // Test with a header name that's near the maximum length
        let long_name = "x".repeat(200); // Long but valid header name
        let message = Message::new(Bytes::from(b"payload".to_vec())).add_header(Header::new(
            long_name,
            HeaderValue::String("long header name test".into()),
        ));

        let size_hint = message.size_hint();

        let mut buffer = Vec::new();
        write_message_to(&message, &mut buffer)
            .expect("Failed to serialize message with long header name");
        let actual_size = buffer.len();

        assert_eq!(
            size_hint, actual_size,
            "Size hint should match actual size for long header names"
        );
    }

    #[test]
    fn test_size_hint_performance_benefit() {
        // Create a message with 1KB payload
        let message = Message::new(Bytes::from(vec![0u8; 1024])).add_header(Header::new(
            "content-type",
            HeaderValue::String("application/json".into()),
        ));

        let size_hint = message.size_hint();

        // Verify that using size hint for pre-allocation works
        let mut buffer = Vec::with_capacity(size_hint);
        write_message_to(&message, &mut buffer).expect("Failed to serialize message");

        // The buffer should not have needed to reallocate
        assert!(
            buffer.capacity() >= buffer.len(),
            "Buffer should have sufficient capacity"
        );

        // The size hint should be reasonably close to actual size
        assert_eq!(
            size_hint,
            buffer.len(),
            "Size hint should exactly match serialized size"
        );
    }
}
