// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bitstream_io::{BigEndian, BitWrite, BitWriter};
use std::io;

#[test]
fn test_writer_be() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // writing individual bits
    let mut w = BitWriter::endian(Vec::with_capacity(2), BigEndian);
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data[0..2]);

    // writing unsigned values
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    assert!(w.byte_aligned());
    w.write_var(2, 2u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 6u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 7u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 5u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x53BC1u32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    assert!(w.byte_aligned());
    w.write::<2, u32>(2).unwrap();
    assert!(!w.byte_aligned());
    w.write::<3, u32>(6).unwrap();
    assert!(!w.byte_aligned());
    w.write::<5, u32>(7).unwrap();
    assert!(!w.byte_aligned());
    w.write::<3, u32>(5).unwrap();
    assert!(!w.byte_aligned());
    w.write::<19, u32>(0x53BC1).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    assert!(w.byte_aligned());
    w.write::<1, u8>(1).unwrap();
    assert!(!w.byte_aligned());
    w.write::<14, u16>(0b0110_0011_1101_10).unwrap();
    assert!(!w.byte_aligned());
    w.write::<16, u16>(0b1001_1101_1110_0000).unwrap();
    assert!(!w.byte_aligned());
    w.write::<1, u8>(1).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing signed values
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_signed_var(2, -2).unwrap();
    w.write_signed_var(3, -2).unwrap();
    w.write_signed_var(5, 7).unwrap();
    w.write_signed_var(3, -3).unwrap();
    w.write_signed_var(19, -181311).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_signed::<2, i32>(-2).unwrap();
    w.write_signed::<3, i32>(-2).unwrap();
    w.write_signed::<5, i32>(7).unwrap();
    w.write_signed::<3, i32>(-3).unwrap();
    w.write_signed::<19, i32>(-181311).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing unary 0 values
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(1, 1u32).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing unary 1 values
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(5).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing unsigned vbr
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_unsigned_vbr::<4, _>(11u8).unwrap(); // 001 011 -> <1>011 <0>001
    w.write_unsigned_vbr::<4, _>(238u8).unwrap(); // 011 101 110 -> <1>110 <1>101 <0>011
    w.write_unsigned_vbr::<4, _>(99u8).unwrap(); // 001 100 011 -> <1>011 <1>100 <0>001
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing signed vbr
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_signed_vbr::<4, _>(-6i16).unwrap(); // 001 011 -> <1>011 <0>001
    w.write_signed_vbr::<4, _>(119i16).unwrap(); // 011 101 110 -> <1>110 <1>101 <0>011
    w.write_signed_vbr::<4, _>(-50i16).unwrap(); // 001 100 011 -> <1>011 <1>100 <0>001
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // byte aligning
    let aligned_data = [0xA0, 0xE0, 0x3B, 0xC0];
    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer().as_slice(), &aligned_data);

    // writing bytes, aligned
    let final_data = [0xB1, 0xED];
    let mut w = BitWriter::endian(Vec::with_capacity(2), BigEndian);
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing bytes, un-aligned
    let final_data = [0xBB, 0x1E, 0xD0];
    let mut w = BitWriter::endian(Vec::with_capacity(3), BigEndian);
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);
}

