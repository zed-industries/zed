// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chrono::{DateTime, Utc};
use livekit_protocol::{data_stream as proto, enum_dispatch};
use std::collections::HashMap;
use thiserror::Error;

mod incoming;
mod outgoing;

pub use incoming::*;
pub use outgoing::*;

use crate::e2ee::EncryptionType;

/// Result type for data stream operations.
pub type StreamResult<T> = Result<T, StreamError>;

/// Error type for data stream operations.
#[derive(Debug, Error)]
pub enum StreamError {
    // TODO(ladvoc): standardize error cases and expose over FFI.
    #[error("stream has already been closed")]
    AlreadyClosed,

    #[error("stream closed abnormally: {0}")]
    AbnormalEnd(String),

    #[error("UTF-8 decoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("incoming header was invalid")]
    InvalidHeader,

    #[error("expected chunk index to be exactly one more than the previous")]
    MissedChunk,

    #[error("read length exceeded total length specified in stream header")]
    LengthExceeded,

    #[error("stream data is incomplete")]
    Incomplete,

    #[error("unable to send packet")]
    SendFailed,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error")]
    Internal,

    #[error("encryption type mismatch")]
    EncryptionTypeMismatch,
}

/// Progress of a data stream.
#[derive(Clone, Copy, Default, Debug, Hash, Eq, PartialEq)]
struct StreamProgress {
    chunk_index: u64,
    /// Number of bytes read or written so far.
    bytes_processed: u64,
    /// Total number of bytes expected to be read or written for finite streams.
    bytes_total: Option<u64>,
}

impl StreamProgress {
    /// Returns the completion percentage for finite streams.
    #[allow(dead_code)]
    fn percentage(&self) -> Option<f32> {
        self.bytes_total.map(|total| self.bytes_processed as f32 / total as f32)
    }
}

/// Information about a byte data stream.
#[derive(Clone, Debug)]
pub struct ByteStreamInfo {
    /// Unique identifier of the stream.
    pub id: String,
    /// Topic name used to route the stream to the appropriate handler.
    pub topic: String,
    /// When the stream was created.
    pub timestamp: DateTime<Utc>,
    /// Total expected size in bytes, if known.
    pub total_length: Option<u64>,
    /// Additional attributes as needed for your application.
    pub attributes: HashMap<String, String>,
    /// The MIME type of the stream data.
    pub mime_type: String,
    /// The name of the file being sent.
    pub name: String,
    /// The encryption used
    pub encryption_type: EncryptionType,
}

/// Information about a text data stream.
#[derive(Clone, Debug)]
pub struct TextStreamInfo {
    /// Unique identifier of the stream.
    pub id: String,
    /// Topic name used to route the stream to the appropriate handler.
    pub topic: String,
    /// When the stream was created.
    pub timestamp: DateTime<Utc>,
    /// Total expected size in bytes, if known.
    pub total_length: Option<u64>,
    /// Additional attributes as needed for your application.
    pub attributes: HashMap<String, String>,
    /// The MIME type of the stream data.
    pub mime_type: String,
    pub operation_type: OperationType,
    pub version: i32,
    pub reply_to_stream_id: Option<String>,
    pub attached_stream_ids: Vec<String>,
    pub generated: bool,
    /// The encryption used
    pub encryption_type: EncryptionType,
}

/// Operation type for text streams.
#[derive(Clone, Copy, Default, Debug, Hash, Eq, PartialEq)]
pub enum OperationType {
    #[default]
    Create,
    Update,
    Delete,
    Reaction,
}

// MARK: - Protocol type conversion

impl TryFrom<proto::Header> for AnyStreamInfo {
    type Error = StreamError;

    fn try_from(mut header: proto::Header) -> Result<Self, Self::Error> {
        Self::try_from_with_encryption(header, EncryptionType::None)
    }
}

