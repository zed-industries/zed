// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

#[test]
fn test_reader_be() {
    use bitstream_io::{BigEndian, BitRead, BitReader};

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // reading individual bits
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);

    // reading unsigned values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.byte_aligned());
    assert_eq!(r.read_var::<u32>(2).unwrap(), 2);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(3).unwrap(), 6);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(5).unwrap(), 7);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(3).unwrap(), 5);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(19).unwrap(), 0x53BC1);
    assert!(r.byte_aligned());
    assert!(r.read_var::<u32>(1).is_err());

    // reading const unsigned values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.byte_aligned());
    assert_eq!(r.read::<2, u32>().unwrap(), 2);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<3, u32>().unwrap(), 6);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<5, u32>().unwrap(), 7);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<3, u32>().unwrap(), 5);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<19, u32>().unwrap(), 0x53BC1);
    assert!(r.byte_aligned());
    assert!(r.read::<1, u32>().is_err());

    // 16-bit values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read::<1, u8>().unwrap(), 1);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b0110_0011_1101_10);
    assert_eq!(r.read::<16, u16>().unwrap(), 0b1001_1101_1110_0000);
    assert_eq!(r.read::<1, u8>().unwrap(), 1);

    // skipping bits
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_var::<u32>(2).unwrap(), 2);
    assert!(r.skip(3).is_ok());
    assert_eq!(r.read_var::<u32>(5).unwrap(), 7);
    assert!(r.skip(3).is_ok());
    assert_eq!(r.read_var::<u32>(19).unwrap(), 0x53BC1);

    // reading signed values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_signed_var::<i32>(2).unwrap(), -2);
    assert_eq!(r.read_signed_var::<i32>(3).unwrap(), -2);
    assert_eq!(r.read_signed_var::<i32>(5).unwrap(), 7);
    assert_eq!(r.read_signed_var::<i32>(3).unwrap(), -3);
    assert_eq!(r.read_signed_var::<i32>(19).unwrap(), -181311);

    // reading const signed values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_signed::<2, i32>().unwrap(), -2);
    assert_eq!(r.read_signed::<3, i32>().unwrap(), -2);
    assert_eq!(r.read_signed::<5, i32>().unwrap(), 7);
    assert_eq!(r.read_signed::<3, i32>().unwrap(), -3);
    assert_eq!(r.read_signed::<19, i32>().unwrap(), -181311);

    // reading unary 0 values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_unary::<0>().unwrap(), 1);
    assert_eq!(r.read_unary::<0>().unwrap(), 2);
    assert_eq!(r.read_unary::<0>().unwrap(), 0);
    assert_eq!(r.read_unary::<0>().unwrap(), 0);
    assert_eq!(r.read_unary::<0>().unwrap(), 4);

    // reading unary 1 values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);
    assert_eq!(r.read_unary::<1>().unwrap(), 1);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);
    assert_eq!(r.read_unary::<1>().unwrap(), 3);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);

    // reading unsigned vbr
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_unsigned_vbr::<4, u8>().unwrap(), 11);
    assert_eq!(r.read_unsigned_vbr::<4, u8>().unwrap(), 238);
    assert_eq!(r.read_unsigned_vbr::<4, u8>().unwrap(), 99);
    assert!(r.read_unsigned_vbr::<4, u8>().is_err());

    // reading signed vbr
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_signed_vbr::<4, i8>().unwrap(), -6);
    assert_eq!(r.read_signed_vbr::<4, i8>().unwrap(), 119);
    assert_eq!(r.read_signed_vbr::<4, i8>().unwrap(), -50);
    assert!(r.read_signed_vbr::<4, i8>().is_err());

    // byte aligning
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert_eq!(r.read_var::<u32>(3).unwrap(), 5);
    r.byte_align();
    assert_eq!(r.read_var::<u32>(3).unwrap(), 7);
    r.byte_align();
    r.byte_align();
    assert_eq!(r.read_var::<u32>(8).unwrap(), 59);
    r.byte_align();
    assert_eq!(r.read_var::<u32>(4).unwrap(), 12);

    // reading bytes, aligned
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    let mut sub_data = [0; 2];
    assert!(r.read_bytes(&mut sub_data).is_ok());
    assert_eq!(&sub_data, b"\xB1\xED");

    // reading bytes, un-aligned
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    let mut sub_data = [0; 2];
    assert_eq!(r.read_var::<u32>(4).unwrap(), 11);
    assert!(r.read_bytes(&mut sub_data).is_ok());
    assert_eq!(&sub_data, b"\x1E\xD3");
}

