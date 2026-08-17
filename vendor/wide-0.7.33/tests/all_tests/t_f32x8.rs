use wide::*;

use bytemuck::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<f32x8>(), 32);
  assert_eq!(core::mem::align_of::<f32x8>(), 32);
}

#[test]
fn impl_debug_for_f32x8() {
  let expected = "(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0)";
  let actual =
    format!("{:?}", f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
  assert_eq!(expected, actual);

  let expected = "(1.000, 2.000, 3.000, 4.000, 5.000, 6.000, 7.000, 8.000)";
  let actual =
    format!("{:.3?}", f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
  assert_eq!(expected, actual);
}

#[test]
fn impl_add_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = f32x8::from([5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
  let expected = f32x8::from([6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_add_const_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let expected = f32x8::from([6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0]);
  let actual = a + 5.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_const_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let expected = f32x8::from([-1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
  let actual = a - 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_const_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let expected = f32x8::from([2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
  let actual = a * 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_const_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let expected = f32x8::from([0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0]);
  let actual = a / 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = f32x8::from([5.0, 7.0, 17.0, 1.0, 1.0, 9.0, 2.0, 6.0]);
  let expected = f32x8::from([-4.0, -5.0, -14.0, 3.0, 4.0, -3.0, 5.0, 2.0]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_f32x8() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = f32x8::from([5.0, 7.0, 17.0, 1.0, 5.0, 6.0, 7.0, 8.0]);
  let expected = f32x8::from([5.0, 14.0, 51.0, 4.0, 25.0, 36.0, 49.0, 64.0]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_for_f32x8() {
  let a = f32x8::from([4.0, 9.0, 10.0, 12.0, 5.0, 6.0, 7.0, 8.0]);
  let b = f32x8::from([2.0, 2.0, 5.0, -3.0, 2.0, 1.5, 3.0, 2.5]);
  let expected = f32x8::from([2.0, 4.5, 2.0, -4.0, 2.5, 4.0, 2.3333333, 3.2]);
  let actual = a / b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_f32x8() {
  let a = f32x8::from([0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0]);
  let b = f32x8::from([0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
  let expected = f32x8::from([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_f32x8() {
  let a = f32x8::from([0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0]);
  let b = f32x8::from([0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
  let expected = f32x8::from([0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_f32x8() {
  let a = f32x8::from([0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0]);
  let b = f32x8::from([0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0]);
  let expected = f32x8::from([0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_eq() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 1.0]);
  let b = f32x8::from([2.0; 8]);
  let expected: [i32; 8] = [0, -1, 0, 0, 0, 0, -1, 0];
  let actual: [i32; 8] = cast(a.cmp_eq(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_ne() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 1.0]);
  let b = f32x8::from([2.0; 8]);
  let expected: [i32; 8] = [-1, 0, -1, -1, -1, -1, 0, -1];
  let actual: [i32; 8] = cast(a.cmp_ne(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_ge() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 1.0]);
  let b = f32x8::from([2.0; 8]);
  let expected: [i32; 8] = [0, -1, -1, -1, -1, -1, -1, 0];
  let actual: [i32; 8] = cast(a.cmp_ge(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_gt() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 3.0, 1.0]);
  let b = f32x8::from([3.0; 8]);
  let expected: [i32; 8] = [0, 0, 0, -1, -1, -1, 0, 0];
  let actual: [i32; 8] = cast(a.cmp_gt(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_le() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 1.0]);
  let b = f32x8::from([4.0; 8]);
  let expected: [i32; 8] = [-1, -1, -1, -1, 0, 0, -1, -1];
  let actual: [i32; 8] = cast(a.cmp_le(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_cmp_lt() {
  let a = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 2.0, 1.0]);
  let b = f32x8::from([3.0; 8]);
  let expected: [i32; 8] = [-1, -1, 0, 0, 0, 0, -1, -1];
  let actual: [i32; 8] = cast(a.cmp_lt(b));
  assert_eq!(expected, actual);

  let expected: [i32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
  let actual: [i32; 8] = cast(a.cmp_lt(a));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_blend() {
  let use_t: f32 = f32::from_bits(u32::MAX);
  let t = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let f = f32x8::from([5.0, 6.0, 7.0, 8.0, 21.0, 22.0, 23.0, 24.0]);
  let mask = f32x8::from([use_t, 0.0, use_t, 0.0, 0.0, 0.0, 0.0, use_t]);
  let expected = f32x8::from([1.0, 6.0, 3.0, 8.0, 21.0, 22.0, 23.0, 8.0]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_abs() {
  let a =
    f32x8::from([-1.0, 2.0, -3.5, f32::NEG_INFINITY, 6.0, 15.0, -19.0, -9.0]);
  let expected =
    f32x8::from([1.0, 2.0, 3.5, f32::INFINITY, 6.0, 15.0, 19.0, 9.0]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_floor() {
  let a = f32x8::from([-1.1, 60.9, 1.1, f32::INFINITY, 96.6, -53.2, 0.1, 9.2]);
  let expected =
    f32x8::from([-2.0, 60.0, 1.0, f32::INFINITY, 96.0, -54.0, 0.0, 9.0]);
  let actual = a.floor();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x4_ceil() {
  let a =
    f32x8::from([-1.1, 60.9, 1.1, f32::NEG_INFINITY, 96.6, -53.2, 0.1, 9.2]);
  let expected =
    f32x8::from([-1.0, 61.0, 2.0, f32::NEG_INFINITY, 97.0, -53.0, 1.0, 10.0]);
  let actual = a.ceil();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_fast_max() {
  let a = f32x8::from([1.0, 5.0, 3.0, 0.0, 6.0, -8.0, 12.0, 9.0]);
  let b = f32x8::from([2.0, -3.0, f32::INFINITY, 10.0, 19.0, -5.0, -1.0, -9.0]);
  let expected =
    f32x8::from([2.0, 5.0, f32::INFINITY, 10.0, 19.0, -5.0, 12.0, 9.0]);
  let actual = a.fast_max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_max() {
  let a = f32x8::from([1.0, 5.0, 3.0, f32::NAN, 6.0, -8.0, 12.0, f32::NAN]);
  let b =
    f32x8::from([2.0, -3.0, f32::INFINITY, 10.0, 19.0, f32::NAN, -1.0, -9.0]);
  let expected =
    f32x8::from([2.0, 5.0, f32::INFINITY, 10.0, 19.0, -8.0, 12.0, -9.0]);
  let actual = a.max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_fast_min() {
  let a = f32x8::from([1.0, 5.0, 3.0, f32::NEG_INFINITY, 6.0, -8.0, 12.0, 9.0]);
  let b = f32x8::from([2.0, -3.0, f32::INFINITY, 10.0, 19.0, -5.0, -1.0, -9.0]);
  let expected =
    f32x8::from([1.0, -3.0, 3.0, f32::NEG_INFINITY, 6.0, -8.0, -1.0, -9.0]);
  let actual = a.fast_min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_min() {
  let a =
    f32x8::from([1.0, 5.0, 3.0, f32::NEG_INFINITY, 6.0, -8.0, 12.0, f32::NAN]);
  let b =
    f32x8::from([2.0, -3.0, f32::INFINITY, 10.0, 19.0, f32::NAN, -1.0, -9.0]);
  let expected =
    f32x8::from([1.0, -3.0, 3.0, f32::NEG_INFINITY, 6.0, -8.0, -1.0, -9.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_is_nan() {
  let a = f32x8::from([0.0, f32::NAN, f32::NAN, 0.0, 0.0, 0.0, f32::NAN, 0.0]);
  let expected: [u32; 8] = [0, u32::MAX, u32::MAX, 0, 0, 0, u32::MAX, 0];
  let actual: [u32; 8] = cast(a.is_nan());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_is_finite() {
  let a = f32x8::from([
    f32::NAN,
    1.0,
    f32::INFINITY,
    f32::NEG_INFINITY,
    2.0,
    5.0,
    f32::INFINITY,
    9.0,
  ]);
  let expected: [u32; 8] = [0, u32::MAX, 0, 0, u32::MAX, u32::MAX, 0, u32::MAX];
  let actual: [u32; 8] = cast(a.is_finite());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_round() {
  let a = f32x8::from([1.1, 2.5, 3.7, 4.0, 7.2, 10.5, 12.7, 35.12]);
  let expected = f32x8::from([1.0, 2.0, 4.0, 4.0, 7.0, 10.0, 13.0, 35.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x8::from([-1.1, -2.5, -3.7, -4.0, -7.2, -10.5, -12.7, -35.12]);
  let expected =
    f32x8::from([-1.0, -2.0, -4.0, -4.0, -7.0, -10.0, -13.0, -35.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x8::from([
    f32::INFINITY,
    f32::NEG_INFINITY,
    5.5,
    5.0,
    7.2,
    10.5,
    12.7,
    35.12,
  ]);
  let expected = f32x8::from([
    f32::INFINITY,
    f32::NEG_INFINITY,
    6.0,
    5.0,
    7.0,
    10.0,
    13.0,
    35.0,
  ]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x8::from(f32::NAN);
  let expected: [u32; 8] = [u32::MAX; 8];
  let actual: [u32; 8] = cast(a.round().is_nan());
  assert_eq!(expected, actual);
  //
  let a = f32x8::from(-0.0);
  let expected = a;
  let actual = a.round();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_fast_round_int() {
  for (f, i) in [(1.0, 1), (1.1, 1), (-2.1, -2), (2.5, 2), (0.0, 0), (-0.0, 0)]
    .iter()
    .copied()
  {
    let a = f32x8::from(f);
    let expected = i32x8::from(i);
    let actual = a.fast_round_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x8_round_int() {
  for (f, i) in [
    (1.0, 1),
    (1.1, 1),
    (-2.1, -2),
    (2.5, 2),
    (0.0, 0),
    (-0.0, 0),
    (f32::NAN, 0),
    (f32::INFINITY, i32::MAX),
    (f32::NEG_INFINITY, i32::MIN),
  ]
  .iter()
  .copied()
  {
    let a = f32x8::from(f);
    let expected = i32x8::from(i);
    let actual = a.round_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x8_fast_trunc_int() {
  for (f, i) in [(1.0, 1), (1.1, 1), (-2.1, -2), (2.5, 2), (3.7, 3), (-0.0, 0)]
    .iter()
    .copied()
  {
    let a = f32x8::from(f);
    let expected = i32x8::from(i);
    let actual = a.fast_trunc_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x8_trunc_int() {
  for (f, i) in [
    (1.0, 1),
    (1.1, 1),
    (-2.1, -2),
    (2.5, 2),
    (3.7, 3),
    (-0.0, 0),
    (f32::NAN, 0),
    (f32::INFINITY, i32::MAX),
    (f32::NEG_INFINITY, i32::MIN),
  ]
  .iter()
  .copied()
  {
    let a = f32x8::from(f);
    let expected = i32x8::from(i);
    let actual = a.trunc_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x8_mul_add() {
  let a = f32x8::from([2.0, 3.0, 4.0, 5.0, 6.7, 9.2, 11.5, 12.2]);
  let b = f32x8::from([4.0, 5.0, 6.0, 7.0, 1.5, 8.9, 4.2, 5.6]);
  let c = f32x8::from([1.0; 8]);
  let expected: [f32; 8] =
    cast(f32x8::from([9.0, 16.0, 25.0, 36.0, 11.05, 82.88, 49.3, 69.32]));
  let actual: [f32; 8] = cast(a.mul_add(b, c));
  for (act, exp) in actual.iter().zip(expected.iter()) {
    assert!((exp - act).abs() < 0.000001);
  }
}

#[test]
fn impl_f32x8_mul_neg_add() {
  let a = f32x8::from([2.0, 3.0, 4.0, 5.0, 6.7, 9.2, 11.5, 12.2]);
  let b = f32x8::from([4.0, 5.0, 6.0, 7.0, 1.5, 8.9, 4.2, -5.6]);
  let c = f32x8::from([1.0; 8]);
  let expected: [f32; 8] =
    cast(f32x8::from([-7.0, -14.0, -23.0, -34.0, -9.05, -80.88, -47.3, 69.32]));
  let actual: [f32; 8] = cast(a.mul_neg_add(b, c));
  for (act, exp) in actual.iter().zip(expected.iter()) {
    assert!((exp - act).abs() < 0.00001);
  }
}

#[test]
fn impl_f32x8_flip_signs() {
  let a = f32x8::from([1.0, 1.0, -1.0, -1.0, 5.2, 6.7, -8.2, -12.5]);
  let b = f32x8::from([2.0, -3.0, 4.0, -5.0, 5.2, 6.7, -8.2, -12.5]);
  let expected = f32x8::from([1.0, -1.0, -1.0, 1.0, 5.2, 6.7, 8.2, 12.5]);
  let actual = a.flip_signs(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_copysign() {
  let a = f32x8::from([1.0, 1.0, -1.0, -1.0, 5.2, 6.7, -8.2, -12.5]);
  let b = f32x8::from([2.0, -3.0, 4.0, -5.0, 5.2, 6.7, -8.2, -12.5]);
  let expected = f32x8::from([1.0, -1.0, 1.0, -1.0, 5.2, 6.7, -8.2, -12.5]);
  let actual = a.copysign(b);
  assert_eq!(expected, actual);
}

// NOTE: Disabled
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x8_asin_acos() {
  let inc = 1.0 / 2501.0 / 8.0;
  for x in -2500..=2500 {
    let base = (x * 8) as f32 * inc;
    let origs = [
      base,
      base + inc,
      base + 2.0 * inc,
      base + 3.0 * inc,
      base + 4.0 * inc,
      base + 5.0 * inc,
      base + 6.0 * inc,
      base + 7.0 * inc,
    ];
    let (actual_asins, actual_acoses) = f32x8::from(origs).asin_acos();
    for i in 0..8 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x8, expected: f32| {
        let actual_arr: [f32; 8] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.0000006,
          "Wanted {name}({orig}) to be {expected} but got {actual}",
          name = name,
          orig = orig,
          expected = expected,
          actual = actual
        );
      };
      check("asin", actual_asins, orig.asin());
      check("acos", actual_acoses, orig.acos());
    }
  }
}

// FIXME: remove cfg requirement once masks as their own types are implemented
#[cfg(target_feature = "avx")]
#[test]
fn impl_f32x8_asin() {
  let inc = 1.0 / 2501.0 / 8.0;
  for x in -2500..=2500 {
    let base = (x * 4) as f32 * inc;
    let origs = [
      base,
      base + inc,
      base + 2.0 * inc,
      base + 3.0 * inc,
      base + 4.0 * inc,
      base + 5.0 * inc,
      base + 6.0 * inc,
      base + 7.0 * inc,
    ];
    let actual_asins = f32x8::from(origs).asin();
    for i in 0..8 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x8, expected: f32| {
        let actual_arr: [f32; 8] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.0000006,
          "Wanted {name}({orig}) to be {expected} but got {actual}",
          name = name,
          orig = orig,
          expected = expected,
          actual = actual
        );
      };
      check("asin", actual_asins, orig.asin());
    }
  }
}

// FIXME: remove cfg requirement once masks as their own types are implemented
#[cfg(target_feature = "avx")]
#[test]
fn impl_f32x8_acos() {
  let inc = 1.0 / 2501.0 / 8.0;
  for x in -2500..=2500 {
    let base = (x * 8) as f32 * inc;
    let origs = [
      base,
      base + inc,
      base + 2.0 * inc,
      base + 3.0 * inc,
      base + 4.0 * inc,
      base + 5.0 * inc,
      base + 6.0 * inc,
      base + 7.0 * inc,
    ];
    let actual_acoses = f32x8::from(origs).acos();
    for i in 0..8 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x8, expected: f32| {
        let actual_arr: [f32; 8] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.0000006,
          "Wanted {name}({orig}) to be {expected} but got {actual}",
          name = name,
          orig = orig,
          expected = expected,
          actual = actual
        );
      };
      check("acos", actual_acoses, orig.acos());
    }
  }
}

// FIXME: remove cfg requirement once masks as their own types are implemented
#[cfg(target_feature = "avx")]
#[test]
fn impl_f32x8_atan() {
  let inc = 1.0 / 2501.0 / 8.0;
  for x in -2500..=2500 {
    let base = (x * 8) as f32 * inc;
    let origs = [
      base,
      base + inc,
      base + 2.0 * inc,
      base + 3.0 * inc,
      base + 4.0 * inc,
      base + 5.0 * inc,
      base + 6.0 * inc,
      base + 7.0 * inc,
    ];
    let actual_atans = f32x8::from(origs).atan();
    for i in 0..8 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x8, expected: f32| {
        let actual_arr: [f32; 8] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.0000006,
          "Wanted {name}({orig}) to be {expected} but got {actual}",
          name = name,
          orig = orig,
          expected = expected,
          actual = actual
        );
      };
      check("atan", actual_atans, orig.atan());
    }
  }
}

// FIXME: remove cfg requirement once masks as their own types are implemented
#[cfg(target_feature = "avx")]
#[test]
fn impl_f32x8_atan2() {
  let inc_y = 1.0 / 51.0 / 8.0;
  let inc_x = 1.0 / 2501.0 / 8.0;
  for y in -50..=50 {
    let base_y = (y * 8) as f32 * inc_y;
    let origs_y = [
      base_y,
      base_y + inc_y,
      base_y + 2.0 * inc_y,
      base_y + 3.0 * inc_y,
      base_y + 4.0 * inc_y,
      base_y + 5.0 * inc_y,
      base_y + 6.0 * inc_y,
      base_y + 7.0 * inc_y,
    ];
    let actual_y = f32x8::from(origs_y);
    for x in -2500..=2500 {
      let base_x = (x * 8) as f32 * inc_x;
      let origs_x = [
        base_x,
        base_x + inc_x,
        base_x + 2.0 * inc_x,
        base_x + 3.0 * inc_x,
        base_x + 4.0 * inc_x,
        base_x + 5.0 * inc_x,
        base_x + 6.0 * inc_x,
        base_x + 7.0 * inc_x,
      ];
      let actual_x = f32x8::from(origs_x);
      let actual_atan2s = actual_y.atan2(actual_x);
      for i in 0..8 {
        let orig_y = origs_y[i];
        let orig_x = origs_x[i];
        let check = |name: &str, vals: f32x8, expected: f32| {
          let actual_arr: [f32; 8] = cast(vals);
          let actual = actual_arr[i];
          assert!(
          (actual - expected).abs() < 0.0000006,
          "Wanted {name}({orig_y}, {orig_x}) to be {expected} but got {actual}",
          name = name,
          orig_y = orig_y,
          orig_x = orig_x,
          expected = expected,
          actual = actual
        );
        };
        check("atan2", actual_atan2s, orig_y.atan2(orig_x));
      }
    }
  }
}

#[test]
fn impl_f32x8_sin_cos() {
  for x in -2500..=2500 {
    let base = (x * 4) as f32;
    let angles = [
      base,
      base + 1.0,
      base + 2.0,
      base + 3.0,
      base + 4.0,
      base + 5.0,
      base + 6.0,
      base + 7.0,
    ];
    let (actual_sins, actual_coses) = f32x8::from(angles).sin_cos();
    for i in 0..4 {
      let angle = angles[i];
      let check = |name: &str, vals: f32x8, expected: f32| {
        let actual_arr: [f32; 8] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.0000002,
          "Wanted {name}({angle}) to be {expected} but got {actual}",
          name = name,
          angle = angle,
          expected = expected,
          actual = actual
        );
      };
      check("sin", actual_sins, angle.sin());
      check("cos", actual_coses, angle.cos());
    }
  }
}

#[test]
fn impl_f32x8_to_degrees() {
  let pi = core::f32::consts::PI;
  let a =
    f32x8::from([0.0, pi / 2.0, pi, 2.0 * pi, 0.0, pi / 2.0, pi, 2.0 * pi]);
  let expected =
    f32x8::from([0.0, 90.0, 180.0, 360.0, 0.0, 90.0, 180.0, 360.0]);
  let actual = a.to_degrees();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_to_radians() {
  let pi = core::f32::consts::PI;
  let a = f32x8::from([0.0, 90.0, 180.0, 360.0, 0.0, 90.0, 180.0, 360.0]);
  let expected =
    f32x8::from([0.0, pi / 2.0, pi, 2.0 * pi, 0.0, pi / 2.0, pi, 2.0 * pi]);
  let actual = a.to_radians();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x8_recip() {
  {
    let expected = f32x8::from(0.0);
    let actual = f32x8::from(f32::INFINITY).recip();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x8::from(0.0);
    let actual = f32x8::from(-f32::INFINITY).recip();
    assert_eq!(expected, actual);
  }
  {
    let actual = f32x8::from(f32::NAN).recip();
    assert!(actual.is_nan().any());
  }
  {
    let expected = f32x8::from(f32::INFINITY);
    let actual = f32x8::from(0.0).recip();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x8::from(0.49987793);
    let actual = f32x8::from(2.0).recip();
    let diff: [f32; 8] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
  {
    let expected = f32x8::from(-0.08102417);
    let actual = f32x8::from(-12.34).recip();
    let diff: [f32; 8] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
}

#[test]
fn impl_f32x8_recip_sqrt() {
  {
    let expected = f32x8::from(0.0);
    let actual = f32x8::from(f32::INFINITY).recip_sqrt();
    assert_eq!(expected, actual);
  }
  {
    let actual = f32x8::from(-f32::INFINITY).recip_sqrt();
    assert!(actual.is_nan().any());
  }
  {
    let actual = f32x8::from(f32::NAN).recip_sqrt();
    assert!(actual.is_nan().any());
  }
  {
    let expected = f32x8::from(f32::INFINITY);
    let actual = f32x8::from(0.0).recip_sqrt();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x8::from(0.70703125);
    let actual = f32x8::from(2.0).recip_sqrt();
    let diff: [f32; 8] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
  {
    let actual = f32x8::from(-12.34).recip_sqrt();
    assert!(actual.is_nan().any());
  }
}

#[test]
fn impl_f32x8_sqrt() {
  for (f, e) in [
    (f32::INFINITY, f32::INFINITY),
    (0.0, 0.0),
    (-0.0, -0.0),
    (4.0, 2.0),
    (9.0, 3.0),
    (16.0, 4.0),
    (25.0, 5.0),
    (5000.0 * 5000.0, 5000.0),
  ]
  .iter()
  .copied()
  {
    let expected = f32x8::from(e);
    let actual = f32x8::from(f).sqrt();
    assert_eq!(expected, actual);
  }
  assert_eq!(
    cast::<_, i32x8>(f32x8::from(f32::NAN).sqrt().is_nan()),
    i32x8::from(-1)
  );
  assert_eq!(
    cast::<_, i32x8>(f32x8::from(f32::NEG_INFINITY).sqrt().is_nan()),
    i32x8::from(-1)
  );
  assert_eq!(
    cast::<_, i32x8>(f32x8::from(-1.0).sqrt().is_nan()),
    i32x8::from(-1)
  );
}

#[test]
fn impl_f32x8_exp() {
  for f in [(-2.0), (-1.0), (0.0), (1.0), (1.5), (2.0), (10.0)].iter().copied()
  {
    let expected = f32x8::from((f as f32).exp());
    let actual = f32x8::from(f).exp();
    let diff_from_std: [f32; 8] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000000000000001);
  }
}

#[test]
fn test_f32x8_move_mask() {
  let a = f32x8::from([-1.0, 0.0, -2.0, -3.0, -1.0, 0.0, -2.0, -3.0]);
  let expected = 0b11011101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = f32x8::from([1.0, 0.0, 2.0, -3.0, 1.0, 0.0, 2.0, -3.0]);
  let expected = 0b10001000;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
}

#[test]
fn test_f32x8_any() {
  let a =
    f32x8::from([-1.0, 0.0, -2.0, -3.0, 2.0, -1.0, -2.0, f32::NAN]).is_nan();
  assert!(a.any());
  //
  let a = f32x8::from([1.0, 0.0, 2.0, 3.0, 2.0, 5.0, 6.7, 7.1]).is_nan();
  assert!(!a.any());
}

#[test]
fn test_f32x8_all() {
  let a = f32x8::from([f32::NAN; 8]).is_nan();
  assert!(a.all());
  //
  let a = f32x8::from([1.0, -0.0, 2.0, 3.0, 4.0, 9.0, 7.2, f32::NAN]).is_nan();
  assert!(!a.all());
}

#[test]
fn test_f32x8_none() {
  let a = f32x8::from([1.0, 0.0, 2.0, 3.0, 1.0, 0.0, 2.0, 3.0]).is_nan();
  assert!(a.none());
  //
  let a = f32x8::from([1.0, -0.0, 2.0, 3.0, 1.0, -0.0, 2.0, f32::NAN]).is_nan();
  assert!(!a.none());
}

#[test]
fn impl_f32x8_ln() {
  for f in [0.1, 0.5, 1.0, 2.718282, 10.0, 35.0, 1250.0].iter().copied() {
    let expected = f32x8::from((f as f32).ln());
    let actual = f32x8::from(f).ln();
    let diff_from_std: [f32; 8] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.0000001);
  }
}

#[test]
fn impl_f32x8_pow() {
  for f in [0.1, 0.5, 1.0, 2.718282, 3.0, 4.0, 2.5, -1.0].iter().copied() {
    let expected = f32x8::splat(2.0 as f32).powf(f);
    let actual = f32x8::from(2.0_f32.powf(f));
    let diff_from_std: [f32; 8] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000001);
  }
}

#[test]
fn impl_f32x8_pow_n() {
  let p = f32x8::from([29.0, 0.1, 0.5, 1.0, 2.718282, -0.2, -1.5, 3.4]);
  let f = f32x8::from([1.2, 2.0, 3.0, 1.5, 9.2, 6.1, 2.5, -4.5]);
  let res = f.pow_f32x8(p);

  let p: [f32; 8] = cast(p);
  let f: [f32; 8] = cast(f);
  let res: [f32; 8] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }
}

#[test]
fn impl_f32x8_reduce_add() {
  let p = f32x8::from([0.001, 0.002, 0.003, 0.004, 0.005, 0.006, 0.007, 0.009]);
  assert!((p.reduce_add() - 0.037) < 0.000000001);
}

#[test]
fn impl_f32x8_sum() {
  let mut p = Vec::with_capacity(250_000);
  for _ in 0..125_000 {
    p.push(f32x8::splat(0.001));
  }
  let now = std::time::Instant::now();
  let sum: f32 = p.iter().map(|x| x.reduce_add()).sum();
  let duration = now.elapsed().as_micros();
  println!("Time take {} {}us", sum, duration);

  let p = vec![0.001; 1_000_000];
  let now = std::time::Instant::now();
  let sum2: f32 = p.iter().sum();
  let duration = now.elapsed().as_micros();
  println!("Time take {} {}us", sum2, duration);
}

#[test]
fn impl_transpose_for_f32x8() {
  let a = [
    f32x8::new([0.1, 1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1]),
    f32x8::new([8.1, 9.1, 10.1, 11.1, 12.1, 13.1, 14.1, 15.1]),
    f32x8::new([16.1, 17.1, 18.1, 19.1, 20.1, 21.1, 22.1, 23.1]),
    f32x8::new([24.1, 25.1, 26.1, 27.1, 28.1, 29.1, 30.1, 31.1]),
    f32x8::new([32.1, 33.1, 34.1, 35.1, 36.1, 37.1, 38.1, 39.1]),
    f32x8::new([40.1, 41.1, 42.1, 43.1, 44.1, 45.1, 46.1, 47.1]),
    f32x8::new([48.1, 49.1, 50.1, 51.1, 52.1, 53.1, 54.1, 55.1]),
    f32x8::new([
      5600000.1, 5700000.1, 5800000.1, 5900000.1, 6000000.1, 6100000.1,
      6200000.1, 6300000.1,
    ]),
  ];

  let result = f32x8::transpose(a);

  let expected = [
    f32x8::new([0.1, 8.1, 16.1, 24.1, 32.1, 40.1, 48.1, 5600000.1]),
    f32x8::new([1.1, 9.1, 17.1, 25.1, 33.1, 41.1, 49.1, 5700000.1]),
    f32x8::new([2.1, 10.1, 18.1, 26.1, 34.1, 42.1, 50.1, 5800000.1]),
    f32x8::new([3.1, 11.1, 19.1, 27.1, 35.1, 43.1, 51.1, 5900000.1]),
    f32x8::new([4.1, 12.1, 20.1, 28.1, 36.1, 44.1, 52.1, 6000000.1]),
    f32x8::new([5.1, 13.1, 21.1, 29.1, 37.1, 45.1, 53.1, 6100000.1]),
    f32x8::new([6.1, 14.1, 22.1, 30.1, 38.1, 46.1, 54.1, 6200000.1]),
    f32x8::new([7.1, 15.1, 23.1, 31.1, 39.1, 47.1, 55.1, 6300000.1]),
  ];

  assert_eq!(result, expected);
}

#[test]
fn impl_f32x8_from_i32x8() {
  let i = i32x8::from([1, 2, 3, 4, 5, 6, 7, 8]);
  let f = f32x8::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  assert_eq!(f32x8::from_i32x8(i), f)
}

#[cfg(feature = "serde")]
#[test]
fn impl_f32x8_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&f32x8::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(f32x8::ZERO, deserialized);
}
