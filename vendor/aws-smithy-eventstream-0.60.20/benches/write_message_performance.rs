/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_eventstream::frame::write_message_to;
use aws_smithy_eventstream::message_size_hint::MessageSizeHint;
use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
use bytes::{BufMut, Bytes};
use crc32fast::Hasher;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::mem::size_of;

/// Configuration for buffer allocation strategies in benchmarks
#[derive(Debug, Clone)]
pub enum BufferConfig {
    /// Start with an empty buffer (Vec::new() - zero capacity)
    Empty,
    /// Pre-allocate buffer with exact message size
    ExactSize(usize),
    /// Pre-allocate buffer smaller than message size (factor < 1.0)
    Undersized(f32),
    /// Pre-allocate buffer larger than message size (factor > 1.0)
    Oversized(f32),
}

impl BufferConfig {
    /// Get a description of the buffer configuration for benchmark naming
    pub fn description(&self) -> &'static str {
        match self {
            BufferConfig::Empty => "empty",
            BufferConfig::ExactSize(_) => "exact",
            BufferConfig::Undersized(_) => "undersized",
            BufferConfig::Oversized(_) => "oversized",
        }
    }
}

/// Get the actual serialized size of a message by running serialization once
pub fn get_message_size(message: &Message) -> usize {
    let mut buffer = Vec::new();
    write_message_to(message, &mut buffer).expect("Failed to serialize message");
    buffer.len()
}

/// Create a buffer based on the configuration and actual message size
pub fn create_buffer(config: &BufferConfig, message_size: usize) -> Vec<u8> {
    match config {
        BufferConfig::Empty => Vec::new(),
        BufferConfig::ExactSize(_) => Vec::with_capacity(message_size),
        BufferConfig::Undersized(factor) => {
            let capacity = ((message_size as f32) * factor) as usize;
            Vec::with_capacity(capacity.max(1)) // Ensure at least 1 byte capacity
        }
        BufferConfig::Oversized(factor) => {
            let capacity = ((message_size as f32) * factor) as usize;
            Vec::with_capacity(capacity)
        }
    }
}

// Global allocator configuration for different allocators
#[cfg(all(feature = "__bench-jemalloc", not(feature = "__bench-mimalloc")))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(feature = "__bench-mimalloc", not(feature = "__bench-jemalloc")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Constants from the frame module
const PRELUDE_LENGTH_BYTES: u32 = 3 * size_of::<u32>() as u32;
const MESSAGE_CRC_LENGTH_BYTES: u32 = size_of::<u32>() as u32;
const MAX_HEADER_NAME_LEN: usize = 255;

const TYPE_TRUE: u8 = 0;
const TYPE_FALSE: u8 = 1;
const TYPE_BYTE: u8 = 2;
const TYPE_INT16: u8 = 3;
const TYPE_INT32: u8 = 4;
const TYPE_INT64: u8 = 5;
const TYPE_BYTE_ARRAY: u8 = 6;
const TYPE_STRING: u8 = 7;
const TYPE_TIMESTAMP: u8 = 8;
const TYPE_UUID: u8 = 9;

/// Optimized version of write_message to remove inline buffer. Stashing this here—this implemenetation
/// is about 10% faster but needs more testing.
pub fn write_message_to_optimized_v1(
    message: &Message,
    buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Pre-calculate header size without allocating
    let mut headers_len = 0u32;
    for header in message.headers() {
        headers_len += calculate_header_size(header)?;
    }

    let payload_len = message.payload().len() as u32;
    let message_len = PRELUDE_LENGTH_BYTES + headers_len + payload_len + MESSAGE_CRC_LENGTH_BYTES;

    // Reserve space upfront
    buffer.reserve(message_len as usize);

    // Write prelude with CRC
    let mut crc = Hasher::new();

    // Write message length
    buffer.put_u32(message_len);
    crc.update(&message_len.to_be_bytes());

    // Write headers length
    buffer.put_u32(headers_len);
    crc.update(&headers_len.to_be_bytes());

    // Write prelude CRC and include it in the ongoing CRC calculation
    let prelude_crc = crc.clone().finalize();
    buffer.put_u32(prelude_crc);
    crc.update(&prelude_crc.to_be_bytes());

    // Write headers directly to buffer
    for header in message.headers() {
        write_header_to_optimized(header, buffer, &mut crc)?;
    }

    // Write payload
    buffer.put_slice(message.payload());
    crc.update(message.payload());

    // Write message CRC
    buffer.put_u32(crc.finalize());

    Ok(())
}

