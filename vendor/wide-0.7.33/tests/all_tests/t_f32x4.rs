use wide::*;

use bytemuck::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<f32x4>(), 16);
  assert_eq!(core::mem::align_of::<f32x4>(), 16);
}

#[test]
fn impl_debug_for_f32x4() {
  let expected = "(1.0, 2.0, 3.0, 4.0)";
  let actual = format!("{:?}", f32x4::from([1.0, 2.0, 3.0, 4.0]));
  assert_eq!(expected, actual);

  let expected = "(1.000, 2.000, 3.000, 4.000)";
  let actual = format!("{:.3?}", f32x4::from([1.0, 2.0, 3.0, 4.0]));
  assert_eq!(expected, actual);
}

#[test]
fn impl_add_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([5.0, 6.0, 7.0, 8.0]);
  let expected = f32x4::from([6.0, 8.0, 10.0, 12.0]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_add_const_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let expected = f32x4::from([6.0, 7.0, 8.0, 9.0]);
  let actual = a + 5.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_const_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let expected = f32x4::from([-1.0, 0.0, 1.0, 2.0]);
  let actual = a - 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_const_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let expected = f32x4::from([2.0, 4.0, 6.0, 8.0]);
  let actual = a * 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_const_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let expected = f32x4::from([0.5, 1.0, 1.5, 2.0]);
  let actual = a / 2.0;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([5.0, 7.0, 17.0, 1.0]);
  let expected = f32x4::from([-4.0, -5.0, -14.0, 3.0]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_f32x4() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([5.0, 7.0, 17.0, 1.0]);
  let expected = f32x4::from([5.0, 14.0, 51.0, 4.0]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_div_for_f32x4() {
  let a = f32x4::from([4.0, 9.0, 10.0, 12.0]);
  let b = f32x4::from([2.0, 2.0, 5.0, -3.0]);
  let expected = f32x4::from([2.0, 4.5, 2.0, -4.0]);
  let actual = a / b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_f32x4() {
  let a = f32x4::from([0.0, 0.0, 1.0, 1.0]);
  let b = f32x4::from([0.0, 1.0, 0.0, 1.0]);
  let expected = f32x4::from([0.0, 0.0, 0.0, 1.0]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_f32x4() {
  let a = f32x4::from([0.0, 0.0, 1.0, 1.0]);
  let b = f32x4::from([0.0, 1.0, 0.0, 1.0]);
  let expected = f32x4::from([0.0, 1.0, 1.0, 1.0]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_f32x4() {
  let a = f32x4::from([0.0, 0.0, 1.0, 1.0]);
  let b = f32x4::from([0.0, 1.0, 0.0, 1.0]);
  let expected = f32x4::from([0.0, 1.0, 1.0, 0.0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_eq() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [0, -1, 0, 0];
  let actual: [i32; 4] = cast(a.cmp_eq(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_ne() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [-1, 0, -1, -1];
  let actual: [i32; 4] = cast(a.cmp_ne(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_ge() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [0, -1, -1, -1];
  let actual: [i32; 4] = cast(a.cmp_ge(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_gt() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [0, 0, -1, -1];
  let actual: [i32; 4] = cast(a.cmp_gt(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_le() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [-1, -1, 0, 0];
  let actual: [i32; 4] = cast(a.cmp_le(b));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_cmp_lt() {
  let a = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let b = f32x4::from([2.0, 2.0, 2.0, 2.0]);
  let expected: [i32; 4] = [-1, 0, 0, 0];
  let actual: [i32; 4] = cast(a.cmp_lt(b));
  assert_eq!(expected, actual);

  let expected: [i32; 4] = [0, 0, 0, 0];
  let actual: [i32; 4] = cast(a.cmp_lt(a));
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_blend() {
  let use_t: f32 = f32::from_bits(u32::MAX);
  let t = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  let f = f32x4::from([5.0, 6.0, 7.0, 8.0]);
  let mask = f32x4::from([use_t, 0.0, use_t, 0.0]);
  let expected = f32x4::from([1.0, 6.0, 3.0, 8.0]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_abs() {
  let a = f32x4::from([-1.0, 2.0, -3.5, f32::NEG_INFINITY]);
  let expected = f32x4::from([1.0, 2.0, 3.5, f32::INFINITY]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_floor() {
  let a = f32x4::from([-1.1, 60.9, 1.1, f32::INFINITY]);
  let expected = f32x4::from([-2.0, 60.0, 1.0, f32::INFINITY]);
  let actual = a.floor();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_ceil() {
  let a = f32x4::from([-1.1, 60.9, 1.1, f32::NEG_INFINITY]);
  let expected = f32x4::from([-1.0, 61.0, 2.0, f32::NEG_INFINITY]);
  let actual = a.ceil();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_fast_max() {
  let a = f32x4::from([1.0, 5.0, 3.0, -4.0]);
  let b = f32x4::from([2.0, 3.0, -5.0, -10.0]);
  let expected = f32x4::from([2.0, 5.0, 3.0, -4.0]);
  let actual = a.fast_max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_max() {
  let a = f32x4::from([1.0, 5.0, -3.0, -0.0]);
  let b = f32x4::from([2.0, -5.0, -1.0, 0.0]);
  let expected = f32x4::from([2.0, 5.0, -1.0, 0.0]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  let a = f32x4::from([1.0, 5.0, 3.0, f32::NAN]);
  let b = f32x4::from([f32::NAN, f32::NEG_INFINITY, f32::INFINITY, 10.0]);
  let expected = f32x4::from([1.0, 5.0, f32::INFINITY, 10.0]);
  let actual = a.max(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_fast_min() {
  let a = f32x4::from([1.0, 5.0, 3.0, -4.0]);
  let b = f32x4::from([2.0, 3.0, -5.0, -10.0]);
  let expected = f32x4::from([1.0, 3.0, -5.0, -10.0]);
  let actual = a.fast_min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_min() {
  let a = f32x4::from([1.0, 5.0, -3.0, -0.0]);
  let b = f32x4::from([2.0, -5.0, -1.0, 0.0]);
  let expected = f32x4::from([1.0, -5.0, -3.0, -0.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  let a = f32x4::from([1.0, 5.0, 3.0, f32::NAN]);
  let b = f32x4::from([f32::NAN, f32::NEG_INFINITY, f32::INFINITY, 10.0]);
  let expected = f32x4::from([1.0, f32::NEG_INFINITY, 3.0, 10.0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_is_nan() {
  let a = f32x4::from([0.0, f32::NAN, f32::NAN, 0.0]);
  let expected = [0, u32::MAX, u32::MAX, 0];
  let actual: [u32; 4] = cast(a.is_nan());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_is_finite() {
  let a = f32x4::from([f32::NAN, 1.0, f32::INFINITY, f32::NEG_INFINITY]);
  let expected = [0, u32::MAX, 0, 0];
  let actual: [u32; 4] = cast(a.is_finite());
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_is_inf() {
  let a = f32x4::from([f32::NAN, 1.0, f32::INFINITY, f32::NEG_INFINITY]);
  let expected = [0, 0, u32::MAX, u32::MAX];
  let actual: [u32; 4] = cast(a.is_inf());
  assert_eq!(expected, actual);
}
#[test]
fn impl_f32x4_round() {
  let a = f32x4::from([1.1, 2.5, 3.7, 4.0]);
  let expected = f32x4::from([1.0, 2.0, 4.0, 4.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([-1.1, -2.5, -3.7, -4.0]);
  let expected = f32x4::from([-1.0, -2.0, -4.0, -4.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([f32::INFINITY, f32::NEG_INFINITY, 5.5, 5.0]);
  let expected = f32x4::from([f32::INFINITY, f32::NEG_INFINITY, 6.0, 5.0]);
  let actual = a.round();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from(f32::NAN);
  let expected: [u32; 4] = [u32::MAX; 4];
  let actual: [u32; 4] = cast(a.round().is_nan());
  assert_eq!(expected, actual);
  //
  let a = f32x4::from(-0.0);
  let expected = a;
  let actual = a.round();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_fast_round_int() {
  for (f, i) in [(1.0, 1), (1.1, 1), (-2.1, -2), (2.5, 2), (0.0, 0), (-0.0, 0)]
    .iter()
    .copied()
  {
    let a = f32x4::from(f);
    let expected = i32x4::from(i);
    let actual = a.fast_round_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x4_round_int() {
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
    let a = f32x4::from(f);
    let expected = i32x4::from(i);
    let actual = a.round_int();
    assert_eq!(expected, actual);
  }
}

#[test]
fn impl_f32x4_fast_trunc_int() {
  let a = f32x4::from([1.1, 2.5, 3.7, 4.0]);
  let expected = i32x4::from([1, 2, 3, 4]);
  let actual = a.fast_trunc_int();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([-1.1, -2.5, -3.7, -4.0]);
  let expected = i32x4::from([-1, -2, -3, -4]);
  let actual = a.fast_trunc_int();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_trunc_int() {
  let a = f32x4::from([1.1, 2.5, 3.7, 4.0]);
  let expected = i32x4::from([1, 2, 3, 4]);
  let actual = a.trunc_int();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([-1.1, -2.5, -3.7, -4.0]);
  let expected = i32x4::from([-1, -2, -3, -4]);
  let actual = a.trunc_int();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([f32::NEG_INFINITY, f32::INFINITY, f32::NAN, 0.0]);
  let expected = i32x4::from([i32::MIN, i32::MAX, 0, 0]);
  let actual = a.trunc_int();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_mul_add() {
  let a = f32x4::from([2.0, 3.0, 4.0, 5.0]);
  let b = f32x4::from([4.0, 5.0, 6.0, 7.0]);
  let c = f32x4::from([1.0, 1.0, 1.0, 1.0]);
  let expected = f32x4::from([9.0, 16.0, 25.0, 36.0]);
  let actual = a.mul_add(b, c);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_mul_neg_add() {
  let a = f32x4::from([2.0, 3.0, 4.0, 5.0]);
  let b = f32x4::from([4.0, 5.0, 6.0, 7.0]);
  let c = f32x4::from([1.0, 1.0, 1.0, 1.0]);
  let expected = f32x4::from([-7.0, -14.0, -23.0, -34.0]);
  let actual = a.mul_neg_add(b, c);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_flip_signs() {
  let a = f32x4::from([1.0, 1.0, -1.0, -1.0]);
  let b = f32x4::from([2.0, -3.0, 4.0, -5.0]);
  let expected = f32x4::from([1.0, -1.0, -1.0, 1.0]);
  let actual = a.flip_signs(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_copysign() {
  let a = f32x4::from([1.0, 1.0, -1.0, -1.0]);
  let b = f32x4::from([2.0, -3.0, 4.0, -5.0]);
  let expected = f32x4::from([1.0, -1.0, 1.0, -1.0]);
  let actual = a.copysign(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_sin_cos() {
  for x in -2500..=2500 {
    let base = (x * 4) as f32;
    let angles = [base, base + 1.0, base + 2.0, base + 3.0];
    let (actual_sins, actual_coses) = f32x4::from(angles).sin_cos();
    for i in 0..4 {
      let angle = angles[i];
      let check = |name: &str, vals: f32x4, expected: f32| {
        let actual_arr: [f32; 4] = cast(vals);
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

// NOTE:Disabled for i586
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x4_asin_acos() {
  let inc = 1.0 / 2501.0 / 4.0;
  for x in -2500..=2500 {
    let base = (x * 4) as f32 * inc;
    let origs = [base, base + inc, base + 2.0 * inc, base + 3.0 * inc];
    let (actual_asins, actual_acoses) = f32x4::from(origs).asin_acos();
    for i in 0..4 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x4, expected: f32| {
        let actual_arr: [f32; 4] = cast(vals);
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x4_asin() {
  let inc = 1.0 / 2501.0 / 4.0;
  for x in -2500..=2500 {
    let base = (x * 4) as f32 * inc;
    let origs = [base, base + inc, base + 2.0 * inc, base + 3.0 * inc];
    let actual_asins = f32x4::from(origs).asin();
    for i in 0..4 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x4, expected: f32| {
        let actual_arr: [f32; 4] = cast(vals);
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x4_acos() {
  let inc = 1.0 / 2501.0 / 4.0;
  for x in -2500..=2500 {
    let base = (x * 4) as f32 * inc;
    let origs = [base, base + inc, base + 2.0 * inc, base + 3.0 * inc];
    let actual_acoses = f32x4::from(origs).acos();
    for i in 0..4 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x4, expected: f32| {
        let actual_arr: [f32; 4] = cast(vals);
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x4_atan() {
  let inc = 1.0 / 2501.0 / 4.0;
  for x in -2500..=2500 {
    let base = (x * 4) as f32 * inc;
    let origs = [base, base + inc, base + 2.0 * inc, base + 3.0 * inc];
    let actual_atans = f32x4::from(origs).atan();
    for i in 0..4 {
      let orig = origs[i];
      let check = |name: &str, vals: f32x4, expected: f32| {
        let actual_arr: [f32; 4] = cast(vals);
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
#[cfg(target_feature = "sse")]
#[test]
fn impl_f32x4_atan2() {
  let inc_y = 1.0 / 51.0 / 4.0;
  let inc_x = 1.0 / 2501.0 / 4.0;
  for y in -50..=50 {
    let base_y = (y * 4) as f32 * inc_y;
    let origs_y =
      [base_y, base_y + inc_y, base_y + 2.0 * inc_y, base_y + 3.0 * inc_y];
    let actual_y = f32x4::from(origs_y);
    for x in -2500..=2500 {
      let base_x = (x * 4) as f32 * inc_x;
      let origs_x =
        [base_x, base_x + inc_x, base_x + 2.0 * inc_x, base_x + 3.0 * inc_x];
      let actual_x = f32x4::from(origs_x);
      let actual_atan2s = actual_y.atan2(actual_x);
      for i in 0..4 {
        let orig_y = origs_y[i];
        let orig_x = origs_x[i];
        let check = |name: &str, vals: f32x4, expected: f32| {
          let actual_arr: [f32; 4] = cast(vals);
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
fn impl_f32x4_to_degrees() {
  let pi = core::f32::consts::PI;
  let a = f32x4::from([0.0, pi / 2.0, pi, 2.0 * pi]);
  let expected = f32x4::from([0.0, 90.0, 180.0, 360.0]);
  let actual = a.to_degrees();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_to_radians() {
  let pi = core::f32::consts::PI;
  let a = f32x4::from([0.0, 90.0, 180.0, 360.0]);
  let expected = f32x4::from([0.0, pi / 2.0, pi, 2.0 * pi]);
  let actual = a.to_radians();
  assert_eq!(expected, actual);
}

#[test]
fn impl_f32x4_recip() {
  {
    let expected = f32x4::from(0.0);
    let actual = f32x4::from(f32::INFINITY).recip();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x4::from(0.0);
    let actual = f32x4::from(-f32::INFINITY).recip();
    assert_eq!(expected, actual);
  }
  {
    let actual = f32x4::from(f32::NAN).recip();
    assert!(actual.is_nan().any());
  }
  {
    let expected = f32x4::from(f32::INFINITY);
    let actual = f32x4::from(0.0).recip();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x4::from(0.49987793);
    let actual = f32x4::from(2.0).recip();
    let diff: [f32; 4] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
  {
    let expected = f32x4::from(-0.08102417);
    let actual = f32x4::from(-12.34).recip();
    let diff: [f32; 4] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
}

#[test]
fn impl_f32x4_recip_sqrt() {
  {
    let expected = f32x4::from(0.0);
    let actual = f32x4::from(f32::INFINITY).recip_sqrt();
    assert_eq!(expected, actual);
  }
  {
    let actual = f32x4::from(-f32::INFINITY).recip_sqrt();
    assert!(actual.is_nan().any());
  }
  {
    let actual = f32x4::from(f32::NAN).recip_sqrt();
    assert!(actual.is_nan().any());
  }
  {
    let expected = f32x4::from(f32::INFINITY);
    let actual = f32x4::from(0.0).recip_sqrt();
    assert_eq!(expected, actual);
  }
  {
    let expected = f32x4::from(0.70703125);
    let actual = f32x4::from(2.0).recip_sqrt();
    let diff: [f32; 4] = cast((actual - expected).abs());
    assert!(diff[0] < 0.001);
  }
  {
    let actual = f32x4::from(-12.34).recip_sqrt();
    assert!(actual.is_nan().any());
  }
}

#[test]
fn impl_f32x4_sqrt() {
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
    let expected = f32x4::from(e);
    let actual = f32x4::from(f).sqrt();
    assert_eq!(expected, actual);
  }
  assert_eq!(
    cast::<_, i32x4>(f32x4::from(f32::NAN).sqrt().is_nan()),
    i32x4::from(-1)
  );
  assert_eq!(
    cast::<_, i32x4>(f32x4::from(f32::NEG_INFINITY).sqrt().is_nan()),
    i32x4::from(-1)
  );
  assert_eq!(
    cast::<_, i32x4>(f32x4::from(-1.0).sqrt().is_nan()),
    i32x4::from(-1)
  );
}

#[test]
fn impl_f32x4_exp() {
  for f in [(-2.0), (-1.0), (0.0), (1.0), (1.5), (2.0), (10.0)].iter().copied()
  {
    let expected = f32x4::from((f as f32).exp());
    let actual = f32x4::from(f).exp();
    let diff_from_std: [f32; 4] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000000000000001);
  }
}

#[test]
fn test_f32x4_move_mask() {
  let a = f32x4::from([-1.0, 0.0, -2.0, -3.0]);
  let expected = 0b1101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = f32x4::from([1.0, 0.0, 2.0, -3.0]);
  let expected = 0b1000;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
}

#[test]
fn test_f32x4_any() {
  let a = f32x4::from([-1.0, 0.0, -2.0, f32::NAN]).is_nan();
  assert!(a.any());
  //
  let a = f32x4::from([1.0, 0.0, 2.0, 3.0]).is_nan();
  assert!(!a.any());
}

#[test]
fn test_f32x4_all() {
  let a = f32x4::from([f32::NAN; 4]).is_nan();
  assert!(a.all());
  //
  let a = f32x4::from([1.0, -0.0, 2.0, f32::NAN]).is_nan();
  assert!(!a.all());
}

#[test]
fn test_f32x4_none() {
  let a = f32x4::from([1.0, 0.0, 2.0, 3.0]).is_nan();
  assert!(a.none());
  //
  let a = f32x4::from([1.0, -0.0, 2.0, f32::NAN]).is_nan();
  assert!(!a.none());
}

#[test]
fn impl_f32x4_ln() {
  for f in [0.1, 0.5, 1.0, 2.718282, 10.0, 35.0, 1250.0].iter().copied() {
    let expected = f32x4::from((f as f32).ln());
    let actual = f32x4::from(f).ln();
    let diff_from_std: [f32; 4] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000001);
  }
}

#[test]
fn impl_f32x4_pow() {
  for f in [0.1, 0.5, 1.0, 2.718282, 3.0, 4.0, 2.5, -1.0].iter().copied() {
    let expected = f32x4::splat(2.0 as f32).powf(f);
    let actual = f32x4::from(2.0_f32.powf(f));
    let diff_from_std: [f32; 4] = cast((actual - expected).abs());
    assert!(diff_from_std[0] < 0.000001);
  }
}

#[test]
fn impl_f32x4_pow_n() {
  let p = f32x4::from([29.0, 0.1, 0.5, 1.0]);
  let f = f32x4::from([1.2, 2.0, 3.0, 1.5]);
  let res = f.pow_f32x4(p);

  let p: [f32; 4] = cast(p);
  let f: [f32; 4] = cast(f);
  let res: [f32; 4] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }

  let p = f32x4::from([2.718282, -0.2, -1.5, 3.4]);
  let f = f32x4::from([9.2, 6.1, 2.5, -4.5]);
  let res = f.pow_f32x4(p);

  let p: [f32; 4] = cast(p);
  let f: [f32; 4] = cast(f);
  let res: [f32; 4] = cast(res);
  for i in 0..p.len() {
    let expected = f[i].powf(p[i]);
    if !(expected.is_nan() && res[i].is_nan()) {
      assert!((expected - res[i]).abs() < 0.0001);
    }
  }
}

#[test]
fn impl_f32x4_reduce_add() {
  let p = f32x4::splat(0.001);
  assert_eq!(p.reduce_add(), 0.004);
}

#[test]
fn impl_f32x4_sum() {
  let mut p = Vec::with_capacity(250_000);
  for _ in 0..250_000 {
    p.push(f32x4::splat(0.001));
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
fn impl_f32x4_transpose() {
  let a = [
    f32x4::from([1.0, 2.0, 3.0, 4.0]),
    f32x4::from([5.0, 6.0, 7.0, 8.0]),
    f32x4::from([9.0, 10.0, 11.0, 12.0]),
    f32x4::from([13.0, 14.0, 15.0, 16.0]),
  ];
  let a_t = [
    f32x4::from([1.0, 5.0, 9.0, 13.0]),
    f32x4::from([2.0, 6.0, 10.0, 14.0]),
    f32x4::from([3.0, 7.0, 11.0, 15.0]),
    f32x4::from([4.0, 8.0, 12.0, 16.0]),
  ];
  assert_eq!(f32x4::transpose(a), a_t);
}

#[test]
fn impl_f32x4_from_i32x4() {
  let i = i32x4::from([1, 2, 3, 4]);
  let f = f32x4::from([1.0, 2.0, 3.0, 4.0]);
  assert_eq!(f32x4::from_i32x4(i), f)
}

#[cfg(feature = "serde")]
#[test]
fn impl_f32x4_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&f32x4::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(f32x4::ZERO, deserialized);
}
