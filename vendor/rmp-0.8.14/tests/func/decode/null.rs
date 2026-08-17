use super::Cursor;

use rmp::decode::*;

#[test]
fn pass() {
    let buf = [0xc0];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!((), read_nil(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn fail_invalid_marker() {
    let buf = [0xc1];
    let mut cur = Cursor::new(&buf[..]);

    match read_nil(&mut cur) {
        Err(ValueReadError::TypeMismatch(..)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn fail_unexpected_eof() {
    let buf = [];
    let mut cur = Cursor::new(&buf[..]);

    read_nil(&mut cur).err().unwrap();
    assert_eq!(0, cur.position());
}

#[test]
#[cfg(feature = "std")]
fn interrupt_safe() {
    use std::io::{Error, ErrorKind, Read};

    struct MockRead { state_: u8 }

    impl MockRead {
        fn state(&self) -> u8 { self.state_ }
    }

    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
            if self.state_ == 0 {
                self.state_ = 1;
                Err(Error::new(ErrorKind::Interrupted, "interrupted"))
            } else {
                assert!(!buf.is_empty());

                buf[0] = 0xc0;
                Ok(1)
            }
        }
    }

    let mut cur = MockRead { state_: 0 };

    // The function is interruption-safe, the first read should succeed.
    read_nil(&mut cur).unwrap();

    assert_eq!(1, cur.state());
}