#[test]
fn test_edge_cases_be() {
    use bitstream_io::{BigEndian, BitRead, BitReader};

    let data: Vec<u8> = vec![
        0, 0, 0, 0, 255, 255, 255, 255, 128, 0, 0, 0, 127, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 127, 255, 255, 255, 255,
        255, 255, 255,
    ];

    // 0 bit reads
    let mut r = BitReader::endian([255; 1].as_slice(), BigEndian);
    assert_eq!(r.read_var::<u8>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u16>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u32>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u64>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u8>(8).unwrap(), 255);

    let mut r = BitReader::endian([255; 1].as_slice(), BigEndian);
    assert_eq!(r.read::<0, u8>().unwrap(), 0);
    assert_eq!(r.read::<0, u16>().unwrap(), 0);
    assert_eq!(r.read::<0, u32>().unwrap(), 0);
    assert_eq!(r.read::<0, u64>().unwrap(), 0);
    assert_eq!(r.read::<8, u8>().unwrap(), 255);

    let mut r = BitReader::endian([255; 1].as_slice(), BigEndian);
    assert!(r.read_signed_var::<i8>(0).is_err());
    assert!(r.read_signed_var::<i16>(0).is_err());
    assert!(r.read_signed_var::<i32>(0).is_err());
    assert!(r.read_signed_var::<i64>(0).is_err());

    // unsigned 32 and 64-bit values
    let mut r = BitReader::endian(data.as_slice(), BigEndian);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 0);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 4294967295);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 2147483648);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 2147483647);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 0);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 9223372036854775808);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 9223372036854775807);

    let mut r = BitReader::endian(data.as_slice(), BigEndian);
    assert_eq!(r.read::<32, u32>().unwrap(), 0);
    assert_eq!(r.read::<32, u32>().unwrap(), 4294967295);
    assert_eq!(r.read::<32, u32>().unwrap(), 2147483648);
    assert_eq!(r.read::<32, u32>().unwrap(), 2147483647);
    assert_eq!(r.read::<64, u64>().unwrap(), 0);
    assert_eq!(r.read::<64, u64>().unwrap(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(r.read::<64, u64>().unwrap(), 9223372036854775808);
    assert_eq!(r.read::<64, u64>().unwrap(), 9223372036854775807);

    // signed 32 and 64-bit values
    let mut r = BitReader::endian(data.as_slice(), BigEndian);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), 0);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), -1);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), -2147483648);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), 2147483647);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), 0);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), -1);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), -9223372036854775808);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), 9223372036854775807);

    let mut r = BitReader::endian(data.as_slice(), BigEndian);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), 0);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), -1);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), -2147483648);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), 2147483647);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), 0);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), -1);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), -9223372036854775808);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), 9223372036854775807);
}

#[test]
fn test_reader_huffman_be() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BigEndian, BitRead, BitReader};

    define_huffman_tree!(SomeTree : i32 = [[[4, 3], 2], [1, 0]]);

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);

    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 4);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 4);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
}

#[test]
fn test_read_chunks_be() {
    use bitstream_io::{BigEndian, BitRead, BitReader};

    let data: &[u8] = &[0b1011_0001, 0b1110_1101, 0b0011_1011, 0b1100_0001];
    let mut chunk: [u8; 2] = [0; 2];

    // test non-aligned chunk reading
    let mut r = BitReader::endian(data, BigEndian);
    assert_eq!(r.read::<2, u8>().unwrap(), 0b10);
    r.read_bytes(&mut chunk).unwrap();
    assert_eq!(&chunk, &[0b11_0001_11, 0b10_1101_00]);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b11_1011_1100_0001);

    // test the smallest chunk
    let mut chunk = 0;
    let mut r = BitReader::endian(data, BigEndian);
    assert_eq!(r.read::<2, u8>().unwrap(), 0b10);
    r.read_bytes(core::slice::from_mut(&mut chunk)).unwrap();
    assert_eq!(chunk, 0b11_0001_11);
    r.read_bytes(core::slice::from_mut(&mut chunk)).unwrap();
    assert_eq!(chunk, 0b10_1101_00);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b11_1011_1100_0001);

    // test a larger chunk
    let data = include_bytes!("random.bin");
    let mut chunk: [u8; 127] = [0; 127];

    let mut r = BitReader::endian(data.as_slice(), BigEndian);
    assert_eq!(r.read::<3, u8>().unwrap(), 0b000);
    r.read_bytes(&mut chunk).unwrap();
    assert_eq!(
        chunk.as_slice(),
        include_bytes!("random-3be.bin").as_slice()
    );
    assert_eq!(r.read::<5, u8>().unwrap(), 0b10110);
}

