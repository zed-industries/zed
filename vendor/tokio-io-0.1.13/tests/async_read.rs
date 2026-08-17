extern crate bytes;
extern crate futures;
extern crate tokio_io;

use bytes::{BufMut, BytesMut};
use futures::Async;
use tokio_io::AsyncRead;

use std::io::{self, Read};

#[test]
fn read_buf_success() {
    struct R;

    impl Read for R {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf[0..11].copy_from_slice(b"hello world");
            Ok(11)
        }
    }

    impl AsyncRead for R {}

    let mut buf = BytesMut::with_capacity(65);

    let n = match R.read_buf(&mut buf).unwrap() {
        Async::Ready(n) => n,
        _ => panic!(),
    };

    assert_eq!(11, n);
    assert_eq!(buf[..], b"hello world"[..]);
}

#[test]
fn read_buf_error() {
    struct R;

    impl Read for R {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "other"))
        }
    }

    impl AsyncRead for R {}

    let mut buf = BytesMut::with_capacity(65);

    let err = R.read_buf(&mut buf).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::Other);
}

#[test]
fn read_buf_no_capacity() {
    struct R;

    impl Read for R {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            unimplemented!();
        }
    }

    impl AsyncRead for R {}

    // Can't create BytesMut w/ zero capacity, so fill it up
    let mut buf = BytesMut::with_capacity(64);
    buf.put(&[0; 64][..]);

    let n = match R.read_buf(&mut buf).unwrap() {
        Async::Ready(n) => n,
        _ => panic!(),
    };

    assert_eq!(0, n);
}

#[test]
fn read_buf_no_uninitialized() {
    struct R;

    impl Read for R {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for b in buf {
                assert_eq!(0, *b);
            }

            Ok(0)
        }
    }

    impl AsyncRead for R {}

    // Can't create BytesMut w/ zero capacity, so fill it up
    let mut buf = BytesMut::with_capacity(64);

    let n = match R.read_buf(&mut buf).unwrap() {
        Async::Ready(n) => n,
        _ => panic!(),
    };

    assert_eq!(0, n);
}

#[test]
fn read_buf_uninitialized_ok() {
    struct R;

    impl Read for R {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            assert_eq!(buf[0..11], b"hello world"[..]);
            Ok(0)
        }
    }

    impl AsyncRead for R {
        unsafe fn prepare_uninitialized_buffer(&self, _: &mut [u8]) -> bool {
            false
        }
    }

    // Can't create BytesMut w/ zero capacity, so fill it up
    let mut buf = BytesMut::with_capacity(64);
    unsafe {
        buf.bytes_mut()[0..11].copy_from_slice(b"hello world");
    }

    let n = match R.read_buf(&mut buf).unwrap() {
        Async::Ready(n) => n,
        _ => panic!(),
    };

    assert_eq!(0, n);
}

#[test]
fn read_buf_translate_wouldblock_to_not_ready() {
    struct R;

    impl Read for R {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::WouldBlock, ""))
        }
    }

    impl AsyncRead for R {}

    let mut buf = BytesMut::with_capacity(65);
    assert!(!R.read_buf(&mut buf).unwrap().is_ready());
}
