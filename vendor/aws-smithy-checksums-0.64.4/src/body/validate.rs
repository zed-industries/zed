/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Functionality for validating an HTTP body against a given precalculated checksum and emitting an
//! error if it doesn't match.

use crate::http::HttpChecksum;

use aws_smithy_types::body::SdkBody;

use bytes::Bytes;
use pin_project_lite::pin_project;

use std::fmt::Display;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    /// A body-wrapper that will calculate the `InnerBody`'s checksum and emit an error if it
    /// doesn't match the precalculated checksum.
    pub struct ChecksumBody<InnerBody> {
        #[pin]
        inner: InnerBody,
        checksum: Option<Box<dyn HttpChecksum>>,
        precalculated_checksum: Bytes,
    }
}

impl ChecksumBody<SdkBody> {
    /// Given an `SdkBody`, a `Box<dyn HttpChecksum>`, and a precalculated checksum represented
    /// as `Bytes`, create a new `ChecksumBody<SdkBody>`.
    pub fn new(
        body: SdkBody,
        checksum: Box<dyn HttpChecksum>,
        precalculated_checksum: Bytes,
    ) -> Self {
        Self {
            inner: body,
            checksum: Some(checksum),
            precalculated_checksum,
        }
    }

    fn poll_inner(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Bytes>, aws_smithy_types::body::Error>>> {
        use http_body_1x::Body;

        let this = self.project();
        let checksum = this.checksum;

        match this.inner.poll_frame(cx) {
            Poll::Ready(Some(Ok(frame))) => {
                let data = frame.data_ref().expect("Data frame should have data");
                tracing::trace!(
                    "reading {} bytes from the body and updating the checksum calculation",
                    data.len()
                );
                let checksum = match checksum.as_mut() {
                    Some(checksum) => checksum,
                    None => {
                        unreachable!("The checksum must exist because it's only taken out once the inner body has been completely polled.");
                    }
                };

                checksum.update(data);
                Poll::Ready(Some(Ok(frame)))
            }
            // Once the inner body has stopped returning data, check the checksum
            // and return an error if it doesn't match.
            Poll::Ready(None) => {
                tracing::trace!("finished reading from body, calculating final checksum");
                let checksum = match checksum.take() {
                    Some(checksum) => checksum,
                    None => {
                        // If the checksum was already taken and this was polled again anyways,
                        // then return nothing
                        return Poll::Ready(None);
                    }
                };

                let actual_checksum = checksum.finalize();
                if *this.precalculated_checksum == actual_checksum {
                    Poll::Ready(None)
                } else {
                    // So many parens it's starting to look like LISP
                    Poll::Ready(Some(Err(Box::new(Error::ChecksumMismatch {
                        expected: this.precalculated_checksum.clone(),
                        actual: actual_checksum,
                    }))))
                }
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Errors related to checksum calculation and validation
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The actual checksum didn't match the expected checksum. The checksummed data has been
    /// altered since the expected checksum was calculated.
    ChecksumMismatch { expected: Bytes, actual: Bytes },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::ChecksumMismatch { expected, actual } => write!(
                f,
                "body checksum mismatch. expected body checksum to be {} but it was {}",
                hex::encode(expected),
                hex::encode(actual)
            ),
        }
    }
}

impl std::error::Error for Error {}

impl http_body_1x::Body for ChecksumBody<SdkBody> {
    type Data = Bytes;
    type Error = aws_smithy_types::body::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<http_body_1x::Frame<Self::Data>, Self::Error>>> {
        self.poll_inner(cx)
    }
}

#[cfg(test)]
mod tests {
    use crate::body::validate::{ChecksumBody, Error};
    use crate::ChecksumAlgorithm;
    use aws_smithy_types::body::SdkBody;
    use bytes::{Buf, Bytes};
    use bytes_utils::SegmentedBuf;
    use http_body_util::BodyExt;
    use std::io::Read;

    fn calculate_crc32_checksum(input: &str) -> Bytes {
        let checksum =
            crc_fast::checksum(crc_fast::CrcAlgorithm::Crc32IsoHdlc, input.as_bytes()) as u32;

        Bytes::copy_from_slice(&checksum.to_be_bytes())
    }

    #[tokio::test]
    async fn test_checksum_validated_body_errors_on_mismatch() {
        let input_text = "This is some test text for an SdkBody";
        let actual_checksum = calculate_crc32_checksum(input_text);
        let body = SdkBody::from(input_text);
        let non_matching_checksum = Bytes::copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        let mut body = ChecksumBody::new(
            body,
            "crc32".parse::<ChecksumAlgorithm>().unwrap().into_impl(),
            non_matching_checksum.clone(),
        );

        while let Some(data) = body.frame().await {
            match data {
                Ok(_) => { /* Do nothing */ }
                Err(e) => {
                    match e.downcast_ref::<Error>().unwrap() {
                        Error::ChecksumMismatch { expected, actual } => {
                            assert_eq!(expected, &non_matching_checksum);
                            assert_eq!(actual, &actual_checksum);
                        }
                    }

                    return;
                }
            }
        }

        panic!("didn't hit expected error condition");
    }

    #[tokio::test]
    async fn test_checksum_validated_body_succeeds_on_match() {
        let input_text = "This is some test text for an SdkBody";
        let actual_checksum = calculate_crc32_checksum(input_text);
        let body = SdkBody::from(input_text);
        let http_checksum = "crc32".parse::<ChecksumAlgorithm>().unwrap().into_impl();
        let mut body = ChecksumBody::new(body, http_checksum, actual_checksum);

        let mut output = SegmentedBuf::new();
        while let Some(buf) = body.frame().await {
            let data = buf.unwrap().into_data().unwrap();
            output.push(data);
        }

        let mut output_text = String::new();
        output
            .reader()
            .read_to_string(&mut output_text)
            .expect("Doesn't cause IO errors");
        // Verify data is complete and unaltered
        assert_eq!(input_text, output_text);
    }
}
