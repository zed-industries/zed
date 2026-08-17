/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Adapters to use http-body 1.0 bodies with SdkBody & ByteStream

use crate::body::SdkBody;
use crate::byte_stream::ByteStream;
use bytes::Bytes;

impl ByteStream {
    /// Construct a `ByteStream` from a type that implements [`http_body_1_0::Body<Data = Bytes>`](http_body_1_0::Body).
    ///
    /// _Note: This is only available when the `http-body-1-x` feature is enabled._
    pub fn from_body_1_x<T, E>(body: T) -> Self
    where
        T: http_body_1_0::Body<Data = Bytes, Error = E> + Send + Sync + 'static,
        E: Into<crate::body::Error> + 'static,
    {
        ByteStream::new(SdkBody::from_body_1_x(body))
    }
}
