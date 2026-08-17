use core::f64;

use wide::*;

use bytemuck::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<f64x2>(), 16);
  assert_eq!(core::mem::align_of::<f64x2>(), 16);
}

#[test]
fn impl_add_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([5.0, 6.0]);
  let expected = f64x2::from([6.0, 8.0]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([5.0, -10.0]);
  let expected = f64x2::from([-4.0, 12.0]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([5.0, -10.0]);
  let expected = f64x2::from([5.0, -20.0]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_for_f64x2() {
  let a = f64x2::from([50.0, 2.0]);
  let b = f64x2::from([5.0, -10.0]);
  let expected = f64x2::from([10.0, -0.2]);
  let actual = a / b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_const_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let expected = f64x2::from([-1.0, 0.0]);
  let actual = a - 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_const_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let expected = f64x2::from([2.0, 4.0]);
  let actual = a * 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_const_for_f64x2() {
  let a = f64x2::from([1.0, 2.0]);
  let expected = f64x2::from([0.5, 1.0]);
  let actual = a / 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_f64x2() {
  let a = f64x2::from([0.0, 1.0]);
  let b = f64x2::from([1.0, 1.0]);
  let expected = f64x2::from([0.0, 1.0]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_f64x2() {
  let a = f64x2::from([0.0, 1.0]);
  let b = f64x2::from([1.0, 1.0]);
  let expected = f64x2::from([1.0, 1.0]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_f64x2() {
  let a = f64x2::from([0.0, 1.0]);
  let b = f64x2::from([1.0, 1.0]);
  let expected = f64x2::from([1.0, 0.0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_eq() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [0, -1];
  let actual: [i64; 2] = cast(a.cmp_eq(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_ne() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [-1, 0];
  let actual: [i64; 2] = cast(a.cmp_ne(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_ge() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [0, -1];
  let actual: [i64; 2] = cast(a.cmp_ge(b));
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.0, 4.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [-1, -1];
  let actual: [i64; 2] = cast(a.cmp_ge(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_gt() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [0, 0];
  let actual: [i64; 2] = cast(a.cmp_gt(b));
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.0, 4.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [-1, -1];
  let actual: [i64; 2] = cast(a.cmp_gt(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_le() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [-1, -1];
  let actual: [i64; 2] = cast(a.cmp_le(b));
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.0, 4.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [0, 0];
  let actual: [i64; 2] = cast(a.cmp_le(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_cmp_lt() {
  let a = f64x2::from([1.0, 2.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [-1, 0];
  let actual: [i64; 2] = cast(a.cmp_lt(b));
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.0, 4.0]);
  let b = f64x2::from([2.0, 2.0]);
  let expected: [i64; 2] = [0, 0];
  let actual: [i64; 2] = cast(a.cmp_lt(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_const_cmp_lt() {
  let a = f64x2::from([1.0, 2.0]);
  let expected: [i64; 2] = [-1, 0];
  let actual: [i64; 2] = cast(a.cmp_lt(2.0));
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.0, 4.0]);
  let expected: [i64; 2] = [0, 0];
  let actual: [i64; 2] = cast(a.cmp_lt(2.0));
  assert_eq!(expected, actual);

  let a = f64x2::from([3.0, 4.0]);
  let expected: [i64; 2] = [0, 0];
  let actual: [i64; 2] = cast(a.cmp_lt(a));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_blend() {
  let use_t: f64 = f64::from_bits(u64::MAX);
  let t = f64x2::from([1.0, 2.0]);
  let f = f64x2::from([5.0, 6.0]);
  let mask = f64x2::from([use_t, 0.0]);
  let expected = f64x2::from([1.0, 6.0]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_abs() {
  let a = f64x2::from([-1.0, 2.0]);
  let expected = f64x2::from([1.0, 2.0]);
  let actual = a.abs();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([-3.5, f64::NEG_INFINITY]);
  let expected = f64x2::from([3.5, f64::INFINITY]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_floor() {
  let a = f64x2::from([-1.1, 2.0]);
  let expected = f64x2::from([-2.0, 2.0]);
  let actual = a.floor();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([60.9, f64::INFINITY]);
  let expected = f64x2::from([60.0, f64::INFINITY]);
  let actual = a.floor();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_ceil() {
  let a = f64x2::from([-1.1, 2.0]);
  let expected = f64x2::from([-1.0, 2.0]);
  let actual = a.ceil();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([60.9, f64::NEG_INFINITY]);
  let expected = f64x2::from([61.0, f64::NEG_INFINITY]);
  let actual = a.ceil();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_fast_max() {
  let a = f64x2::from([-0.0, -5.0]);
  let b = f64x2::from([0.0, 3.0]);
  let expected = f64x2::from([0.0, 3.0]);
  let actual = a.fast_max(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let b = f64x2::from([2.0, f64::INFINITY]);
  let expected = f64x2::from([2.0, f64::INFINITY]);
  let actual = a.fast_max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_max() {
  let a = f64x2::from([-0.0, -5.0]);
  let b = f64x2::from([0.0, 3.0]);
  let expected = f64x2::from([0.0, 3.0]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let b = f64x2::from([2.0, f64::INFINITY]);
  let expected = f64x2::from([2.0, f64::INFINITY]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NAN, 5.0]);
  let b = f64x2::from([2.0, f64::NAN]);
  let expected = f64x2::from([2.0, 5.0]);
  let actual = a.max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_fast_min() {
  let a = f64x2::from([-0.0, -5.0]);
  let b = f64x2::from([0.0, 3.0]);
  let expected = f64x2::from([-0.0, -5.0]);
  let actual = a.fast_min(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let b = f64x2::from([2.0, f64::INFINITY]);
  let expected = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let actual = a.fast_min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_min() {
  let a = f64x2::from([-0.0, -5.0]);
  let b = f64x2::from([0.0, 3.0]);
  let expected = f64x2::from([-0.0, -5.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let b = f64x2::from([2.0, f64::INFINITY]);
  let expected = f64x2::from([f64::NEG_INFINITY, 5.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  let a = f64x2::from([f64::NAN, 5.0]);
  let b = f64x2::from([2.0, f64::NAN]);
  let expected = f64x2::from([2.0, 5.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_is_nan() {
  let a = f64x2::from([0.0, f64::NAN]);
  let expected = [0, u64::MAX];
  let actual: [u64; 2] = cast(a.is_nan());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_is_finite() {
  let a = f64x2::from([f64::NAN, 1.0]);
  let expected = [0, u64::MAX];
  let actual: [u64; 2] = cast(a.is_finite());
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([f64::INFINITY, f64::NEG_INFINITY]);
  let expected = [0, 0];
  let actual: [u64; 2] = cast(a.is_finite());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_round() {
  let a = f64x2::from([1.1, 2.5]);
  let expected = f64x2::from([1.0, 2.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([3.7, 4.0]);
  let expected = f64x2::from([4.0, 4.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([-1.1, -2.5]);
  let expected = f64x2::from([-1.0, -2.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([-3.7, -4.0]);
  let expected = f64x2::from([-4.0, -4.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([f64::INFINITY, f64::NEG_INFINITY]);
  let expected = f64x2::from([f64::INFINITY, f64::NEG_INFINITY]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([5.5, 5.0]);
  let expected = f64x2::from([6.0, 5.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from(f64::NAN);
  let expected: [u64; 2] = [u64::MAX; 2];
  let actual: [u64; 2] = cast(a.round().is_nan());
  assert_eq!(expected, actual);
  //
  let a = f64x2::from(-0.0);
  let expected = a;
  let actual = a.round();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_round_int() {
  for (f, i) in [
    (1.0, 1),
    (1.1, 1),
    (-2.1, -2),
    (2.5, 2),
    (0.0, 0),
    (-0.0, 0),
    (f64::NAN, 0),
    (f64::INFINITY, i64::MAX),
    (f64::NEG_INFINITY, i64::MIN),
  ]
  .iter()
  .copied()
  {
    let a = f64x2::from(f);
    let expected = i64x2::from(i);
    let actual = a.round_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f64x2_mul_add() {
  let a = f64x2::from([2.0, 3.0]);
  let b = f64x2::from([4.0, 5.0]);
  let c = f64x2::from([1.0, 1.0]);
  let expected = f64x2::from([9.0, 16.0]);
  let actual = a.mul_add(b, c);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_mul_neg_add() {
  let a = f64x2::from([2.0, 3.0]);
  let b = f64x2::from([4.0, 5.0]);
  let c = f64x2::from([1.0, 1.0]);
  let expected = f64x2::from([-7.0, -14.0]);
  let actual = a.mul_neg_add(b, c);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_flip_signs() {
  let a = f64x2::from([1.0, 1.0]);
  let b = f64x2::from([2.0, -3.0]);
  let expected = f64x2::from([1.0, -1.0]);
  let actual = a.flip_signs(b);
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([-1.0, -1.0]);
  let b = f64x2::from([4.0, -5.0]);
  let expected = f64x2::from([-1.0, 1.0]);
  let actual = a.flip_signs(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_copysign() {
  let a = f64x2::from([1.0, 1.0]);
  let b = f64x2::from([2.0, -3.0]);
  let expected = f64x2::from([1.0, -1.0]);
  let actual = a.copysign(b);
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([-1.0, -1.0]);
  let b = f64x2::from([4.0, -5.0]);
  let expected = f64x2::from([1.0, -1.0]);
  let actual = a.copysign(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_sin_cos() {
  for x in -2500..=2500 {
    let base = (x * 4) as f64;
    let angles = [base, base + 1.0];
    let (actual_sins, actual_coses) = f64x2::from(angles).sin_cos();
    for i in 0..2 {
      let angle = angles[i];
      let check = |name: &str, vals: f64x2, expected: f64| {
        let actual_arr: [f64; 2] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.00000006,
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

// FIXME: remove cfg requirement once masks as their own types are implemented
#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_asin_acos() {
  let inc = 1.0 / 2501.0 / 2.0;
  for x in -2500..=2500 {
    let base = (x * 2) as f64 * inc;
    let origs = [base, base + inc];
    let (actual_asins, actual_acoses) = f64x2::from(origs).asin_acos();
    for i in 0..2 {
      let orig = origs[i];
      let check = |name: &str, vals: f64x2, expected: f64| {
        let actual_arr: [f64; 2] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.000000000000001,
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_asin() {
  let inc = 1.0 / 2501.0 / 2.0;
  for x in -2500..=2500 {
    let base = (x * 2) as f64 * inc;
    let origs = [base, base + inc];
    let actual_asins = f64x2::from(origs).asin();
    for i in 0..2 {
      let orig = origs[i];
      let check = |name: &str, vals: f64x2, expected: f64| {
        let actual_arr: [f64; 2] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.000000000000001,
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_acos() {
  let inc = 1.0 / 2501.0 / 2.0;
  for x in -2500..=2500 {
    let base = (x * 2) as f64 * inc;
    let origs = [base, base + inc];
    let actual_acoses = f64x2::from(origs).acos();
    for i in 0..2 {
      let orig = origs[i];
      let check = |name: &str, vals: f64x2, expected: f64| {
        let actual_arr: [f64; 2] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.000000000000001,
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_atan() {
  let inc = 1.0 / 2501.0 / 2.0;
  for x in -2500..=2500 {
    let base = (x * 2) as f64 * inc;
    let origs = [base, base + inc];
    let actual_atans = f64x2::from(origs).atan();
    for i in 0..2 {
      let orig = origs[i];
      let check = |name: &str, vals: f64x2, expected: f64| {
        let actual_arr: [f64; 2] = cast(vals);
        let actual = actual_arr[i];
        assert!(
          (actual - expected).abs() < 0.000000000000001,
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_atan2() {
  let inc_y = 1.0 / 51.0 / 2.0;
  let inc_x = 1.0 / 2501.0 / 2.0;
  for y in -50..=50 {
    let base_y = (y * 2) as f64 * inc_y;
    let origs_y = [base_y, base_y + inc_y];
    let actual_y = f64x2::from(origs_y);
    for x in -2500..=2500 {
      let base_x = (x * 2) as f64 * inc_x;
      let origs_x = [base_x, base_x + inc_x];
      let actual_x = f64x2::from(origs_x);
      let actual_atan2s = actual_y.atan2(actual_x);
      for i in 0..2 {
        let orig_y = origs_y[i];
        let orig_x = origs_x[i];
        let check = |name: &str, vals: f64x2, expected: f64| {
          let actual_arr: [f64; 2] = cast(vals);
          let actual = actual_arr[i];
          assert!(
            (actual - expected).abs() < 0.000000000000001,
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
fn impl_f64x2_to_degrees() {
  let pi = core::f64::consts::PI;
  let a = f64x2::from([0.0, pi / 2.0]);
  let expected = f64x2::from([0.0, 90.0]);
  let actual = a.to_degrees();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([pi, pi * 2.0]);
  let expected = f64x2::from([180.0, 360.0]);
  let actual = a.to_degrees();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_to_radians() {
  let pi = core::f64::consts::PI;
  let a = f64x2::from([0.0, 90.0]);
  let expected = f64x2::from([0.0, pi / 2.0]);
  let actual = a.to_radians();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([180.0, 360.0]);
  let expected = f64x2::from([pi, pi * 2.0]);
  let actual = a.to_radians();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_sqrt() {
  for (f, e) in [
    (f64::INFINITY, f64::INFINITY),
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
    let expected = f64x2::from(e);
    let actual = f64x2::from(f).sqrt();
    assert_eq!(expected, actual);
  }
  assert_eq!(
    cast::<_, i64x2>(f64x2::from(f64::NAN).sqrt().is_nan()),
    i64x2::from(-1)
  );
  assert_eq!(
    cast::<_, i64x2>(f64x2::from(f64::NEG_INFINITY).sqrt().is_nan()),
    i64x2::from(-1)
  );
  assert_eq!(
    cast::<_, i64x2>(f64x2::from(-1.0).sqrt().is_nan()),
    i64x2::from(-1)
  );
}

#[test]
fn test_f64x2_move_mask() {
  let a = f64x2::from([-1.0, 0.0]);
  let expected = 0b01;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = f64x2::from([1.0, -0.0]);
  let expected = 0b10;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f64x2_exp() {
  for f in [(-2.0), (-1.0), (0.0), (1.0), (1.5), (2.0), (10.0)].iter().copied()
  {
    let expected = f64x2::from((f as f64).exp());
    let actual = f64x2::from(f).exp();
    let diff_from_std: [f64; 2] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000000000000001);
  }
}

#[test]
fn test_f64x2_any() {
  let a = f64x2::from([-1.0, f64::NAN]).is_nan();
  assert!(a.any());
  //
  let a = f64x2::from([1.0, 0.0]).is_nan();
  assert!(!a.any());
}

#[test]
fn test_f64x2_all() {
  let a = f64x2::from([f64::NAN, f64::NAN]).is_nan();
  assert!(a.all());
  //
  let a = f64x2::from([1.0, f64::NAN]).is_nan();
  assert!(!a.all());
}

#[test]
fn test_f64x2_none() {
  let a = f64x2::from([1.0, 0.0]).is_nan();
  assert!(a.none());
  //
  let a = f64x2::from([1.0, f64::NAN]).is_nan();
  assert!(!a.none());
}

#[test]
fn impl_f64x2_ln() {
  for f in [0.1f64, 0.5, 1.0, 2.718282, 10.0, 35.0, 1250.0].iter().copied() {
    let expected = f64x2::from((f as f64).ln());
    let actual = f64x2::from(f).ln();
    let diff_from_std: [f64; 2] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000000000001);
  }
}

#[test]
fn impl_f64x2_pow_single() {
  for f in [0.1, 0.5, 1.0, 2.718282, 3.0, 4.0, 2.5, -1.0].iter().copied() {
    let expected = f64x2::splat(2.0 as f64).powf(f);
    let actual = f64x2::from(2.0_f64.powf(f));
    let diff_from_std: [f64; 2] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000001);
  }
}

#[cfg(target_feature = "sse")]
#[test]
fn impl_f64x2_pow_nan() {
  for f in [3.4].iter().copied() {
    let expected: [f64; 2] = cast(f64x2::splat(-4.5 as f64).powf(f));
    let actual = (-4.5_f64).powf(f);
    dbg!(&actual);
    dbg!(&expected);
    assert!(expected[0].is_nan());
    assert!(actual.is_nan());
  }
}

#[test]
fn impl_f64x2_pow_multiple() {
  let p = f64x2::from([29.0, 0.1]);
  let f = f64x2::from([1.2, 2.0]);
  let res = f.pow_f64x2(p);

  let p: [f64; 2] = cast(p);
  let f: [f64; 2] = cast(f);
  let res: [f64; 2] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }

  let p = f64x2::from([2.718282, -0.2]);
  let f = f64x2::from([9.2, 6.1]);
  let res = f.pow_f64x2(p);

  let p: [f64; 2] = cast(p);
  let f: [f64; 2] = cast(f);
  let res: [f64; 2] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }

  let p = f64x2::from([-1.5, 3.4]);
  let f = f64x2::from([2.5, 4.5]);
  let res = f.pow_f64x2(p);

  let p: [f64; 2] = cast(p);
  let f: [f64; 2] = cast(f);
  let res: [f64; 2] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      dbg!(expected);
      dbg!(res[i]);
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }
}

#[test]
fn impl_f64x2_reduce_add() {
  let p = f64x2::splat(0.001);
  assert_eq!(p.reduce_add(), 0.002);
}

#[test]
fn impl_f64x2_sum() {
  let mut p = Vec::with_capacity(250_000);
  for _ in 0..500_000 {
    p.push(f64x2::splat(0.001));
  }
  let now = std::time::Instant::now();
  let sum: f64 = p.iter().map(|x| x.reduce_add()).sum();
  let duration = now.elapsed().as_micros();
  println!("Time take {} {}us", sum, duration);

  let p = vec![0.001; 1_000_000];
  let now = std::time::Instant::now();
  let sum2: f64 = p.iter().sum();
  let duration = now.elapsed().as_micros();
  println!("Time take {} {}us", sum2, duration);
}

#[test]
fn impl_f64x2_from_i32x4() {
  let i = i32x4::from([1, 2, 3, 4]);
  let f = f64x2::from([1.0, 2.0]);
  assert_eq!(f64x2::from_i32x4_lower2(i), f)
}

#[cfg(feature = "serde")]
#[test]
fn impl_f64x2_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&f64x2::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(f64x2::ZERO, deserialized);
}