#[test]
fn test_reader_le() {
    use bitstream_io::{BitRead, BitReader, LittleEndian};

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // reading individual bits
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), false);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_bit().unwrap(), true);

    // reading unsigned values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.byte_aligned());
    assert_eq!(r.read_var::<u32>(2).unwrap(), 1);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(3).unwrap(), 4);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(5).unwrap(), 13);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(3).unwrap(), 3);
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(19).unwrap(), 0x609DF);
    assert!(r.byte_aligned());
    assert!(r.read_var::<u32>(1).is_err());

    // reading const unsigned values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.byte_aligned());
    assert_eq!(r.read::<2, u32>().unwrap(), 1);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<3, u32>().unwrap(), 4);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<5, u32>().unwrap(), 13);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<3, u32>().unwrap(), 3);
    assert!(!r.byte_aligned());
    assert_eq!(r.read::<19, u32>().unwrap(), 0x609DF);
    assert!(r.byte_aligned());
    assert!(r.read::<1, u32>().is_err());

    // 16-bit values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read::<1, u8>().unwrap(), 1);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b1101_1011_0110_00);
    assert_eq!(r.read::<16, u16>().unwrap(), 0b1000_0010_0111_0111);
    assert_eq!(r.read::<1, u8>().unwrap(), 1);

    // skipping bits
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_var::<u32>(2).unwrap(), 1);
    assert!(r.skip(3).is_ok());
    assert_eq!(r.read_var::<u32>(5).unwrap(), 13);
    assert!(r.skip(3).is_ok());
    assert_eq!(r.read_var::<u32>(19).unwrap(), 0x609DF);

    // reading signed values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_signed_var::<i32>(2).unwrap(), 1);
    assert_eq!(r.read_signed_var::<i32>(3).unwrap(), -4);
    assert_eq!(r.read_signed_var::<i32>(5).unwrap(), 13);
    assert_eq!(r.read_signed_var::<i32>(3).unwrap(), 3);
    assert_eq!(r.read_signed_var::<i32>(19).unwrap(), -128545);

    // reading const signed values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_signed::<2, i32>().unwrap(), 1);
    assert_eq!(r.read_signed::<3, i32>().unwrap(), -4);
    assert_eq!(r.read_signed::<5, i32>().unwrap(), 13);
    assert_eq!(r.read_signed::<3, i32>().unwrap(), 3);
    assert_eq!(r.read_signed::<19, i32>().unwrap(), -128545);

    // reading unary 0 values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_unary::<0>().unwrap(), 1);
    assert_eq!(r.read_unary::<0>().unwrap(), 0);
    assert_eq!(r.read_unary::<0>().unwrap(), 0);
    assert_eq!(r.read_unary::<0>().unwrap(), 2);
    assert_eq!(r.read_unary::<0>().unwrap(), 2);

    // reading unary 1 values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);
    assert_eq!(r.read_unary::<1>().unwrap(), 3);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);
    assert_eq!(r.read_unary::<1>().unwrap(), 1);
    assert_eq!(r.read_unary::<1>().unwrap(), 0);

    // byte aligning
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert_eq!(r.read_var::<u32>(3).unwrap(), 1);
    r.byte_align();
    assert_eq!(r.read_var::<u32>(3).unwrap(), 5);
    r.byte_align();
    r.byte_align();
    assert_eq!(r.read_var::<u32>(8).unwrap(), 59);
    r.byte_align();
    assert_eq!(r.read_var::<u32>(4).unwrap(), 1);

    // reading bytes, aligned
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    let mut sub_data = [0; 2];
    assert!(r.read_bytes(&mut sub_data).is_ok());
    assert_eq!(&sub_data, b"\xB1\xED");

    // reading bytes, un-aligned
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    let mut sub_data = [0; 2];
    assert_eq!(r.read_var::<u32>(4).unwrap(), 1);
    assert!(r.read_bytes(&mut sub_data).is_ok());
    assert_eq!(&sub_data, b"\xDB\xBE");
}

