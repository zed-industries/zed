/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! ByteStream Abstractions
//!
//! When the SDK returns streaming binary data, the inner Http Body is
//! wrapped in [`ByteStream`]. ByteStream provides misuse-resistant primitives
//! to make it easier to handle common patterns with streaming data.
//!
//! # Examples
//!
//! ### Writing a ByteStream into a file:
//! ```no_run
//! use aws_smithy_types::byte_stream::ByteStream;
//! use std::error::Error;
//! use tokio::fs::File;
//! use tokio::io::AsyncWriteExt;
//! struct SynthesizeSpeechOutput {
//!     audio_stream: ByteStream,
//! }
//!
//! async fn audio_to_file(
//!     output: SynthesizeSpeechOutput,
//! ) -> Result<(), Box<dyn Error + Send + Sync>> {
//!     let mut buf = output.audio_stream.collect().await?;
//!     let mut file = File::open("audio.mp3").await?;
//!     file.write_all_buf(&mut buf).await?;
//!     file.flush().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Converting a ByteStream into Bytes
//! ```no_run
//! use bytes::Bytes;
//! use aws_smithy_types::byte_stream::ByteStream;
//! use std::error::Error;
//! struct SynthesizeSpeechOutput {
//!     audio_stream: ByteStream,
//! }
//! async fn load_audio(
//!     output: SynthesizeSpeechOutput,
//! ) -> Result<Bytes, Box<dyn Error + Send + Sync>> {
//!     Ok(output.audio_stream.collect().await?.into_bytes())
//! }
//! ```
//!
//! ### Stream a ByteStream into a file
//! The previous example is recommended in cases where loading the entire file into memory first is desirable. For extremely large
//! files, you may wish to stream the data directly to the file system, chunk by chunk.
//! This is possible using the [`.next()`](crate::byte_stream::ByteStream::next) method.
//!
//! ```no_run
//! use bytes::{Buf, Bytes};
//! use aws_smithy_types::byte_stream::ByteStream;
//! use std::error::Error;
//! use tokio::fs::File;
//! use tokio::io::AsyncWriteExt;
//! use tokio_stream::StreamExt;
//! struct SynthesizeSpeechOutput {
//!     audio_stream: ByteStream,
//! }
//!
//! async fn audio_to_file(
//!     output: SynthesizeSpeechOutput,
//! ) -> Result<(), Box<dyn Error + Send + Sync>> {
//!     let mut file = File::open("audio.mp3").await?;
//!     let mut stream = output.audio_stream;
//!     while let Some(bytes) = stream.next().await {
//!         let bytes: Bytes = bytes?;
//!         file.write_all(&bytes).await?;
//!     }
//!     file.flush().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Create a ByteStream from a file
//!
//! _Note: This is only available with `rt-tokio` enabled._
//!
//! ```no_run
//! # #[cfg(feature = "rt-tokio")]
//! # {
//! use aws_smithy_types::byte_stream::ByteStream;
//! use std::path::Path;
//! struct GetObjectInput {
//!   body: ByteStream
//! }
//!
//! async fn bytestream_from_file() -> GetObjectInput {
//!     let bytestream = ByteStream::from_path("docs/some-large-file.csv")
//!         .await
//!         .expect("valid path");
//!     GetObjectInput { body: bytestream }
//! }
//! # }
//! ```
//!
//! If you want more control over how the file is read, such as specifying the size of the buffer used to read the file
//! or the length of the file, use an `FsBuilder`.
//!
//! ```no_run
//! # #[cfg(feature = "rt-tokio")]
//! # {
//! use aws_smithy_types::byte_stream::{ByteStream, Length};
//! use std::path::Path;
//! struct GetObjectInput {
//!     body: ByteStream
//! }
//!
//! async fn bytestream_from_file() -> GetObjectInput {
//!     let bytestream = ByteStream::read_from().path("docs/some-large-file.csv")
//!         .buffer_size(32_784)
//!         .length(Length::Exact(123_456))
//!         .build()
//!         .await
//!         .expect("valid path");
//!     GetObjectInput { body: bytestream }
//! }
//! # }
//! ```