/// Calculate the size a header will take when serialized
fn calculate_header_size(header: &Header) -> Result<u32, Box<dyn std::error::Error>> {
    let name_len = header.name().as_bytes().len();
    if name_len > MAX_HEADER_NAME_LEN {
        return Err("Header name too long".into());
    }

    let mut size = 1 + name_len; // name length byte + name bytes

    use HeaderValue::*;
    match header.value() {
        Bool(_) => size += 1,                            // type byte only
        Byte(_) => size += 2,                            // type + value
        Int16(_) => size += 3,                           // type + value
        Int32(_) => size += 5,                           // type + value
        Int64(_) => size += 9,                           // type + value
        ByteArray(val) => size += 3 + val.len(),         // type + length + data
        String(val) => size += 3 + val.as_bytes().len(), // type + length + data
        Timestamp(_) => size += 9,                       // type + value
        Uuid(_) => size += 17,                           // type + value
        _ => return Err("Unsupported header value type".into()),
    }

    Ok(size as u32)
}

/// Write header directly to buffer with CRC update
fn write_header_to_optimized(
    header: &Header,
    buffer: &mut Vec<u8>,
    crc: &mut Hasher,
) -> Result<(), Box<dyn std::error::Error>> {
    let name_bytes = header.name().as_bytes();
    if name_bytes.len() > MAX_HEADER_NAME_LEN {
        return Err("Header name too long".into());
    }

    // Write name length
    let name_len = name_bytes.len() as u8;
    buffer.put_u8(name_len);
    crc.update(&[name_len]);

    // Write name
    buffer.put_slice(name_bytes);
    crc.update(name_bytes);

    // Write value
    write_header_value_to_optimized(header.value(), buffer, crc)?;

    Ok(())
}

/// Get the name of the current allocator for benchmark naming
fn get_allocator_name() -> &'static str {
    #[cfg(all(feature = "__bench-jemalloc", not(feature = "__bench-mimalloc")))]
    return "jemalloc";

    #[cfg(all(feature = "__bench-mimalloc", not(feature = "__bench-jemalloc")))]
    return "mimalloc";

    #[cfg(not(any(feature = "__bench-jemalloc", feature = "__bench-mimalloc")))]
    return "system";

    #[cfg(all(feature = "__bench-jemalloc", feature = "__bench-mimalloc"))]
    return "system"; // When both features are enabled, default to system allocator
}