#[test]
fn test_writer_edge_cases_be() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter};

    // 0 bit writes
    let mut w = BitWriter::endian(Vec::new(), BigEndian);
    w.write_var(0, 0u8).unwrap();
    w.write_var(0, 0u16).unwrap();
    w.write_var(0, 0u32).unwrap();
    w.write_var(0, 0u64).unwrap();
    assert!(w.into_writer().is_empty());

    let mut w = BitWriter::endian(Vec::new(), BigEndian);
    assert!(w.write_signed_var(0, 0i8).is_err());
    assert!(w.write_signed_var(0, 0i16).is_err());
    assert!(w.write_signed_var(0, 0i32).is_err());
    assert!(w.write_signed_var(0, 0i64).is_err());
    assert!(w.into_writer().is_empty());

    let final_data: Vec<u8> = vec![
        0, 0, 0, 0, 255, 255, 255, 255, 128, 0, 0, 0, 127, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 128, 0, 0, 0, 0, 0, 0, 0, 127, 255, 255, 255, 255,
        255, 255, 255,
    ];

    // unsigned 32 and 64-bit values
    let mut w = BitWriter::endian(Vec::with_capacity(48), BigEndian);
    w.write_var(32, 0u32).unwrap();
    w.write_var(32, 4294967295u32).unwrap();
    w.write_var(32, 2147483648u32).unwrap();
    w.write_var(32, 2147483647u32).unwrap();
    w.write_var(64, 0u64).unwrap();
    w.write_var(64, 0xFFFFFFFFFFFFFFFFu64).unwrap();
    w.write_var(64, 9223372036854775808u64).unwrap();
    w.write_var(64, 9223372036854775807u64).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(48), BigEndian);
    w.write::<32, u32>(0).unwrap();
    w.write::<32, u32>(4294967295).unwrap();
    w.write::<32, u32>(2147483648).unwrap();
    w.write::<32, u32>(2147483647).unwrap();
    w.write::<64, u64>(0).unwrap();
    w.write::<64, u64>(0xFFFFFFFFFFFFFFFF).unwrap();
    w.write::<64, u64>(9223372036854775808).unwrap();
    w.write::<64, u64>(9223372036854775807).unwrap();
    assert_eq!(w.into_writer(), final_data);

    // signed 32 and 64-bit values
    let mut w = BitWriter::endian(Vec::with_capacity(48), BigEndian);
    w.write_signed_var(32, 0i32).unwrap();
    w.write_signed_var(32, -1i32).unwrap();
    w.write_signed_var(32, -2147483648i32).unwrap();
    w.write_signed_var(32, 2147483647i32).unwrap();
    w.write_signed_var(64, 0i64).unwrap();
    w.write_signed_var(64, -1i64).unwrap();
    w.write_signed_var(64, -9223372036854775808i64).unwrap();
    w.write_signed_var(64, 9223372036854775807i64).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(48), BigEndian);
    w.write_signed::<32, i32>(0).unwrap();
    w.write_signed::<32, i32>(-1).unwrap();
    w.write_signed::<32, i32>(-2147483648).unwrap();
    w.write_signed::<32, i32>(2147483647).unwrap();
    w.write_signed::<64, i64>(0).unwrap();
    w.write_signed::<64, i64>(-1).unwrap();
    w.write_signed::<64, i64>(-9223372036854775808).unwrap();
    w.write_signed::<64, i64>(9223372036854775807).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(8, core::i8::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i8::MAX.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(8, core::i8::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i8::MIN.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(16, core::i16::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i16::MAX.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(16, core::i16::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i16::MIN.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(32, core::i32::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i32::MAX.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(32, core::i32::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i32::MIN.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(64, core::i64::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i64::MAX.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(64, core::i64::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i64::MIN.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(128, core::i128::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i128::MAX.to_be_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, BigEndian)
            .write_signed_var(128, core::i128::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i128::MIN.to_be_bytes());
}

#[test]
fn test_writer_huffman_be() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BigEndian, BitWrite, BitWriter};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    define_huffman_tree!(TreeName : i32 = [[[4, 3], 2], [1, 0]]);

    let mut w = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);
}

#[test]
fn test_write_chunks_be() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter};

    let data: &[u8] = &[0b1011_0001, 0b1110_1101, 0b0011_1011, 0b1100_0001];

    // test non-aligned chunk writing
    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write::<2, u8>(0b10).unwrap();
    w.write_bytes(&[0b11_0001_11, 0b10_1101_00]).unwrap();
    w.write::<14, u16>(0b11_1011_1100_0001).unwrap();
    assert_eq!(w.into_writer().as_slice(), data);

    // test the smallest chunk
    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write::<2, u8>(0b10).unwrap();
    w.write_bytes(core::slice::from_ref(&0b11_0001_11)).unwrap();
    w.write_bytes(core::slice::from_ref(&0b10_1101_00)).unwrap();
    w.write::<14, u16>(0b11_1011_1100_0001).unwrap();
    assert_eq!(w.into_writer().as_slice(), data);

    // test a larger chunk
    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write::<3, u8>(0b000).unwrap();
    w.write_bytes(include_bytes!("random-3be.bin").as_slice())
        .unwrap();
    w.write::<5, u8>(0b10110).unwrap();
    assert_eq!(
        w.into_writer().as_slice(),
        include_bytes!("random.bin").as_slice()
    );
}

#[test]
fn test_writer_le() {
    use bitstream_io::{BitWrite, BitWriter, LittleEndian};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // writing individual bits
    let mut w = BitWriter::endian(Vec::with_capacity(2), LittleEndian);
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data[0..2]);

    // writing unsigned values
    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    assert!(w.byte_aligned());
    w.write_var(2, 1u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 4u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 13u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 3u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x609DFu32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    assert!(w.byte_aligned());
    w.write::<2, u32>(1).unwrap();
    assert!(!w.byte_aligned());
    w.write::<3, u32>(4).unwrap();
    assert!(!w.byte_aligned());
    w.write::<5, u32>(13).unwrap();
    assert!(!w.byte_aligned());
    w.write::<3, u32>(3).unwrap();
    assert!(!w.byte_aligned());
    w.write::<19, u32>(0x609DF).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    assert!(w.byte_aligned());
    w.write::<1, u8>(1).unwrap();
    assert!(!w.byte_aligned());
    w.write::<14, u16>(0b1101_1011_0110_00).unwrap();
    assert!(!w.byte_aligned());
    w.write::<16, u16>(0b1000_0010_0111_0111).unwrap();
    assert!(!w.byte_aligned());
    w.write::<1, u8>(1).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing signed values
    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_signed_var(2, 1).unwrap();
    w.write_signed_var(3, -4).unwrap();
    w.write_signed_var(5, 13).unwrap();
    w.write_signed_var(3, 3).unwrap();
    w.write_signed_var(19, -128545).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_signed::<2, i32>(1).unwrap();
    w.write_signed::<3, i32>(-4).unwrap();
    w.write_signed::<5, i32>(13).unwrap();
    w.write_signed::<3, i32>(3).unwrap();
    w.write_signed::<19, i32>(-128545).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing unary 0 values
    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(5).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(2, 3u32).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing unary 1 values
    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(5).unwrap();
    w.write_unary::<1>(0).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // byte aligning
    let aligned_data = [0x05, 0x07, 0x3B, 0x0C];
    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer().as_slice(), &aligned_data);

    // writing bytes, aligned
    let final_data = [0xB1, 0xED];
    let mut w = BitWriter::endian(Vec::with_capacity(2), LittleEndian);
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);

    // writing bytes, un-aligned
    let final_data = [0x1B, 0xDB, 0x0E];
    let mut w = BitWriter::endian(Vec::with_capacity(3), LittleEndian);
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);
}

