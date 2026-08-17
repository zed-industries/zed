use super::Cursor;

use rmp::decode::*;

#[test]
fn from_bool_false() {
    let buf = [0xc2];
    let mut cur = Cursor::new(&buf[..]);

    assert!(!read_bool(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_bool_true() {
    let buf = [0xc3];
    let mut cur = Cursor::new(&buf[..]);

    assert!(read_bool(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}
