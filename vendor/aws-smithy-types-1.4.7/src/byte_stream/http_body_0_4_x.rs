/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::body::SdkBody;
use crate::byte_stream::ByteStream;
use bytes::Bytes;

impl ByteStream {
    /// Construct a `ByteStream` from a type that implements [`http_body_0_4::Body<Data = Bytes>`](http_body_0_4::Body).
    ///
    /// _Note: This is only available when the `http-body-0-4-x` feature is enabled._
    pub fn from_body_0_4<T, E>(body: T) -> Self
    where
        T: http_body_0_4::Body<Data = Bytes, Error = E> + Send + Sync + 'static,
        E: Into<crate::body::Error> + 'static,
    {
        ByteStream::new(SdkBody::from_body_0_4(body))
    }
}

#[cfg(feature = "hyper-0-14-x")]
impl From<hyper_0_14::Body> for ByteStream {
    fn from(input: hyper_0_14::Body) -> Self {
        ByteStream::new(SdkBody::from_body_0_4(input))
    }
}

#[cfg(test)]
mod tests {
    use crate::body::SdkBody;
    use crate::byte_stream::Inner;
    use bytes::Bytes;

    #[cfg(feature = "hyper-0-14-x")]
    #[tokio::test]
    async fn read_from_channel_body() {
        let (mut sender, body) = hyper_0_14::Body::channel();
        let byte_stream = Inner::new(SdkBody::from_body_0_4(body));
        tokio::spawn(async move {
            sender.send_data(Bytes::from("data 1")).await.unwrap();
            sender.send_data(Bytes::from("data 2")).await.unwrap();
            sender.send_data(Bytes::from("data 3")).await.unwrap();
        });
        assert_eq!(
            byte_stream.collect().await.expect("no errors").into_bytes(),
            Bytes::from("data 1data 2data 3")
        );
    }

    #[cfg(feature = "rt-tokio")]
    #[tokio::test]
    async fn path_based_bytestreams() -> Result<(), Box<dyn std::error::Error>> {
        use super::ByteStream;
        use bytes::Buf;
        use std::io::Write;
        use tempfile::NamedTempFile;
        let mut file = NamedTempFile::new()?;

        for i in 0..10000 {
            writeln!(file, "Brian was here. Briefly. {i}")?;
        }
        let body = ByteStream::from_path(&file).await?.into_inner();
        // assert that a valid size hint is immediately ready
        assert_eq!(body.content_length(), Some(298890));
        let mut body1 = body.try_clone().expect("retryable bodies are cloneable");
        // read a little bit from one of the clones
        let some_data = body1
            .next()
            .await
            .expect("should have some data")
            .expect("read should not fail");
        assert!(!some_data.is_empty());
        // make some more clones
        let body2 = body.try_clone().expect("retryable bodies are cloneable");
        let body3 = body.try_clone().expect("retryable bodies are cloneable");
        let body2 = ByteStream::new(body2).collect().await?.into_bytes();
        let body3 = ByteStream::new(body3).collect().await?.into_bytes();
        assert_eq!(body2, body3);
        assert!(body2.starts_with(b"Brian was here."));
        assert!(body2.ends_with(b"9999\n"));
        assert_eq!(body2.len(), 298890);

        assert_eq!(
            ByteStream::new(body1).collect().await?.remaining(),
            298890 - some_data.len()
        );

        Ok(())
    }
}
