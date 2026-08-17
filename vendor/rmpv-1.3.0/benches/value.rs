#![feature(test)]

extern crate test;

use test::Bencher;

use rmpv::decode::*;

// Encoded value: [1, 0, [[["127.0.0.1", 59074]], 1, {0: ["read", {}, {0: ["value", {}], 1: ["error", {}]}], 1: ["write", {}, {0: ["value", {}], 1: ["error", {}]}], 2: ["remove", {}, {0: ["value", {}], 1: ["error", {}]}], 3: ["find", {}, {0: ["value", {}], 1: ["error", {}]}]}], [[80, 81, 82]]].
const COMPLEX: [u8; 137] = [
    0x94, 0x01, 0x00, 0x93, 0x91, 0x92, 0xa9, 0x31,
    0x32, 0x37, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x31,
    0xcd, 0xe6, 0xc2, 0x01, 0x84, 0x00, 0x93, 0xa4,
    0x72, 0x65, 0x61, 0x64, 0x80, 0x82, 0x00, 0x92,
    0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x80, 0x01,
    0x92, 0xa5, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x80,
    0x01, 0x93, 0xa5, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x80, 0x82, 0x00, 0x92, 0xa5, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x80, 0x01, 0x92, 0xa5, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x80, 0x02, 0x93, 0xa6, 0x72,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x80, 0x82, 0x00,
    0x92, 0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x80,
    0x01, 0x92, 0xa5, 0x65, 0x72, 0x72, 0x6f, 0x72,
    0x80, 0x03, 0x93, 0xa4, 0x66, 0x69, 0x6e, 0x64,
    0x80, 0x82, 0x00, 0x92, 0xa5, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x80, 0x01, 0x92, 0xa5, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x80, 0x91, 0x93, 0x50, 0x51,
    0x52,
];

#[bench]
fn from_string_read_value(b: &mut Bencher) {
    // Lorem ipsum dolor sit amet.
    let buf = [
        0xbb, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75,
        0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74,
        0x20, 0x61, 0x6d, 0x65, 0x74, 0x2e
    ];

    b.iter(|| {
        let res = read_value(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
}

#[bench]
fn from_string_read_value_ref(b: &mut Bencher) {
    // Lorem ipsum dolor sit amet.
    let buf = [
        0xbb, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75,
        0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74,
        0x20, 0x61, 0x6d, 0x65, 0x74, 0x2e
    ];

    b.iter(|| {
        let res = read_value_ref(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
}

#[bench]
fn from_complex_read_value(b: &mut Bencher) {
    b.iter(|| {
        let res = read_value(&mut &COMPLEX[..]).unwrap();
        test::black_box(res);
    });
    b.bytes = COMPLEX.len() as u64;
}

 #[bench]
 fn from_complex_read_value_ref(b: &mut Bencher) {
     b.iter(|| {
         let res = read_value_ref(&mut &COMPLEX[..]).unwrap();
         test::black_box(res);
     });
     b.bytes = COMPLEX.len() as u64;
 }

#[bench]
fn from_complex_write_value_ref(b: &mut Bencher) {
    use rmpv::encode::write_value_ref;
    use rmpv::ValueRef::*;

    let val = Array(vec![
        Nil,
        Integer(42.into()),
        F64(3.1415),
        String("Lorem ipsum dolor sit amet.".into()),
        Map(vec![
            (String("key".into()), String("value".into())),
        ]),
    ]);

    let mut buf = [0u8; 64];

    b.iter(|| {
        write_value_ref(&mut &mut buf[..], &val).unwrap();
    });
    b.bytes = buf.len() as u64;
}

#[bench]
fn from_complex_read_value_ref_to_owned(b: &mut Bencher) {
    let buf = [
        0x95, // Fixed array with 5 len.
        0xc0, // Nil.
        0x2a, // 42.
        0xcb, 0x40, 0x9, 0x21, 0xca, 0xc0, 0x83, 0x12, 0x6f, // 3.1415
        // Fixed string with "Lorem ipsum dolor sit amet." content.
        0xbb, 0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70, 0x73, 0x75,
        0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x74,
        0x20, 0x61, 0x6d, 0x65, 0x74, 0x2e,
        0x81, // Fixed map with 1 len.
        0xa3, 0x6b, 0x65, 0x79, // Key "key".
        0xa5, 0x76, 0x61, 0x6c, 0x75, 0x65 // Value: "value".
    ];

    b.iter(|| {
        let res = read_value_ref(&mut &buf[..]).unwrap().to_owned();
        test::black_box(res);
    });
    b.bytes = buf.len() as u64;
}

/// Read a single large bin32 value.
fn read_large_bin32(b: &mut Bencher, size: u32) {
    // Creat buffer, fill it with bytes
    let mut buf = Vec::with_capacity(size as usize);
    buf.resize(size as usize, 42);

    // Write header (bin32 format family containing size-5 bytes)
    let size_bytes: [u8; 4] = (size - 5).to_be_bytes();
    buf[0] = 0xc6;
    buf[1] = size_bytes[0];
    buf[2] = size_bytes[1];
    buf[3] = size_bytes[2];
    buf[4] = size_bytes[3];

    // Read value
    b.iter(|| {
        let res = read_value(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
    b.bytes = u64::from(size);
}

#[bench]
fn read_bin32_50kib(b: &mut Bencher) {
    read_large_bin32(b, 50 * 1024);
}

#[bench]
fn read_bin32_100kib(b: &mut Bencher) {
    read_large_bin32(b, 100 * 1024);
}

#[bench]
fn read_bin32_1mib(b: &mut Bencher) {
    read_large_bin32(b, 1024 * 1024);
}

#[bench]
fn read_bin32_20mib(b: &mut Bencher) {
    read_large_bin32(b, 20 * 1024 * 1024);
}

#[bench]
fn read_bin32_100mib(b: &mut Bencher) {
    read_large_bin32(b, 100 * 1024 * 1024);
}

/// Read a flat array containing positive 32-bit unsigned integers.
fn read_large_array(b: &mut Bencher, element_count: usize) {
    // Creat buffer, fill it with bytes
    let size = element_count * 5 /* uint32 size */ + 5 /* array overhead */;
    let mut buf = Vec::with_capacity(size);
    buf.resize(size, 0);

    // Write header
    let size_bytes: [u8; 4] = (size as u32 - 5).to_be_bytes();
    buf[0] = 0xc6;
    buf[1] = size_bytes[0];
    buf[2] = size_bytes[1];
    buf[3] = size_bytes[2];
    buf[4] = size_bytes[3];

    // Write elements
    let elements = &mut buf[5..];
    for i in 0..element_count {
        let offset = i * 5;
        let value_bytes = 42u32.to_be_bytes();
        elements[offset] = 0xce; // u32
        elements[offset + 1] = value_bytes[0];
        elements[offset + 2] = value_bytes[1];
        elements[offset + 3] = value_bytes[2];
        elements[offset + 4] = value_bytes[3];
    }

    // Read value
    b.iter(|| {
        let res = read_value(&mut &buf[..]).unwrap();
        test::black_box(res);
    });
    b.bytes = size as u64;
}

#[bench]
fn read_array_50kib(b: &mut Bencher) {
    read_large_array(b, 50 * 1024);
}

#[bench]
fn read_array_100kib(b: &mut Bencher) {
    read_large_array(b, 100 * 1024);
}

#[bench]
fn read_array_1mib(b: &mut Bencher) {
    read_large_array(b, 1024 * 1024);
}

#[bench]
fn read_array_20mib(b: &mut Bencher) {
    read_large_array(b, 20 * 1024 * 1024);
}
