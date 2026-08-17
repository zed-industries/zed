//! Creates an asynchronous piped reader and writer pair using `tokio.rs` and `futures`.
//!
//! # Examples
//!
//! ```
//! # async fn run() {
//! use async_pipe;
//! use tokio::io::{AsyncWriteExt, AsyncReadExt};
//!
//! let (mut w, mut r) = async_pipe::pipe();
//!  
//! tokio::spawn(async move {
//!     w.write_all(b"hello world").await.unwrap();
//! });
//!  
//! let mut v = Vec::new();
//! r.read_to_end(&mut v).await.unwrap();
//!
//! println!("Received: {:?}", String::from_utf8(v));
//! # }
//!
//! tokio::runtime::Runtime::new().unwrap().block_on(run());
//! ```
//!
//! # Featues
//!
//! * `futures` (default) Implement `AsyncWrite` and `AsyncRead` from `futures::io`
//! * `tokio` Implement `AsyncWrite` and `AsyncRead` from `tokio::io`.

use state::State;
use std::sync::{Arc, Mutex};

pub use self::reader::PipeReader;
pub use self::writer::PipeWriter;

mod reader;
mod state;
mod writer;

/// Creates a piped pair of an [`AsyncWrite`](https://docs.rs/tokio/1.9.0/tokio/io/trait.AsyncWrite.html) and an [`AsyncRead`](https://docs.rs/tokio/1.9.0/tokio/io/trait.AsyncRead.html).
pub fn pipe() -> (PipeWriter, PipeReader) {
    let shared_state = Arc::new(Mutex::new(State {
        reader_waker: None,
        writer_waker: None,
        closed: false,
        buffer: Vec::new(),
    }));

    let w = PipeWriter {
        state: shared_state.clone(),
    };

    let r = PipeReader {
        state: shared_state,
    };

    (w, r)
}

#[cfg(test)]
mod test {
    #[cfg(feature = "tokio")]
    mod test_tokio {
        use crate::*;
        use std::io;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        #[tokio::test]
        async fn read_write() {
            let (mut writer, mut reader) = pipe();
            let data = b"hello world";

            let write_handle = tokio::spawn(async move {
                writer.write_all(data).await.unwrap();
            });

            let mut read_buf = Vec::new();
            reader.read_to_end(&mut read_buf).await.unwrap();
            write_handle.await.unwrap();

            assert_eq!(&read_buf, data);
        }

        #[tokio::test]
        async fn eof_when_writer_is_shutdown() {
            let (mut writer, mut reader) = pipe();
            writer.shutdown().await.unwrap();
            let mut buf = [0u8; 8];
            let bytes_read = reader.read(&mut buf).await.unwrap();
            assert_eq!(bytes_read, 0);
        }

        #[tokio::test]
        async fn broken_pipe_when_reader_is_dropped() {
            let (mut writer, reader) = pipe();
            drop(reader);
            let io_error = writer.write_all(&[0u8; 8]).await.unwrap_err();
            assert_eq!(io_error.kind(), io::ErrorKind::BrokenPipe);
        }

        #[tokio::test]
        async fn eof_when_writer_is_dropped() {
            let (writer, mut reader) = pipe();
            drop(writer);
            let mut buf = [0u8; 8];
            let bytes_read = reader.read(&mut buf).await.unwrap();
            assert_eq!(bytes_read, 0);
        }

        #[tokio::test]
        async fn drop_read_exact() {
            let (mut writer, mut reader) = pipe();
            const BUF_SIZE: usize = 8;

            let write_handle = tokio::spawn(async move {
                writer.write_all(&[0u8; BUF_SIZE]).await.unwrap();
            });

            let mut buf = [0u8; BUF_SIZE];
            reader.read_exact(&mut buf).await.unwrap();
            drop(reader);
            write_handle.await.unwrap();
        }
    }
}
