/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::{PathBody, State, DEFAULT_OFFSET};
use http_body_1_0::{Body, Frame, SizeHint};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use tokio_util::io::ReaderStream;

impl Body for PathBody {
    type Data = bytes::Bytes;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let offset = self.offset.unwrap_or(DEFAULT_OFFSET);
        loop {
            match self.state {
                State::Unloaded(ref path_buf) => {
                    let buf = path_buf.clone();
                    self.state = State::Loading(Box::pin(async move {
                        let mut file = File::open(&buf).await?;

                        if offset != 0 {
                            let _s = file.seek(SeekFrom::Start(offset)).await?;
                        }

                        Ok(file)
                    }));
                }
                State::Loading(ref mut future) => {
                    match futures_core::ready!(Pin::new(future).poll(cx)) {
                        Ok(file) => {
                            self.state = State::Loaded {
                                stream: ReaderStream::with_capacity(
                                    file.take(self.length),
                                    self.buffer_size,
                                ),
                                bytes_left: self.length,
                            };
                        }
                        Err(e) => return Poll::Ready(Some(Err(e.into()))),
                    };
                }
                State::Loaded {
                    ref mut stream,
                    ref mut bytes_left,
                } => {
                    use futures_core::Stream;
                    return match futures_core::ready!(Pin::new(stream).poll_next(cx)) {
                        Some(Ok(bytes)) => {
                            *bytes_left -= bytes.len() as u64;
                            Poll::Ready(Some(Ok(Frame::data(bytes))))
                        }
                        None => Poll::Ready(None),
                        Some(Err(e)) => Poll::Ready(Some(Err(e.into()))),
                    };
                }
            };
        }
    }

    fn is_end_stream(&self) -> bool {
        match self.state {
            State::Unloaded(_) | State::Loading(_) => self.length == 0,
            State::Loaded { bytes_left, .. } => bytes_left == 0,
        }
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.length)
    }
}

#[cfg(test)]
mod test {
    use crate::byte_stream::{ByteStream, FsBuilder, Length};
    use bytes::Buf;
    use http_body_util::BodyExt;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn path_based_bytestreams_with_builder() {
        let mut file = NamedTempFile::new().unwrap();

        for i in 0..10000 {
            writeln!(file, "Brian was here. Briefly. {i}").unwrap();
        }
        let file_length = file
            .as_file()
            .metadata()
            .expect("file metadata is accessible")
            .len();

        let body = FsBuilder::new()
            .path(&file)
            .buffer_size(16384)
            .length(Length::Exact(file_length))
            .build()
            .await
            .unwrap()
            .into_inner();

        // assert that the specified length is used as size hint
        assert_eq!(body.content_length(), Some(file_length));

        let mut body = body.try_clone().expect("retryable bodies are cloneable");
        // read a little bit from one of the clones
        let some_data = body
            .next()
            .await
            .expect("should have some data")
            .expect("read should not fail");
        // The size of one read should be equal to that of the buffer size
        assert_eq!(some_data.len(), 16384);

        assert_eq!(
            ByteStream::new(body).collect().await.unwrap().remaining() as u64,
            file_length - some_data.len() as u64
        );
    }

