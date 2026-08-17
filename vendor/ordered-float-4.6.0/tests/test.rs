#![allow(clippy::float_cmp, clippy::eq_op, clippy::op_ref)]

extern crate num_traits;
extern crate ordered_float;

pub use num_traits::float::FloatCore;
pub use num_traits::{Bounded, FloatConst, FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
#[cfg(any(feature = "std", feature = "libm"))]
pub use num_traits::{Float, Pow};
pub use ordered_float::*;

pub use std::cmp::Ordering::*;
pub use std::convert::TryFrom;
pub use std::{f32, f64, panic};

pub use std::collections::hash_map::RandomState;
pub use std::collections::HashSet;
pub use std::hash::*;

fn not_nan<T: FloatCore>(x: T) -> NotNan<T> {
    NotNan::new(x).unwrap()
}

#[test]
fn test_total_order() {
    let numberline = [
        (-f32::INFINITY, 0),
        (-1.0, 1),
        (-0.0, 2),
        (0.0, 2),
        (1.0, 3),
        (f32::INFINITY, 4),
        (f32::NAN, 5),
        (-f32::NAN, 5),
    ];

    for &(fi, i) in &numberline {
        for &(fj, j) in &numberline {
            assert_eq!(OrderedFloat(fi) < OrderedFloat(fj), i < j);
            assert_eq!(OrderedFloat(fi) > OrderedFloat(fj), i > j);
            assert_eq!(OrderedFloat(fi) <= OrderedFloat(fj), i <= j);
            assert_eq!(OrderedFloat(fi) >= OrderedFloat(fj), i >= j);
            assert_eq!(OrderedFloat(fi) == OrderedFloat(fj), i == j);
            assert_eq!(OrderedFloat(fi) != OrderedFloat(fj), i != j);
            assert_eq!(OrderedFloat(fi).cmp(&OrderedFloat(fj)), i.cmp(&j));
        }
    }
}

#[test]
fn ordered_f32_compare_regular_floats() {
    assert_eq!(OrderedFloat(7.0f32).cmp(&OrderedFloat(7.0)), Equal);
    assert_eq!(OrderedFloat(8.0f32).cmp(&OrderedFloat(7.0)), Greater);
    assert_eq!(OrderedFloat(4.0f32).cmp(&OrderedFloat(7.0)), Less);
}

#[test]
fn ordered_f32_compare_regular_floats_op() {
    assert!(OrderedFloat(7.0f32) == OrderedFloat(7.0));
    assert!(OrderedFloat(7.0f32) <= OrderedFloat(7.0));
    assert!(OrderedFloat(7.0f32) >= OrderedFloat(7.0));
    assert!(OrderedFloat(8.0f32) > OrderedFloat(7.0));
    assert!(OrderedFloat(8.0f32) >= OrderedFloat(7.0));
    assert!(OrderedFloat(4.0f32) < OrderedFloat(7.0));
    assert!(OrderedFloat(4.0f32) <= OrderedFloat(7.0));
}

#[test]
fn ordered_f32_compare_nan() {
    let f32_nan: f32 = FloatCore::nan();
    assert_eq!(
        OrderedFloat(f32_nan).cmp(&OrderedFloat(FloatCore::nan())),
        Equal
    );
    assert_eq!(
        OrderedFloat(f32_nan).cmp(&OrderedFloat(-100000.0f32)),
        Greater
    );
    assert_eq!(
        OrderedFloat(-100.0f32).cmp(&OrderedFloat(FloatCore::nan())),
        Less
    );
}

#[test]
fn ordered_f32_compare_nan_op() {
    let f32_nan: OrderedFloat<f32> = OrderedFloat(FloatCore::nan());
    assert!(f32_nan == f32_nan);
    assert!(f32_nan <= f32_nan);
    assert!(f32_nan >= f32_nan);
    assert!(f32_nan > OrderedFloat(-100000.0f32));
    assert!(f32_nan >= OrderedFloat(-100000.0f32));
    assert!(OrderedFloat(-100.0f32) < f32_nan);
    assert!(OrderedFloat(-100.0f32) <= f32_nan);
    assert!(f32_nan > OrderedFloat(<f32 as FloatCore>::infinity()));
    assert!(f32_nan >= OrderedFloat(<f32 as FloatCore>::infinity()));
    assert!(f32_nan > OrderedFloat(<f32 as FloatCore>::neg_infinity()));
    assert!(f32_nan >= OrderedFloat(<f32 as FloatCore>::neg_infinity()));
}

#[test]
fn ordered_f64_compare_regular_floats() {
    assert_eq!(OrderedFloat(7.0f64).cmp(&OrderedFloat(7.0)), Equal);
    assert_eq!(OrderedFloat(8.0f64).cmp(&OrderedFloat(7.0)), Greater);
    assert_eq!(OrderedFloat(4.0f64).cmp(&OrderedFloat(7.0)), Less);
}

/// This code is not run, but successfully compiling it checks that the given bounds
/// are *sufficient* to write code that is generic over float type.
fn _generic_code_can_use_float_core<T>(inputs: &mut [OrderedFloat<T>])
where
    T: num_traits::float::FloatCore,
{
    inputs.sort();
}

#[test]
fn not_nan32_zero() {
    assert_eq!(NotNan::<f32>::zero(), 0.0f32);
    assert!(NotNan::<f32>::zero().is_zero());
}

#[test]
fn not_nan32_one() {
    assert_eq!(NotNan::<f32>::one(), 1.0f32)
}

#[test]
fn not_nan32_bounded() {
    assert_eq!(NotNan::<f32>::min_value(), <f32 as Bounded>::min_value());
    assert_eq!(NotNan::<f32>::max_value(), <f32 as Bounded>::max_value());
}

#[test]
fn not_nan32_from_primitive() {
    assert_eq!(NotNan::<f32>::from_i8(42i8), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_u8(42u8), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_i16(42i16), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_u16(42u16), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_i32(42i32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_u32(42u32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_i64(42i64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_u64(42u64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_isize(42isize), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_usize(42usize), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_f32(42f32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_f32(42f32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f32>::from_f32(FloatCore::nan()), None);
    assert_eq!(NotNan::<f32>::from_f64(FloatCore::nan()), None);
}

#[test]
fn not_nan32_to_primitive() {
    let x = not_nan(42.0f32);
    assert_eq!(x.to_u8(), Some(42u8));
    assert_eq!(x.to_i8(), Some(42i8));
    assert_eq!(x.to_u16(), Some(42u16));
    assert_eq!(x.to_i16(), Some(42i16));
    assert_eq!(x.to_u32(), Some(42u32));
    assert_eq!(x.to_i32(), Some(42i32));
    assert_eq!(x.to_u64(), Some(42u64));
    assert_eq!(x.to_i64(), Some(42i64));
    assert_eq!(x.to_usize(), Some(42usize));
    assert_eq!(x.to_isize(), Some(42isize));
    assert_eq!(x.to_f32(), Some(42f32));
    assert_eq!(x.to_f32(), Some(42f32));
    assert_eq!(x.to_f64(), Some(42f64));
    assert_eq!(x.to_f64(), Some(42f64));
}

#[test]
fn not_nan32_num() {
    assert_eq!(NotNan::<f32>::from_str_radix("42.0", 10).unwrap(), 42.0f32);
    assert!(NotNan::<f32>::from_str_radix("NaN", 10).is_err());
}

#[test]
fn not_nan32_signed() {
    assert_eq!(not_nan(42f32).abs(), 42f32);
    assert_eq!(not_nan(-42f32).abs(), 42f32);

    assert_eq!(not_nan(50f32).abs_sub(&not_nan(8f32)), 42f32);
    assert_eq!(not_nan(8f32).abs_sub(&not_nan(50f32)), 0f32);
}

#[test]
fn not_nan32_num_cast() {
    assert_eq!(
        <NotNan<f32> as num_traits::NumCast>::from(42).unwrap(),
        42f32
    );
    assert_eq!(
        <NotNan<f32> as num_traits::NumCast>::from(<f32 as FloatCore>::nan()),
        None
    );
}

#[test]
fn ordered_f64_compare_nan() {
    let f64_nan: f64 = FloatCore::nan();
    assert_eq!(
        OrderedFloat(f64_nan).cmp(&OrderedFloat(FloatCore::nan())),
        Equal
    );
    assert_eq!(
        OrderedFloat(f64_nan).cmp(&OrderedFloat(-100000.0f64)),
        Greater
    );
    assert_eq!(
        OrderedFloat(-100.0f64).cmp(&OrderedFloat(FloatCore::nan())),
        Less
    );
}

#[test]
fn ordered_f64_compare_regular_floats_op() {
    assert!(OrderedFloat(7.0) == OrderedFloat(7.0));
    assert!(OrderedFloat(7.0) <= OrderedFloat(7.0));
    assert!(OrderedFloat(7.0) >= OrderedFloat(7.0));
    assert!(OrderedFloat(8.0) > OrderedFloat(7.0));
    assert!(OrderedFloat(8.0) >= OrderedFloat(7.0));
    assert!(OrderedFloat(4.0) < OrderedFloat(7.0));
    assert!(OrderedFloat(4.0) <= OrderedFloat(7.0));
}

#[test]
fn ordered_f64_compare_nan_op() {
    let f64_nan: OrderedFloat<f64> = OrderedFloat(FloatCore::nan());
    assert!(f64_nan == f64_nan);
    assert!(f64_nan <= f64_nan);
    assert!(f64_nan >= f64_nan);
    assert!(f64_nan > OrderedFloat(-100000.0));
    assert!(f64_nan >= OrderedFloat(-100000.0));
    assert!(OrderedFloat(-100.0) < f64_nan);
    assert!(OrderedFloat(-100.0) <= f64_nan);
    assert!(f64_nan > OrderedFloat(<f64 as FloatCore>::infinity()));
    assert!(f64_nan >= OrderedFloat(<f64 as FloatCore>::infinity()));
    assert!(f64_nan > OrderedFloat(<f64 as FloatCore>::neg_infinity()));
    assert!(f64_nan >= OrderedFloat(<f64 as FloatCore>::neg_infinity()));
}

#[test]
fn not_nan32_compare_regular_floats() {
    assert_eq!(not_nan(7.0f32).cmp(&not_nan(7.0)), Equal);
    assert_eq!(not_nan(8.0f32).cmp(&not_nan(7.0)), Greater);
    assert_eq!(not_nan(4.0f32).cmp(&not_nan(7.0)), Less);
}

#[test]
fn not_nan32_fail_when_constructing_with_nan() {
    let f32_nan: f32 = FloatCore::nan();
    assert!(NotNan::new(f32_nan).is_err());
}

#[test]
fn not_nan32_calculate_correctly() {
    assert_eq!(*(not_nan(5.0f32) + not_nan(4.0f32)), 5.0f32 + 4.0f32);
    assert_eq!(*(not_nan(5.0f32) + 4.0f32), 5.0f32 + 4.0f32);
    assert_eq!(*(not_nan(5.0f32) - not_nan(4.0f32)), 5.0f32 - 4.0f32);
    assert_eq!(*(not_nan(5.0f32) - 4.0f32), 5.0f32 - 4.0f32);
    assert_eq!(*(not_nan(5.0f32) * not_nan(4.0f32)), 5.0f32 * 4.0f32);
    assert_eq!(*(not_nan(5.0f32) * 4.0f32), 5.0f32 * 4.0f32);
    assert_eq!(*(not_nan(8.0f32) / not_nan(4.0f32)), 8.0f32 / 4.0f32);
    assert_eq!(*(not_nan(8.0f32) / 4.0f32), 8.0f32 / 4.0f32);
    assert_eq!(*(not_nan(8.0f32) % not_nan(4.0f32)), 8.0f32 % 4.0f32);
    assert_eq!(*(not_nan(8.0f32) % 4.0f32), 8.0f32 % 4.0f32);
    assert_eq!(*(-not_nan(1.0f32)), -1.0f32);

    assert!(panic::catch_unwind(|| not_nan(0.0f32) + f32::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f32) - f32::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f32) * f32::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f32) / f32::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f32) % f32::NAN).is_err());

    let mut number = not_nan(5.0f32);
    number += not_nan(4.0f32);
    assert_eq!(*number, 9.0f32);
    number -= not_nan(4.0f32);
    assert_eq!(*number, 5.0f32);
    number *= not_nan(4.0f32);
    assert_eq!(*number, 20.0f32);
    number /= not_nan(4.0f32);
    assert_eq!(*number, 5.0f32);
    number %= not_nan(4.0f32);
    assert_eq!(*number, 1.0f32);

    number = not_nan(5.0f32);
    number += 4.0f32;
    assert_eq!(*number, 9.0f32);
    number -= 4.0f32;
    assert_eq!(*number, 5.0f32);
    number *= 4.0f32;
    assert_eq!(*number, 20.0f32);
    number /= 4.0f32;
    assert_eq!(*number, 5.0f32);
    number %= 4.0f32;
    assert_eq!(*number, 1.0f32);

    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f32);
        tmp += f32::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f32);
        tmp -= f32::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f32);
        tmp *= f32::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f32);
        tmp /= f32::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f32);
        tmp %= f32::NAN;
    })
    .is_err());
}

