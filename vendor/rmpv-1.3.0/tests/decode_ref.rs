use rmpv::decode::{read_value_ref, Error};
use rmpv::ValueRef;

#[test]
fn from_nil() {
    let buf = [0xc0];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Nil, read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_bool_false() {
    let buf = [0xc2];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Boolean(false), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_bool_true() {
    let buf = [0xc3];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Boolean(true), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_null_read_twice() {
    use std::io::Cursor;

    let buf = [0xc0, 0xc0];

    let mut cur1 = Cursor::new(&buf[..]);
    let v1 = read_value_ref(&mut cur1).unwrap();

    let mut cur2 = Cursor::new(&buf[cur1.position() as usize..]);
    let v2 = read_value_ref(&mut cur2).unwrap();

    assert_eq!(ValueRef::Nil, v1);
    assert_eq!(ValueRef::Nil, v2);
}

#[test]
fn from_pfix() {
    let buf = [0x1f];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(31), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_nfix() {
    let buf = [0xe0];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(-32), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_u8() {
    let buf = [0xcc, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(255), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_u16() {
    let buf = [0xcd, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(65535), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_u32() {
    let buf = [0xce, 0xff, 0xff, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(4294967295u32), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_u64() {
    let buf = [0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(18446744073709551615u64), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_i8() {
    let buf = [0xd0, 0x7f];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(127), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_i16() {
    let buf = [0xd1, 0x7f, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(32767), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_i32() {
    let buf = [0xd2, 0x7f, 0xff, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(2147483647), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_i64() {
    let buf = [0xd3, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::from(9223372036854775807i64), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_f32() {
    let buf = [0xca, 0x7f, 0x7f, 0xff, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::F32(3.4028234e38_f32), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_f64() {
    use std::f64;

    let buf = [0xcb, 0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::F64(f64::INFINITY), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_strfix() {
    let buf = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];

    assert_eq!(ValueRef::from("le message"), read_value_ref(&mut &buf[..]).unwrap());
}

#[test]
fn from_str8() {
    let buf = [
        0xd9, // Type.
        0x20, // Size
        0x42, // B
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45  // E
    ];

    let mut slice = &buf[..];

    assert_eq!(ValueRef::from("B123456789012345678901234567890E"),
        read_value_ref(&mut slice).ok().unwrap());
}

#[test]
fn from_str16() {
    let buf = [
        0xda, // Type.
        0x00, 0x20, // Size
        0x42, // B
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45  // E
    ];

    let mut slice = &buf[..];

    assert_eq!(ValueRef::from("B123456789012345678901234567890E"),
        read_value_ref(&mut slice).ok().unwrap());
}

#[test]
fn from_str32() {
    let buf = [
        0xdb, // Type.
        0x00, 0x00, 0x00, 0x20, // Size
        0x42, // B
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x45  // E
    ];

    let mut slice = &buf[..];

    assert_eq!(ValueRef::from("B123456789012345678901234567890E"),
        read_value_ref(&mut slice).ok().unwrap());
}

#[test]
fn from_empty_buffer_invalid_marker_read() {
    let buf = [];

    let mut slice = &buf[..];

    match read_value_ref(&mut slice).err().unwrap() {
        Error::InvalidMarkerRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_empty_buffer_invalid_buffer_fill() {
    use rmpv::decode::value_ref::BorrowRead;
    use std::io::{self, Read};

    struct ErrorRead;

    impl Read for ErrorRead {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Mock Error"))
        }
    }

    impl<'a> BorrowRead<'a> for ErrorRead {
        fn fill_buf(&self) -> &'a [u8] { &[] }
        fn consume(&mut self, _: usize) {}
    }

    let mut rd = ErrorRead;

    match read_value_ref(&mut rd).err().unwrap() {
        Error::InvalidMarkerRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_string_insufficient_bytes_while_reading_length() {
    let buf = [0xd9];
    let mut rd = &buf[..];

    match read_value_ref(&mut rd).err().unwrap() {
        Error::InvalidDataRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_string_insufficient_bytes_while_reading_data() {
    let buf = [
        0xd9, // Type.
        0x20, // Size == 32
        0x42, // B
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x30
    ];

    let mut rd = &buf[..];

    match read_value_ref(&mut rd).err().unwrap() {
        Error::InvalidDataRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_string_invalid_utf8() {
    // Invalid 2 Octet Sequence.
    let buf = [0xd9, 0x02, 0xc3, 0x28];

    let mut rd = &buf[..];

    match read_value_ref(&mut rd).unwrap() {
        ValueRef::String(s) => {
            assert!(s.is_err());
            assert_eq!([0xc3, 0x28], s.as_bytes());
        }
        _ => panic!("wrong type"),
    }
}

#[test]
fn from_bin8() {
    let buf = [0xc4, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Binary(&[0, 1, 2, 3, 4]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_bin16() {
    let buf = [0xc5, 0x00, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Binary(&[0, 1, 2, 3, 4]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_bin32() {
    let buf = [0xc6, 0x00, 0x00, 0x00, 0x05, 0x00, 0x01, 0x02, 0x03, 0x04];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Binary(&[0, 1, 2, 3, 4]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_bin8_eof_while_reading_data() {
    let buf = [0xc4, 0x05, 0x00, 0x01, 0x02, 0x03];

    let mut rd = &buf[..];

    match read_value_ref(&mut rd).err().unwrap() {
        Error::InvalidDataRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_fixext1() {
    let buf = [0xd4, 0x2a, 0xff];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_ext1_eof_while_reading_type() {
    let buf = [0xd4];

    let mut rd = &buf[..];

    match read_value_ref(&mut rd).err().unwrap() {
        Error::InvalidDataRead(..) => (),
        _ => panic!(),
    }
}

#[test]
fn from_fixext2() {
    let buf = [0xd5, 0x2a, 0xff, 0xee];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_fixext4() {
    let buf = [0xd6, 0x2a, 0xff, 0xee, 0xdd, 0xcc];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_fixext8() {
    let buf = [0xd7, 0x2a, 0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204, 187, 170, 153, 136]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_fixext16() {
    let buf = [
        0xd8, 0x2a,
        0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88,
        0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00
    ];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204, 187, 170, 153, 136, 119, 102, 85, 68, 51, 34, 17, 0]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_ext8() {
    let buf = [0xc7, 0x04, 0x2a, 0xff, 0xee, 0xdd, 0xcc];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_ext16() {
    let buf = [0xc8, 0x00, 0x04, 0x2a, 0xff, 0xee, 0xdd, 0xcc];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_ext32() {
    let buf = [0xc9, 0x00, 0x00, 0x00, 0x04, 0x2a, 0xff, 0xee, 0xdd, 0xcc];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Ext(42, &[255, 238, 221, 204]),
        read_value_ref(&mut rd).ok().unwrap());
}

#[test]
fn from_fixmap() {
    let buf = [
        0x82, // size: 2
        0x2a, // 42
        0xce, 0x0, 0x1, 0x88, 0x94, // 100500
        0xa3, 0x6b, 0x65, 0x79, // 'key'
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65 // 'value'
    ];
    let mut rd = &buf[..];

    let map = vec![
        (ValueRef::from(42), ValueRef::from(100500)),
        (ValueRef::from("key"), ValueRef::from("value")),
    ];
    let expected = ValueRef::Map(map);

    assert_eq!(expected, read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_map16() {
    let buf = [
        0xde,
        0x00, 0x01,
        0xa3, 0x6b, 0x65, 0x79, // 'key'
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65 // 'value'
    ];
    let mut rd = &buf[..];

    let map = vec![(ValueRef::from("key"), ValueRef::from("value"))];
    let expected = ValueRef::Map(map);

    assert_eq!(expected, read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_map32() {
    let buf = [
        0xdf,
        0x00, 0x00, 0x00, 0x01,
        0xa3, 0x6b, 0x65, 0x79, // 'key'
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65 // 'value'
    ];
    let mut rd = &buf[..];

    let map = vec![(ValueRef::from("key"), ValueRef::from("value"))];
    let expected = ValueRef::Map(map);

    assert_eq!(expected, read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_fixarray() {
    let buf = [
        0x92,
        0xa2, 0x76, 0x31,
        0xa2, 0x76, 0x32,
    ];
    let mut rd = &buf[..];

    let vec = vec![ValueRef::from("v1"), ValueRef::from("v2")];

    assert_eq!(ValueRef::Array(vec), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_array16() {
    let buf = [
        0xdc,
        0x00, 0x02,
        0xa2, 0x76, 0x31,
        0xa2, 0x76, 0x32,
    ];
    let mut rd = &buf[..];

    let vec = vec![ValueRef::from("v1"), ValueRef::from("v2")];

    assert_eq!(ValueRef::Array(vec), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_array32() {
    let buf = [
        0xdd,
        0x00, 0x00, 0x00, 0x02,
        0xa2, 0x76, 0x31,
        0xa2, 0x76, 0x32,
    ];
    let mut rd = &buf[..];

    let vec = vec![ValueRef::from("v1"), ValueRef::from("v2")];

    assert_eq!(ValueRef::Array(vec), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_fixmap_using_cursor() {
    use std::io::Cursor;

    let buf = [
        0x82, // size: 2
        0x2a, // 42
        0xce, 0x0, 0x1, 0x88, 0x94, // 100500
        0xa3, 0x6b, 0x65, 0x79, // 'key'
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65 // 'value'
    ];
    let mut rd = Cursor::new(&buf[..]);

    let map = vec![
        (ValueRef::from(42), ValueRef::from(100500)),
        (ValueRef::from("key"), ValueRef::from("value")),
    ];
    let expected = ValueRef::Map(map);

    assert_eq!(expected, read_value_ref(&mut rd).unwrap());
    assert_eq!(17, rd.position());
}

// [None, 42, ['le message'], {'map': [True, {42: 100500}], 'key': 'value'}, [1, 2, 3], {'key': {'k1': 'v1'}}]
const COMPLEX_MSGPACK: [u8; 55] = [
    0x96, 0xc0, 0x2a, 0x91, 0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x82, 0xa3, 0x6d, 0x61, 0x70, 0x92, 0xc3, 0x81, 0x2a, 0xce, 0x0, 0x1, 0x88, 0x94, 0xa3,
    0x6b, 0x65, 0x79, 0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x93, 0x1, 0x2, 0x3, 0x81, 0xa3, 0x6b,
    0x65, 0x79, 0x81, 0xa2, 0x6b, 0x31, 0xa2, 0x76, 0x31
];

fn get_complex_msgpack_value<'a>() -> ValueRef<'a> {
    ValueRef::Array(vec![
        ValueRef::Nil,
        ValueRef::from(42),
        ValueRef::Array(vec![
            ValueRef::from("le message"),
        ]),
        ValueRef::Map(vec![
            (
                ValueRef::from("map"),
                ValueRef::Array(vec![
                    ValueRef::Boolean(true),
                    ValueRef::Map(vec![
                        (
                            ValueRef::from(42),
                            ValueRef::from(100500)
                        )
                    ])
                ])
            ),
            (
                ValueRef::from("key"),
                ValueRef::from("value")
            )
        ]),
        ValueRef::Array(vec![
            ValueRef::from(1),
            ValueRef::from(2),
            ValueRef::from(3),
        ]),
        ValueRef::Map(vec![
            (
                ValueRef::from("key"),
                ValueRef::Map(vec![
                    (
                        ValueRef::from("k1"),
                        ValueRef::from("v1")
                    )
                ])
            )
        ])
    ])
}

#[test]
fn from_complex_value_using_slice() {
    let buf = COMPLEX_MSGPACK;
    let mut rd = &buf[..];

    assert_eq!(get_complex_msgpack_value(), read_value_ref(&mut rd).unwrap());
}

#[test]
fn from_complex_value_using_cursor() {
    use std::io::Cursor;

    let buf = COMPLEX_MSGPACK;
    let mut rd = Cursor::new(&buf[..]);

    assert_eq!(get_complex_msgpack_value(), read_value_ref(&mut rd).unwrap());
    assert_eq!(buf.len() as u64, rd.position());
}

#[test]
fn from_reserved() {
    let buf = [0xc1];

    let mut rd = &buf[..];

    assert_eq!(ValueRef::Nil, read_value_ref(&mut rd).unwrap());
}

#[test]
fn into_owned() {
    use rmpv::Value;

    let val = get_complex_msgpack_value();

    let expected = Value::Array(vec![
        Value::Nil,
        Value::from(42),
        Value::Array(vec![
            Value::from("le message"),
        ]),
        Value::Map(vec![
            (
                Value::from("map"),
                Value::Array(vec![
                    Value::Boolean(true),
                    Value::Map(vec![
                        (
                            Value::from(42),
                            Value::from(100500)
                        )
                    ])
                ])
            ),
            (
                Value::from("key"),
                Value::from("value")
            )
        ]),
        Value::Array(vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
        ]),
        Value::Map(vec![
            (
                Value::from("key"),
                Value::Map(vec![
                    (
                        Value::from("k1"),
                        Value::from("v1")
                    )
                ])
            )
        ])
    ]);

    assert_eq!(expected, val.to_owned());
    assert_eq!(expected.as_ref(), val);
}
