use super::Cursor;

use rmp::decode::*;
use rmp::Marker;

#[test]
fn from_fixstr_min_read_str_len() {
    let buf: &[u8] = &[0xa0];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_str_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_fixstr_rnd_read_str_len() {
    let buf: &[u8] = &[0xaa];
    let mut cur = Cursor::new(buf);

    assert_eq!(10, read_str_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_fixstr_max_read_str_len() {
    let buf: &[u8] = &[0xbf];
    let mut cur = Cursor::new(buf);

    assert_eq!(31, read_str_len(&mut cur).unwrap());
    assert_eq!(1, cur.position());
}

#[test]
fn from_str8_min_read_str_len() {
    let buf: &[u8] = &[0xd9, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_str_len(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_str8_rnd_read_str_len() {
    let buf: &[u8] = &[0xd9, 0x0a];
    let mut cur = Cursor::new(buf);

    assert_eq!(10, read_str_len(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_str8_read_str_len_eof() {
    let buf: &[u8] = &[0xd9];
    let mut cur = Cursor::new(buf);

    read_str_len(&mut cur).err().unwrap();
    assert_eq!(1, cur.position());
}

#[test]
fn from_str8_max_read_str_len() {
    let buf: &[u8] = &[0xd9, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(255, read_str_len(&mut cur).unwrap());
    assert_eq!(2, cur.position());
}

#[test]
fn from_str16_min_read_str_len() {
    let buf: &[u8] = &[0xda, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_str_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_str16_max_read_str_len() {
    let buf: &[u8] = &[0xda, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(65535, read_str_len(&mut cur).unwrap());
    assert_eq!(3, cur.position());
}

#[test]
fn from_str16_read_str_len_eof() {
    let buf: &[u8] = &[0xda, 0x00];
    let mut cur = Cursor::new(buf);

    read_str_len(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_str32_min_read_str_len() {
    let buf: &[u8] = &[0xdb, 0x00, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    assert_eq!(0, read_str_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_str32_max_read_str_len() {
    let buf: &[u8] = &[0xdb, 0xff, 0xff, 0xff, 0xff];
    let mut cur = Cursor::new(buf);

    assert_eq!(4294967295, read_str_len(&mut cur).unwrap());
    assert_eq!(5, cur.position());
}

#[test]
fn from_str32_read_str_len_eof() {
    let buf: &[u8] = &[0xdb, 0x00, 0x00, 0x00];
    let mut cur = Cursor::new(buf);

    read_str_len(&mut cur).err().unwrap();
    assert!(cur.position() >= 1);
}

#[test]
fn from_null_read_str_len() {
    let buf: &[u8] = &[0xc0];
    let mut cur = Cursor::new(buf);

    match read_str_len(&mut cur) {
        Err(ValueReadError::TypeMismatch(Marker::Null)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_str_strfix() {
    let buf: &[u8] = &[0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
    let mut cur = Cursor::new(buf);

    let out: &mut [u8] = &mut [0u8; 16];

    assert_eq!("le message", read_str(&mut cur, out).unwrap());
    assert_eq!(11, cur.position());
}

#[test]
fn from_str_strfix_extra_data() {
    let buf: &[u8] = &[0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x00];
    let mut cur = Cursor::new(buf);

    let out: &mut [u8] = &mut [0u8; 16];

    assert_eq!("le message", read_str(&mut cur, out).unwrap());
    assert_eq!(11, cur.position());
}

#[test]
fn from_str_strfix_exact_buffer() {
    let buf: &[u8] = &[0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
    let mut cur = Cursor::new(buf);

    let out: &mut [u8] = &mut [0u8; 10];

    assert_eq!("le message", read_str(&mut cur, out).unwrap());
    assert_eq!(11, cur.position());
}

#[test]
fn from_str_strfix_invalid_utf8() {
    // Invalid 2 Octet Sequence.
    let buf: &[u8] = &[0xa2, 0xc3, 0x28];
    let mut cur = Cursor::new(buf);

    let out: &mut [u8] = &mut [0u8; 16];

    match read_str(&mut cur, out) {
        Err(DecodeStringError::InvalidUtf8(raw, _)) => {
            assert_eq!(&[0xc3, 0x28], raw);
        }
        other => panic!("unexpected result: {other:?}"),
    }

    assert_eq!(3, cur.position());
}

#[test]
fn from_str_strfix_buffer_too_small() {
    let buf: &[u8] = &[0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
    let mut cur = Cursor::new(buf);

    let out: &mut [u8] = &mut [0u8; 9];

    match read_str(&mut cur, out) {
        Err(DecodeStringError::BufferSizeTooSmall(10)) => (),
        other => panic!("unexpected result: {other:?}"),
    }
    assert_eq!(1, cur.position());
}

#[test]
fn from_str_strfix_decode_from_slice() {
    // Wrap an incomplete buffer into the Cursor to see how many bytes were consumed.
    let mut buf = vec![0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73];
    assert!(read_str_from_slice(&buf).is_err());

    // ... complete the buffer and try to parse again.
    buf.append(&mut vec![0x73, 0x61, 0x67, 0x65]);
    assert_eq!(("le message", &[][..]), read_str_from_slice(&buf).unwrap());
}

#[test]
fn from_str_strfix_decode_from_slice_with_trailing_bytes() {
    let buf = vec![
        0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x01, 0x02, 0x03,
    ];

    assert_eq!(("le message", &[0x01, 0x02, 0x03][..]), read_str_from_slice(&buf).unwrap());
}

#[test]
fn example_process_sequence_of_strings() {
    // Encoded: 'Unpacking', 'multiple', 'strings'.
    let vec = [
        0xa9, 0x55, 0x6e, 0x70, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67,
        0xa8, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0xa7,
        0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x73
    ];

    let mut chunks = Vec::new();
    let mut unparsed = &vec[..];
    loop {
        match read_str_from_slice(unparsed) {
            Ok((chunk, tail)) => {
                chunks.push(chunk);
                unparsed = tail;
            }
            Err(..) => break,
        }
    }

    assert_eq!(["Unpacking", "multiple", "strings"], chunks[..]);
}