#[test]
fn test_writer_edge_cases_le() {
    use bitstream_io::{BitWrite, BitWriter, LittleEndian};

    // 0 bit writes
    let mut w = BitWriter::endian(Vec::new(), LittleEndian);
    w.write_var(0, 0u8).unwrap();
    w.write_var(0, 0u16).unwrap();
    w.write_var(0, 0u32).unwrap();
    w.write_var(0, 0u64).unwrap();
    assert!(w.into_writer().is_empty());

    let mut w = BitWriter::endian(Vec::new(), LittleEndian);
    assert!(w.write_signed_var(0, 0i8).is_err());
    assert!(w.write_signed_var(0, 0i16).is_err());
    assert!(w.write_signed_var(0, 0i32).is_err());
    assert!(w.write_signed_var(0, 0i64).is_err());
    assert!(w.into_writer().is_empty());

    let final_data: Vec<u8> = vec![
        0, 0, 0, 0, 255, 255, 255, 255, 0, 0, 0, 128, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0,
        255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 255, 255,
        255, 255, 127,
    ];

    // unsigned 32 and 64-bit values
    let mut w = BitWriter::endian(Vec::with_capacity(48), LittleEndian);
    w.write_var(32, 0u32).unwrap();
    w.write_var(32, 4294967295u32).unwrap();
    w.write_var(32, 2147483648u32).unwrap();
    w.write_var(32, 2147483647u32).unwrap();
    w.write_var(64, 0u64).unwrap();
    w.write_var(64, 0xFFFFFFFFFFFFFFFFu64).unwrap();
    w.write_var(64, 9223372036854775808u64).unwrap();
    w.write_var(64, 9223372036854775807u64).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(48), LittleEndian);
    w.write::<32, u32>(0).unwrap();
    w.write::<32, u32>(4294967295).unwrap();
    w.write::<32, u32>(2147483648).unwrap();
    w.write::<32, u32>(2147483647).unwrap();
    w.write::<64, u64>(0).unwrap();
    w.write::<64, u64>(0xFFFFFFFFFFFFFFFF).unwrap();
    w.write::<64, u64>(9223372036854775808).unwrap();
    w.write::<64, u64>(9223372036854775807).unwrap();
    assert_eq!(w.into_writer(), final_data);

    // signed 32 and 64-bit values
    let mut w = BitWriter::endian(Vec::with_capacity(48), LittleEndian);
    w.write_signed_var(32, 0i32).unwrap();
    w.write_signed_var(32, -1i32).unwrap();
    w.write_signed_var(32, -2147483648i32).unwrap();
    w.write_signed_var(32, 2147483647i32).unwrap();
    w.write_signed_var(64, 0i64).unwrap();
    w.write_signed_var(64, -1i64).unwrap();
    w.write_signed_var(64, -9223372036854775808i64).unwrap();
    w.write_signed_var(64, 9223372036854775807i64).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut w = BitWriter::endian(Vec::with_capacity(48), LittleEndian);
    w.write_signed::<32, i32>(0).unwrap();
    w.write_signed::<32, i32>(-1).unwrap();
    w.write_signed::<32, i32>(-2147483648).unwrap();
    w.write_signed::<32, i32>(2147483647).unwrap();
    w.write_signed::<64, i64>(0).unwrap();
    w.write_signed::<64, i64>(-1).unwrap();
    w.write_signed::<64, i64>(-9223372036854775808).unwrap();
    w.write_signed::<64, i64>(9223372036854775807).unwrap();
    assert_eq!(w.into_writer(), final_data);

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(8, core::i8::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i8::MAX.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(8, core::i8::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i8::MIN.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(16, core::i16::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i16::MAX.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(16, core::i16::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i16::MIN.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(32, core::i32::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i32::MAX.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(32, core::i32::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i32::MIN.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(64, core::i64::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i64::MAX.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(64, core::i64::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i64::MIN.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(128, core::i128::MAX)
            .unwrap();
    }
    assert_eq!(bytes, core::i128::MAX.to_le_bytes());

    let mut bytes = Vec::new();
    {
        BitWriter::endian(&mut bytes, LittleEndian)
            .write_signed_var(128, core::i128::MIN)
            .unwrap();
    }
    assert_eq!(bytes, core::i128::MIN.to_le_bytes());
}

#[test]
fn test_writer_huffman_le() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BitWrite, BitWriter, LittleEndian};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    define_huffman_tree!(TreeName : i32= [[[4, 3], 2], [1, 0]]);

    let mut w = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write_var(1, 1u8).unwrap();
    assert_eq!(w.into_writer().as_slice(), &final_data);
}

#[test]
fn test_write_chunks_le() {
    use bitstream_io::{BitWrite, BitWriter, LittleEndian};

    let data: &[u8] = &[0b1011_0001, 0b1110_1101, 0b0011_1011, 0b1100_0001];

    // test non-aligned chunk writing
    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write::<2, u8>(0b01).unwrap();
    w.write_bytes(&[0b01_1011_00, 0b11_1110_11]).unwrap();
    w.write::<14, u16>(0b1100_0001_0011_10).unwrap();
    assert_eq!(w.into_writer().as_slice(), data);

    // test the smallest chunk
    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write::<2, u8>(0b01).unwrap();
    w.write_bytes(core::slice::from_ref(&0b01_1011_00)).unwrap();
    w.write_bytes(core::slice::from_ref(&0b11_1110_11)).unwrap();
    w.write::<14, u16>(0b1100_0001_0011_10).unwrap();
    assert_eq!(w.into_writer().as_slice(), data);

    // test a larger chunk
    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write::<3, u8>(0b010).unwrap();
    w.write_bytes(include_bytes!("random-3le.bin").as_slice())
        .unwrap();
    w.write::<5, u8>(0b00110).unwrap();
    assert_eq!(
        w.into_writer().as_slice(),
        include_bytes!("random.bin").as_slice()
    );
}

struct LimitedWriter {
    can_write: usize,
}

impl LimitedWriter {
    fn new(max_bytes: usize) -> LimitedWriter {
        LimitedWriter {
            can_write: max_bytes,
        }
    }
}

impl io::Write for LimitedWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let to_write = buf.len().min(self.can_write);
        self.can_write -= to_write;
        Ok(to_write)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

#[test]
fn test_writer_io_errors_be() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter};
    use io::ErrorKind;

    // individual bits
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert_eq!(w.write_bit(true).unwrap_err().kind(), ErrorKind::WriteZero);

    // unsigned values
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_var(2, 2u32).is_ok());
    assert!(w.write_var(3, 6u32).is_ok());
    assert!(w.write_var(5, 7u32).is_ok());
    assert!(w.write_var(3, 5u32).is_ok());
    assert_eq!(
        w.write_var(19, 0x53BC1u32).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // signed values
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_signed_var(2, -2).is_ok());
    assert!(w.write_signed_var(3, -2).is_ok());
    assert!(w.write_signed_var(5, 7).is_ok());
    assert!(w.write_signed_var(3, -3).is_ok());
    assert_eq!(
        w.write_signed_var(19, -181311).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // unary 0 values
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_unary::<0>(1).is_ok());
    assert!(w.write_unary::<0>(2).is_ok());
    assert!(w.write_unary::<0>(0).is_ok());
    assert!(w.write_unary::<0>(0).is_ok());
    assert!(w.write_unary::<0>(4).is_ok());
    assert!(w.write_unary::<0>(2).is_ok());
    assert_eq!(
        w.write_unary::<0>(1).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // unary 1 values
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(1).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(3).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(1).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert_eq!(
        w.write_unary::<1>(1).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // byte aligning
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_var::<u16>(9, 0b111111111).is_ok());
    assert_eq!(w.byte_align().unwrap_err().kind(), ErrorKind::WriteZero);

    // aligned bytes
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert_eq!(
        w.write_bytes(b"\xB1\xED").unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // un-aligned bytes
    let mut w = BitWriter::endian(LimitedWriter::new(1), BigEndian);
    assert!(w.write_var(4, 11u8).is_ok());
    assert_eq!(
        w.write_bytes(b"\xB1\xED").unwrap_err().kind(),
        ErrorKind::WriteZero
    );
}

#[test]
fn test_writer_io_errors_le() {
    use bitstream_io::{BitWrite, BitWriter, LittleEndian};
    use io::ErrorKind;

    // individual bits
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(false).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert!(w.write_bit(true).is_ok());
    assert_eq!(w.write_bit(true).unwrap_err().kind(), ErrorKind::WriteZero);

    // unsigned values
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_var(2, 1u32).is_ok());
    assert!(w.write_var(3, 4u32).is_ok());
    assert!(w.write_var(5, 13u32).is_ok());
    assert!(w.write_var(3, 3u32).is_ok());
    assert_eq!(
        w.write_var(19, 0x609DFu32).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // signed values
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_signed_var(2, 1).is_ok());
    assert!(w.write_signed_var(3, -4).is_ok());
    assert!(w.write_signed_var(5, 13).is_ok());
    assert!(w.write_signed_var(3, 3).is_ok());
    assert_eq!(
        w.write_signed_var(19, -128545).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // unary 0 values
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_unary::<0>(1).is_ok());
    assert!(w.write_unary::<0>(0).is_ok());
    assert!(w.write_unary::<0>(0).is_ok());
    assert!(w.write_unary::<0>(2).is_ok());
    assert!(w.write_unary::<0>(2).is_ok());
    assert!(w.write_unary::<0>(2).is_ok());
    assert_eq!(
        w.write_unary::<0>(5).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // unary 1 values
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(3).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(1).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(1).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert!(w.write_unary::<1>(1).is_ok());
    assert!(w.write_unary::<1>(0).is_ok());
    assert_eq!(
        w.write_unary::<1>(1).unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // byte aligning
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_var::<u16>(9, 0b111111111).is_ok());
    assert_eq!(w.byte_align().unwrap_err().kind(), ErrorKind::WriteZero);

    // aligned bytes
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert_eq!(
        w.write_bytes(b"\xB1\xED").unwrap_err().kind(),
        ErrorKind::WriteZero
    );

    // un-aligned bytes
    let mut w = BitWriter::endian(LimitedWriter::new(1), LittleEndian);
    assert!(w.write_var(4, 11u8).is_ok());
    assert_eq!(
        w.write_bytes(b"\xB1\xED").unwrap_err().kind(),
        ErrorKind::WriteZero
    );
}

#[test]
fn test_writer_bits_errors() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter, LittleEndian};
    use io::{sink, ErrorKind};

    let mut w = BitWriter::endian(sink(), BigEndian);
    assert_eq!(
        w.write_var(9, 0u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(17, 0u16).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(33, 0u32).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(65, 0u64).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    assert_eq!(
        w.write_var(1, 0b10u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(2, 0b100u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(3, 0b1000u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    for bits in 1..8 {
        let val = 1u8 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 8..16 {
        let val = 1u16 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 16..32 {
        let val = 1u32 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 32..64 {
        let val = 1u64 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }

    assert_eq!(
        w.write_signed_var(9, 0i8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(17, 0i16).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(33, 0i32).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(65, 0i64).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    let mut w = BitWriter::endian(sink(), LittleEndian);
    assert_eq!(
        w.write_var(9, 0u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(17, 0u16).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(33, 0u32).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(65, 0u64).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    assert_eq!(
        w.write_var(1, 0b10u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(2, 0b100u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_var(3, 0b1000u8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    for bits in 1..8 {
        let val = 1u8 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 8..16 {
        let val = 1u16 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 16..32 {
        let val = 1u32 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }
    for bits in 32..64 {
        let val = 1u64 << bits;
        assert_eq!(
            w.write_var(bits, val).unwrap_err().kind(),
            ErrorKind::InvalidInput
        );
    }

    assert_eq!(
        w.write_signed_var(9, 0i8).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(17, 0i16).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(33, 0i32).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        w.write_signed_var(65, 0i64).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
}

#[test]
fn test_counter_be() {
    use bitstream_io::{BitWrite, BitsWritten};

    // writing individual bits
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.written(), 16);

    // writing unsigned values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    assert!(w.byte_aligned());
    w.write_var(2, 2u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 6u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 7u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 5u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x53BC1u32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.written(), 32);

    // writing signed values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_signed_var(2, -2).unwrap();
    w.write_signed_var(3, -2).unwrap();
    w.write_signed_var(5, 7).unwrap();
    w.write_signed_var(3, -3).unwrap();
    w.write_signed_var(19, -181311).unwrap();
    assert_eq!(w.written(), 32);

    // writing unary 0 values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(1, 1u32).unwrap();
    assert_eq!(w.written(), 32);

    // writing unary 1 values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(5).unwrap();
    assert_eq!(w.written(), 32);

    // byte aligning
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 32);

    // writing bytes, aligned
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.written(), 16);

    // writing bytes, un-aligned
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 24);
}

#[test]
fn test_counter_huffman_be() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BitWrite, BitsWritten};

    define_huffman_tree!(TreeName : i32 = [[[4, 3], 2], [1, 0]]);

    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 32);
}

#[test]
fn test_counter_le() {
    use bitstream_io::{BitWrite, BitsWritten};

    // writing individual bits
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.written(), 16);

    // writing unsigned values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    assert!(w.byte_aligned());
    w.write_var(2, 1u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 4u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 13u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 3u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x609DFu32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.written(), 32);

    // writing signed values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_signed_var(2, 1).unwrap();
    w.write_signed_var(3, -4).unwrap();
    w.write_signed_var(5, 13).unwrap();
    w.write_signed_var(3, 3).unwrap();
    w.write_signed_var(19, -128545).unwrap();
    assert_eq!(w.written(), 32);

    // writing unary 0 values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(5).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(2, 3u32).unwrap();
    assert_eq!(w.written(), 32);

    // writing unary 1 values
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(5).unwrap();
    w.write_unary::<1>(0).unwrap();
    assert_eq!(w.written(), 32);

    // byte aligning
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 32);

    // writing bytes, aligned
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.written(), 16);

    // writing bytes, un-aligned
    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 24);
}

#[test]
fn test_counter_huffman_le() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BitWrite, BitsWritten};

    define_huffman_tree!(TreeName : i32= [[[4, 3], 2], [1, 0]]);

    let mut w: BitsWritten<u32> = BitsWritten::new();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write::<1, u8>(1).unwrap();
    assert_eq!(w.written(), 32);
}

#[test]
fn test_recorder_be() {
    use bitstream_io::{BigEndian, BitRecorder, BitWrite, BitWriter};

    // partial writes
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_bit(true).unwrap();

    let mut w2 = BitWriter::endian(vec![], BigEndian);
    w.playback(&mut w2).unwrap();
    w2.byte_align().unwrap();

    let mut w3 = BitWriter::endian(vec![], BigEndian);
    w3.write_bit(true).unwrap();
    w3.byte_align().unwrap();

    assert_eq!(w2.into_writer(), w3.into_writer());

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // writing individual bits
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.written(), 16);
    let mut w2 = BitWriter::endian(Vec::with_capacity(2), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data[0..2]);

    // writing unsigned values
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    assert!(w.byte_aligned());
    w.write_var(2, 2u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 6u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 7u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 5u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x53BC1u32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing signed values
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_signed_var(2, -2).unwrap();
    w.write_signed_var(3, -2).unwrap();
    w.write_signed_var(5, 7).unwrap();
    w.write_signed_var(3, -3).unwrap();
    w.write_signed_var(19, -181311).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing unary 0 values
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(4).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(1, 1u32).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing unary 1 values
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(5).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // byte aligning
    let aligned_data = [0xA0, 0xE0, 0x3B, 0xC0];
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &aligned_data);

    // writing bytes, aligned
    let final_data = [0xB1, 0xED];
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.written(), 16);
    let mut w2 = BitWriter::endian(Vec::with_capacity(2), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing bytes, un-aligned
    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    let final_data = [0xBB, 0x1E, 0xD0];
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    let mut w2 = BitWriter::endian(Vec::with_capacity(3), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);
}

#[test]
fn test_recorder_huffman_be() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BigEndian, BitRecorder, BitWrite, BitWriter};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];
    define_huffman_tree!(TreeName : i32 = [[[4, 3], 2], [1, 0]]);

    let mut w: BitRecorder<u32, BigEndian> = BitRecorder::new();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.byte_align().unwrap();
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), BigEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);
}

#[test]
fn test_recorder_le() {
    use bitstream_io::{BitRecorder, BitWrite, BitWriter, LittleEndian};

    // partial writes
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_bit(true).unwrap();

    let mut w2 = BitWriter::endian(vec![], LittleEndian);
    w.playback(&mut w2).unwrap();
    w2.byte_align().unwrap();

    let mut w3 = BitWriter::endian(vec![], LittleEndian);
    w3.write_bit(true).unwrap();
    w3.byte_align().unwrap();

    assert_eq!(w2.into_writer(), w3.into_writer());

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    // writing individual bits
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(false).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    w.write_bit(true).unwrap();
    assert_eq!(w.written(), 16);
    let mut w2 = BitWriter::endian(Vec::with_capacity(2), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data[0..2]);

    // writing unsigned values
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    assert!(w.byte_aligned());
    w.write_var(2, 1u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 4u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(5, 13u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(3, 3u32).unwrap();
    assert!(!w.byte_aligned());
    w.write_var(19, 0x609DFu32).unwrap();
    assert!(w.byte_aligned());
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing signed values
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_signed_var(2, 1).unwrap();
    w.write_signed_var(3, -4).unwrap();
    w.write_signed_var(5, 13).unwrap();
    w.write_signed_var(3, 3).unwrap();
    w.write_signed_var(19, -128545).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing unary 0 values
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(2).unwrap();
    w.write_unary::<0>(5).unwrap();
    w.write_unary::<0>(3).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(1).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_unary::<0>(0).unwrap();
    w.write_var(2, 3u32).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing unary 1 values
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(3).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(1).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(0).unwrap();
    w.write_unary::<1>(2).unwrap();
    w.write_unary::<1>(5).unwrap();
    w.write_unary::<1>(0).unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // byte aligning
    let aligned_data = [0x05, 0x07, 0x3B, 0x0C];
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_var(3, 5u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(3, 7u32).unwrap();
    w.byte_align().unwrap();
    w.byte_align().unwrap();
    w.write_var(8, 59u32).unwrap();
    w.byte_align().unwrap();
    w.write_var(4, 12u32).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 32);
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &aligned_data);

    // writing bytes, aligned
    let final_data = [0xB1, 0xED];
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_bytes(b"\xB1\xED").unwrap();
    assert_eq!(w.written(), 16);
    let mut w2 = BitWriter::endian(Vec::with_capacity(2), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);

    // writing bytes, un-aligned
    let final_data = [0x1B, 0xDB, 0x0E];
    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_var(4, 11u32).unwrap();
    w.write_bytes(b"\xB1\xED").unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.written(), 24);
    let mut w2 = BitWriter::endian(Vec::with_capacity(3), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);
}

#[test]
fn test_recorder_huffman_le() {
    use bitstream_io::define_huffman_tree;
    use bitstream_io::{BitRecorder, BitWrite, BitWriter, LittleEndian};

    let final_data: [u8; 4] = [0xB1, 0xED, 0x3B, 0xC1];

    define_huffman_tree!(TreeName : i32 = [[[4, 3], 2], [1, 0]]);

    let mut w: BitRecorder<u32, LittleEndian> = BitRecorder::new();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(0).unwrap();
    w.write_huffman::<TreeName>(1).unwrap();
    w.write_huffman::<TreeName>(2).unwrap();
    w.write_huffman::<TreeName>(4).unwrap();
    w.write_huffman::<TreeName>(3).unwrap();
    w.write::<1, u8>(1).unwrap();
    let mut w2 = BitWriter::endian(Vec::with_capacity(4), LittleEndian);
    w.playback(&mut w2).unwrap();
    assert_eq!(w2.into_writer().as_slice(), &final_data);
}

#[test]
fn test_pad() {
    use bitstream_io::{BigEndian, Endianness, LittleEndian};

    fn test_pad_endian<E: Endianness>() {
        use bitstream_io::BitWriter;

        let mut plain: BitWriter<_, E> = BitWriter::new(Vec::new());
        let mut padded: BitWriter<_, E> = BitWriter::new(Vec::new());

        for bits in 1..64 {
            plain.write_bit(true).unwrap();
            plain.write_var(bits, 0u64).unwrap();

            padded.write_bit(true).unwrap();
            padded.pad(bits).unwrap();
        }
        // plain.write_from([0u8; 1024]).unwrap();
        // plain.write_bit(true).unwrap();

        // padded.pad(1024 * 8).unwrap();
        // padded.write_bit(true).unwrap();

        plain.byte_align().unwrap();
        padded.byte_align().unwrap();

        assert_eq!(plain.into_writer(), padded.into_writer());
    }

    test_pad_endian::<BigEndian>();
    test_pad_endian::<LittleEndian>();
}

#[test]
fn test_counter_overflow() {
    use bitstream_io::BitsWritten;

    // overflow u8 with many small writes
    let mut counter: BitsWritten<u8> = BitsWritten::new();
    for _ in 0..255 {
        assert!(counter.write_bit(false).is_ok());
    }
    assert!(counter.write_bit(false).is_err());

    // overflow u8 with one big write
    let mut counter: BitsWritten<u8> = BitsWritten::new();
    assert!(counter.write_from([0u8; 31]).is_ok());

    let mut counter: BitsWritten<u8> = BitsWritten::new();
    assert!(counter.write_from([0u8; 32]).is_err());
}

#[test]
fn test_negative_write() {
    let mut bit_writer = BitWriter::endian(Vec::new(), BigEndian);
    assert!(bit_writer.write_bit(false).is_ok());
    assert!(bit_writer.write_var(8, -1i8).is_ok());
    assert!(bit_writer.write_var(7, 0u8).is_ok());
    if let Some(writer) = bit_writer.writer() {
        assert_eq!(writer[0] >> 7, 0);
    } else {
        panic!("writer() returned None");
    }
}

#[test]
fn test_bitcount_write() {
    use bitstream_io::{BigEndian, BitCount, BitWrite, BitWriter};

    // 1 bit count - writing 1 bit
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b1>::new::<1>();
    writer.write_count(count).unwrap();
    writer.write_counted::<1, u8>(count, 0b1).unwrap();
    writer.byte_align().unwrap();
    assert_eq!(writer.into_writer(), &[0b1_1_000000]);

    // 2 bit count - writing 3 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b11>::new::<0b11>();
    writer.write_count(count).unwrap();
    writer.write_counted::<0b11, u8>(count, 0b111).unwrap();
    writer.byte_align().unwrap();
    assert_eq!(writer.into_writer(), &[0b11_111_000]);

    // 3 bit count - writing 7 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b111>::new::<0b111>();
    writer.write_count(count).unwrap();
    writer
        .write_counted::<0b111, u8>(count, 0b11111_11)
        .unwrap();
    writer.byte_align().unwrap();
    assert_eq!(writer.into_writer(), &[0b111_11111, 0b11_000000]);

    // 4 bit count - writing 15 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b1111>::new::<0b1111>();
    writer.write_count(count).unwrap();
    writer
        .write_counted::<0b1111, u16>(count, 0b1111_11111111_111)
        .unwrap();
    writer.byte_align().unwrap();
    assert_eq!(
        writer.into_writer(),
        &[0b1111_1111, 0b11111111, 0b111_00000]
    );

    // 5 bits count - writing 31 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b11111>::new::<0b11111>();
    writer.write_count(count).unwrap();
    writer
        .write_counted::<0b11111, u32>(count, 0b111_11111111_11111111_11111111_1111)
        .unwrap();
    writer.byte_align().unwrap();
    assert_eq!(
        writer.into_writer(),
        &[0b11111_111, 0b11111111, 0b11111111, 0b11111111, 0b1111_0000]
    );

    // 6 bits count - writing 63 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b111111>::new::<0b111111>();
    writer.write_count(count).unwrap();
    writer
        .write_counted::<0b111111, u64>(
            count,
            0b11_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111,
        )
        .unwrap();
    writer.byte_align().unwrap();
    assert_eq!(
        writer.into_writer(),
        &[
            0b111111_11,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111_000
        ]
    );

    // 7 bits count - writing 127 bits
    let bytes = vec![];
    let mut writer = BitWriter::endian(bytes, BigEndian);
    let count = BitCount::<0b1111111>::new::<0b1111111>();
    writer.write_count(count).unwrap();
    writer
        .write_counted::<0b1111111, u128>(count, 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
        .unwrap();
    writer.byte_align().unwrap();
    assert_eq!(
        writer.into_writer(),
        &[
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
        ]
    );
}

#[test]
fn test_nonzero_writes() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter, LittleEndian};
    use core::num::NonZero;

    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write::<3, u8>(1).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b001_00000]);

    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write::<3, NonZero<u8>>(NonZero::new(2).unwrap()).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b001_00000]);

    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write_var::<u8>(3, 1).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b001_00000]);

    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write_var::<NonZero<u8>>(3, NonZero::new(2).unwrap())
        .unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b001_00000]);

    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write::<3, u8>(1).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b00000_001]);

    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write::<3, NonZero<u8>>(NonZero::new(2).unwrap()).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b00000_001]);

    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write_var::<u8>(3, 1).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b00000_001]);

    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write_var::<NonZero<u8>>(3, NonZero::new(2).unwrap())
        .unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), &[0b00000_001]);
}

