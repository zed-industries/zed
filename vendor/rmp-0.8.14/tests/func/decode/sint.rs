use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_nfix_min() {
    let buf = [0xe0];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-32, read_nfix(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_nfix_max() {
    let buf = [0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-1, read_nfix(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_nfix_type_mismatch() {
    let buf = &[0xc0];
    let mut cur = Cursor::new(&buf[..]);

    match read_nfix(&mut cur) {
        Err(ValueReadError::TypeMismatch(..)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_i8_min() {
    let buf = [0xd0, 0x80];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-128, read_i8(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_i8_max() {
    let buf = [0xd0, 0x7f];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(127, read_i8(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_i8_type_mismatch() {
    let buf = [0xc0, 0x80];
    let mut cur = Cursor::new(&buf[..]);

    match read_i8(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_i8_unexpected_eof() {
    let buf = [0xd0];
    let mut cur = Cursor::new(&buf[..]);

    read_i8(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_i16_min() {
    let buf = [0xd1, 0x80, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-32768, read_i16(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_i16_max() {
    let buf = [0xd1, 0x7f, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(32767, read_i16(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_i16_type_mismatch() {
    let buf = [0xc0, 0x80, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    match read_i16(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_i16_unexpected_eof() {
    let buf = [0xd1, 0x7f];
    let mut cur = Cursor::new(&buf[..]);

    read_i16(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_i32_min() {
    let buf = [0xd2, 0x80, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-2_147_483_648, read_i32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_i32_max() {
    let buf = &[0xd2, 0x7f, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(2147483647, read_i32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_i32_type_mismatch() {
    let buf = &[0xc0, 0x80, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    match read_i32(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_i32_unexpected_eof() {
    let buf = &[0xd2, 0x7f, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    read_i32(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_i64_min() {
    let buf = [0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(-9223372036854775808, read_i64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_i64_max() {
    let buf = [0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(9223372036854775807, read_i64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_i64_type_mismatch() {
    let buf = [0xc0, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    match read_i64(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_i64_unexpected_eof() {
    let buf = [0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    read_i64(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_nfix_min_read_int() {
    let buf: &[u8] = &[0xe0];
    let mut cur = Cursor::new(buf);

    assert_eq!(-32, read_int(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_nfix_max_read_int() {
    let buf: &[u8] = &[0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(-1, read_int(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_i8_min_read_int() {
    let buf: &[u8] = &[0xd0, 0x80];
    let mut cur = Cursor::new(buf);

    assert_eq!(-128, read_int(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_i8_max_read_int() {
    let buf: &[u8] = &[0xd0, 0x7f];
    let mut cur = Cursor::new(buf);

    assert_eq!(127, read_int(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_i16_min_read_int() {
    let buf: &[u8] = &[0xd1, 0x80, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(-32768, read_int(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_i16_max_read_int() {
    let buf: &[u8] = &[0xd1, 0x7f, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(32767, read_int(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_i32_min_read_int() {
    let buf: &[u8] = &[0xd2, 0x80, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(-2147483648, read_int(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_i32_max_read_int() {
    let buf: &[u8] = &[0xd2, 0x7f, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(2147483647, read_int(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_i64_min_read_int() {
    let buf: &[u8] = &[0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(-9223372036854775808i64, read_int(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_i64_max_read_int() {
    let buf: &[u8] = &[0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(9223372036854775807i64, read_int(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}
