#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio_test::assert_ok;

use bytes::BytesMut;
use std::cmp;
use std::io::{self, Cursor};
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::test]
async fn write_all() {
    struct Wr {
        buf: BytesMut,
        cnt: usize,
    }

    impl AsyncWrite for Wr {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            assert_eq!(self.cnt, 0);

            let n = cmp::min(4, buf.len());
            let buf = &buf[0..n];

            self.cnt += 1;
            self.buf.extend(buf);
            Ok(buf.len()).into()
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Ok(()).into()
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Ok(()).into()
        }
    }

    let mut wr = Wr {
        buf: BytesMut::with_capacity(64),
        cnt: 0,
    };

    let mut buf = Cursor::new(&b"hello world"[..]);

    assert_ok!(wr.write_buf(&mut buf).await);
    assert_eq!(wr.buf, b"hell"[..]);
    assert_eq!(wr.cnt, 1);
    assert_eq!(buf.position(), 4);
}

#[tokio::test]
async fn write_buf_vectored() {
    struct Wr {
        buf: BytesMut,
        cnt: usize,
    }

    impl AsyncWrite for Wr {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            panic!("shouldn't be called")
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Ok(()).into()
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            Ok(()).into()
        }

        fn poll_write_vectored(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            bufs: &[io::IoSlice<'_>],
        ) -> Poll<Result<usize, io::Error>> {
            let mut n = 0;
            for buf in bufs {
                self.buf.extend_from_slice(buf);
                n += buf.len();
            }
            self.cnt += 1;
            Ok(n).into()
        }

        fn is_write_vectored(&self) -> bool {
            true
        }
    }

    let mut wr = Wr {
        buf: BytesMut::with_capacity(64),
        cnt: 0,
    };

    let mut buf = Cursor::new(&b"hello world"[..]);

    assert_ok!(wr.write_buf(&mut buf).await);
    assert_eq!(wr.buf, b"hello world"[..]);
    assert_eq!(wr.cnt, 1);
    assert_eq!(buf.position(), 11);
}