use crate::body::SdkBody;
use crate::byte_stream::error::Error;
use bytes::Buf;
use bytes::Bytes;
use bytes_utils::SegmentedBuf;
use pin_project_lite::pin_project;
use std::future::poll_fn;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "rt-tokio")]
mod bytestream_util;
#[cfg(feature = "rt-tokio")]
pub use bytestream_util::Length;

pub mod error;

#[cfg(feature = "rt-tokio")]
pub use self::bytestream_util::FsBuilder;

/// This module is named after the `http-body` version number since we anticipate
/// needing to provide equivalent functionality for 1.x of that crate in the future.
/// The name has a suffix `_x` to avoid name collision with a third-party `http-body-0-4`.
#[cfg(feature = "http-body-0-4-x")]
pub mod http_body_0_4_x;

#[cfg(feature = "http-body-1-x")]
pub mod http_body_1_x;

pin_project! {
    /// Stream of binary data
    ///
    /// `ByteStream` wraps a stream of binary data for ease of use.
    ///
    /// ## Getting data out of a `ByteStream`
    ///
    /// `ByteStream` provides two primary mechanisms for accessing the data:
    /// 1. With `.collect()`:
    ///
    ///     [`.collect()`](crate::byte_stream::ByteStream::collect) reads the complete ByteStream into memory and stores it in `AggregatedBytes`,
    ///     a non-contiguous ByteBuffer.
    ///     ```no_run
    ///     use aws_smithy_types::byte_stream::{ByteStream, AggregatedBytes};
    ///     use aws_smithy_types::body::SdkBody;
    ///     use bytes::Buf;
    ///     async fn example() {
    ///        let stream = ByteStream::new(SdkBody::from("hello! This is some data"));
    ///        // Load data from the stream into memory:
    ///        let data = stream.collect().await.expect("error reading data");
    ///        // collect returns a `bytes::Buf`:
    ///        println!("first chunk: {:?}", data.chunk());
    ///     }
    ///     ```
    /// 2. Via [`.next()`](crate::byte_stream::ByteStream::next) or [`.try_next()`](crate::byte_stream::ByteStream::try_next):
    ///
    ///     For use-cases where holding the entire ByteStream in memory is unnecessary, use the
    ///     `Stream` implementation:
    ///     ```no_run
    ///     # mod crc32 {
    ///     #   pub struct Digest { }
    ///     #   impl Digest {
    ///     #       pub fn new() -> Self { Digest {} }
    ///     #       pub fn write(&mut self, b: &[u8]) { }
    ///     #       pub fn finish(&self) -> u64 { 6 }
    ///     #   }
    ///     # }
    ///     use aws_smithy_types::byte_stream::{ByteStream, AggregatedBytes, error::Error};
    ///     use aws_smithy_types::body::SdkBody;
    ///
    ///     async fn example() -> Result<(), Error> {
    ///        let mut stream = ByteStream::from(vec![1, 2, 3, 4, 5, 99]);
    ///        let mut digest = crc32::Digest::new();
    ///        while let Some(bytes) = stream.try_next().await? {
    ///            digest.write(&bytes);
    ///        }
    ///        println!("digest: {}", digest.finish());
    ///        Ok(())
    ///     }
    ///     ```
    ///
    /// 3. Via [`.into_async_read()`](crate::byte_stream::ByteStream::into_async_read):
    ///
    ///     _Note: The `rt-tokio` feature must be active to use `.into_async_read()`._
    ///
    ///     It's possible to convert a `ByteStream` into a struct that implements [`tokio::io::AsyncBufRead`](tokio::io::AsyncBufRead).
    ///     ```no_run
    ///     use aws_smithy_types::byte_stream::ByteStream;
    ///     use aws_smithy_types::body::SdkBody;
    ///     use tokio::io::AsyncBufReadExt;
    ///     #[cfg(feature = "rt-tokio")]
    ///     async fn example() -> std::io::Result<()> {
    ///        let stream = ByteStream::new(SdkBody::from("hello!\nThis is some data"));
    ///        // Convert the stream to a BufReader
    ///        let buf_reader = stream.into_async_read();
    ///        let mut lines = buf_reader.lines();
    ///        assert_eq!(lines.next_line().await?, Some("hello!".to_owned()));
    ///        assert_eq!(lines.next_line().await?, Some("This is some data".to_owned()));
    ///        assert_eq!(lines.next_line().await?, None);
    ///        Ok(())
    ///     }
    ///     ```
    ///
    /// ## Getting data into a ByteStream
    /// ByteStreams can be created in one of three ways:
    /// 1. **From in-memory binary data**: ByteStreams created from in-memory data are always retryable. Data
    /// will be converted into `Bytes` enabling a cheap clone during retries.
    ///     ```no_run
    ///     use bytes::Bytes;
    ///     use aws_smithy_types::byte_stream::ByteStream;
    ///     let stream = ByteStream::from(vec![1,2,3]);
    ///     let stream = ByteStream::from(Bytes::from_static(b"hello!"));
    ///     ```
    ///
    /// 2. **From a file**: ByteStreams created from a path can be retried. A new file descriptor will be opened if a retry occurs.
    ///     ```no_run
    ///     #[cfg(feature = "tokio-rt")]
    ///     # {
    ///     use aws_smithy_types::byte_stream::ByteStream;
    ///     let stream = ByteStream::from_path("big_file.csv");
    ///     # }
    ///     ```
    ///
    /// 3. **From an `SdkBody` directly**: For more advanced / custom use cases, a ByteStream can be created directly
    /// from an SdkBody. **When created from an SdkBody, care must be taken to ensure retriability.** An SdkBody is retryable
    /// when constructed from in-memory data or when using [`SdkBody::retryable`](crate::body::SdkBody::retryable).
    ///     ```ignore
    ///     # use hyper_0_14 as hyper;
    ///     use aws_smithy_types::byte_stream::ByteStream;
    ///     use aws_smithy_types::body::SdkBody;
    ///     use bytes::Bytes;
    ///     let (mut tx, channel_body) = hyper::Body::channel();
    ///     // this will not be retryable because the SDK has no way to replay this stream
    ///     let stream = ByteStream::new(SdkBody::from_body_0_4(channel_body));
    ///     tx.send_data(Bytes::from_static(b"hello world!"));
    ///     tx.send_data(Bytes::from_static(b"hello again!"));
    ///     // NOTE! You must ensure that `tx` is dropped to ensure that EOF is sent
    ///     ```
    ///
    #[derive(Debug)]
    pub struct ByteStream {
        #[pin]
        inner: Inner,
    }
}

