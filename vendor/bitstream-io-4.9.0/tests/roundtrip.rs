// Copyright 2017 Brian Langenberger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bitstream_io::{
    BigEndian, BitRead, BitReader, BitWrite, BitWriter, Endianness, Integer, LittleEndian,
    Primitive,
};

fn roundtrip<E: Endianness>() {
    /*unsigned values*/
    for bits in 1..17 {
        let max = 1 << bits;
        let mut output: Vec<u8> = Vec::with_capacity(max);
        {
            let mut writer = BitWriter::<_, E>::new(&mut output);
            for value in 0..max {
                writer.write_var(bits, value as u32).unwrap();
            }
            writer.byte_align().unwrap();
        }
        {
            let mut reader = BitReader::<_, E>::new(output.as_slice());
            for value in 0..max {
                assert_eq!(reader.read_var::<u32>(bits).unwrap(), value as u32);
            }
        }
    }

    /*signed values*/
    for bits in 2..17 {
        let min = -1i32 << (bits - 1);
        let max = 1i32 << (bits - 1);
        let mut output: Vec<u8> = Vec::with_capacity(max as usize);
        {
            let mut writer = BitWriter::<_, E>::new(&mut output);
            for value in min..max {
                writer.write_signed_var(bits, value as i32).unwrap();
            }
            writer.byte_align().unwrap();
        }
        {
            let mut reader = BitReader::<_, E>::new(output.as_slice());
            for value in min..max {
                assert_eq!(reader.read_signed_var::<i32>(bits).unwrap(), value as i32);
            }
        }
    }
}

#[test]
fn test_rountrip_be() {
    roundtrip::<BigEndian>();
}

#[test]
fn test_roundtrip_le() {
    roundtrip::<LittleEndian>();
}

fn wide_roundtrip<const BITS: u32, E, I>(start: I, end: I, increment: I)
where
    E: Endianness,
    I: Integer
        + Copy
        + std::fmt::Debug
        + std::ops::AddAssign
        + std::cmp::PartialEq
        + std::cmp::PartialOrd,
{
    let mut w = BitWriter::<Vec<u8>, E>::new(vec![]);
    // add an extra bit to check non byte-aligned values
    w.write_bit(true).unwrap();
    let mut v = start;
    while v < end {
        w.write::<BITS, I>(v).unwrap();
        v += increment;
    }
    w.byte_align().unwrap();

    let vec = w.into_writer();
    let mut r = BitReader::<_, E>::new(vec.as_slice());
    assert_eq!(r.read_bit().unwrap(), true);
    let mut v = start;
    while v < end {
        assert_eq!(r.read::<BITS, I>().unwrap(), v);
        v += increment;
    }
}

#[test]
fn test_roundtrip_u7_be() {
    wide_roundtrip::<7, BigEndian, u8>(u8::MIN, u8::MAX / 2, 1);
}

#[test]
fn test_roundtrip_u7_le() {
    wide_roundtrip::<7, LittleEndian, u8>(u8::MIN, u8::MAX / 2, 1);
}

#[test]
fn test_roundtrip_u8_be() {
    wide_roundtrip::<8, BigEndian, u8>(u8::MIN, u8::MAX, 1);
}

#[test]
fn test_roundtrip_u8_le() {
    wide_roundtrip::<8, LittleEndian, u8>(u8::MIN, u8::MAX, 1);
}

#[test]
fn test_roundtrip_i8_be() {
    wide_roundtrip::<8, BigEndian, i8>(i8::MIN, i8::MAX, 1);
}

#[test]
fn test_roundtrip_i8_le() {
    wide_roundtrip::<8, LittleEndian, i8>(i8::MIN, i8::MAX, 1);
}

#[test]
fn test_roundtrip_u9_be() {
    wide_roundtrip::<9, BigEndian, u16>(u16::MIN, u8::MAX as u16 * 2, 1);
}

#[test]
fn test_roundtrip_u9_le() {
    wide_roundtrip::<9, LittleEndian, u16>(u16::MIN, u8::MAX as u16 * 2, 1);
}

