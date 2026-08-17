// If `src` can be promoted to `$dst`, then it must be Ok to cast `dst` back to
// `$src`
macro_rules! promote_and_back {
    ($($src:ident => $($dst:ident),+);+;) => {
        mod demoting_to {
            $(
                mod $src {
                    mod from {
                        use crate::From;

                        $(
                            quickcheck! {
                                fn $dst(src: $src) -> bool {
                                    $src::cast($dst::cast(src)).is_ok()
                                }
                            }
                         )+
                    }
                }
             )+
        }
    }
}

#[cfg(target_pointer_width = "32")]
promote_and_back! {
    i8    => f32, f64,     i16, i32, isize, i64, i128                            ;
    i16   => f32, f64,          i32, isize, i64, i128                            ;
    i32   =>      f64,                      i64, i128                            ;
    isize =>      f64,                      i64, i128                            ;
    i64   =>                                     i128                            ;
    u8    => f32, f64,     i16, i32, isize, i64, i128, u16, u32, usize, u64, u128;
    u16   => f32, f64,          i32, isize, i64, i128,      u32, usize, u64, u128;
    u32   =>      f64,                      i64, i128,                  u64, u128;
    usize =>      f64,                      i64, i128,                  u64, u128;
    u64   =>                                     i128,                       u128;
}

#[cfg(target_pointer_width = "64")]
promote_and_back! {
    i8    => f32, f64,     i16, i32, i64, isize, i128                            ;
    i16   => f32, f64,          i32, i64, isize, i128                            ;
    i32   =>      f64,               i64, isize, i128                            ;
    i64   =>                                     i128                            ;
    isize =>                                     i128                            ;
    u8    => f32, f64,     i16, i32, i64, isize, i128, u16, u32, u64, usize, u128;
    u16   => f32, f64,          i32, i64, isize, i128,      u32, u64, usize, u128;
    u32   =>      f64,               i64, isize, i128,           u64, usize, u128;
    u64   =>                                     i128,                       u128;
    usize =>                                     i128,                       u128;
}

// If it's Ok to cast `src` to `$dst`, it must also be Ok to cast `dst` back to
// `$src`
macro_rules! symmetric_cast_between {
    ($($src:ident => $($dst:ident),+);+;) => {
        mod symmetric_cast_between {
            $(
                mod $src {
                    mod and {
                        use quickcheck::TestResult;

                        use crate::From;

                        $(
                            quickcheck! {
                                fn $dst(src: $src) -> TestResult {
                                    if let Ok(dst) = $dst::cast(src) {
                                        TestResult::from_bool(
                                            $src::cast(dst).is_ok())
                                    } else {
                                        TestResult::discard()
                                    }
                                }
                            }
                         )+
                    }
                }
             )+
        }
    }
}

#[cfg(target_pointer_width = "32")]
symmetric_cast_between! {
    u8    =>           i8                      ;
    u16   =>           i8, i16                 ;
    u32   =>           i8, i16, i32            ;
    usize =>           i8, i16, i32            ;
    u64   =>           i8, i16, i32, i64, isize;
}

#[cfg(target_pointer_width = "64")]
symmetric_cast_between! {
    u8    =>           i8                            ;
    u16   =>           i8, i16                       ;
    u32   =>           i8, i16, i32                  ;
    u64   =>           i8, i16, i32, i64, isize      ;
    usize =>           i8, i16, i32, i64, isize      ;
    u128  =>           i8, i16, i32, i64, isize, i128;
}

macro_rules! from_float {
    ($($src:ident => $($dst:ident),+);+;) => {
        $(
            mod $src {
                mod inf {
                    mod to {
                        use crate::{Error, From};

                        $(
                            #[test]
                            fn $dst() {
                                let _0: $src = 0.;
                                let _1: $src = 1.;
                                let inf = _1 / _0;
                                let neg_inf = -_1 / _0;

                                assert_eq!($dst::cast(inf),
                                           Err(Error::Infinite));
                                assert_eq!($dst::cast(neg_inf),
                                           Err(Error::Infinite));
                            }
                         )+
                    }
                }

                mod nan {
                    mod to {
                        use crate::{Error, From};

                        $(
                            #[test]
                            fn $dst() {
                                let _0: $src = 0.;
                                let nan = _0 / _0;

                                assert_eq!($dst::cast(nan),
                                           Err(Error::NaN));
                            }
                         )+
                    }
                }
            }
         )+
    }
}

from_float! {
    f32 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
    f64 => i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
}

#[test]
fn test_fl_conversion() {
    use crate::u128;
    assert_eq!(u128(42.0f32), Ok(42));
}

#[test]
fn gh16() {
    assert_eq!(super::u64(-0.01_f64), Ok(0));
    assert_eq!(super::u64(-0.99_f32), Ok(0));

    assert_eq!(super::u32(-0.99_f64), Ok(0));
    assert_eq!(super::u32(-0.01_f32), Ok(0));

    assert_eq!(super::u64(0.01_f64), Ok(0));
    assert_eq!(super::u64(0.99_f32), Ok(0));

    assert_eq!(super::u32(0.99_f64), Ok(0));
    assert_eq!(super::u32(0.01_f32), Ok(0));
}

#[test]
fn gh15() {
    assert_eq!(super::u32(32_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u32(32_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u64(64_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u64(64_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u8(8_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u8(8_f64.exp2()), Err(super::Error::Overflow));

    assert_eq!(super::u16(16_f32.exp2()), Err(super::Error::Overflow));
    assert_eq!(super::u16(16_f64.exp2()), Err(super::Error::Overflow));
}

#[test]
fn gh23_lossless_integer_max_min_to_float() {
    // f32::MANTISSA_DIGITS = 24
    assert_eq!(Ok(u8::MAX), super::u8(255f32));
    assert_eq!(Ok(u16::MAX), super::u16(65_535f32));

    // f64::MANTISSA_DIGITS = 53
    assert_eq!(Ok(u8::MAX), super::u8(255f64));
    assert_eq!(Ok(u16::MAX), super::u16(65_535f64));
    assert_eq!(Ok(u32::MAX), super::u32(4_294_967_295f64));

    // also check negative values (not part of the original bug)
    assert_eq!(Ok(i8::MIN), super::i8(-128f32));
    assert_eq!(Ok(i16::MIN), super::i16(-32_768f32));

    assert_eq!(Ok(i8::MIN), super::i8(-128f64));
    assert_eq!(Ok(i16::MIN), super::i16(-32_768f64));
    assert_eq!(Ok(i32::MIN), super::i32(-2_147_483_648f64));
}