impl ByteStream {
    /// Create a new `ByteStream` from an [`SdkBody`].
    pub fn new(body: SdkBody) -> Self {
        Self {
            inner: Inner::new(body),
        }
    }

    /// Create a new `ByteStream` from a static byte slice.
    pub fn from_static(bytes: &'static [u8]) -> Self {
        Self {
            inner: Inner::new(SdkBody::from(Bytes::from_static(bytes))),
        }
    }

    /// Consume the `ByteStream`, returning the wrapped SdkBody.
    // Backwards compatibility note: Because SdkBody has a dyn variant,
    // we will always be able to implement this method, even if we stop using
    // SdkBody as the internal representation
    pub fn into_inner(self) -> SdkBody {
        self.inner.body
    }

    /// Return the next item in the `ByteStream`.
    ///
    /// There is also a sibling method [`try_next`](ByteStream::try_next), which returns a `Result<Option<Bytes>, Error>`
    /// instead of an `Option<Result<Bytes, Error>>`.
    pub async fn next(&mut self) -> Option<Result<Bytes, Error>> {
        Some(self.inner.next().await?.map_err(Error::streaming))
    }

    #[cfg(feature = "byte-stream-poll-next")]
    /// Attempt to pull out the next value of this stream, returning `None` if the stream is
    /// exhausted.
    // This should only be used when one needs to implement a trait method like
    // `futures_core::stream::Stream::poll_next` on a new-type wrapping a `ByteStream`.
    // In general, use the `next` method instead.
    pub fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Error>>> {
        self.project().inner.poll_next(cx).map_err(Error::streaming)
    }