#[test]
fn test_const_writes() {
    use bitstream_io::{BigEndian, BitWrite, BitWriter, LittleEndian};

    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write_const::<0, 0b0>().unwrap();
    w.write_const::<1, 0b1>().unwrap();
    w.write_const::<2, 0b10>().unwrap();
    w.write_const::<3, 0b100>().unwrap();
    w.write_const::<4, 0b1000>().unwrap();
    w.write_const::<5, 0b10000>().unwrap();
    w.write_const::<6, 0b100000>().unwrap();
    w.write_const::<7, 0b1000000>().unwrap();
    w.write_const::<8, 0b10000000>().unwrap();
    w.write_const::<9, 0b100000000>().unwrap();
    w.write_const::<10, 0b1000000000>().unwrap();
    w.byte_align().unwrap();

    assert_eq!(
        w.into_writer(),
        &[
            0b1_10_100_10,
            0b00_100001,
            0b00000_100,
            0b0000_1000,
            0b0000_1000,
            0b00000_100,
            0b00000000
        ]
    );

    let mut w = BitWriter::endian(vec![], LittleEndian);
    w.write_const::<0, 0b0>().unwrap();
    w.write_const::<1, 0b1>().unwrap();
    w.write_const::<2, 0b10>().unwrap();
    w.write_const::<3, 0b100>().unwrap();
    w.write_const::<4, 0b1000>().unwrap();
    w.write_const::<5, 0b10000>().unwrap();
    w.write_const::<6, 0b100000>().unwrap();
    w.write_const::<7, 0b1000000>().unwrap();
    w.write_const::<8, 0b10000000>().unwrap();
    w.write_const::<9, 0b100000000>().unwrap();
    w.write_const::<10, 0b1000000000>().unwrap();
    w.byte_align().unwrap();

    assert_eq!(
        w.into_writer(),
        &[
            0b00_100_10_1,
            0b0_10000_10,
            0b000_10000,
            0b0000_1000,
            0b0000_1000,
            0b000_10000,
            0b0_1000000
        ]
    );
}

#[test]
fn test_byte_count() {
    use bitstream_io::{ByteWrite, ToByteStream};

    #[derive(Default)]
    struct Builder {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        e: u128,
    }

    impl ToByteStream for Builder {
        type Error = io::Error;

        fn to_writer<W: ByteWrite + ?Sized>(&self, w: &mut W) -> io::Result<()> {
            w.write(self.a)?;
            w.write(self.b)?;
            w.write(self.c)?;
            w.write(self.d)?;
            w.write(self.e)?;
            Ok(())
        }
    }

    assert_eq!(
        Builder::default().bytes::<u32>().unwrap(),
        1 + 2 + 4 + 8 + 16
    );
}
