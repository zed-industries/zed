/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::body::SdkBody;
use crate::byte_stream::{error::Error, error::ErrorKind, ByteStream};
use std::cmp::min;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

// TODO(https://github.com/smithy-lang/smithy-rs/issues/1925)
//     Feature gating this now would break the
//     `cargo check --no-default-features --features rt-tokio` test.
// #[cfg(feature = "http-body-0-4-x")]
mod http_body_0_4_x;

#[cfg(feature = "http-body-1-x")]
mod http_body_1_x;

// 4KB corresponds to the default buffer size used by Tokio's ReaderStream
const DEFAULT_BUFFER_SIZE: usize = 4096;
// By default, read files from their start
const DEFAULT_OFFSET: u64 = 0;

/// An HTTP Body designed to wrap files
///
/// PathBody is a three-phase HTTP body designed to wrap files with three specific features:
/// 1. The underlying file is wrapped with StreamReader to implement HTTP body
/// 2. It can be constructed directly from a path so it's easy to use during retries
/// 3. Provide size hint
struct PathBody {
    state: State,
    // The number of bytes to read
    length: u64,
    buffer_size: usize,
    // The byte-offset to start reading from
    offset: Option<u64>,
}

impl PathBody {
    fn from_path(path_buf: PathBuf, length: u64, buffer_size: usize, offset: Option<u64>) -> Self {
        PathBody {
            state: State::Unloaded(path_buf),
            length,
            buffer_size,
            offset,
        }
    }

    fn from_file(file: File, length: u64, buffer_size: usize) -> Self {
        PathBody {
            state: State::Loaded {
                stream: ReaderStream::with_capacity(file.take(length), buffer_size),
                bytes_left: length,
            },
            length,
            buffer_size,
            // The file used to create this `PathBody` should have already had an offset applied
            offset: None,
        }
    }
}

/// Builder for creating [`ByteStreams`](ByteStream) from a file/path, with full control over advanced options.
///
/// ```no_run
/// # #[cfg(feature = "rt-tokio")]
/// # {
/// use aws_smithy_types::byte_stream::{ByteStream, Length};
/// use std::path::Path;
/// struct GetObjectInput {
///     body: ByteStream
/// }
///
/// async fn bytestream_from_file() -> GetObjectInput {
///     let bytestream = ByteStream::read_from()
///         .path("docs/some-large-file.csv")
///         // Specify the size of the buffer used to read the file (in bytes, default is 4096)
///         .buffer_size(32_784)
///         // Specify the length of the file used (skips an additional call to retrieve the size)
///         .length(Length::UpTo(123_456))
///         .build()
///         .await
///         .expect("valid path");
///     GetObjectInput { body: bytestream }
/// }
/// # }
/// ```
#[allow(missing_debug_implementations)]
pub struct FsBuilder {
    file: Option<File>,
    path: Option<PathBuf>,
    length: Option<Length>,
    buffer_size: usize,
    offset: Option<u64>,
}

impl Default for FsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The length (in bytes) to read. Determines whether or not a short read counts as an error.
#[allow(missing_debug_implementations)]
pub enum Length {
    /// Read this number of bytes exactly. Returns an error if the file is smaller than expected.
    Exact(u64),
    /// Read up to this number of bytes. May read less than the specified amount if the file
    /// is smaller than expected.
    UpTo(u64),
}

