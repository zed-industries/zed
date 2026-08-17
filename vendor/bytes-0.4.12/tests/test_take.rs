extern crate bytes;

use bytes::Buf;
use std::io::Cursor;

#[test]
fn long_take() {
    // Tests that take with a size greater than the buffer length will not
    // overrun the buffer. Regression test for #138.
    let buf = Cursor::new(b"hello world").take(100);
    assert_eq!(11, buf.remaining());
    assert_eq!(b"hello world", buf.bytes());
}