    #[tokio::test]
    async fn fsbuilder_length_is_used_as_size_hint() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            "A very long sentence that's clearly longer than a single byte."
        )
        .unwrap();
        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            // The file is longer than 1 byte, let's see if this is used to generate the size hint
            .length(Length::Exact(1))
            .build()
            .await
            .unwrap()
            .into_inner();

        assert_eq!(body.content_length(), Some(1));
    }

    #[tokio::test]
    async fn fsbuilder_is_end_stream() {
        let sentence = "A very long sentence that's clearly longer than a single byte.";
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(sentence.as_bytes()).unwrap();
        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            .build()
            .await
            .unwrap()
            .into_inner();

        assert!(!body.is_end_stream());
        assert_eq!(body.content_length(), Some(sentence.len() as u64));
        let data = BodyExt::collect(body).await.unwrap().to_bytes();
        assert_eq!(data, sentence);
    }

    #[tokio::test]
    async fn fsbuilder_respects_length() {
        let mut file = NamedTempFile::new().unwrap();
        let line_0 = "Line 0\n";
        let line_1 = "Line 1\n";

        write!(file, "{line_0}").unwrap();
        write!(file, "{line_1}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            // We're going to read line 0 only
            .length(Length::Exact(line_0.len() as u64))
            .build()
            .await
            .unwrap();

        let data = body.collect().await.unwrap().into_bytes();
        let data_str = String::from_utf8(data.to_vec()).unwrap();

        assert_eq!(&data_str, line_0);
    }

    #[tokio::test]
    async fn fsbuilder_length_exact() {
        let mut file = NamedTempFile::new().unwrap();
        let test_sentence = "This sentence is 30 bytes long";
        assert_eq!(test_sentence.len(), 30);
        write!(file, "{test_sentence}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        assert!(FsBuilder::new()
            .path(&file)
            // The file is 30 bytes so this is fine
            .length(Length::Exact(29))
            .build()
            .await
            .is_ok());

        assert!(FsBuilder::new()
            .path(&file)
            // The file is 30 bytes so this is fine
            .length(Length::Exact(30))
            .build()
            .await
            .is_ok());

        assert!(FsBuilder::new()
            .path(&file)
            // Larger than 30 bytes, this will cause an error
            .length(Length::Exact(31))
            .build()
            .await
            .is_err());
    }

    #[tokio::test]
    async fn fsbuilder_supports_offset() {
        let mut file = NamedTempFile::new().unwrap();
        let line_0 = "Line 0\n";
        let line_1 = "Line 1\n";

        write!(file, "{line_0}").unwrap();
        write!(file, "{line_1}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            // We're going to skip the first line by using offset
            .offset(line_0.len() as u64)
            .build()
            .await
            .unwrap();

        let data = body.collect().await.unwrap().into_bytes();
        let data_str = String::from_utf8(data.to_vec()).unwrap();

        assert_eq!(&data_str, line_1);
    }

    #[tokio::test]
    async fn fsbuilder_offset_and_length_work_together() {
        let mut file = NamedTempFile::new().unwrap();
        let line_0 = "Line 0\n";
        let line_1 = "Line 1\n";
        let line_2 = "Line 2\n";

        write!(file, "{line_0}").unwrap();
        write!(file, "{line_1}").unwrap();
        write!(file, "{line_2}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            // We're going to skip line 0 by using offset
            .offset(line_0.len() as u64)
            // We want to read only line 1 and stop before we get to line 2
            .length(Length::Exact(line_1.len() as u64))
            .build()
            .await
            .unwrap();

        let data = body.collect().await.unwrap().into_bytes();
        let data_str = String::from_utf8(data.to_vec()).unwrap();

        assert_eq!(&data_str, line_1);
    }

    #[tokio::test]
    async fn fsbuilder_with_offset_greater_than_file_length_returns_error() {
        let mut file = NamedTempFile::new().unwrap();
        let line_0 = "Line 0\n";
        let line_1 = "Line 1\n";

        write!(file, "{line_0}").unwrap();
        write!(file, "{line_1}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        assert_eq!(
            FsBuilder::new()
                .path(&file)
                // We're going to skip all file contents by setting an offset
                // much larger than the file size
                .offset(9000)
                .build()
                .await
                .unwrap_err()
                .to_string(),
            "offset must be less than or equal to file size but was greater than"
        );
    }

    #[tokio::test]
    async fn fsbuilder_with_length_greater_than_file_length_reads_everything() {
        let mut file = NamedTempFile::new().unwrap();
        let line_0 = "Line 0\n";
        let line_1 = "Line 1\n";

        write!(file, "{line_0}").unwrap();
        write!(file, "{line_1}").unwrap();

        // Ensure that the file was written to
        file.flush().expect("flushing is OK");

        let body = FsBuilder::new()
            .path(&file)
            .length(Length::UpTo(9000))
            .build()
            .await
            .unwrap();

        let data = body.collect().await.unwrap().into_bytes();
        let data_str = String::from_utf8(data.to_vec()).unwrap();

        assert_eq!(data_str, format!("{line_0}{line_1}"));
    }

    #[tokio::test]
    async fn fsbuilder_can_be_used_for_chunking() {
        let mut file = NamedTempFile::new().unwrap();
        let mut in_memory_copy_of_file_contents = String::new();
        // I put these two write loops in separate blocks so that the traits wouldn't conflict
        {
            use std::io::Write;
            for i in 0..1000 {
                writeln!(file, "Line {i:04}").unwrap();
            }
        }

        {
            use std::fmt::Write;
            for i in 0..1000 {
                writeln!(in_memory_copy_of_file_contents, "Line {i:04}").unwrap();
            }
            // Check we wrote the lines
            assert!(!in_memory_copy_of_file_contents.is_empty());
        }

        let file_size = file.as_file().metadata().unwrap().len();
        // Check that our in-memory copy has the same size as the file
        assert_eq!(file_size, in_memory_copy_of_file_contents.len() as u64);
        let file_path = file.path().to_path_buf();
        let chunks = 7;
        let chunk_size = file_size / chunks;

        let mut byte_streams = Vec::new();
        for i in 0..chunks {
            let length = if i == chunks - 1 {
                // If we're on the last chunk, the length to read might be less than a whole chunk.
                // We subtract the size of all previous chunks from the total file size to get the
                // size of the final chunk.
                file_size - (i * chunk_size)
            } else {
                chunk_size
            };

            let byte_stream = FsBuilder::new()
                .path(&file_path)
                .offset(i * chunk_size)
                .length(Length::Exact(length))
                .build()
                .await
                .unwrap();

            byte_streams.push(byte_stream);
        }

        let mut collected_bytes = Vec::new();

        for byte_stream in byte_streams.into_iter() {
            let bytes = byte_stream.collect().await.unwrap().into_bytes();
            collected_bytes.push(bytes);
        }

        let bytes = collected_bytes.concat();
        let data_str = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(data_str, in_memory_copy_of_file_contents);
    }
}
