use rmpv::decode::{read_value, Error};
use rmpv::Value;

#[test]
fn from_null_decode_value() {
    let buf = [0xc0];
    assert_eq!(Value::Nil, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_pfix_decode_value() {
    let buf = [0x1f];
    assert_eq!(Value::from(31), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_bool_decode_value() {
    let buf = [0xc3, 0xc2];
    assert_eq!(Value::Boolean(true), read_value(&mut &buf[..]).unwrap());
    assert_eq!(Value::Boolean(false), read_value(&mut &buf[1..]).unwrap());
}

#[test]
fn from_bin8_decode_value() {
    let buf: &[u8] = &[
        0xc4,
        0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    let expected = Value::Binary(vec!(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08));

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_bin16_decode_value() {
    let buf: &[u8] = &[
        0xc5,
        0x00, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    let expected = Value::Binary(vec!(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08));

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_bin32_decode_value() {
    let buf: &[u8] = &[
        0xc6,
        0x00, 0x00, 0x00, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    let expected = Value::Binary(vec!(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08));

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_i32_decode_value() {
    let buf = [0xd2, 0xff, 0xff, 0xff, 0xff];
    assert_eq!(Value::from(-1), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_f64_decode_value() {
    let buf = [0xcb, 0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(Value::F64(::std::f64::NEG_INFINITY), read_value(&mut &buf[..]).unwrap());
}


#[test]
fn from_strfix_decode_value() {
    let buf = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
    assert_eq!(Value::String("le message".into()), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixarray_decode_value() {
    let buf = [
        0x93,
        0x00, 0x2a, 0xf7
    ];

    let expected = Value::Array(vec![
        Value::from(0),
        Value::from(42),
        Value::from(-9),
    ]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixarray_incomplete_decode_value() {
    let buf = [
        0x93,
        0x00, 0x2a
    ];

    match read_value(&mut &buf[..]) {
        Err(Error::InvalidMarkerRead(..)) => {}
        other => panic!("unexpected result: {other:?}"),
    }
}

#[test]
fn from_fixarray_decode_empty() {
    let buf: &[u8] = &[0x90];

    assert_eq!(Value::Array(vec![]), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixarray_of_decode_max_length() {
    let buf: &[u8] = &[0x9f, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                       0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

    let expected = Value::Array(vec![Value::from(1), Value::from(2), Value::from(3),
                                     Value::from(4), Value::from(5), Value::from(6),
                                     Value::from(7), Value::from(8), Value::from(9),
                                     Value::from(10), Value::from(11), Value::from(12),
                                     Value::from(13), Value::from(14), Value::from(15)]);
    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_array16_decode_value() {
    let buf: &[u8] = &[
        0xdc,
        0x00, 0x06,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

    let expected = Value::Array(vec![Value::from(1), Value::from(2), Value::from(3),
                                     Value::from(4), Value::from(5), Value::from(6)]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}


#[test]
fn from_array32_decode_value() {
    let buf: &[u8] = &[
        0xdd,
        0x00, 0x00, 0x00, 0x06,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

    let expected = Value::Array(vec![Value::from(1), Value::from(2), Value::from(3),
                                     Value::from(4), Value::from(5), Value::from(6)]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixmap_decode_value() {
    let buf = [
        0x82,
        0x2a,
        0xce, 0x0, 0x1, 0x88, 0x94,
        0xa3, 0x6b, 0x65, 0x79,
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65
    ];

    let expected = Value::Map(vec![
        (Value::from(42), Value::from(100500)),
        (Value::String("key".into()), Value::String("value".into())),
    ]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_map16_decode_value() {
    let buf = [
        0xde,
        0x00, 0x02,
        0x2a,
        0xce, 0x0, 0x1, 0x88, 0x94,
        0xa3, 0x6b, 0x65, 0x79,
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65
    ];

    let expected = Value::Map(vec![
        (Value::from(42), Value::from(100500)),
        (Value::String("key".into()), Value::String("value".into())),
    ]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_map32_decode_value() {
    let buf = [
        0xdf,
        0x00, 0x00, 0x00, 0x02,
        0x2a,
        0xce, 0x0, 0x1, 0x88, 0x94,
        0xa3, 0x6b, 0x65, 0x79,
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65
    ];

    let expected = Value::Map(vec![
        (Value::from(42), Value::from(100500)),
        (Value::String("key".into()), Value::String("value".into())),
    ]);

    assert_eq!(expected, read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixext1_decode_value() {
    let buf = [0xd4, 0x01, 0x02];
    assert_eq!(Value::Ext(1, vec![2]), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixext2_decode_value() {
    let buf = [0xd5, 0x01, 0x02, 0x03];
    assert_eq!(Value::Ext(1, vec![2, 3]), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixext4_decode_value() {
    let buf = [0xd6, 0x01, 0x02, 0x03, 0x04, 0x05];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5]), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixext8_decode_value() {
    let buf = [0xd7, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5, 6, 7, 8, 9]), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_fixext16_decode_value() {
    let buf = [0xd8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                           0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6, 7, 8, 9]),
               read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_ext8_decode_value() {
    let buf = [0xc7, 0x04, 0x01, 0x02, 0x03, 0x04, 0x05];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5]),
               read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_ext16_decode_value() {
    let buf = [0xc8, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04, 0x05];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5]),
               read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_ext32_decode_value() {
    let buf = [0xc9, 0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04, 0x05];
    assert_eq!(Value::Ext(1, vec![2, 3, 4, 5]),
               read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_str8_decode_value() {
    let buf: &[u8] = &[
        0xd9,
        0x20,
        0x42,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45
    ];

    assert_eq!(Value::String("B123456789012345678901234567890E".into()),
        read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_str8_invalid_utf8() {
    // Invalid 2 Octet Sequence.
    let buf: &[u8] = &[0xd9, 0x02, 0xc3, 0x28];

    match read_value(&mut &buf[..]).unwrap() {
        Value::String(s) => {
            assert!(s.is_err());
            assert_eq!(vec![0xc3, 0x28], s.into_bytes());
        }
        _ => panic!("wrong type"),
    }
}

#[test]
fn from_str16_decode_value() {
    let buf: &[u8] = &[
        0xda,
        0x00,
        0x20,
        0x42,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45
    ];

    assert_eq!(Value::String("B123456789012345678901234567890E".into()),
        read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_str16_invalid_utf8() {
    // Invalid 2 Octet Sequence.
    let buf: &[u8] = &[0xda, 0x00, 0x02, 0xc3, 0x28];

    match read_value(&mut &buf[..]).unwrap() {
        Value::String(s) => {
            assert!(s.is_err());
            assert_eq!(vec![0xc3, 0x28], s.into_bytes());
        }
        _ => panic!("wrong type"),
    }
}

#[test]
fn from_str32_decode_value() {
    let buf: &[u8] = &[
        0xdb,
        0x00,
        0x00,
        0x00,
        0x20,
        0x42,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45
    ];

    assert_eq!(Value::String("B123456789012345678901234567890E".into()),
        read_value(&mut &buf[..]).unwrap());
}

#[test]
fn from_str32_invalid_utf8() {
    // Invalid 2 Octet Sequence.
    let buf: &[u8] = &[0xdb, 0x00, 0x00, 0x00, 0x02, 0xc3, 0x28];

    match read_value(&mut &buf[..]).unwrap() {
        Value::String(s) => {
            assert!(s.is_err());
            assert_eq!(vec![0xc3, 0x28], s.into_bytes());
        }
        _ => panic!("wrong type"),
    }
}

#[test]
fn from_array_of_two_integers() {
    let buf: &[u8] = &[0x92, 0x04, 0x2a];

    let vec = vec![Value::from(4), Value::from(42)];
    assert_eq!(Value::Array(vec), read_value(&mut &buf[..]).unwrap());
}

#[test]
fn invalid_buf_size_bin32() {
    // This invalid buffer requests a 4 GiB byte vec.
    let buf: &[u8] = &[0xc6, 0xff, 0xff, 0xff, 0xff, 0x00];
    match read_value(&mut &buf[..]) {
        Ok(_) => panic!("Unexpected success"),
        Err(Error::InvalidDataRead(_)) => { /* expected */ },
        Err(e) => panic!("Unexpected error: {e}"),
    }
}

#[test]
fn invalid_buf_size_arr() {
    // This invalid buffer requests a nested array of depth 10.
    // All arrays contain the maximum possible number of elements.
    // If a byte is preallocated for every array content,
    // that would require 40 GiB of RAM.
    let buf: &[u8] = &[
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
        0xdd, 0xff, 0xff, 0xff, 0xff,
    ];
    match read_value(&mut &buf[..]) {
        Ok(_) => panic!("Unexpected success"),
        Err(Error::InvalidMarkerRead(_)) => { /* expected */ },
        Err(e) => panic!("Unexpected error: {e}"),
    }
}
