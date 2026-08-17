/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Functionality for calculating the checksum of an HTTP body and emitting it as trailers.

use super::ChecksumCache;
use crate::http::HttpChecksum;

use aws_smithy_http::header::append_merge_header_maps_http_1x;
use aws_smithy_types::body::SdkBody;
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::warn;

pin_project! {
    /// A body-wrapper that will calculate the `InnerBody`'s checksum and emit it as a trailer.
    pub struct ChecksumBody<InnerBody> {
            #[pin]
            body: InnerBody,
            checksum: Option<Box<dyn HttpChecksum>>,
            written_trailers: bool,
            cache: Option<ChecksumCache>
    }
}

impl ChecksumBody<SdkBody> {
    /// Given an `SdkBody` and a `Box<dyn HttpChecksum>`, create a new `ChecksumBody<SdkBody>`.
    pub fn new(body: SdkBody, checksum: Box<dyn HttpChecksum>) -> Self {
        Self {
            body,
            checksum: Some(checksum),
            written_trailers: false,
            cache: None,
        }
    }

    /// Configure a cache for this body.
    ///
    /// When used across multiple requests (e.g. retries) a cached checksum previously
    /// calculated will be favored if available.
    pub fn with_cache(self, cache: ChecksumCache) -> Self {
        Self {
            body: self.body,
            checksum: self.checksum,
            written_trailers: false,
            cache: Some(cache),
        }
    }

    // It would be nicer if this could take &self, but I couldn't make that
    // work out with the Pin/Projection types, so its a static method for now
    fn extract_or_set_cached_headers(
        maybe_cache: &Option<ChecksumCache>,
        checksum: Box<dyn HttpChecksum>,
    ) -> http_1x::HeaderMap {
        let calculated_headers = checksum.headers();
        if let Some(cache) = maybe_cache {
            if let Some(cached_headers) = cache.get() {
                if cached_headers != calculated_headers {
                    warn!(cached = ?cached_headers, calculated = ?calculated_headers, "calculated checksum differs from cached checksum!");
                }
                cached_headers
            } else {
                cache.set(calculated_headers.clone());
                calculated_headers
            }
        } else {
            calculated_headers
        }
    }
}

impl http_body_1x::Body for ChecksumBody<SdkBody> {
    type Data = bytes::Bytes;
    type Error = aws_smithy_types::body::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        let poll_res = this.body.poll_frame(cx);

        match &poll_res {
            Poll::Ready(Some(Ok(frame))) => {
                // Update checksum for data frames
                if frame.is_data() {
                    if let Some(checksum) = this.checksum {
                        checksum.update(frame.data_ref().expect("Data frame has data"));
                    }
                } else {
                    // Add checksum trailer to other trailers if necessary
                    let checksum_headers = if let Some(checksum) = this.checksum.take() {
                        ChecksumBody::extract_or_set_cached_headers(this.cache, checksum)
                    } else {
                        return Poll::Ready(None);
                    };
                    let trailers = frame
                        .trailers_ref()
                        .expect("Trailers frame has trailers")
                        .clone();
                    *this.written_trailers = true;
                    return Poll::Ready(Some(Ok(http_body_1x::Frame::trailers(
                        append_merge_header_maps_http_1x(trailers, checksum_headers),
                    ))));
                }
            }
            Poll::Ready(None) => {
                // If the trailers have not already been written (because there were no existing
                // trailers on the body) we write them here
                if !*this.written_trailers {
                    let checksum_headers = if let Some(checksum) = this.checksum.take() {
                        ChecksumBody::extract_or_set_cached_headers(this.cache, checksum)
                    } else {
                        return Poll::Ready(None);
                    };
                    let trailers = http_1x::HeaderMap::new();
                    return Poll::Ready(Some(Ok(http_body_1x::Frame::trailers(
                        append_merge_header_maps_http_1x(trailers, checksum_headers),
                    ))));
                }
            }
            _ => {}
        };
        poll_res
    }
}

#[cfg(test)]
mod tests {
    use super::ChecksumBody;
    use crate::{http::CRC_32_HEADER_NAME, ChecksumAlgorithm, CRC_32_NAME};
    use aws_smithy_types::base64;
    use aws_smithy_types::body::SdkBody;
    use bytes::Buf;
    use bytes_utils::SegmentedBuf;
    use http_1x::HeaderMap;
    use http_body_util::BodyExt;
    use std::fmt::Write;
    use std::io::Read;

    fn header_value_as_checksum_string(header_value: &http_1x::HeaderValue) -> String {
        let decoded_checksum = base64::decode(header_value.to_str().unwrap()).unwrap();
        let decoded_checksum = decoded_checksum
            .into_iter()
            .fold(String::new(), |mut acc, byte| {
                write!(acc, "{byte:02X?}").expect("string will always be writeable");
                acc
            });

        format!("0x{decoded_checksum}")
    }

    #[tokio::test]
    async fn test_checksum_body() {
        let input_text = "This is some test text for an SdkBody";
        let body = SdkBody::from(input_text);
        let checksum = CRC_32_NAME
            .parse::<ChecksumAlgorithm>()
            .unwrap()
            .into_impl();
        let mut body = ChecksumBody::new(body, checksum);

        let mut output_data = SegmentedBuf::new();
        let mut trailers = HeaderMap::new();
        while let Some(buf) = body.frame().await {
            let buf = buf.unwrap();
            if buf.is_data() {
                output_data.push(buf.into_data().unwrap());
            } else if buf.is_trailers() {
                let map = buf.into_trailers().unwrap();
                map.into_iter().for_each(|(k, v)| {
                    trailers.insert(k.unwrap(), v);
                });
            }
        }

        let mut output_text = String::new();
        output_data
            .reader()
            .read_to_string(&mut output_text)
            .expect("Doesn't cause IO errors");
        // Verify data is complete and unaltered
        assert_eq!(input_text, output_text);

        let checksum_trailer = trailers
            .get(CRC_32_HEADER_NAME)
            .expect("trailers contain crc32 checksum");
        let checksum_trailer = header_value_as_checksum_string(checksum_trailer);

        // Known correct checksum for the input "This is some test text for an SdkBody"
        assert_eq!("0x99B01F72", checksum_trailer);
    }
}