#[test]
fn test_roundtrip_u15_be() {
    wide_roundtrip::<15, BigEndian, u16>(u16::MIN, u16::MAX / 2, 1);
}

#[test]
fn test_roundtrip_u15_le() {
    wide_roundtrip::<15, LittleEndian, u16>(u16::MIN, u16::MAX / 2, 1);
}

#[test]
fn test_roundtrip_u16_be() {
    wide_roundtrip::<16, BigEndian, u16>(u16::MIN, u16::MAX, 1);
}

#[test]
fn test_roundtrip_u16_le() {
    wide_roundtrip::<16, LittleEndian, u16>(u16::MIN, u16::MAX, 1);
}

#[test]
fn test_roundtrip_i16_be() {
    wide_roundtrip::<16, BigEndian, i16>(i16::MIN, i16::MAX, 1);
}

#[test]
fn test_roundtrip_i16_le() {
    wide_roundtrip::<16, LittleEndian, i16>(i16::MIN, i16::MAX, 1);
}

#[test]
fn test_roundtrip_u32_be() {
    wide_roundtrip::<32, BigEndian, u32>(u32::MIN, u32::MAX - 65536, 65536);
}

#[test]
fn test_roundtrip_u32_le() {
    wide_roundtrip::<32, LittleEndian, u32>(u32::MIN, u32::MAX - 65536, 65536);
}

#[test]
fn test_roundtrip_i32_be() {
    wide_roundtrip::<32, BigEndian, i32>(i32::MIN, i32::MAX - 65536, 65536);
}

#[test]
fn test_roundtrip_i32_le() {
    wide_roundtrip::<32, LittleEndian, i32>(i32::MIN, i32::MAX - 65536, 65536);
}

#[test]
fn test_roundtrip_u64_be() {
    wide_roundtrip::<64, BigEndian, u64>(u64::MIN, u64::MAX - u64::MAX / 65536, u64::MAX / 65536);
}

#[test]
fn test_roundtrip_u64_le() {
    wide_roundtrip::<64, LittleEndian, u64>(
        u64::MIN,
        u64::MAX - u64::MAX / 65536,
        u64::MAX / 65536,
    );
}

#[test]
fn test_roundtrip_i64_be() {
    wide_roundtrip::<64, BigEndian, i64>(i64::MIN, i64::MAX - i64::MAX / 65536, i64::MAX / 65536);
}

#[test]
fn test_roundtrip_i64_le() {
    wide_roundtrip::<64, LittleEndian, i64>(
        i64::MIN,
        i64::MAX - i64::MAX / 65536,
        i64::MAX / 65536,
    );
}

#[test]
fn test_roundtrip_u128_be() {
    wide_roundtrip::<128, BigEndian, u128>(
        u128::MIN,
        u128::MAX - u128::MAX / 65536,
        u128::MAX / 65536,
    );
}

#[test]
fn test_roundtrip_u128_le() {
    wide_roundtrip::<128, LittleEndian, u128>(
        u128::MIN,
        u128::MAX - u128::MAX / 65536,
        u128::MAX / 65536,
    );
}

#[test]
fn test_roundtrip_i128_be() {
    wide_roundtrip::<128, BigEndian, i128>(
        i128::MIN,
        i128::MAX - i128::MAX / 65536,
        i128::MAX / 65536,
    );
}

#[test]
fn test_roundtrip_i128_le() {
    wide_roundtrip::<128, LittleEndian, i128>(
        i128::MIN,
        i128::MAX - i128::MAX / 65536,
        i128::MAX / 65536,
    );
}