    /// Consume and return the next item in the `ByteStream` or return an error if an error is
    /// encountered.
    ///
    /// Similar to the [`next`](ByteStream::next) method, but this returns a `Result<Option<Bytes>, Error>` rather than
    /// an `Option<Result<Bytes, Error>>`, making for easy use with the `?` operator.
    pub async fn try_next(&mut self) -> Result<Option<Bytes>, Error> {
        self.next().await.transpose()
    }

    /// Returns a reference to the data if it is already available in memory
    pub fn bytes(&self) -> Option<&[u8]> {
        let Inner { body } = &self.inner;
        body.bytes()
    }

    /// Return the bounds on the remaining length of the `ByteStream`.
    pub fn size_hint(&self) -> (u64, Option<u64>) {
        self.inner.size_hint()
    }

    /// Read all the data from this `ByteStream` into memory
    ///
    /// If an error in the underlying stream is encountered, `ByteStreamError` is returned.
    ///
    /// Data is read into an `AggregatedBytes` that stores data non-contiguously as it was received
    /// over the network. If a contiguous slice is required, use `into_bytes()`.
    /// ```no_run
    /// use bytes::Bytes;
    /// use aws_smithy_types::body;
    /// use aws_smithy_types::body::SdkBody;
    /// use aws_smithy_types::byte_stream::{ByteStream, error::Error};
    /// async fn get_data() {
    ///     let stream = ByteStream::new(SdkBody::from("hello!"));
    ///     let data: Result<Bytes, Error> = stream.collect().await.map(|data| data.into_bytes());
    /// }
    /// ```
    pub async fn collect(self) -> Result<AggregatedBytes, Error> {
        self.inner.collect().await.map_err(Error::streaming)
    }

    /// Returns a [`FsBuilder`], allowing you to build a `ByteStream` with
    /// full control over how the file is read (eg. specifying the length of
    /// the file or the size of the buffer used to read the file).
    ///
    /// ```no_run
    /// # #[cfg(feature = "rt-tokio")]
    /// # {
    /// use aws_smithy_types::byte_stream::{ByteStream, Length};
    ///
    /// async fn bytestream_from_file() -> ByteStream {
    ///     let bytestream = ByteStream::read_from()
    ///         .path("docs/some-large-file.csv")
    ///         // Specify the size of the buffer used to read the file (in bytes, default is 4096)
    ///         .buffer_size(32_784)
    ///         // Specify the length of the file used (skips an additional call to retrieve the size)
    ///         .length(Length::Exact(123_456))
    ///         .build()
    ///         .await
    ///         .expect("valid path");
    ///     bytestream
    /// }
    /// # }
    /// ```
    #[cfg(feature = "rt-tokio")]
    pub fn read_from() -> crate::byte_stream::FsBuilder {
        crate::byte_stream::FsBuilder::new()
    }