#[test]
fn test_edge_cases_le() {
    use bitstream_io::{BitRead, BitReader, LittleEndian};

    let data: Vec<u8> = vec![
        0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 255, 255,
        255, 255, 127,
    ];

    // 0 bit reads
    let mut r = BitReader::endian([255; 1].as_slice(), LittleEndian);
    assert_eq!(r.read_var::<u8>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u16>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u32>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u64>(0).unwrap(), 0);
    assert_eq!(r.read_var::<u8>(8).unwrap(), 255);

    let mut r = BitReader::endian([255; 1].as_slice(), LittleEndian);
    assert_eq!(r.read::<0, u8>().unwrap(), 0);
    assert_eq!(r.read::<0, u16>().unwrap(), 0);
    assert_eq!(r.read::<0, u32>().unwrap(), 0);
    assert_eq!(r.read::<0, u64>().unwrap(), 0);
    assert_eq!(r.read::<8, u8>().unwrap(), 255);

    let mut r = BitReader::endian([255; 1].as_slice(), LittleEndian);
    assert!(r.read_signed_var::<i8>(0).is_err());
    assert!(r.read_signed_var::<i16>(0).is_err());
    assert!(r.read_signed_var::<i32>(0).is_err());
    assert!(r.read_signed_var::<i64>(0).is_err());

    // unsigned 32 and 64-bit values
    let mut r = BitReader::endian(data.as_slice(), LittleEndian);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 0);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 4294967295);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 2147483648);
    assert_eq!(r.read_var::<u32>(32).unwrap(), 2147483647);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 0);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 9223372036854775808);
    assert_eq!(r.read_var::<u64>(64).unwrap(), 9223372036854775807);

    let mut r = BitReader::endian(data.as_slice(), LittleEndian);
    assert_eq!(r.read::<32, u32>().unwrap(), 0);
    assert_eq!(r.read::<32, u32>().unwrap(), 4294967295);
    assert_eq!(r.read::<32, u32>().unwrap(), 2147483648);
    assert_eq!(r.read::<32, u32>().unwrap(), 2147483647);
    assert_eq!(r.read::<64, u64>().unwrap(), 0);
    assert_eq!(r.read::<64, u64>().unwrap(), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(r.read::<64, u64>().unwrap(), 9223372036854775808);
    assert_eq!(r.read::<64, u64>().unwrap(), 9223372036854775807);

    let mut r = BitReader::endian(data.as_slice(), LittleEndian);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), 0);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), -1);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), -2147483648);
    assert_eq!(r.read_signed_var::<i32>(32).unwrap(), 2147483647);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), 0);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), -1);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), -9223372036854775808);
    assert_eq!(r.read_signed_var::<i64>(64).unwrap(), 9223372036854775807);

    let mut r = BitReader::endian(data.as_slice(), LittleEndian);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), 0);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), -1);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), -2147483648);
    assert_eq!(r.read_signed::<32, i32>().unwrap(), 2147483647);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), 0);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), -1);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), -9223372036854775808);
    assert_eq!(r.read_signed::<64, i64>().unwrap(), 9223372036854775807);
}

#[test]
fn test_reader_huffman_le() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BitRead, BitReader, LittleEndian};

    define_huffman_tree!(SomeTree : i32 = [[[4, 3], 2], [1, 0]]);

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);

    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 3);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 0);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 1);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 2);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 4);
    assert_eq!(r.read_huffman::<SomeTree>().unwrap(), 3);
}

