use std::io::{Cursor, Read};
use std::str;
use system_interface::io::Peek;

#[test]
fn test_peek() {
    let mut input = Cursor::new("hello".to_string());
    let mut buf = vec![0_u8; 20];

    // Do a peek.
    assert_eq!(Peek::peek(&mut input, &mut buf).unwrap(), 5);
    assert_eq!(str::from_utf8(&buf[..5]).unwrap(), "hello");
    assert_eq!(input.position(), 0);

    // Peek doesn't advance the cursor.
    assert_eq!(Peek::peek(&mut input, &mut buf).unwrap(), 5);
    assert_eq!(str::from_utf8(&buf[..5]).unwrap(), "hello");
    assert_eq!(input.position(), 0);

    // Read does.
    assert_eq!(Read::read(&mut input, &mut buf).unwrap(), 5);
    assert_eq!(str::from_utf8(&buf[..5]).unwrap(), "hello");
    assert_eq!(input.position(), 5);
}
