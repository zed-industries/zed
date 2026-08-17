#![cfg(feature = "NSValue")]
use alloc::format;

use crate::Foundation::NSNumber;

#[test]
fn basic() {
    let val = NSNumber::new_u32(13);
    assert_eq!(val.as_u32(), 13);
}

#[test]
fn roundtrip() {
    assert!(NSNumber::new_bool(true).as_bool());
    assert!(!NSNumber::new_bool(false).as_bool());

    fn assert_roundtrip_signed(val: i64) {
        assert_eq!(NSNumber::new_i8(val as i8).as_i8(), val as i8);
        assert_eq!(NSNumber::new_i16(val as i16).as_i16(), val as i16);
        assert_eq!(NSNumber::new_i32(val as i32).as_i32(), val as i32);
        assert_eq!(NSNumber::new_i64(val).as_i64(), val);
        assert_eq!(NSNumber::new_isize(val as isize).as_isize(), val as isize);
    }

    assert_roundtrip_signed(i64::MIN);
    assert_roundtrip_signed(i32::MIN as i64);
    assert_roundtrip_signed(i16::MIN as i64);
    assert_roundtrip_signed(i8::MIN as i64);
    assert_roundtrip_signed(-1);
    assert_roundtrip_signed(0);
    assert_roundtrip_signed(1);
    assert_roundtrip_signed(i8::MAX as i64);
    assert_roundtrip_signed(i16::MAX as i64);
    assert_roundtrip_signed(i32::MAX as i64);
    assert_roundtrip_signed(i64::MAX);

    fn assert_roundtrip_unsigned(val: u64) {
        assert_eq!(NSNumber::new_u8(val as u8).as_u8(), val as u8);
        assert_eq!(NSNumber::new_u16(val as u16).as_u16(), val as u16);
        assert_eq!(NSNumber::new_u32(val as u32).as_u32(), val as u32);
        assert_eq!(NSNumber::new_u64(val).as_u64(), val);
        assert_eq!(NSNumber::new_usize(val as usize).as_usize(), val as usize);
    }

    assert_roundtrip_unsigned(0);
    assert_roundtrip_unsigned(1);
    assert_roundtrip_unsigned(u8::MAX as u64);
    assert_roundtrip_unsigned(u16::MAX as u64);
    assert_roundtrip_unsigned(u32::MAX as u64);
    assert_roundtrip_unsigned(u64::MAX);

    fn assert_roundtrip_float(val: f64) {
        assert_eq!(NSNumber::new_f32(val as f32).as_f32(), val as f32);
        assert_eq!(NSNumber::new_f64(val).as_f64(), val);
    }

    assert_roundtrip_float(0.0);
    assert_roundtrip_float(-1.0);
    assert_roundtrip_float(1.0);
    assert_roundtrip_float(f64::INFINITY);
    assert_roundtrip_float(-f64::INFINITY);
    assert_roundtrip_float(f64::MAX);
    assert_roundtrip_float(f64::MIN);
    assert_roundtrip_float(f64::MIN_POSITIVE);

    assert!(NSNumber::new_f32(f32::NAN).as_f32().is_nan());
    assert!(NSNumber::new_f64(f64::NAN).as_f64().is_nan());
    assert!(NSNumber::new_f32(-f32::NAN).as_f32().is_nan());
    assert!(NSNumber::new_f64(-f64::NAN).as_f64().is_nan());
}

#[test]
fn cast_between_types() {
    assert_eq!(NSNumber::new_bool(true).as_i8(), 1);
    assert_eq!(NSNumber::new_i32(i32::MAX).as_u32(), i32::MAX as u32);
    assert_eq!(NSNumber::new_f32(1.0).as_u32(), 1);
    assert_eq!(NSNumber::new_f32(1.0).as_u32(), 1);
}

#[test]
fn equality() {
    let val1 = NSNumber::new_u32(123);
    let val2 = NSNumber::new_u32(123);
    let val3 = NSNumber::new_u8(123);
    assert_eq!(val1, val1);
    assert_eq!(val1, val2);
    assert_eq!(val1, val3);

    let val4 = NSNumber::new_u32(456);
    assert_ne!(val1, val4);
}

#[test]
#[cfg_attr(feature = "gnustep-1-7", ignore = "GNUStep handles NaNs differently")]
fn nan_equality() {
    let nan = NSNumber::new_f32(f32::NAN);
    let nan2 = NSNumber::new_f32(f32::NAN);
    let neg_nan = NSNumber::new_f32(-f32::NAN);
    assert_eq!(nan, nan);
    assert_eq!(nan, nan2);
    assert_eq!(neg_nan, neg_nan);
    assert_eq!(nan, neg_nan);
}

// Ensure that comparisons are made on the number, and not the bits of the floating point value
#[test]
fn float_int_equality() {
    let val1 = NSNumber::new_f32(1.0);
    let val2 = NSNumber::new_u32(1);
    let val3 = NSNumber::new_u32(1.0f32.to_bits());
    assert_eq!(val1, val2);
    assert_ne!(val1, val3);
}

#[test]
#[cfg(feature = "NSString")]
fn display_debug() {
    use std::fmt;

    fn assert_display_debug<T: fmt::Debug + fmt::Display>(val: T, expected: &str) {
        // The two impls for these happen to be the same
        assert_eq!(format!("{val}"), expected);
        assert_eq!(format!("{val:?}"), expected);
    }
    assert_display_debug(NSNumber::new_u8(171), "171");
    assert_display_debug(NSNumber::new_i8(-12), "-12");
    assert_display_debug(NSNumber::new_u32(0xdeadbeef), "3735928559");
    assert_display_debug(NSNumber::new_f32(1.1), "1.1");
    assert_display_debug(NSNumber::new_f32(1.0), "1");
    assert_display_debug(NSNumber::new_bool(true), "1");
    assert_display_debug(NSNumber::new_bool(false), "0");
}