#[test]
fn not_nan64_compare_regular_floats() {
    assert_eq!(not_nan(7.0f64).cmp(&not_nan(7.0)), Equal);
    assert_eq!(not_nan(8.0f64).cmp(&not_nan(7.0)), Greater);
    assert_eq!(not_nan(4.0f64).cmp(&not_nan(7.0)), Less);
}

#[test]
fn not_nan64_fail_when_constructing_with_nan() {
    let f64_nan: f64 = FloatCore::nan();
    assert!(NotNan::new(f64_nan).is_err());
}

#[test]
fn not_nan64_calculate_correctly() {
    assert_eq!(*(not_nan(5.0f64) + not_nan(4.0f64)), 5.0f64 + 4.0f64);
    assert_eq!(*(not_nan(5.0f64) + 4.0f64), 5.0f64 + 4.0f64);
    assert_eq!(*(not_nan(5.0f64) - not_nan(4.0f64)), 5.0f64 - 4.0f64);
    assert_eq!(*(not_nan(5.0f64) - 4.0f64), 5.0f64 - 4.0f64);
    assert_eq!(*(not_nan(5.0f64) * not_nan(4.0f64)), 5.0f64 * 4.0f64);
    assert_eq!(*(not_nan(5.0f64) * 4.0f64), 5.0f64 * 4.0f64);
    assert_eq!(*(not_nan(8.0f64) / not_nan(4.0f64)), 8.0f64 / 4.0f64);
    assert_eq!(*(not_nan(8.0f64) / 4.0f64), 8.0f64 / 4.0f64);
    assert_eq!(*(not_nan(8.0f64) % not_nan(4.0f64)), 8.0f64 % 4.0f64);
    assert_eq!(*(not_nan(8.0f64) % 4.0f64), 8.0f64 % 4.0f64);
    assert_eq!(*(-not_nan(1.0f64)), -1.0f64);

    assert!(panic::catch_unwind(|| not_nan(0.0f64) + f64::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f64) - f64::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f64) * f64::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f64) / f64::NAN).is_err());
    assert!(panic::catch_unwind(|| not_nan(0.0f64) % f64::NAN).is_err());

    let mut number = not_nan(5.0f64);
    number += not_nan(4.0f64);
    assert_eq!(*number, 9.0f64);
    number -= not_nan(4.0f64);
    assert_eq!(*number, 5.0f64);
    number *= not_nan(4.0f64);
    assert_eq!(*number, 20.0f64);
    number /= not_nan(4.0f64);
    assert_eq!(*number, 5.0f64);
    number %= not_nan(4.0f64);
    assert_eq!(*number, 1.0f64);

    number = not_nan(5.0f64);
    number += 4.0f64;
    assert_eq!(*number, 9.0f64);
    number -= 4.0f64;
    assert_eq!(*number, 5.0f64);
    number *= 4.0f64;
    assert_eq!(*number, 20.0f64);
    number /= 4.0f64;
    assert_eq!(*number, 5.0f64);
    number %= 4.0f64;
    assert_eq!(*number, 1.0f64);

    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f64);
        tmp += f64::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f64);
        tmp -= f64::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f64);
        tmp *= f64::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f64);
        tmp /= f64::NAN;
    })
    .is_err());
    assert!(panic::catch_unwind(|| {
        let mut tmp = not_nan(0.0f64);
        tmp %= f64::NAN;
    })
    .is_err());
}

