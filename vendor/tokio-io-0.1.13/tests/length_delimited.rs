// This file is testing deprecated code.
#![allow(deprecated)]

extern crate futures;
extern crate tokio_io;

use tokio_io::codec::length_delimited::*;
use tokio_io::{AsyncRead, AsyncWrite};

use futures::Async::*;
use futures::{Poll, Sink, Stream};

use std::collections::VecDeque;
use std::io;

macro_rules! mock {
    ($($x:expr,)*) => {{
        let mut v = VecDeque::new();
        v.extend(vec![$($x),*]);
        Mock { calls: v }
    }};
}

#[test]
fn read_empty_io_yields_nothing() {
    let mut io = FramedRead::new(mock!());

    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_frame_one_packet() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00\x00\x09abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_frame_one_packet_little_endian() {
    let mut io = Builder::new().little_endian().new_read(mock! {
        Ok(b"\x09\x00\x00\x00abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_frame_one_packet_native_endian() {
    let data = if cfg!(target_endian = "big") {
        b"\x00\x00\x00\x09abcdefghi"
    } else {
        b"\x09\x00\x00\x00abcdefghi"
    };
    let mut io = Builder::new().native_endian().new_read(mock! {
        Ok(data[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_multi_frame_one_packet() {
    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(b"\x00\x00\x00\x09abcdefghi");
    data.extend_from_slice(b"\x00\x00\x00\x03123");
    data.extend_from_slice(b"\x00\x00\x00\x0bhello world");

    let mut io = FramedRead::new(mock! {
        Ok(data.into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"123"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"hello world"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_frame_multi_packet() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00"[..].into()),
        Ok(b"\x00\x09abc"[..].into()),
        Ok(b"defghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_multi_frame_multi_packet() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00"[..].into()),
        Ok(b"\x00\x09abc"[..].into()),
        Ok(b"defghi"[..].into()),
        Ok(b"\x00\x00\x00\x0312"[..].into()),
        Ok(b"3\x00\x00\x00\x0bhello world"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"123"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"hello world"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_frame_multi_packet_wait() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00"[..].into()),
        Err(would_block()),
        Ok(b"\x00\x09abc"[..].into()),
        Err(would_block()),
        Ok(b"defghi"[..].into()),
        Err(would_block()),
    });

    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_multi_frame_multi_packet_wait() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00"[..].into()),
        Err(would_block()),
        Ok(b"\x00\x09abc"[..].into()),
        Err(would_block()),
        Ok(b"defghi"[..].into()),
        Err(would_block()),
        Ok(b"\x00\x00\x00\x0312"[..].into()),
        Err(would_block()),
        Ok(b"3\x00\x00\x00\x0bhello world"[..].into()),
        Err(would_block()),
    });

    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), Ready(Some(b"123"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"hello world"[..].into())));
    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_incomplete_head() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00"[..].into()),
    });

    assert!(io.poll().is_err());
}

#[test]
fn read_incomplete_head_multi() {
    let mut io = FramedRead::new(mock! {
        Err(would_block()),
        Ok(b"\x00"[..].into()),
        Err(would_block()),
    });

    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), NotReady);
    assert!(io.poll().is_err());
}

#[test]
fn read_incomplete_payload() {
    let mut io = FramedRead::new(mock! {
        Ok(b"\x00\x00\x00\x09ab"[..].into()),
        Err(would_block()),
        Ok(b"cd"[..].into()),
        Err(would_block()),
    });

    assert_eq!(io.poll().unwrap(), NotReady);
    assert_eq!(io.poll().unwrap(), NotReady);
    assert!(io.poll().is_err());
}

#[test]
fn read_max_frame_len() {
    let mut io = Builder::new().max_frame_length(5).new_read(mock! {
        Ok(b"\x00\x00\x00\x09abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap_err().kind(), io::ErrorKind::InvalidData);
}

#[test]
fn read_update_max_frame_len_at_rest() {
    let mut io = Builder::new().new_read(mock! {
        Ok(b"\x00\x00\x00\x09abcdefghi"[..].into()),
        Ok(b"\x00\x00\x00\x09abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    io.set_max_frame_length(5);
    assert_eq!(io.poll().unwrap_err().kind(), io::ErrorKind::InvalidData);
}

#[test]
fn read_update_max_frame_len_in_flight() {
    let mut io = Builder::new().new_read(mock! {
        Ok(b"\x00\x00\x00\x09abcd"[..].into()),
        Err(would_block()),
        Ok(b"efghi"[..].into()),
        Ok(b"\x00\x00\x00\x09abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), NotReady);
    io.set_max_frame_length(5);
    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap_err().kind(), io::ErrorKind::InvalidData);
}

#[test]
fn read_one_byte_length_field() {
    let mut io = Builder::new().length_field_length(1).new_read(mock! {
        Ok(b"\x09abcdefghi"[..].into()),
    });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_header_offset() {
    let mut io = Builder::new()
        .length_field_length(2)
        .length_field_offset(4)
        .new_read(mock! {
            Ok(b"zzzz\x00\x09abcdefghi"[..].into()),
        });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_multi_frame_one_packet_skip_none_adjusted() {
    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(b"xx\x00\x09abcdefghi");
    data.extend_from_slice(b"yy\x00\x03123");
    data.extend_from_slice(b"zz\x00\x0bhello world");

    let mut io = Builder::new()
        .length_field_length(2)
        .length_field_offset(2)
        .num_skip(0)
        .length_adjustment(4)
        .new_read(mock! {
            Ok(data.into()),
        });

    assert_eq!(
        io.poll().unwrap(),
        Ready(Some(b"xx\x00\x09abcdefghi"[..].into()))
    );
    assert_eq!(io.poll().unwrap(), Ready(Some(b"yy\x00\x03123"[..].into())));
    assert_eq!(
        io.poll().unwrap(),
        Ready(Some(b"zz\x00\x0bhello world"[..].into()))
    );
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn read_single_multi_frame_one_packet_length_includes_head() {
    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(b"\x00\x0babcdefghi");
    data.extend_from_slice(b"\x00\x05123");
    data.extend_from_slice(b"\x00\x0dhello world");

    let mut io = Builder::new()
        .length_field_length(2)
        .length_adjustment(-2)
        .new_read(mock! {
            Ok(data.into()),
        });

    assert_eq!(io.poll().unwrap(), Ready(Some(b"abcdefghi"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"123"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(Some(b"hello world"[..].into())));
    assert_eq!(io.poll().unwrap(), Ready(None));
}

#[test]
fn write_single_frame_length_adjusted() {
    let mut io = Builder::new().length_adjustment(-2).new_write(mock! {
        Ok(b"\x00\x00\x00\x0b"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
    });
    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_nothing_yields_nothing() {
    let mut io: FramedWrite<_, &'static [u8]> = FramedWrite::new(mock!());
    assert!(io.poll_complete().unwrap().is_ready());
}

#[test]
fn write_single_frame_one_packet() {
    let mut io = FramedWrite::new(mock! {
        Ok(b"\x00\x00\x00\x09"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_single_multi_frame_one_packet() {
    let mut io = FramedWrite::new(mock! {
        Ok(b"\x00\x00\x00\x09"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(b"\x00\x00\x00\x03"[..].into()),
        Ok(b"123"[..].into()),
        Ok(b"\x00\x00\x00\x0b"[..].into()),
        Ok(b"hello world"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.start_send("123").unwrap().is_ready());
    assert!(io.start_send("hello world").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_single_multi_frame_multi_packet() {
    let mut io = FramedWrite::new(mock! {
        Ok(b"\x00\x00\x00\x09"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
        Ok(b"\x00\x00\x00\x03"[..].into()),
        Ok(b"123"[..].into()),
        Ok(Flush),
        Ok(b"\x00\x00\x00\x0b"[..].into()),
        Ok(b"hello world"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.start_send("123").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.start_send("hello world").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_single_frame_would_block() {
    let mut io = FramedWrite::new(mock! {
        Err(would_block()),
        Ok(b"\x00\x00"[..].into()),
        Err(would_block()),
        Ok(b"\x00\x09"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(!io.poll_complete().unwrap().is_ready());
    assert!(!io.poll_complete().unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());

    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_single_frame_little_endian() {
    let mut io = Builder::new().little_endian().new_write(mock! {
        Ok(b"\x09\x00\x00\x00"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_single_frame_with_short_length_field() {
    let mut io = Builder::new().length_field_length(1).new_write(mock! {
        Ok(b"\x09"[..].into()),
        Ok(b"abcdefghi"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdefghi").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_max_frame_len() {
    let mut io = Builder::new().max_frame_length(5).new_write(mock! {});

    assert_eq!(
        io.start_send("abcdef").unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_zero() {
    let mut io = Builder::new().new_write(mock! {});

    assert!(io.start_send("abcdef").unwrap().is_ready());
    assert_eq!(
        io.poll_complete().unwrap_err().kind(),
        io::ErrorKind::WriteZero
    );
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_update_max_frame_len_at_rest() {
    let mut io = Builder::new().new_write(mock! {
        Ok(b"\x00\x00\x00\x06"[..].into()),
        Ok(b"abcdef"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdef").unwrap().is_ready());
    assert!(io.poll_complete().unwrap().is_ready());
    io.set_max_frame_length(5);
    assert_eq!(
        io.start_send("abcdef").unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(io.get_ref().calls.is_empty());
}

#[test]
fn write_update_max_frame_len_in_flight() {
    let mut io = Builder::new().new_write(mock! {
        Ok(b"\x00\x00\x00\x06"[..].into()),
        Ok(b"ab"[..].into()),
        Err(would_block()),
        Ok(b"cdef"[..].into()),
        Ok(Flush),
    });

    assert!(io.start_send("abcdef").unwrap().is_ready());
    assert!(!io.poll_complete().unwrap().is_ready());
    io.set_max_frame_length(5);
    assert!(io.poll_complete().unwrap().is_ready());
    assert_eq!(
        io.start_send("abcdef").unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(io.get_ref().calls.is_empty());
}

// ===== Test utils =====

fn would_block() -> io::Error {
    io::Error::new(io::ErrorKind::WouldBlock, "would block")
}

struct Mock {
    calls: VecDeque<io::Result<Op>>,
}

enum Op {
    Data(Vec<u8>),
    Flush,
}

use self::Op::*;

impl io::Read for Mock {
    fn read(&mut self, dst: &mut [u8]) -> io::Result<usize> {
        match self.calls.pop_front() {
            Some(Ok(Op::Data(data))) => {
                debug_assert!(dst.len() >= data.len());
                dst[..data.len()].copy_from_slice(&data[..]);
                Ok(data.len())
            }
            Some(Ok(_)) => panic!(),
            Some(Err(e)) => Err(e),
            None => Ok(0),
        }
    }
}

impl AsyncRead for Mock {}

impl io::Write for Mock {
    fn write(&mut self, src: &[u8]) -> io::Result<usize> {
        match self.calls.pop_front() {
            Some(Ok(Op::Data(data))) => {
                let len = data.len();
                assert!(src.len() >= len, "expect={:?}; actual={:?}", data, src);
                assert_eq!(&data[..], &src[..len]);
                Ok(len)
            }
            Some(Ok(_)) => panic!(),
            Some(Err(e)) => Err(e),
            None => Ok(0),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.calls.pop_front() {
            Some(Ok(Op::Flush)) => Ok(()),
            Some(Ok(_)) => panic!(),
            Some(Err(e)) => Err(e),
            None => Ok(()),
        }
    }
}

impl AsyncWrite for Mock {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        Ok(Ready(()))
    }
}

impl<'a> From<&'a [u8]> for Op {
    fn from(src: &'a [u8]) -> Op {
        Op::Data(src.into())
    }
}

impl From<Vec<u8>> for Op {
    fn from(src: Vec<u8>) -> Op {
        Op::Data(src)
    }
}
