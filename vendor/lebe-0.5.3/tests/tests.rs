extern crate lebe;

use lebe::prelude::*;
use std::mem;

use byteorder::{WriteBytesExt, LittleEndian, BigEndian, ReadBytesExt};

#[test]
fn make_le_u32_slice() {
    // as seen on https://doc.rust-lang.org/std/primitive.u32.html#method.to_le
    let n = 0x1Au32;

    let mut n_le = [n];
    n_le.convert_current_to_little_endian();

    if cfg!(target_endian = "little") {
        assert_eq!(n_le, [n])
    }
    else {
        assert_eq!(n_le, [u32::swap_bytes(n)])
    }
}

#[test]
fn make_be_u32_slice() {
    // as seen on https://doc.rust-lang.org/std/primitive.u32.html#method.to_be
    let n = 0x1Au32;

    let mut n_be = [n];
    n_be.convert_current_to_big_endian();

    if cfg!(target_endian = "big") {
        assert_eq!(n_be, [n])
    }
    else {
        assert_eq!(n_be, [n.swap_bytes()])
    }
}

#[test]
fn make_le_u16_slice() {
    // as seen on https://doc.rust-lang.org/std/primitive.u16.html#method.to_le
    let n = 0x1Au16;

    let mut n_le = [n];
    n_le.convert_current_to_little_endian();

    if cfg!(target_endian = "little") {
        assert_eq!(n_le, [n])
    }
    else {
        assert_eq!(n_le, [n.swap_bytes()])
    }
}

#[test]
fn make_le_i64_slice() {
    // as seen on https://doc.rust-lang.org/std/primitive.u64.html#method.to_be
    let n1 = 0x14F3EEBCCD93895A_i64;
    let n2 = 0x114F3EF99B81CC5A_i64;

    let mut n_be = [n1, n2];
    n_be.convert_current_to_big_endian();

    if cfg!(target_endian = "big") {
        assert_eq!(n_be, [n1, n2])
    }
    else {
        assert_eq!(n_be, [n1.swap_bytes(), n2.swap_bytes()])
    }
}

#[test]
fn make_be_f64() {
    let i = 0x14F3EEBCCD93895A_u64;

    let mut f = f64::from_bits(i);
    f.convert_current_to_big_endian();

    assert_eq!(f, f64::from_bits(i.to_be()))
}

#[test]
fn into_be_f64() {
    let i = 0x14F3EEBCCD93895A_u64;

    let f: f64 = f64::from_bits(i);
    let f = f.from_current_into_big_endian();

    assert_eq!(f, f64::from_bits(i.to_be()))
}

#[test]
fn into_be_i16() {
    let i = 0x195A_i16;
    let be = i.from_current_into_big_endian();

    if cfg!(target_endian = "big") {
        assert_eq!(be, i)
    }
    else {
        assert_eq!(be, i.swap_bytes())
    }
}

#[test]
fn into_be_u32() {
    let i = 0x1220943_u32;
    let be = i.from_current_into_big_endian();

    if cfg!(target_endian = "big") {
        assert_eq!(be, i)
    }
    else {
        assert_eq!(be, i.swap_bytes())
    }
}

#[test]
fn cmp_read_be_u16() {
    let read: &[u8] = &[0x33, 0xbb];
    let a = u16::read_from_big_endian(&mut read.clone()).unwrap();
    let b: u16 = read.clone().read_from_big_endian().unwrap();
    let c = read.clone().read_u16::<BigEndian>().unwrap();

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn cmp_read_le_u16() {
    let read: &[u8] = &[0x33, 0xbb];
    let a = u16::read_from_little_endian(&mut read.clone()).unwrap();
    let b: u16 = read.clone().read_from_little_endian().unwrap();
    let c = read.clone().read_u16::<LittleEndian>().unwrap();

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn cmp_read_le_f32() {
    let read: &[u8] = &[0x33, 0xBB, 0x44, 0xCC];
    let a = f32::read_from_little_endian(&mut read.clone()).unwrap();
    let b: f32 = read.clone().read_from_little_endian().unwrap();
    let c = read.clone().read_f32::<LittleEndian>().unwrap();

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn cmp_read_be_slice()  {
    let mut write_expected = Vec::new();
    let mut write_actual = Vec::new();

    let data: Vec<f32> = (0..31*31).map(|i| i as f32).collect();

    for number in &data {
        write_expected.write_f32::<BigEndian>(*number).unwrap();
    }

    write_actual.write_as_big_endian(data.as_slice()).unwrap();
    assert_eq!(write_actual, write_expected);
}

#[test]
fn cmp_write_le_slice() {
    let mut write_expected = Vec::new();
    let mut write_actual = Vec::new();

    let data: Vec<f32> = (0..31*31).map(|i| i as f32).collect();

    for number in &data {
        write_expected.write_f32::<LittleEndian>(*number).unwrap();
    }

    write_actual.write_as_little_endian(data.as_slice()).unwrap();

    assert_eq!(write_actual, write_expected);
}

#[test]
fn cmp_write_le_u32() {
    let mut write_expected = Vec::new();
    let mut write_actual = Vec::new();

    let data = 0x23573688_u32;
    write_expected.write_u32::<LittleEndian>(data).unwrap();
    write_actual.write_as_little_endian(&data).unwrap();

    assert_eq!(write_actual, write_expected);
}



#[test]
fn cmp_write_le_slice_u64() {
    let mut write_expected = Vec::new();
    let mut write_actual = Vec::new();

    let data: Vec<u64> = (1000..1000+310*31).map(|i| i as u64).collect();

    for number in &data {
        write_expected.write_u64::<LittleEndian>(*number).unwrap();
    }

    write_actual.write_as_little_endian(data.as_slice()).unwrap();

    assert_eq!(write_actual, write_expected);
}