#[test]
fn not_nan64_zero() {
    assert_eq!(NotNan::<f64>::zero(), not_nan(0.0f64));
    assert!(NotNan::<f64>::zero().is_zero());
}

#[test]
fn not_nan64_one() {
    assert_eq!(NotNan::<f64>::one(), not_nan(1.0f64))
}

#[test]
fn not_nan64_bounded() {
    assert_eq!(NotNan::<f64>::min_value(), <f64 as Bounded>::min_value());
    assert_eq!(NotNan::<f64>::max_value(), <f64 as Bounded>::max_value());
}

#[test]
fn not_nan64_from_primitive() {
    assert_eq!(NotNan::<f64>::from_i8(42i8), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_u8(42u8), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_i16(42i16), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_u16(42u16), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_i32(42i32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_u32(42u32), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_i64(42i64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_u64(42u64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_isize(42isize), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_usize(42usize), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_f64(42f64), Some(not_nan(42.0)));
    assert_eq!(NotNan::<f64>::from_f64(FloatCore::nan()), None);
    assert_eq!(NotNan::<f64>::from_f64(FloatCore::nan()), None);
}

#[test]
fn not_nan64_to_primitive() {
    let x = not_nan(42.0f64);
    assert_eq!(x.to_u8(), Some(42u8));
    assert_eq!(x.to_i8(), Some(42i8));
    assert_eq!(x.to_u16(), Some(42u16));
    assert_eq!(x.to_i16(), Some(42i16));
    assert_eq!(x.to_u32(), Some(42u32));
    assert_eq!(x.to_i32(), Some(42i32));
    assert_eq!(x.to_u64(), Some(42u64));
    assert_eq!(x.to_i64(), Some(42i64));
    assert_eq!(x.to_usize(), Some(42usize));
    assert_eq!(x.to_isize(), Some(42isize));
    assert_eq!(x.to_f64(), Some(42f64));
    assert_eq!(x.to_f64(), Some(42f64));
    assert_eq!(x.to_f64(), Some(42f64));
    assert_eq!(x.to_f64(), Some(42f64));
}

#[test]
fn not_nan64_num() {
    assert_eq!(
        NotNan::<f64>::from_str_radix("42.0", 10).unwrap(),
        not_nan(42.0f64)
    );
    assert!(NotNan::<f64>::from_str_radix("NaN", 10).is_err());
}

#[test]
fn not_nan64_signed() {
    assert_eq!(not_nan(42f64).abs(), not_nan(42f64));
    assert_eq!(not_nan(-42f64).abs(), not_nan(42f64));

    assert_eq!(not_nan(50f64).abs_sub(&not_nan(8f64)), not_nan(42f64));
    assert_eq!(not_nan(8f64).abs_sub(&not_nan(50f64)), not_nan(0f64));
}

#[test]
fn not_nan64_num_cast() {
    assert_eq!(
        <NotNan<f64> as num_traits::NumCast>::from(42),
        Some(not_nan(42f64))
    );
    assert_eq!(
        <NotNan<f64> as num_traits::NumCast>::from(<f64 as FloatCore>::nan()),
        None
    );
}

#[test]
fn hash_zero_and_neg_zero_to_the_same_hc_ordered_float64() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedFloat::from(0f64).hash(&mut h1);
    OrderedFloat::from(-0f64).hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn hash_zero_and_neg_zero_to_the_same_hc_not_nan32() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    NotNan::try_from(0f32).unwrap().hash(&mut h1);
    NotNan::try_from(-0f32).unwrap().hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn hash_different_nans_to_the_same_hc() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedFloat::from(<f64 as FloatCore>::nan()).hash(&mut h1);
    OrderedFloat::from(-<f64 as FloatCore>::nan()).hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn hash_inf_and_neg_inf_to_different_hcs() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedFloat::from(f64::INFINITY).hash(&mut h1);
    OrderedFloat::from(f64::NEG_INFINITY).hash(&mut h2);
    assert!(h1.finish() != h2.finish());
}

#[test]
fn hash_is_good_for_whole_numbers() {
    let state = RandomState::new();
    let limit = 10000;

    let mut set = ::std::collections::HashSet::with_capacity(limit);
    for i in 0..limit {
        let mut h = state.build_hasher();
        OrderedFloat::from(i as f64).hash(&mut h);
        set.insert(h.finish());
    }

    // This allows 100 collisions, which is far too
    // many, but should guard against transient issues
    // that will result from using RandomState
    let pct_unique = set.len() as f64 / limit as f64;
    assert!(0.99f64 < pct_unique, "percent-unique={}", pct_unique);
}

#[test]
fn hash_is_good_for_fractional_numbers() {
    let state = RandomState::new();
    let limit = 10000;

    let mut set = ::std::collections::HashSet::with_capacity(limit);
    for i in 0..limit {
        let mut h = state.build_hasher();
        OrderedFloat::from(i as f64 * (1f64 / limit as f64)).hash(&mut h);
        set.insert(h.finish());
    }

    // This allows 100 collisions, which is far too
    // many, but should guard against transient issues
    // that will result from using RandomState
    let pct_unique = set.len() as f64 / limit as f64;
    assert!(0.99f64 < pct_unique, "percent-unique={}", pct_unique);
}

#[test]
#[should_panic]
fn test_add_fails_on_nan() {
    let a = not_nan(f32::INFINITY);
    let b = not_nan(f32::NEG_INFINITY);
    let _c = a + b;
}

#[test]
#[should_panic]
fn test_add_fails_on_nan_ref() {
    let a = not_nan(f32::INFINITY);
    let b = not_nan(f32::NEG_INFINITY);
    let _c = a + &b;
}

#[test]
#[should_panic]
fn test_add_fails_on_nan_ref_ref() {
    let a = not_nan(f32::INFINITY);
    let b = not_nan(f32::NEG_INFINITY);
    let _c = &a + &b;
}

#[test]
#[should_panic]
fn test_add_fails_on_nan_t_ref() {
    let a = not_nan(f32::INFINITY);
    let b = f32::NEG_INFINITY;
    let _c = a + &b;
}

#[test]
#[should_panic]
fn test_add_fails_on_nan_ref_t_ref() {
    let a = not_nan(f32::INFINITY);
    let b = f32::NEG_INFINITY;
    let _c = &a + &b;
}

#[test]
#[should_panic]
fn test_add_fails_on_nan_ref_t() {
    let a = not_nan(f32::INFINITY);
    let b = f32::NEG_INFINITY;
    let _c = &a + b;
}

#[test]
#[should_panic]
fn test_add_assign_fails_on_nan_ref() {
    let mut a = not_nan(f32::INFINITY);
    let b = not_nan(f32::NEG_INFINITY);
    a += &b;
}

#[test]
#[should_panic]
fn test_add_assign_fails_on_nan_t_ref() {
    let mut a = not_nan(f32::INFINITY);
    let b = f32::NEG_INFINITY;
    a += &b;
}

#[test]
#[should_panic]
fn test_add_assign_fails_on_nan_t() {
    let mut a = not_nan(f32::INFINITY);
    let b = f32::NEG_INFINITY;
    a += b;
}

#[test]
fn add() {
    assert_eq!(not_nan(0.0) + not_nan(0.0), 0.0);
    assert_eq!(not_nan(0.0) + &not_nan(0.0), 0.0);
    assert_eq!(&not_nan(0.0) + not_nan(0.0), 0.0);
    assert_eq!(&not_nan(0.0) + &not_nan(0.0), 0.0);
    assert_eq!(not_nan(0.0) + 0.0, 0.0);
    assert_eq!(not_nan(0.0) + &0.0, 0.0);
    assert_eq!(&not_nan(0.0) + 0.0, 0.0);
    assert_eq!(&not_nan(0.0) + &0.0, 0.0);

    assert_eq!(OrderedFloat(0.0) + OrderedFloat(0.0), 0.0);
    assert_eq!(OrderedFloat(0.0) + &OrderedFloat(0.0), 0.0);
    assert_eq!(&OrderedFloat(0.0) + OrderedFloat(0.0), 0.0);
    assert_eq!(&OrderedFloat(0.0) + &OrderedFloat(0.0), 0.0);
    assert_eq!(OrderedFloat(0.0) + 0.0, 0.0);
    assert_eq!(OrderedFloat(0.0) + &0.0, 0.0);
    assert_eq!(&OrderedFloat(0.0) + 0.0, 0.0);
    assert_eq!(&OrderedFloat(0.0) + &0.0, 0.0);
}

#[test]
fn ordered_f32_neg() {
    assert_eq!(OrderedFloat(-7.0f32), -OrderedFloat(7.0f32));
}

#[test]
fn ordered_f64_neg() {
    assert_eq!(OrderedFloat(-7.0f64), -OrderedFloat(7.0f64));
}

#[test]
#[should_panic]
fn test_sum_fails_on_nan() {
    let a = not_nan(f32::INFINITY);
    let b = not_nan(f32::NEG_INFINITY);
    let _c: NotNan<_> = [a, b].iter().sum();
}

#[test]
#[should_panic]
fn test_product_fails_on_nan() {
    let a = not_nan(f32::INFINITY);
    let b = not_nan(0f32);
    let _c: NotNan<_> = [a, b].iter().product();
}

#[test]
fn not_nan64_sum_product() {
    let a = not_nan(2138.1237);
    let b = not_nan(132f64);
    let c = not_nan(5.1);

    assert_eq!(
        std::iter::empty::<NotNan<f64>>().sum::<NotNan<_>>(),
        NotNan::new(0f64).unwrap()
    );
    assert_eq!([a].iter().sum::<NotNan<_>>(), a);
    assert_eq!([a, b].iter().sum::<NotNan<_>>(), a + b);
    assert_eq!([a, b, c].iter().sum::<NotNan<_>>(), a + b + c);

    assert_eq!(
        std::iter::empty::<NotNan<f64>>().product::<NotNan<_>>(),
        NotNan::new(1f64).unwrap()
    );
    assert_eq!([a].iter().product::<NotNan<_>>(), a);
    assert_eq!([a, b].iter().product::<NotNan<_>>(), a * b);
    assert_eq!([a, b, c].iter().product::<NotNan<_>>(), a * b * c);
}

#[test]
fn not_nan_usage_in_const_context() {
    const A: NotNan<f32> = unsafe { NotNan::new_unchecked(111f32) };
    assert_eq!(A, NotNan::new(111f32).unwrap());
}

#[test]
fn not_nan_panic_safety() {
    let catch_op = |mut num, op: fn(&mut NotNan<_>)| {
        let mut num_ref = panic::AssertUnwindSafe(&mut num);
        let _ = panic::catch_unwind(move || op(&mut num_ref));
        num
    };

    assert!(!catch_op(not_nan(f32::INFINITY), |a| *a += f32::NEG_INFINITY).is_nan());
    assert!(!catch_op(not_nan(f32::INFINITY), |a| *a -= f32::INFINITY).is_nan());
    assert!(!catch_op(not_nan(0.0), |a| *a *= f32::INFINITY).is_nan());
    assert!(!catch_op(not_nan(0.0), |a| *a /= 0.0).is_nan());
    assert!(!catch_op(not_nan(0.0), |a| *a %= 0.0).is_nan());
}

#[test]
fn from_ref() {
    let f = 1.0f32;
    let o: &OrderedFloat<f32> = (&f).into();
    assert_eq!(*o, 1.0f32);

    let mut f = 1.0f64;
    let o: &OrderedFloat<f64> = (&f).into();
    assert_eq!(*o, 1.0f64);

    let o: &mut OrderedFloat<f64> = (&mut f).into();
    assert_eq!(*o, 1.0f64);
    *o = OrderedFloat(2.0);
    assert_eq!(*o, 2.0f64);
    assert_eq!(f, 2.0f64);
}

macro_rules! test_float_const_method {
    ($type:ident < $inner:ident >, $method:ident) => {
        assert_eq!($type::<$inner>::$method().into_inner(), $inner::$method())
    };
}

macro_rules! test_float_const_methods {
    ($type:ident < $inner:ident >) => {
        test_float_const_method!($type<$inner>, E);
        test_float_const_method!($type<$inner>, FRAC_1_PI);
        test_float_const_method!($type<$inner>, FRAC_1_SQRT_2);
        test_float_const_method!($type<$inner>, FRAC_2_PI);
        test_float_const_method!($type<$inner>, FRAC_2_SQRT_PI);
        test_float_const_method!($type<$inner>, FRAC_PI_2);
        test_float_const_method!($type<$inner>, FRAC_PI_3);
        test_float_const_method!($type<$inner>, FRAC_PI_4);
        test_float_const_method!($type<$inner>, FRAC_PI_6);
        test_float_const_method!($type<$inner>, FRAC_PI_8);
        test_float_const_method!($type<$inner>, LN_10);
        test_float_const_method!($type<$inner>, LN_2);
        test_float_const_method!($type<$inner>, LOG10_E);
        test_float_const_method!($type<$inner>, LOG2_E);
        test_float_const_method!($type<$inner>, PI);
        test_float_const_method!($type<$inner>, SQRT_2);
    };
}

#[test]
fn float_consts_equal_inner() {
    test_float_const_methods!(OrderedFloat<f64>);
    test_float_const_methods!(OrderedFloat<f32>);
    test_float_const_methods!(NotNan<f64>);
    test_float_const_methods!(NotNan<f32>);
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_pow_ord {
    ($type:ident < $inner:ident >) => {
        assert_eq!($type::<$inner>::from(3.0).pow(2i8), OrderedFloat(9.0));
        assert_eq!($type::<$inner>::from(3.0).pow(2i16), OrderedFloat(9.0));
        assert_eq!($type::<$inner>::from(3.0).pow(2i32), OrderedFloat(9.0));
        assert_eq!($type::<$inner>::from(3.0).pow(2u8), OrderedFloat(9.0));
        assert_eq!($type::<$inner>::from(3.0).pow(2u16), OrderedFloat(9.0));
        assert_eq!($type::<$inner>::from(3.0).pow(2f32), OrderedFloat(9.0));
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! test_pow_nn {
    ($type:ident < $inner:ident >) => {
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2i8),
            NotNan::new(9.0).unwrap()
        );
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2u8),
            NotNan::new(9.0).unwrap()
        );
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2i16),
            NotNan::new(9.0).unwrap()
        );
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2u16),
            NotNan::new(9.0).unwrap()
        );
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2i32),
            NotNan::new(9.0).unwrap()
        );
        assert_eq!(
            $type::<$inner>::new(3.0).unwrap().pow(2f32),
            NotNan::new(9.0).unwrap()
        );
    };
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
fn test_pow_works() {
    assert_eq!(OrderedFloat(3.0).pow(OrderedFloat(2.0)), OrderedFloat(9.0));
    test_pow_ord!(OrderedFloat<f32>);
    test_pow_ord!(OrderedFloat<f64>);
    assert_eq!(
        NotNan::new(3.0).unwrap().pow(NotNan::new(2.0).unwrap()),
        NotNan::new(9.0).unwrap()
    );
    test_pow_nn!(NotNan<f32>);
    test_pow_nn!(NotNan<f64>);
    // Only f64 have Pow<f64> impl by default, so checking those seperate from macro
    assert_eq!(OrderedFloat::<f64>::from(3.0).pow(2f64), OrderedFloat(9.0));
    assert_eq!(
        NotNan::<f64>::new(3.0).unwrap().pow(2f64),
        NotNan::new(9.0).unwrap()
    );
}

