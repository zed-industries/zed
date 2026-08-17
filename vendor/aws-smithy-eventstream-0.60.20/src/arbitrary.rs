/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Defines new-types wrapping inner types from `aws_smithy_types` to enable the `Arbitrary` trait
//! for fuzz testing.

use aws_smithy_types::event_stream::{Header, HeaderValue, Message};
use aws_smithy_types::str_bytes::StrBytes;
use aws_smithy_types::DateTime;
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct ArbHeaderValue(HeaderValue);

impl<'a> arbitrary::Arbitrary<'a> for ArbHeaderValue {
    fn arbitrary(unstruct: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let value_type: u8 = unstruct.int_in_range(0..=9)?;
        let header_value = match value_type {
            crate::frame::TYPE_TRUE => HeaderValue::Bool(true),
            crate::frame::TYPE_FALSE => HeaderValue::Bool(false),
            crate::frame::TYPE_BYTE => HeaderValue::Byte(i8::arbitrary(unstruct)?),
            crate::frame::TYPE_INT16 => HeaderValue::Int16(i16::arbitrary(unstruct)?),
            crate::frame::TYPE_INT32 => HeaderValue::Int32(i32::arbitrary(unstruct)?),
            crate::frame::TYPE_INT64 => HeaderValue::Int64(i64::arbitrary(unstruct)?),
            crate::frame::TYPE_BYTE_ARRAY => {
                HeaderValue::ByteArray(Bytes::from(Vec::<u8>::arbitrary(unstruct)?))
            }
            crate::frame::TYPE_STRING => {
                HeaderValue::String(StrBytes::from(String::arbitrary(unstruct)?))
            }
            crate::frame::TYPE_TIMESTAMP => {
                HeaderValue::Timestamp(DateTime::from_secs(i64::arbitrary(unstruct)?))
            }
            crate::frame::TYPE_UUID => HeaderValue::Uuid(u128::arbitrary(unstruct)?),
            _ => unreachable!(),
        };
        Ok(ArbHeaderValue(header_value))
    }
}

impl From<ArbHeaderValue> for HeaderValue {
    fn from(header_value: ArbHeaderValue) -> Self {
        header_value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArbStrBytes(StrBytes);

#[cfg(feature = "derive-arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for ArbStrBytes {
    fn arbitrary(unstruct: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(ArbStrBytes(String::arbitrary(unstruct)?.into()))
    }
}

impl From<ArbStrBytes> for StrBytes {
    fn from(str_bytes: ArbStrBytes) -> Self {
        str_bytes.0
    }
}

#[derive(Clone, Debug, PartialEq, derive_arbitrary::Arbitrary)]
pub struct ArbHeader {
    name: ArbStrBytes,
    value: ArbHeaderValue,
}

impl From<ArbHeader> for Header {
    fn from(header: ArbHeader) -> Self {
        Self::new(header.name, header.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArbMessage(Message);

impl<'a> arbitrary::Arbitrary<'a> for ArbMessage {
    fn arbitrary(unstruct: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let headers: Vec<ArbHeader> = unstruct
            .arbitrary_iter()?
            .collect::<arbitrary::Result<_>>()?;
        let message = Message::new_from_parts(
            headers.into_iter().map(Into::into).collect(),
            Bytes::from(Vec::<u8>::arbitrary(unstruct)?),
        );
        Ok(ArbMessage(message))
    }
}

impl From<ArbMessage> for Message {
    fn from(message: ArbMessage) -> Self {
        message.0
    }
}