    /// Create a ByteStream that streams data from the filesystem
    ///
    /// This function creates a retryable ByteStream for a given `path`. The returned ByteStream
    /// will provide a size hint when used as an HTTP body. If the request fails, the read will
    /// begin again by reloading the file handle.
    ///
    /// ## Warning
    /// The contents of the file MUST not change during retries. The length & checksum of the file
    /// will be cached. If the contents of the file change, the operation will almost certainly fail.
    ///
    /// Furthermore, a partial write MAY seek in the file and resume from the previous location.
    ///
    /// Note: If you want more control, such as specifying the size of the buffer used to read the file
    /// or the length of the file, use a [`FsBuilder`] as returned from `ByteStream::read_from`.
    ///
    /// # Examples
    /// ```no_run
    /// use aws_smithy_types::byte_stream::ByteStream;
    /// use std::path::Path;
    ///  async fn make_bytestream() -> ByteStream {
    ///     ByteStream::from_path("docs/rows.csv").await.expect("file should be readable")
    /// }
    /// ```
    #[cfg(feature = "rt-tokio")]
    pub async fn from_path(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, crate::byte_stream::error::Error> {
        crate::byte_stream::FsBuilder::new()
            .path(path)
            .build()
            .await
    }

    #[cfg(feature = "rt-tokio")]
    /// Convert this `ByteStream` into a struct that implements [`AsyncBufRead`](tokio::io::AsyncBufRead).
    ///
    /// # Example
    ///
    /// ```rust
    /// use tokio::io::AsyncBufReadExt;
    /// use aws_smithy_types::byte_stream::ByteStream;
    ///
    /// # async fn dox(my_bytestream: ByteStream) -> std::io::Result<()> {
    /// let mut lines =  my_bytestream.into_async_read().lines();
    /// while let Some(line) = lines.next_line().await? {
    ///   // Do something line by line
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn into_async_read(self) -> impl tokio::io::AsyncBufRead {
        // The `Stream` trait is currently unstable so we can only use it in private.
        // Here, we create a local struct just to enable the trait for `ByteStream` and pass it
        // to `StreamReader`.
        struct FuturesStreamCompatByteStream(ByteStream);
        impl futures_core::stream::Stream for FuturesStreamCompatByteStream {
            type Item = Result<Bytes, Error>;
            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                Pin::new(&mut self.0.inner)
                    .poll_next(cx)
                    .map_err(Error::streaming)
            }
        }
        tokio_util::io::StreamReader::new(FuturesStreamCompatByteStream(self))
    }

    /// Given a function to modify an [`SdkBody`], run it on the `SdkBody` inside this `Bytestream`.
    /// returning a new `Bytestream`.
    pub fn map(self, f: impl Fn(SdkBody) -> SdkBody + Send + Sync + 'static) -> ByteStream {
        ByteStream::new(self.into_inner().map(f))
    }
}

impl Default for ByteStream {
    fn default() -> Self {
        Self {
            inner: Inner {
                body: SdkBody::from(""),
            },
        }
    }
}

impl From<SdkBody> for ByteStream {
    fn from(inp: SdkBody) -> Self {
        ByteStream::new(inp)
    }
}

/// Construct a retryable ByteStream from [`bytes::Bytes`].
impl From<Bytes> for ByteStream {
    fn from(input: Bytes) -> Self {
        ByteStream::new(SdkBody::from(input))
    }
}

/// Construct a retryable ByteStream from a `Vec<u8>`.
///
/// This will convert the `Vec<u8>` into [`bytes::Bytes`] to enable efficient retries.
impl From<Vec<u8>> for ByteStream {
    fn from(input: Vec<u8>) -> Self {
        Self::from(Bytes::from(input))
    }
}

/// Non-contiguous Binary Data Storage
///
/// When data is read from the network, it is read in a sequence of chunks that are
/// not in contiguous memory. [`AggregatedBytes`] provides a view of this data via
/// [`impl Buf`](bytes::Buf) or it can be copied into contiguous storage with
/// [`.into_bytes()`](crate::byte_stream::AggregatedBytes::into_bytes).
#[derive(Debug, Clone)]
pub struct AggregatedBytes(SegmentedBuf<Bytes>);

impl AggregatedBytes {
    /// Convert this buffer into [`Bytes`].
    ///
    /// # Why does this consume `self`?
    /// Technically, [`copy_to_bytes`](bytes::Buf::copy_to_bytes) can be called without ownership of self. However, since this
    /// mutates the underlying buffer such that no data is remaining, it is more misuse resistant to
    /// prevent the caller from attempting to reread the buffer.
    ///
    /// If the caller only holds a mutable reference, they may use [`copy_to_bytes`](bytes::Buf::copy_to_bytes)
    /// directly on `AggregatedBytes`.
    pub fn into_bytes(mut self) -> Bytes {
        self.0.copy_to_bytes(self.0.remaining())
    }

    /// Convert this buffer into an [`Iterator`] of underlying non-contiguous segments of [`Bytes`]
    pub fn into_segments(self) -> impl Iterator<Item = Bytes> {
        self.0.into_inner().into_iter()
    }