#[cfg(any(feature = "std", feature = "libm"))]
#[test]
#[should_panic]
fn test_pow_fails_on_nan() {
    let a = not_nan(-1.0);
    let b = f32::NAN;
    a.pow(b);
}

#[test]
fn test_ref_ref_binop_regression() {
    // repro from:
    // https://github.com/reem/rust-ordered-float/issues/91
    //
    // impl<'a, T> $imp<Self> for &'a OrderedFloat<T>
    // where
    //     &'a T: $imp
    // {
    //     type Output = OrderedFloat<<&'a T as $imp>::Output>;
    //     #[inline]
    //     fn $method(self, other: Self) -> Self::Output {
    //         OrderedFloat((self.0).$method(&other.0))
    //     }
    // }
    fn regression<T>(p: T) -> T
    where
        for<'a> &'a T: std::ops::Sub<&'a T, Output = T>,
    {
        &p - &p
    }

    assert_eq!(regression(0.0_f64), 0.0);

    let x = OrderedFloat(50.0);
    let y = OrderedFloat(40.0);
    assert_eq!(&x - &y, OrderedFloat(10.0));
}

#[cfg(feature = "arbitrary")]
mod arbitrary_test {
    use super::{NotNan, OrderedFloat};
    use arbitrary::{Arbitrary, Unstructured};

