use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_f32_zero_plus() {
    let buf: &[u8] = &[0xca, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0.0, read_f32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_f32_max() {
    let buf: &[u8] = &[0xca, 0x7f, 0x7f, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(3.4028234e38_f32, read_f32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_f32_inf() {
    use std::f32;

    let buf: &[u8] = &[0xca, 0x7f, 0x80, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(f32::INFINITY, read_f32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_f32_neg_inf() {
    use std::f32;

    let buf: &[u8] = &[0xca, 0xff, 0x80, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(f32::NEG_INFINITY, read_f32(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_null_read_f32() {
    let buf: &[u8] = &[0xc0];
    let mut cur = Cursor::new(buf);

    match read_f32(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_f64_zero_plus() {
    let buf: &[u8] = &[0xcb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0.0, read_f64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_f64_zero_minus() {
    let buf: &[u8] = &[0xcb, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(-0.0, read_f64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_f64_inf() {
    use std::f64;

    let buf: &[u8] = &[0xcb, 0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(f64::INFINITY, read_f64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_f64_neg_inf() {
    use std::f64;

    let buf: &[u8] = &[0xcb, 0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(f64::NEG_INFINITY, read_f64(&mut cur).unwrap());
    assert_eq!(9, cur.position());
}

#[test]
fn from_null_read_f64() {
    let buf: &[u8] = &[0xc0];
    let mut cur = Cursor::new(buf);

    match read_f64(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}
