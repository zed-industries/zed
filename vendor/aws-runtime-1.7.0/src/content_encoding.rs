/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

mod body;
mod options;
mod sign;

pub use body::AwsChunkedBody;
pub use options::AwsChunkedBodyOptions;
pub(crate) use sign::SignChunk;
pub use sign::{DeferredSigner, DeferredSignerSender};

const CRLF: &str = "\r\n";
const CRLF_RAW: &[u8] = b"\r\n";

const CHUNK_SIGNATURE_BEGIN: &str = ";chunk-signature=";
const CHUNK_SIGNATURE_BEGIN_RAW: &[u8] = b";chunk-signature=";

const CHUNK_TERMINATOR: &str = "0\r\n";
const CHUNK_TERMINATOR_RAW: &[u8] = b"0\r\n";

const TRAILER_SEPARATOR: &[u8] = b":";

const DEFAULT_CHUNK_SIZE_BYTE: usize = 64 * 1024; // 64 KB

const SIGNATURE_LENGTH: usize = 64;

/// Content encoding header name constants
pub mod header {
    /// Header name denoting "x-amz-trailer-signature"
    pub const X_AMZ_TRAILER_SIGNATURE: &str = "x-amz-trailer-signature";
}

/// Content encoding header value constants
pub mod header_value {
    /// Header value denoting "aws-chunked" encoding
    pub const AWS_CHUNKED: &str = "aws-chunked";
}
