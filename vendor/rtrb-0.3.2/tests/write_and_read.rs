#![cfg(feature = "std")]

use std::io::{Read, Write};

use rtrb::RingBuffer;

#[test]
fn write_and_read() {
    let (mut p, mut c) = RingBuffer::new(2);
    assert_eq!(p.write(&[10, 11]).unwrap(), 2);
    // Does nothing:
    assert!(p.flush().is_ok());

    {
        let mut buf = [0];
        assert_eq!(c.read(&mut buf).unwrap(), 1);
        assert_eq!(buf, [10]);
    }

    assert_eq!(p.write(&[12, 99]).unwrap(), 1);

    {
        let mut buf = [0, 0];
        assert_eq!(c.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [11, 12]);
    }

    assert_eq!(p.write(&[13, 14]).unwrap(), 2);

    {
        let mut buf = [0];
        assert_eq!(c.read(&mut buf).unwrap(), 1);
        assert_eq!(buf, [13]);
    }

    {
        let mut buf = [20, 21];
        assert_eq!(c.read(&mut buf).unwrap(), 1);
        assert_eq!(buf, [14, 21]);
    }
}

#[test]
fn write_empty_buf() {
    let (mut p, _c) = RingBuffer::new(2);
    assert_eq!(p.write(&[]).unwrap(), 0);
}

#[test]
fn read_empty_buf() {
    let (mut p, mut c) = RingBuffer::new(2);
    assert_eq!(p.push(99), Ok(()));
    assert_eq!(c.read(&mut []).unwrap(), 0);
}

#[test]
fn write_error() {
    let (mut p, _c) = RingBuffer::new(1);
    assert_eq!(p.push(10), Ok(()));
    assert_eq!(
        p.write(&[99]).unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}

#[test]
fn read_error() {
    let (_p, mut c) = RingBuffer::new(1);
    let mut buf = [0];
    assert_eq!(
        c.read(&mut buf).unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
