use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_empty_array_read_size() {
    let buf: &[u8] = &[0x90];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_array_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_fixarray_max_read_size() {
    let buf: &[u8] = &[
        0x9f,
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e
    ];
    let mut cur = Cursor::new(buf);

    assert_eq!(15, read_array_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_array16_min_read_size() {
    let buf: &[u8] = &[0xdc, 0x00, 0x10];
    let mut cur = Cursor::new(buf);

    assert_eq!(16, read_array_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_array16_max_read_size() {
    let buf: &[u8] = &[0xdc, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(65535, read_array_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_array16_unexpected_eof_read_size() {
    let buf: &[u8] = &[0xdc, 0xff];
    let mut cur = Cursor::new(buf);

    read_array_len(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_array32_min_read_size() {
    let buf: &[u8] = &[0xdd, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_array_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_array32_max_read_size() {
    let buf: &[u8] = &[0xdd, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(4294967295, read_array_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_array32_unexpected_eof_read_size() {
    let buf: &[u8] = &[0xdd, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    read_array_len(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_null_read_array_len() {
    let buf: &[u8] = &[0xc0];
    let mut cur = Cursor::new(buf);

    match read_array_len(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}
