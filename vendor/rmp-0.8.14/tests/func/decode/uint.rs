use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_positive_fixnum() {
    let buf = [0x00, 0x7f, 0x20];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(0u8, read_pfix(&mut cur).unwrap());
    assert_eq!(1, cur.position());

    assert_eq!(127u8, read_pfix(&mut cur).unwrap());
    assert_eq!(2, cur.position());

    assert_eq!(32u8, read_pfix(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_u8_min() {
    let buf = [0xcc, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(0, read_u8(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_u8_max() {
    let buf = [0xcc, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(255, read_u8(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_u8_type_mismatch() {
    let buf = [0xc0, 0x80];
    let mut cur = Cursor::new(&buf[..]);

    match read_u8(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_u8_unexpected_eof() {
    let buf = [0xcc];
    let mut cur = Cursor::new(&buf[..]);

    read_u8(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_u16_min() {
    let buf = [0xcd, 0x00, 0x00];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(0, read_u16(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_u32_max() {
    let buf = [0xce, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(4294967295, read_u32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_unsigned_fixnum_read_int() {
    let buf = [0x00, 0x7f, 0x20];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(0u64, read_int(&mut cur).unwrap());
    assert_eq!(1, cur.position());

    assert_eq!(127u64, read_int(&mut cur).unwrap());
    assert_eq!(2, cur.position());

    assert_eq!(32u64, read_int(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_unsigned_u8_read_int() {
    let buf = [0xcc, 0x80, 0xcc, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(128u64, read_int(&mut cur).unwrap());
    assert_eq!(2, cur.position());

    assert_eq!(255u64, read_int(&mut cur).unwrap());
    assert_eq!(4, cur.position());
}

#[test]
fn from_unsigned_u8_incomplete_read_int() {
    let buf = [0xcc];
    let mut cur = Cursor::new(&buf[..]);

    read_int::<u64, _>(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_unsigned_u16_read_int() {
    let buf = [0xcd, 0x01, 0x00, 0xcd, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(256u64, read_int(&mut cur).unwrap());
    assert_eq!(3, cur.position());

    assert_eq!(65535u64, read_int(&mut cur).unwrap());
    assert_eq!(6, cur.position());
}

#[test]
fn from_unsigned_u16_incomplete_read_int() {
    let buf = [0xcd];
    let mut cur = Cursor::new(&buf[..]);

    read_int::<u64, _>(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_unsigned_u32_read_int() {
    let buf = [0xce, 0x00, 0x01, 0x00, 0x00, 0xce, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(65536u64, read_int(&mut cur).unwrap());
    assert_eq!(5, cur.position());

    assert_eq!(4294967295u64, read_int(&mut cur).unwrap());
    assert_eq!(10, cur.position());
}

#[test]
fn from_unsigned_u32_incomplete_read_int() {
    let buf = [0xce];
    let mut cur = Cursor::new(&buf[..]);

    read_int::<u64, _>(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_unsigned_u64_read_int() {
    let buf = [
        0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
        0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
    ];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!(4294967296u64, read_int(&mut cur).unwrap());
    assert_eq!(9, cur.position());

    assert_eq!(18446744073709551615u64, read_int(&mut cur).unwrap());
    assert_eq!(18, cur.position());
}

#[test]
fn from_unsigned_u64_incomplete_read_int() {
    let buf = [0xcf];
    let mut cur = Cursor::new(&buf[..]);

    read_int::<u64, _>(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_unsigned_invalid_marker_read_int() {
    let buf = [0xc0];
    let mut cur = Cursor::new(&buf[..]);

    match read_int::<u64, _>(&mut cur) {
        Err(NumValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}")
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_unsigned_invalid_unknown_marker_read_int() {
    let buf = [0xc1];
    let mut cur = Cursor::new(&buf[..]);

    match read_int::<u64, _>(&mut cur) {
        Err(NumValueReadError::TypeMismatch(Marker::Reserved)) => (),
        other => panic!("unexpected result: {other:?}")
    }
    assert_eq!(1, cur.position());
}