#[test]
fn test_read_chunks_le() {
    use bitstream_io::{BitRead, BitReader, LittleEndian};

    let data: &[u8] = &[0b1011_0001, 0b1110_1101, 0b0011_1011, 0b1100_0001];
    let mut chunk: [u8; 2] = [0; 2];

    // test non-aligned chunk reading
    let mut r = BitReader::endian(data, LittleEndian);
    assert_eq!(r.read::<2, u8>().unwrap(), 0b01);
    r.read_bytes(&mut chunk).unwrap();
    assert_eq!(&chunk, &[0b01_1011_00, 0b11_1110_11]);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b1100_0001_0011_10);

    // test the smallest chunk
    let mut chunk = 0;
    let mut r = BitReader::endian(data, LittleEndian);
    assert_eq!(r.read::<2, u8>().unwrap(), 0b01);
    r.read_bytes(core::slice::from_mut(&mut chunk)).unwrap();
    assert_eq!(chunk, 0b01_1011_00);
    r.read_bytes(core::slice::from_mut(&mut chunk)).unwrap();
    assert_eq!(chunk, 0b11_1110_11);
    assert_eq!(r.read::<14, u16>().unwrap(), 0b1100_0001_0011_10);

    // test a larger chunk
    let data = include_bytes!("random.bin");
    let mut chunk: [u8; 127] = [0; 127];

    let mut r = BitReader::endian(data.as_slice(), LittleEndian);
    assert_eq!(r.read::<3, u8>().unwrap(), 0b010);
    r.read_bytes(&mut chunk).unwrap();
    assert_eq!(
        chunk.as_slice(),
        include_bytes!("random-3le.bin").as_slice()
    );
    assert_eq!(r.read::<5, u8>().unwrap(), 0b00110);
}

