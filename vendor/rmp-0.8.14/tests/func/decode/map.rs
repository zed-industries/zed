use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_fixmap_min_read_size() {
    let buf: &[u8] = &[0x80];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_map_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_fixmap_max_read_size() {
    let buf: &[u8] = &[0x8f];
    let mut cur = Cursor::new(buf);

    assert_eq!(15, read_map_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_map16_min_read_size() {
    let buf: &[u8] = &[0xde, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_map_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_map16_max_read_size() {
    let buf: &[u8] = &[0xde, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(65535, read_map_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_map32_min_read_size() {
    let buf: &[u8] = &[0xdf, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_map_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_null_read_map_len() {
    let buf: &[u8] = &[0xc0, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    match read_map_len(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}
