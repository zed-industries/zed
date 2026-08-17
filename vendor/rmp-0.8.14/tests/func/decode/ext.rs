use super::Cursor;

use rmp::decode::*;

#[test]
fn from_fixext1_read_fixext1() {
    let buf: &[u8] = &[0xd4, 0x01, 0x02];
    let mut cur = Cursor::new(buf);

    assert_eq!((1, 2), read_fixext1(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_fixext2_read_fixext2() {
    let buf = [0xd5, 0x01, 0x00, 0x02];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!((1, [0x00, 0x02]), read_fixext2(&mut cur).unwrap());
    assert_eq!(4, cur.position());
}

#[test]
fn from_fixext4_read_fixext4() {
    let buf = [0xd6, 0x01, 0x00, 0x00, 0x00, 0x02];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!((1, [0x00, 0x00, 0x00, 0x02]), read_fixext4(&mut cur).unwrap());
    assert_eq!(6, cur.position());
}

#[test]
fn from_fixext8_read_fixext8() {
    let buf = [0xd7, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!((1, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
               read_fixext8(&mut cur).unwrap());
    assert_eq!(10, cur.position());
}

#[test]
fn from_fixext16_read_fixext16() {
    let buf = [
        0xd8,
        0x01,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08
    ];
    let mut cur = Cursor::new(&buf[..]);

    assert_eq!((1, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
               read_fixext16(&mut cur).unwrap());
    assert_eq!(18, cur.position());
}

#[test]
fn from_fixext1_read_ext_meta() {
    let buf: &[u8] = &[0xd4, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 1 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_fixext2_read_ext_meta() {
    let buf: &[u8] = &[0xd5, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 2 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_fixext4_read_ext_meta() {
    let buf: &[u8] = &[0xd6, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 4 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_fixext8_read_ext_meta() {
    let buf: &[u8] = &[0xd7, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 8 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_fixext16_read_ext_meta() {
    let buf: &[u8] = &[0xd8, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 16 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_ext8_read_ext_meta() {
    let buf: &[u8] = &[0xc7, 0xff, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 255 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_ext16_read_ext_meta() {
    let buf: &[u8] = &[0xc8, 0xff, 0xff, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 65535 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(4, cur.position());
}

#[test]
fn from_ext32_read_ext_meta() {
    let buf: &[u8] = &[0xc9, 0xff, 0xff, 0xff, 0xff, 0x01];
    let mut cur = Cursor::new(buf);

    assert_eq!(ExtMeta { typeid: 1, size: 4294967295 }, read_ext_meta(&mut cur).unwrap());
    assert_eq!(6, cur.position());
}