/// Write header value directly to buffer with CRC update
fn write_header_value_to_optimized(
    value: &HeaderValue,
    buffer: &mut Vec<u8>,
    crc: &mut Hasher,
) -> Result<(), Box<dyn std::error::Error>> {
    use HeaderValue::*;
    match value {
        Bool(val) => {
            let type_byte = if *val { TYPE_TRUE } else { TYPE_FALSE };
            buffer.put_u8(type_byte);
            crc.update(&[type_byte]);
        }
        Byte(val) => {
            buffer.put_u8(TYPE_BYTE);
            buffer.put_i8(*val);
            crc.update(&[TYPE_BYTE]);
            crc.update(&val.to_be_bytes());
        }
        Int16(val) => {
            buffer.put_u8(TYPE_INT16);
            buffer.put_i16(*val);
            crc.update(&[TYPE_INT16]);
            crc.update(&val.to_be_bytes());
        }
        Int32(val) => {
            buffer.put_u8(TYPE_INT32);
            buffer.put_i32(*val);
            crc.update(&[TYPE_INT32]);
            crc.update(&val.to_be_bytes());
        }
        Int64(val) => {
            buffer.put_u8(TYPE_INT64);
            buffer.put_i64(*val);
            crc.update(&[TYPE_INT64]);
            crc.update(&val.to_be_bytes());
        }
        ByteArray(val) => {
            if val.len() > u16::MAX as usize {
                return Err("Byte array too long".into());
            }
            buffer.put_u8(TYPE_BYTE_ARRAY);
            buffer.put_u16(val.len() as u16);
            buffer.put_slice(val);
            crc.update(&[TYPE_BYTE_ARRAY]);
            crc.update(&(val.len() as u16).to_be_bytes());
            crc.update(val);
        }
        String(val) => {
            let bytes = val.as_bytes();
            if bytes.len() > u16::MAX as usize {
                return Err("String too long".into());
            }
            buffer.put_u8(TYPE_STRING);
            buffer.put_u16(bytes.len() as u16);
            buffer.put_slice(bytes);
            crc.update(&[TYPE_STRING]);
            crc.update(&(bytes.len() as u16).to_be_bytes());
            crc.update(bytes);
        }
        Timestamp(time) => {
            let millis = time.to_millis().map_err(|_| "Timestamp too large")?;
            buffer.put_u8(TYPE_TIMESTAMP);
            buffer.put_i64(millis);
            crc.update(&[TYPE_TIMESTAMP]);
            crc.update(&millis.to_be_bytes());
        }
        Uuid(val) => {
            buffer.put_u8(TYPE_UUID);
            buffer.put_u128(*val);
            crc.update(&[TYPE_UUID]);
            crc.update(&val.to_be_bytes());
        }
        _ => return Err("Unsupported header value type".into()),
    }
    Ok(())
}

/// Optimized version 2: Just pre-allocate buffer size, keep everything else simple
pub fn write_message_preallocate(
    message: &Message,
    buffer: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Simple size estimation: headers are typically small, so just estimate
    let estimated_size = message.size_hint(); // rough estimate for headers + overhead
    buffer.reserve(estimated_size);

    // Use the original implementation but with Vec<u8> instead of dyn BufMut
    write_message_to(message, buffer).map_err(|e| e.into())
}

fn benchmark_write_message_to(c: &mut Criterion) {
    let allocator = get_allocator_name();
    // Create a simple test message with 1KB payload
    let payload = Bytes::from(vec![0u8; 1024]);
    let message = Message::new(payload).add_header(Header::new(
        "content-type",
        HeaderValue::String("application/json".into()),
    ));

    // Test different buffer configurations
    let buffer_configs = vec![
        BufferConfig::Empty,
        BufferConfig::Undersized(0.5),
        BufferConfig::ExactSize(0), // Will be calculated
        BufferConfig::Oversized(2.0),
    ];

    let actual_size = message.size_hint();

    // Original implementation benchmarks
    for buffer_config in &buffer_configs {
        let bench_name = format!(
            "original_write_message_to_1kb_{}_buffer_{allocator}",
            buffer_config.description()
        );

        c.bench_function(&bench_name, |b| {
            b.iter(|| {
                let mut buffer = create_buffer(buffer_config, actual_size);
                write_message_to(&message, &mut buffer).unwrap();
                black_box(buffer);
            });
        });
    }

    // Optimized v1 implementation benchmarks (complex but fastest)
    for buffer_config in &buffer_configs {
        let bench_name = format!(
            "optimized_v1_write_message_to_1kb_{}_buffer_{allocator}",
            buffer_config.description()
        );

        c.bench_function(&bench_name, |b| {
            b.iter(|| {
                let mut buffer = create_buffer(buffer_config, actual_size);
                write_message_to_optimized_v1(&message, &mut buffer).unwrap();
                black_box(buffer);
            });
        });
    }

    // Optimized v2 implementation benchmarks (simple pre-allocation)
    for buffer_config in &buffer_configs {
        let bench_name = format!(
            "optimized_v2_write_message_to_1kb_{}_buffer_{allocator}",
            buffer_config.description()
        );

        c.bench_function(&bench_name, |b| {
            b.iter(|| {
                let mut buffer = create_buffer(buffer_config, actual_size);
                write_message_preallocate(&message, &mut buffer).unwrap();
                black_box(buffer);
            });
        });
    }
}

