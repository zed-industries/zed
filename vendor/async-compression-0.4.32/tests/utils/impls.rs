pub mod sync {
    use std::io::Read;

    pub fn to_vec(mut read: impl Read) -> Vec<u8> {
        let mut output = vec![];
        read.read_to_end(&mut output).unwrap();
        output
    }
}

#[cfg(feature = "futures-io")]
pub mod futures {
    pub mod bufread {
        pub use futures::io::AsyncBufRead;

        use crate::utils::{InputStream, TrackEof};
        use futures::stream::{StreamExt as _, TryStreamExt as _};

        pub fn from(input: &InputStream) -> impl AsyncBufRead {
            // By using the stream here we ensure that each chunk will require a separate
            // read/poll_fill_buf call to process to help test reading multiple chunks.
            TrackEof::new(input.stream().map(Ok).into_async_read())
        }
    }

    pub mod read {
        use crate::utils::{block_on, pin_mut};
        use futures::io::{copy_buf, AsyncRead, AsyncReadExt, BufReader, Cursor};

        pub fn to_vec(read: impl AsyncRead) -> Vec<u8> {
            // TODO: https://github.com/rust-lang-nursery/futures-rs/issues/1510
            // All current test cases are < 100kB
            let mut output = Cursor::new(vec![0; 102_400]);
            pin_mut!(read);
            // With more flushing from encoders, 4 appears to be the minimal buffer size that works.
            let len = block_on(copy_buf(BufReader::with_capacity(4, read), &mut output)).unwrap();
            let mut output = output.into_inner();
            output.truncate(len as usize);
            output
        }

        pub fn poll_read(reader: impl AsyncRead, output: &mut [u8]) -> std::io::Result<usize> {
            pin_mut!(reader);
            block_on(reader.read(output))
        }
    }

    pub mod write {
        use crate::utils::{block_on, Pin, TrackClosed};
        use futures::io::{AsyncWrite, AsyncWriteExt as _};
        use futures_test::io::AsyncWriteTestExt as _;

        pub fn to_vec(
            input: &[Vec<u8>],
            create_writer: impl for<'a> FnOnce(
                &'a mut (dyn AsyncWrite + Unpin),
            ) -> Pin<Box<dyn AsyncWrite + 'a>>,
            limit: usize,
        ) -> Vec<u8> {
            let mut output = Vec::new();
            {
                let mut test_writer = TrackClosed::new(
                    (&mut output)
                        .limited_write(limit)
                        .interleave_pending_write(),
                );
                {
                    let mut writer = create_writer(&mut test_writer);
                    for chunk in input {
                        block_on(writer.write_all(chunk)).unwrap();
                        block_on(writer.flush()).unwrap();
                    }
                    block_on(writer.close()).unwrap();
                }
                assert!(test_writer.is_closed());
            }
            output
        }
    }
}

#[cfg(feature = "tokio")]
pub mod tokio {
    pub mod bufread {
        use crate::utils::{InputStream, TrackEof};
        use bytes::Bytes;
        use futures::stream::StreamExt;
        pub use tokio::io::AsyncBufRead;
        use tokio_util::io::StreamReader;

        pub fn from(input: &InputStream) -> impl AsyncBufRead {
            // By using the stream here we ensure that each chunk will require a separate
            // read/poll_fill_buf call to process to help test reading multiple chunks.
            TrackEof::new(StreamReader::new(
                input.stream().map(Bytes::from).map(std::io::Result::Ok),
            ))
        }
    }

    pub mod read {
        use crate::utils::{block_on, pin_mut, tokio_ext::copy_buf};
        use std::io::Cursor;
        use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

        pub fn to_vec(read: impl AsyncRead) -> Vec<u8> {
            let mut output = Cursor::new(vec![0; 102_400]);
            pin_mut!(read);
            // With more flushing from encoders, 4 appears to be the minimal buffer size that works.
            let len = block_on(copy_buf(BufReader::with_capacity(4, read), &mut output)).unwrap();
            let mut output = output.into_inner();
            output.truncate(len as usize);
            output
        }

        pub fn poll_read(reader: impl AsyncRead, output: &mut [u8]) -> std::io::Result<usize> {
            pin_mut!(reader);
            block_on(reader.read(output))
        }
    }

    pub mod write {
        use crate::utils::{
            block_on, tokio_ext::AsyncWriteTestExt as _, track_closed::TrackClosed, Pin,
        };
        use std::io::Cursor;
        use tokio::io::{AsyncWrite, AsyncWriteExt as _};

        pub fn to_vec(
            input: &[Vec<u8>],
            create_writer: impl for<'a> FnOnce(
                &'a mut (dyn AsyncWrite + Unpin),
            ) -> Pin<Box<dyn AsyncWrite + 'a>>,
            limit: usize,
        ) -> Vec<u8> {
            let mut output = Cursor::new(Vec::new());
            {
                let mut test_writer = TrackClosed::new(
                    (&mut output)
                        .limited_write(limit)
                        .interleave_pending_write(),
                );
                {
                    let mut writer = create_writer(&mut test_writer);
                    for chunk in input {
                        block_on(writer.write_all(chunk)).unwrap();
                        block_on(writer.flush()).unwrap();
                    }
                    block_on(writer.shutdown()).unwrap();
                }
                assert!(test_writer.is_closed());
            }
            output.into_inner()
        }
    }
}