impl AnyStreamInfo {
    pub fn try_from_with_encryption(
        mut header: proto::Header,
        encryption_type: EncryptionType,
    ) -> Result<Self, StreamError> {
        let Some(content_header) = header.content_header.take() else {
            Err(StreamError::InvalidHeader)?
        };
        let info = match content_header {
            proto::header::ContentHeader::ByteHeader(byte_header) => Self::Byte(
                ByteStreamInfo::from_headers_with_encryption(header, byte_header, encryption_type),
            ),
            proto::header::ContentHeader::TextHeader(text_header) => Self::Text(
                TextStreamInfo::from_headers_with_encryption(header, text_header, encryption_type),
            ),
        };
        Ok(info)
    }
}

impl ByteStreamInfo {
    pub(crate) fn from_headers(header: proto::Header, byte_header: proto::ByteHeader) -> Self {
        Self::from_headers_with_encryption(header, byte_header, EncryptionType::None)
    }

    pub(crate) fn from_headers_with_encryption(
        header: proto::Header,
        byte_header: proto::ByteHeader,
        encryption_type: EncryptionType,
    ) -> Self {
        Self {
            id: header.stream_id,
            topic: header.topic,
            timestamp: DateTime::<Utc>::from_timestamp_millis(header.timestamp)
                .unwrap_or_else(|| Utc::now()),
            total_length: header.total_length,
            attributes: header.attributes,
            mime_type: header.mime_type,
            name: byte_header.name,
            encryption_type,
        }
    }
}

impl TextStreamInfo {
    pub(crate) fn from_headers(header: proto::Header, text_header: proto::TextHeader) -> Self {
        Self::from_headers_with_encryption(header, text_header, EncryptionType::None)
    }

    pub(crate) fn from_headers_with_encryption(
        header: proto::Header,
        text_header: proto::TextHeader,
        encryption_type: EncryptionType,
    ) -> Self {
        Self {
            id: header.stream_id,
            topic: header.topic,
            timestamp: DateTime::<Utc>::from_timestamp_millis(header.timestamp)
                .unwrap_or_else(|| Utc::now()),
            total_length: header.total_length,
            attributes: header.attributes,
            mime_type: header.mime_type,
            operation_type: text_header.operation_type().into(),
            version: text_header.version,
            reply_to_stream_id: (!text_header.reply_to_stream_id.is_empty())
                .then_some(text_header.reply_to_stream_id),
            attached_stream_ids: text_header.attached_stream_ids,
            generated: text_header.generated,
            encryption_type,
        }
    }
}

impl From<proto::OperationType> for OperationType {
    fn from(op_type: proto::OperationType) -> Self {
        match op_type {
            proto::OperationType::Create => OperationType::Create,
            proto::OperationType::Update => OperationType::Update,
            proto::OperationType::Delete => OperationType::Delete,
            proto::OperationType::Reaction => OperationType::Reaction,
        }
    }
}
// MARK: - Dispatch

#[derive(Clone, Debug)]
pub(crate) enum AnyStreamInfo {
    Byte(ByteStreamInfo),
    Text(TextStreamInfo),
}

impl AnyStreamInfo {
    enum_dispatch!(
        [Byte, Text];
        pub fn id(self: &Self) -> &str;
        pub fn total_length(self: &Self) -> Option<u64>;
        pub fn encryption_type(self: &Self) -> EncryptionType;
    );
}

#[rustfmt::skip]
macro_rules! stream_info {
    () => {
        fn id(&self) -> &str { &self.id }
        fn total_length(&self) -> Option<u64> { self.total_length }
        fn encryption_type(&self) -> EncryptionType { self.encryption_type }
    };
}

impl ByteStreamInfo {
    stream_info!();
}

impl TextStreamInfo {
    stream_info!();
}

impl From<ByteStreamInfo> for AnyStreamInfo {
    fn from(info: ByteStreamInfo) -> Self {
        Self::Byte(info)
    }
}

impl From<TextStreamInfo> for AnyStreamInfo {
    fn from(info: TextStreamInfo) -> Self {
        Self::Text(info)
    }
}