/// Verification test to ensure both implementations produce identical output
#[cfg(test)]
fn verify_implementations_match() {
    use bytes::Bytes;

    // Test cases with different message configurations
    let test_cases = vec![
        // Simple message with 1KB payload
        Message::new(Bytes::from(vec![0u8; 1024])).add_header(Header::new(
            "content-type",
            HeaderValue::String("application/json".into()),
        )),
        // Empty payload
        Message::new(Bytes::new()),
        // Message with multiple headers and different value types
        Message::new(Bytes::from(b"test payload".to_vec()))
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
        // Large payload
        Message::new(Bytes::from(vec![0xAB; 4096])).add_header(Header::new(
            "large-content",
            HeaderValue::String("large payload test".into()),
        )),
        // Message with long header name (near limit)
        Message::new(Bytes::from(b"payload".to_vec())).add_header(Header::new(
            "x".repeat(200), // Long but valid header name
            HeaderValue::String("long header name test".into()),
        )),
    ];

    for (i, message) in test_cases.iter().enumerate() {
        println!("Testing case {}: {:?}", i, message.headers().len());

        // Test original implementation
        let mut original_buffer = Vec::new();
        write_message_to(message, &mut original_buffer)
            .unwrap_or_else(|_| panic!("Original implementation failed for test case {i}"));

        // Test all optimized implementations
        let mut optimized_v1_buffer = Vec::new();
        write_message_to_optimized_v1(message, &mut optimized_v1_buffer)
            .unwrap_or_else(|_| panic!("Optimized v1 implementation failed for test case {i}"));

        let mut optimized_v2_buffer = Vec::new();
        write_message_preallocate(message, &mut optimized_v2_buffer)
            .unwrap_or_else(|_| panic!("Optimized v2 implementation failed for test case {i}"));

        // Compare results
        assert_eq!(
            original_buffer, optimized_v1_buffer,
            "V1 implementation produces different output for test case {}\nOriginal length: {}, V1 length: {}",
            i, original_buffer.len(), optimized_v1_buffer.len()
        );

        assert_eq!(
            original_buffer, optimized_v2_buffer,
            "V2 implementation produces different output for test case {}\nOriginal length: {}, V2 length: {}",
            i, original_buffer.len(), optimized_v2_buffer.len()
        );

        // Verify the output can be read back correctly
        let parsed_message = aws_smithy_eventstream::frame::read_message_from(&mut Bytes::from(
            original_buffer.clone(),
        ))
        .unwrap_or_else(|_| panic!("Failed to parse original output for test case {i}"));

        // Verify headers match
        assert_eq!(
            message.headers(),
            parsed_message.headers(),
            "Headers don't match after round-trip for test case {i}"
        );

        // Verify payload matches
        assert_eq!(
            message.payload().as_ref(),
            parsed_message.payload().as_ref(),
            "Payload doesn't match after round-trip for test case {i}"
        );

        println!("✓ Test case {} passed - {} bytes", i, original_buffer.len());
    }

    println!("All verification tests passed!");
}

/// Run verification during benchmarks
fn benchmark_write_message_to_with_verification(c: &mut Criterion) {
    // First run verification
    verify_implementations_match();

    // Then run the actual benchmarks
    benchmark_write_message_to(c);
}

criterion_group!(benches, benchmark_write_message_to_with_verification);
criterion_main!(benches);