    #[test]
    fn exhaustive() {
        // Exhaustively search all patterns of sign and exponent bits plus a few mantissa bits.
        for high_bytes in 0..=u16::MAX {
            let [h1, h2] = high_bytes.to_be_bytes();

            // Each of these should not
            //   * panic,
            //   * return an error, or
            //   * need more bytes than given.
            let n32: NotNan<f32> = Unstructured::new(&[h1, h2, h1, h2])
                .arbitrary()
                .expect("NotNan<f32> failure");
            let n64: NotNan<f64> = Unstructured::new(&[h1, h2, h1, h2, h1, h2, h1, h2])
                .arbitrary()
                .expect("NotNan<f64> failure");
            let _: OrderedFloat<f32> = Unstructured::new(&[h1, h2, h1, h2])
                .arbitrary()
                .expect("OrderedFloat<f32> failure");
            let _: OrderedFloat<f64> = Unstructured::new(&[h1, h2, h1, h2, h1, h2, h1, h2])
                .arbitrary()
                .expect("OrderedFloat<f64> failure");

            // Check for violation of NotNan's property of never containing a NaN.
            assert!(!n32.into_inner().is_nan());
            assert!(!n64.into_inner().is_nan());
        }
    }

    #[test]
    fn size_hints() {
        assert_eq!(NotNan::<f32>::size_hint(0), (4, Some(4)));
        assert_eq!(NotNan::<f64>::size_hint(0), (8, Some(8)));
        assert_eq!(OrderedFloat::<f32>::size_hint(0), (4, Some(4)));
        assert_eq!(OrderedFloat::<f64>::size_hint(0), (8, Some(8)));
    }
}
