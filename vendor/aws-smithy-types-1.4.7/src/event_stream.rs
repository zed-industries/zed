/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types relevant to event stream serialization/deserialization

use crate::str_bytes::StrBytes;
use bytes::Bytes;

mod value {
    use crate::str_bytes::StrBytes;
    use crate::DateTime;
    use bytes::Bytes;

    /// Event Stream frame header value.
    #[non_exhaustive]
    #[derive(Clone, Debug, PartialEq)]
    pub enum HeaderValue {
        /// Represents a boolean value.
        Bool(bool),
        /// Represents a byte value.
        Byte(i8),
        /// Represents an int16 value.
        Int16(i16),
        /// Represents an int32 value.
        Int32(i32),
        /// Represents an int64 value.
        Int64(i64),
        /// Represents a byte array value.
        ByteArray(Bytes),
        /// Represents a string value.
        String(StrBytes),
        /// Represents a timestamp value.
        Timestamp(DateTime),
        /// Represents a uuid value.
        Uuid(u128),
    }

    impl HeaderValue {
        /// If the `HeaderValue` is a `Bool`, returns the associated `bool`. Returns `Err` otherwise.
        pub fn as_bool(&self) -> Result<bool, &Self> {
            match self {
                HeaderValue::Bool(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is a `Byte`, returns the associated `i8`. Returns `Err` otherwise.
        pub fn as_byte(&self) -> Result<i8, &Self> {
            match self {
                HeaderValue::Byte(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is an `Int16`, returns the associated `i16`. Returns `Err` otherwise.
        pub fn as_int16(&self) -> Result<i16, &Self> {
            match self {
                HeaderValue::Int16(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is an `Int32`, returns the associated `i32`. Returns `Err` otherwise.
        pub fn as_int32(&self) -> Result<i32, &Self> {
            match self {
                HeaderValue::Int32(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is an `Int64`, returns the associated `i64`. Returns `Err` otherwise.
        pub fn as_int64(&self) -> Result<i64, &Self> {
            match self {
                HeaderValue::Int64(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is a `ByteArray`, returns the associated [`Bytes`]. Returns `Err` otherwise.
        pub fn as_byte_array(&self) -> Result<&Bytes, &Self> {
            match self {
                HeaderValue::ByteArray(value) => Ok(value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is a `String`, returns the associated [`StrBytes`]. Returns `Err` otherwise.
        pub fn as_string(&self) -> Result<&StrBytes, &Self> {
            match self {
                HeaderValue::String(value) => Ok(value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is a `Timestamp`, returns the associated [`DateTime`]. Returns `Err` otherwise.
        pub fn as_timestamp(&self) -> Result<DateTime, &Self> {
            match self {
                HeaderValue::Timestamp(value) => Ok(*value),
                _ => Err(self),
            }
        }

        /// If the `HeaderValue` is a `Uuid`, returns the associated `u128`. Returns `Err` otherwise.
        pub fn as_uuid(&self) -> Result<u128, &Self> {
            match self {
                HeaderValue::Uuid(value) => Ok(*value),
                _ => Err(self),
            }
        }
    }
}

pub use value::HeaderValue;

/// Event Stream header.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    name: StrBytes,
    value: HeaderValue,
}

impl Header {
    /// Creates a new header with the given `name` and `value`.
    pub fn new(name: impl Into<StrBytes>, value: impl Into<HeaderValue>) -> Header {
        Header {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Returns the header name.
    pub fn name(&self) -> &StrBytes {
        &self.name
    }

    /// Returns the header value.
    pub fn value(&self) -> &HeaderValue {
        &self.value
    }
}

/// Event Stream message.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    headers: Vec<Header>,
    payload: Bytes,
}

impl Message {
    /// Creates a new message with the given `payload`. Headers can be added later.
    pub fn new(payload: impl Into<Bytes>) -> Message {
        Message {
            headers: Vec::new(),
            payload: payload.into(),
        }
    }

    /// Creates a message with the given `headers` and `payload`.
    pub fn new_from_parts(headers: Vec<Header>, payload: impl Into<Bytes>) -> Self {
        Self {
            headers,
            payload: payload.into(),
        }
    }

    /// Adds a header to the message.
    pub fn add_header(mut self, header: Header) -> Self {
        self.headers.push(header);
        self
    }

    /// Returns all headers.
    pub fn headers(&self) -> &[Header] {
        &self.headers
    }

    /// Returns the payload bytes.
    pub fn payload(&self) -> &Bytes {
        &self.payload
    }
}

/// Raw message from an event stream receiver when a response error is encountered.
#[derive(Debug)]
#[non_exhaustive]
pub enum RawMessage {
    /// Message was decoded into a valid frame, but failed to unmarshall into a modeled type.
    Decoded(Message),
    /// Message failed to be decoded into a valid frame. The raw bytes may not be available in the
    /// case where decoding consumed the buffer.
    Invalid(Option<Bytes>),
}

impl RawMessage {
    /// Creates a `RawMessage` for failure to decode a message into a valid frame.
    pub fn invalid(bytes: Option<Bytes>) -> Self {
        Self::Invalid(bytes)
    }
}