#[test]
fn test_reader_io_errors_be() {
    use bitstream_io::{BigEndian, BitRead, BitReader};
    use io::ErrorKind;

    let actual_data: [u8; 1] = [0xB1];

    // individual bits
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert_eq!(r.read_bit().unwrap_err().kind(), ErrorKind::UnexpectedEof);

    // skipping bits
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_var::<u32>(7).is_ok());
    assert_eq!(r.skip(5).unwrap_err().kind(), ErrorKind::UnexpectedEof);

    // signed values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_signed_var::<i32>(2).is_ok());
    assert!(r.read_signed_var::<i32>(3).is_ok());
    assert_eq!(
        r.read_signed_var::<i32>(5).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_signed::<2, i32>().is_ok());
    assert!(r.read_signed::<3, i32>().is_ok());
    assert_eq!(
        r.read_signed::<5, i32>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // unary 0 values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert_eq!(
        r.read_unary::<0>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // unary 1 values
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert_eq!(
        r.read_unary::<1>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // reading bytes, aligned
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    let mut sub_data = [0; 2];
    assert_eq!(
        r.read_bytes(&mut sub_data).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // reading bytes, un-aligned
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    let mut sub_data = [0; 2];
    assert!(r.read_var::<u32>(4).is_ok());
    assert_eq!(
        r.read_bytes(&mut sub_data).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );
}

#[test]
fn test_reader_io_errors_le() {
    use bitstream_io::{BitRead, BitReader, LittleEndian};
    use io::ErrorKind;

    let actual_data: [u8; 1] = [0xB1];

    // individual bits
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert!(r.read_bit().is_ok());
    assert_eq!(r.read_bit().unwrap_err().kind(), ErrorKind::UnexpectedEof);

    // skipping bits
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_var::<u32>(7).is_ok());
    assert_eq!(r.skip(5).unwrap_err().kind(), ErrorKind::UnexpectedEof);

    // signed values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_signed_var::<i32>(2).is_ok());
    assert!(r.read_signed_var::<i32>(3).is_ok());
    assert_eq!(
        r.read_signed_var::<i32>(5).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_signed::<2, i32>().is_ok());
    assert!(r.read_signed::<3, i32>().is_ok());
    assert_eq!(
        r.read_signed::<5, i32>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // unary 0 values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert!(r.read_unary::<0>().is_ok());
    assert_eq!(
        r.read_unary::<0>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // unary 1 values
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert!(r.read_unary::<1>().is_ok());
    assert_eq!(
        r.read_unary::<1>().unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // reading bytes, aligned
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    let mut sub_data = [0; 2];
    assert_eq!(
        r.read_bytes(&mut sub_data).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );

    // reading bytes, un-aligned
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    let mut sub_data = [0; 2];
    assert!(r.read_var::<u32>(4).is_ok());
    assert_eq!(
        r.read_bytes(&mut sub_data).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );
}

#[test]
fn test_reader_bits_errors() {
    use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
    use io::ErrorKind;
    let actual_data = [0u8; 10];

    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);

    assert_eq!(
        r.read_var::<u8>(9).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u16>(17).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u32>(33).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u64>(65).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    assert_eq!(
        r.read_signed_var::<i8>(9).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i16>(17).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i32>(33).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i64>(65).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);

    assert_eq!(
        r.read_var::<u8>(9).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u16>(17).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u32>(33).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_var::<u64>(65).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    assert_eq!(
        r.read_signed_var::<i8>(9).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i16>(17).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i32>(33).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        r.read_signed_var::<i64>(65).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
fn test_clone() {
    use bitstream_io::{BigEndian, BitRead, BitReader};

    // Reading unsigned examples, cloning while unaligned.
    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    assert!(r.byte_aligned());
    assert_eq!(r.read_var::<u32>(4).unwrap(), 0xB);
    let mut r2 = r.clone();
    assert!(!r.byte_aligned());
    assert_eq!(r.read_var::<u32>(4).unwrap(), 0x1);
    assert_eq!(r.read_var::<u32>(8).unwrap(), 0xED);
    assert!(!r2.byte_aligned());
    assert_eq!(r2.read_var::<u32>(4).unwrap(), 0x1);
    assert_eq!(r2.read_var::<u32>(8).unwrap(), 0xED);

    // Can still instantiate a BitReader when the backing std::io::Read is
    // !Clone.
    struct NotCloneRead<'a>(&'a [u8]);
    impl<'a> io::Read for NotCloneRead<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.0.read(buf)
        }
    }
    let _r = BitReader::endian(NotCloneRead(&actual_data[..]), BigEndian);
}

#[test]
fn test_read_bytes() {
    use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(actual_data.as_slice(), BigEndian);
    let read_data: [u8; 4] = r.read_to().unwrap();
    assert_eq!(actual_data, read_data);

    let actual_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    let mut r = BitReader::endian(actual_data.as_slice(), LittleEndian);
    let read_data: [u8; 4] = r.read_to().unwrap();
    assert_eq!(actual_data, read_data);
}

#[test]
fn test_large_reads() {
    use bitstream_io::{BigEndian, BitRead, BitReader, ByteRead, ByteReader, LittleEndian};

    for size in [0, 1, 4096, 4097, 10000] {
        let input = (0..255).cycle().take(size).collect::<Vec<u8>>();
        assert_eq!(input.len(), size);

        let mut r = BitReader::endian(input.as_slice(), BigEndian);
        let output = r.read_to_vec(size).unwrap();
        assert_eq!(input, output);

        let mut r = BitReader::endian(input.as_slice(), LittleEndian);
        let output = r.read_to_vec(size).unwrap();
        assert_eq!(input, output);

        let mut r = ByteReader::endian(input.as_slice(), BigEndian);
        let output = r.read_to_vec(size).unwrap();
        assert_eq!(input, output);

        let mut r = ByteReader::endian(input.as_slice(), LittleEndian);
        let output = r.read_to_vec(size).unwrap();
        assert_eq!(input, output);
    }

    let input: [u8; 5] = [0, 0, 0, 0, 0];

    let mut r = BitReader::endian(input.as_slice(), BigEndian);
    assert!(r.read_to_vec(usize::MAX).is_err());

    let mut r = BitReader::endian(input.as_slice(), LittleEndian);
    assert!(r.read_to_vec(usize::MAX).is_err());

    let mut r = ByteReader::endian(input.as_slice(), BigEndian);
    assert!(r.read_to_vec(usize::MAX).is_err());

    let mut r = ByteReader::endian(input.as_slice(), LittleEndian);
    assert!(r.read_to_vec(usize::MAX).is_err());
}

#[test]
fn test_bitcount_read() {
    use bitstream_io::{BigEndian, BitCount, BitRead, BitReader};

    // 1 bit count - reading 1 bit
    let bytes = &[0b1_1_000000];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b1>().unwrap();
    assert_eq!(count, BitCount::new::<1>());
    assert_eq!(reader.read_counted::<1, u8>(count).unwrap(), 0b1);

    // 2 bits count - reading 3 bits
    let bytes = &[0b11_111_000];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b11>().unwrap();
    assert_eq!(count, BitCount::new::<0b11>());
    assert_eq!(reader.read_counted::<0b11, u8>(count).unwrap(), 0b111);

    // 3 bits count - reading 7 bits
    let bytes = &[0b111_11111, 0b11_000000];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b111>().unwrap();
    assert_eq!(count, BitCount::new::<0b111>());
    assert_eq!(reader.read_counted::<0b111, u8>(count).unwrap(), 0b11111_11);

    // 4 bits count - reading 15 bits
    let bytes = &[0b1111_1111, 0b11111111, 0b111_00000];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b1111>().unwrap();
    assert_eq!(count, BitCount::new::<0b1111>());
    assert_eq!(
        reader.read_counted::<0b1111, u16>(count).unwrap(),
        0b1111_11111111_111
    );

    // 5 bits count - reading 31 bits
    let bytes = &[0b11111_111, 0b11111111, 0b11111111, 0b11111111, 0b1111_0000];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b11111>().unwrap();
    assert_eq!(count, BitCount::new::<0b11111>());
    assert_eq!(
        reader.read_counted::<0b11111, u32>(count).unwrap(),
        0b111_11111111_11111111_11111111_1111,
    );

    // 6 bits count - reading 63 bits
    let bytes = &[
        0b111111_11,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111_000,
    ];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b111111>().unwrap();
    assert_eq!(count, BitCount::new::<0b111111>());
    assert_eq!(
        reader.read_counted::<0b111111, u64>(count).unwrap(),
        0b11_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111,
    );

    // 7 bits count - reading 127 bits
    let bytes = &[
        0b1111111_1,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b11111111,
        0b111111_00,
    ];
    let mut reader = BitReader::endian(bytes.as_slice(), BigEndian);
    let count = reader.read_count::<0b1111111>().unwrap();
    assert_eq!(count, BitCount::new::<0b1111111>());
    assert_eq!(
        reader.read_counted::<0b1111111, u128>(count).unwrap(),
        0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
    );

    // We have to stop here because no primitive type gets any larger.
    // So while BitCount technically extends out to 32 bits,
    // it's not practical to get anywhere near that.
}

#[test]
fn test_nonzero_reads() {
    use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
    use core::num::NonZero;

    let data: &[u8] = &[0b001_00000, 0];

    assert_eq!(
        BitReader::endian(data, BigEndian).read::<3, u8>().unwrap(),
        1,
    );
    assert_eq!(
        BitReader::endian(data, BigEndian)
            .read::<3, NonZero<u8>>()
            .unwrap()
            .get(),
        2,
    );
    assert_eq!(
        BitReader::endian(data, BigEndian)
            .read_var::<u8>(3)
            .unwrap(),
        1,
    );
    assert_eq!(
        BitReader::endian(data, BigEndian)
            .read_var::<NonZero<u8>>(3)
            .unwrap()
            .get(),
        2,
    );
    assert!(BitReader::endian(data, BigEndian)
        .read_var::<NonZero<u8>>(8)
        .is_err());

    let data: &[u8] = &[0b00000_001, 0];

    assert_eq!(
        BitReader::endian(data, LittleEndian)
            .read::<3, u8>()
            .unwrap(),
        1,
    );
    assert_eq!(
        BitReader::endian(data, LittleEndian)
            .read::<3, NonZero<u8>>()
            .unwrap()
            .get(),
        2,
    );
    assert_eq!(
        BitReader::endian(data, LittleEndian)
            .read_var::<u8>(3)
            .unwrap(),
        1,
    );
    assert_eq!(
        BitReader::endian(data, LittleEndian)
            .read_var::<NonZero<u8>>(3)
            .unwrap()
            .get(),
        2,
    );
    assert!(BitReader::endian(data, LittleEndian)
        .read_var::<NonZero<u8>>(8)
        .is_err());
}