impl FsBuilder {
    /// Create a new [`FsBuilder`] (using a default read buffer of 4096 bytes).
    ///
    /// You must then call either [`file`](FsBuilder::file) or [`path`](FsBuilder::path) to specify what to read from.
    pub fn new() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            file: None,
            length: None,
            offset: None,
            path: None,
        }
    }

    /// Sets the path to read from.
    ///
    /// NOTE: The resulting ByteStream (after calling [build](FsBuilder::build)) will be retryable.
    /// The returned ByteStream will provide a size hint when used as an HTTP body.
    /// If the request fails, the read will begin again by reloading the file handle.
    pub fn path(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Sets the file to read from.
    ///
    /// NOTE: The resulting ByteStream (after calling [build](FsBuilder::build)) will not be a retryable ByteStream.
    /// For a ByteStream that can be retried in the case of upstream failures, use [`FsBuilder::path`](FsBuilder::path).
    pub fn file(mut self, file: File) -> Self {
        self.file = Some(file);
        self
    }

    /// Specify the length to read (in bytes).
    ///
    /// By pre-specifying the length, this API skips an additional call to retrieve the size from file-system metadata.
    ///
    /// When used in conjunction with [`offset`](FsBuilder::offset), allows for reading a single "chunk" of a file.
    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }

    /// Specify the size of the buffer used to read the file (in bytes).
    ///
    /// Increasing the read buffer capacity to higher values than the default (4096 bytes) can result in a large reduction
    /// in CPU usage, at the cost of memory increase.
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Specify the offset to start reading from (in bytes)
    ///
    /// When used in conjunction with [`length`](FsBuilder::length), allows for reading a single "chunk" of a file.
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Returns a [`ByteStream`] from this builder.
    pub async fn build(self) -> Result<ByteStream, Error> {
        if self.path.is_some() && self.file.is_some() {
            panic!("The 'file' and 'path' options on an FsBuilder are mutually exclusive but both were set. Please set only one")
        };

        let buffer_size = self.buffer_size;
        let offset = self.offset.unwrap_or(DEFAULT_OFFSET);
        // Checking the file length like this does have a cost, but the benefit is that we can
        // notify users when file/chunk is smaller than expected.
        let file_length = self.get_file_size().await?;
        if offset > file_length {
            return Err(ErrorKind::OffsetLargerThanFileSize.into());
        }

        let remaining_file_length = file_length - offset;
        let length = match self.length {
            Some(Length::Exact(length)) => {
                if length > remaining_file_length {
                    return Err(ErrorKind::LengthLargerThanFileSizeMinusReadOffset.into());
                }
                length
            }
            Some(Length::UpTo(length)) => min(length, remaining_file_length),
            None => remaining_file_length,
        };

        if let Some(path) = self.path {
            let body_loader = move || {
                // If an offset was provided, seeking will be handled in `PathBody::poll_data` each
                // time the file is loaded.
                #[cfg(not(feature = "http-body-1-x"))]
                return SdkBody::from_body_0_4_internal(PathBody::from_path(
                    path.clone(),
                    length,
                    buffer_size,
                    self.offset,
                ));
                #[cfg(feature = "http-body-1-x")]
                return SdkBody::from_body_1_x_internal(PathBody::from_path(
                    path.clone(),
                    length,
                    buffer_size,
                    self.offset,
                ));
            };

            Ok(ByteStream::new(SdkBody::retryable(body_loader)))
        } else if let Some(mut file) = self.file {
            // When starting from a `File`, we need to do our own seeking
            if offset != 0 {
                let _s = file.seek(io::SeekFrom::Start(offset)).await?;
            }

            #[cfg(not(feature = "http-body-1-x"))]
            let body =
                SdkBody::from_body_0_4_internal(PathBody::from_file(file, length, buffer_size));
            #[cfg(feature = "http-body-1-x")]
            let body =
                SdkBody::from_body_1_x_internal(PathBody::from_file(file, length, buffer_size));

            Ok(ByteStream::new(body))
        } else {
            panic!("FsBuilder constructed without a file or a path")
        }
    }

    async fn get_file_size(&self) -> Result<u64, Error> {
        Ok(match self.path.as_ref() {
            Some(path) => tokio::fs::metadata(path).await,
            // If it's not path-based then it's file-based
            None => self.file.as_ref().unwrap().metadata().await,
        }
        .map(|metadata| metadata.len())?)
    }
}

enum State {
    Unloaded(PathBuf),
    Loading(Pin<Box<dyn Future<Output = io::Result<File>> + Send + Sync + 'static>>),
    Loaded {
        stream: ReaderStream<io::Take<File>>,
        bytes_left: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn length_up_to_should_work() {
        const FILE_LEN: usize = 1000;
        // up to less than `FILE_LEN`
        {
            let mut file = NamedTempFile::new().unwrap();
            file.write_all(vec![0; FILE_LEN].as_slice()).unwrap();
            let byte_stream = FsBuilder::new()
                .path(file.path())
                .length(Length::UpTo((FILE_LEN / 2) as u64))
                .build()
                .await
                .unwrap();
            let (lower, upper) = byte_stream.size_hint();
            assert_eq!(lower, upper.unwrap());
            assert_eq!((FILE_LEN / 2) as u64, lower);
        }
        // up to equal to `FILE_LEN`
        {
            let mut file = NamedTempFile::new().unwrap();
            file.write_all(vec![0; FILE_LEN].as_slice()).unwrap();
            let byte_stream = FsBuilder::new()
                .path(file.path())
                .length(Length::UpTo(FILE_LEN as u64))
                .build()
                .await
                .unwrap();
            let (lower, upper) = byte_stream.size_hint();
            assert_eq!(lower, upper.unwrap());
            assert_eq!(FILE_LEN as u64, lower);
        }
        // up to greater than `FILE_LEN`
        {
            let mut file = NamedTempFile::new().unwrap();
            file.write_all(vec![0; FILE_LEN].as_slice()).unwrap();
            let byte_stream = FsBuilder::new()
                .path(file.path())
                .length(Length::UpTo((FILE_LEN * 2) as u64))
                .build()
                .await
                .unwrap();
            let (lower, upper) = byte_stream.size_hint();
            assert_eq!(lower, upper.unwrap());
            assert_eq!(FILE_LEN as u64, lower);
        }
    }
}
