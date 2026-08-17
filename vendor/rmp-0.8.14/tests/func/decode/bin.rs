use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_bin8_min_read_len() {
    let buf: &[u8] = &[0xc4, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_bin_len(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_bin8_max_read_len() {
    let buf: &[u8] = &[0xc4, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(255, read_bin_len(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_bin8_eof_read_len() {
    let buf: &[u8] = &[0xc4];
    let mut cur = Cursor::new(buf);

    read_bin_len(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_null_read_len() {
    let buf: &[u8] = &[0xc0];
    let mut cur = Cursor::new(buf);

    match read_bin_len(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_bin16_max_read_len() {
    let buf: &[u8] = &[0xc5, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(65535, read_bin_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_bin32_max_read_len() {
    let buf: &[u8] = &[0xc6, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(4294967295, read_bin_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}
