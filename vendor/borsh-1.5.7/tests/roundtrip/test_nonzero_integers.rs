use borsh::from_slice;
use core::num::*;

#[test]
fn test_nonzero_integer_u8() {
    let bytes = &[1];
    assert_eq!(from_slice::<NonZeroU8>(bytes).unwrap().get(), 1);
}

#[test]
fn test_nonzero_integer_u32() {
    let bytes = &[255, 0, 0, 0];
    assert_eq!(from_slice::<NonZeroU32>(bytes).unwrap().get(), 255);
}

#[test]
fn test_nonzero_integer_usize() {
    let bytes = &[1, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(from_slice::<NonZeroUsize>(bytes).unwrap().get(), 257);
}

#[test]
fn test_nonzero_integer_i64() {
    let bytes = &[255; 8];
    assert_eq!(from_slice::<NonZeroI64>(bytes).unwrap().get(), -1);
}

#[test]
fn test_nonzero_integer_i16b() {
    let bytes = &[0, 0b1000_0000];
    assert_eq!(from_slice::<NonZeroI16>(bytes).unwrap().get(), i16::MIN);
}
