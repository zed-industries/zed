extern crate bytes;

use bytes::{Buf, IntoBuf, Bytes};

#[test]
fn iter_len() {
    let buf = Bytes::from(&b"hello world"[..]).into_buf();
    let iter = buf.iter();

    assert_eq!(iter.size_hint(), (11, Some(11)));
    assert_eq!(iter.len(), 11);
}


#[test]
fn empty_iter_len() {
    let buf = Bytes::from(&b""[..]).into_buf();
    let iter = buf.iter();

    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.len(), 0);
}