    /// Convert this buffer into a `Vec<u8>`
    pub fn to_vec(self) -> Vec<u8> {
        self.0.into_inner().into_iter().flatten().collect()
    }
}

impl Buf for AggregatedBytes {
    // Forward all methods that SegmentedBuf has custom implementations of.
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn chunk(&self) -> &[u8] {
        self.0.chunk()
    }

    fn chunks_vectored<'a>(&'a self, dst: &mut [IoSlice<'a>]) -> usize {
        self.0.chunks_vectored(dst)
    }

    fn advance(&mut self, cnt: usize) {
        self.0.advance(cnt)
    }

    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        self.0.copy_to_bytes(len)
    }
}

pin_project! {
    #[derive(Debug)]
    struct Inner {
        #[pin]
        body: SdkBody,
    }
}

impl Inner {
    fn new(body: SdkBody) -> Self {
        Self { body }
    }

    async fn next(&mut self) -> Option<Result<Bytes, crate::body::Error>> {
        let mut me = Pin::new(self);
        poll_fn(|cx| me.as_mut().poll_next(cx)).await
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, crate::body::Error>>> {
        self.project().body.poll_next(cx)
    }

    async fn collect(self) -> Result<AggregatedBytes, crate::body::Error> {
        let mut output = SegmentedBuf::new();
        let body = self.body;
        pin_utils::pin_mut!(body);
        while let Some(buf) = body.next().await {
            output.push(buf?);
        }
        Ok(AggregatedBytes(output))
    }

    fn size_hint(&self) -> (u64, Option<u64>) {
        self.body.bounds_on_remaining_length()
    }
}

#[cfg(all(test, feature = "rt-tokio"))]
mod tests {
    use super::{ByteStream, Inner};
    use crate::body::SdkBody;
    use bytes::Bytes;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn read_from_string_body() {
        let body = SdkBody::from("a simple body");
        assert_eq!(
            Inner::new(body)
                .collect()
                .await
                .expect("no errors")
                .into_bytes(),
            Bytes::from("a simple body")
        );
    }

    #[tokio::test]
    async fn bytestream_into_async_read() {
        use tokio::io::AsyncBufReadExt;

        let byte_stream = ByteStream::from_static(b"data 1\ndata 2\ndata 3");
        let async_buf_read = tokio::io::BufReader::new(byte_stream.into_async_read());

        let mut lines = async_buf_read.lines();

        assert_eq!(lines.next_line().await.unwrap(), Some("data 1".to_owned()));
        assert_eq!(lines.next_line().await.unwrap(), Some("data 2".to_owned()));
        assert_eq!(lines.next_line().await.unwrap(), Some("data 3".to_owned()));
        assert_eq!(lines.next_line().await.unwrap(), None);
    }

    #[tokio::test]
    async fn valid_size_hint() {
        assert_eq!(ByteStream::from_static(b"hello").size_hint().1, Some(5));
        assert_eq!(ByteStream::from_static(b"").size_hint().1, Some(0));

        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"hello").unwrap();
        let body = ByteStream::from_path(f.path()).await.unwrap();
        assert_eq!(body.inner.size_hint().1, Some(5));

        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"").unwrap();
        let body = ByteStream::from_path(f.path()).await.unwrap();
        assert_eq!(body.inner.size_hint().1, Some(0));
    }

    #[allow(clippy::bool_assert_comparison)]
    #[tokio::test]
    async fn valid_eos() {
        assert_eq!(
            ByteStream::from_static(b"hello").inner.body.is_end_stream(),
            false
        );
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"hello").unwrap();
        let body = ByteStream::from_path(f.path()).await.unwrap();
        assert_eq!(body.inner.body.content_length(), Some(5));
        assert!(!body.inner.body.is_end_stream());

        assert_eq!(
            ByteStream::from_static(b"").inner.body.is_end_stream(),
            true
        );
        let mut f = NamedTempFile::new().unwrap();
        f.write_all(b"").unwrap();
        let body = ByteStream::from_path(f.path()).await.unwrap();
        assert_eq!(body.inner.body.content_length(), Some(0));
        assert!(body.inner.body.is_end_stream());
    }
}