fn unary<E: Endianness>() {
    let mut output: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<_, E>::new(&mut output);
        for value in 0..1024 {
            writer.write_unary::<0>(value).unwrap();
        }
        writer.byte_align().unwrap();
    }
    {
        let mut c = output.as_slice();
        let mut reader = BitReader::<_, E>::new(&mut c);
        for value in 0..1024 {
            assert_eq!(reader.read_unary::<0>().unwrap(), value);
        }
    }

    let mut output: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<_, E>::new(&mut output);
        for value in 0..1024 {
            writer.write_unary::<1>(value).unwrap();
        }
        writer.byte_align().unwrap();
    }
    {
        let mut c = output.as_slice();
        let mut reader = BitReader::<_, E>::new(&mut c);
        for value in 0..1024 {
            assert_eq!(reader.read_unary::<1>().unwrap(), value);
        }
    }
}

#[test]
fn test_unary_roundtrip_be() {
    unary::<BigEndian>()
}

#[test]
fn test_unary_roundtrip_le() {
    unary::<LittleEndian>()
}

fn float<E, P>()
where
    E: Endianness,
    P: Primitive + From<u16> + core::fmt::Debug + core::cmp::PartialEq,
{
    let mut output: Vec<u8> = Vec::new();
    {
        let mut writer = BitWriter::<_, E>::new(&mut output);
        // these values should all be exact in floating-point
        for value in 0..1024 {
            writer.write_from(P::from(value)).unwrap();
        }
        writer.byte_align().unwrap();
    }
    {
        let mut c = output.as_slice();
        let mut reader = BitReader::<_, E>::new(&mut c);
        for value in 0..1024 {
            assert_eq!(reader.read_to::<P>().unwrap(), P::from(value));
        }
    }
}

#[test]
fn test_f32_roundtrip_be() {
    float::<BigEndian, f32>()
}

#[test]
fn test_f64_roundtrip_be() {
    float::<BigEndian, f64>()
}

#[test]
fn test_f32_roundtrip_le() {
    float::<LittleEndian, f32>()
}

#[test]
fn test_f64_roundtrip_le() {
    float::<LittleEndian, f64>()
}

#[test]
fn test_auto_signedness() {
    use bitstream_io::{
        BigEndian, BitRead, BitReader, BitWrite, BitWriter, Endianness, Integer, LittleEndian,
        SignedInteger, UnsignedInteger,
    };

    macro_rules! define_roundtrip {
        ($n:ident, $i:ident, $w:ident, $r:ident) => {
            fn $n<I: Integer + $i + core::ops::AddAssign, E: Endianness>(
                mut start: I,
                end: I,
                bits: u32,
            ) {
                let mut w: BitWriter<_, E> = BitWriter::new(Vec::new());
                w.write_bit(true).unwrap();
                let mut i = start;
                while i < end {
                    w.$w(bits, i).unwrap();
                    i += I::ONE;
                }
                w.$w(bits, end).unwrap();

                w.byte_align().unwrap();

                let v = w.into_writer();
                let mut r: BitReader<_, E> = BitReader::new(v.as_slice());
                assert_eq!(r.read_bit().unwrap(), true);
                while start < end {
                    assert_eq!(r.$r::<I>(bits).unwrap(), start);
                    start += I::ONE;
                }
                assert_eq!(r.$r::<I>(bits).unwrap(), end);
            }
        };
    }

    define_roundtrip!(
        test_writer_unsigned,
        UnsignedInteger,
        write_var,
        read_unsigned_var
    );
    test_writer_unsigned::<_, BigEndian>(u8::MIN, u8::MAX, 8);
    test_writer_unsigned::<_, LittleEndian>(u8::MIN, u8::MAX, 8);

    define_roundtrip!(
        test_writer_signed,
        SignedInteger,
        write_var,
        read_signed_var
    );
    test_writer_signed::<_, BigEndian>(i8::MIN, i8::MAX, 8);
    test_writer_signed::<_, LittleEndian>(i8::MIN, i8::MAX, 8);

    define_roundtrip!(
        test_reader_unsigned,
        UnsignedInteger,
        write_unsigned_var,
        read_var
    );
    test_reader_unsigned::<_, BigEndian>(u8::MIN, u8::MAX, 8);
    test_reader_unsigned::<_, LittleEndian>(u8::MIN, u8::MAX, 8);

    define_roundtrip!(
        test_reader_signed,
        SignedInteger,
        write_signed_var,
        read_var
    );
    test_reader_signed::<_, BigEndian>(i8::MIN, i8::MAX, 8);
    test_reader_signed::<_, LittleEndian>(i8::MIN, i8::MAX, 8);
}

#[test]
fn test_bit_count() {
    use bitstream_io::{BigEndian, BitRead, BitReader, BitWrite, BitWriter};

    let data: &[u8] = &[0b1111_0000];

    // read a count from a 4-bit field
    let count = BitReader::endian(data, BigEndian)
        .read_count::<0b1111>()
        .unwrap();
    assert_eq!(u32::from(count), 0b1111);

    // increment it by 1
    let count = count.try_map::<16, _>(|i| i.checked_add(1)).unwrap();
    assert_eq!(u32::from(count), 0b1111 + 1);

    // decrement it by 1
    let count = count.try_map::<0b1111, _>(|i| i.checked_sub(1)).unwrap();
    assert_eq!(u32::from(count), 0b1111);

    // write the count back to disk
    let mut w = BitWriter::endian(vec![], BigEndian);
    w.write_count::<0b1111>(count).unwrap();
    w.byte_align().unwrap();
    assert_eq!(w.into_writer(), data);
}

fn test_primitives<E: Endianness>() {
    use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter};

    // try a lot of primitive values, shifted by 1 bit
    let mut w = BitWriter::<Vec<u8>, E>::new(vec![]);
    w.write_bit(true).unwrap();
    w.write_from::<u8>(u8::MAX - 10).unwrap();
    w.write_from::<u16>(u16::MAX - 10).unwrap();
    w.write_from::<u32>(u32::MAX - 10).unwrap();
    w.write_from::<u64>(u64::MAX - 10).unwrap();
    w.write_from::<u128>(u128::MAX - 10).unwrap();
    w.write_from::<i8>(i8::MAX - 10).unwrap();
    w.write_from::<i16>(i16::MAX - 10).unwrap();
    w.write_from::<i32>(i32::MAX - 10).unwrap();
    w.write_from::<i64>(i64::MAX - 10).unwrap();
    w.write_from::<i128>(i128::MAX - 10).unwrap();
    w.write_from::<f32>(1.0).unwrap();
    w.write_from::<f64>(2.0).unwrap();
    w.write_from::<[u8; 3]>([9, 8, 7]).unwrap();
    w.byte_align().unwrap();

    let v = w.into_writer();
    let mut r = BitReader::<&[u8], E>::new(v.as_slice());
    assert_eq!(r.read_bit().unwrap(), true);
    assert_eq!(r.read_to::<u8>().unwrap(), u8::MAX - 10);
    assert_eq!(r.read_to::<u16>().unwrap(), u16::MAX - 10);
    assert_eq!(r.read_to::<u32>().unwrap(), u32::MAX - 10);
    assert_eq!(r.read_to::<u64>().unwrap(), u64::MAX - 10);
    assert_eq!(r.read_to::<u128>().unwrap(), u128::MAX - 10);
    assert_eq!(r.read_to::<i8>().unwrap(), i8::MAX - 10);
    assert_eq!(r.read_to::<i16>().unwrap(), i16::MAX - 10);
    assert_eq!(r.read_to::<i32>().unwrap(), i32::MAX - 10);
    assert_eq!(r.read_to::<i64>().unwrap(), i64::MAX - 10);
    assert_eq!(r.read_to::<i128>().unwrap(), i128::MAX - 10);
    assert_eq!(r.read_to::<f32>().unwrap(), 1.0);
    assert_eq!(r.read_to::<f64>().unwrap(), 2.0);
    assert_eq!(r.read_to::<[u8; 3]>().unwrap(), [9, 8, 7]);
}

#[test]
fn test_primitives_be() {
    test_primitives::<BigEndian>()
}

#[test]
fn test_primitives_le() {
    test_primitives::<LittleEndian>()
}
