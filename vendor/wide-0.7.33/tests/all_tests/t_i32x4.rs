use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i32x4>(), 16);
  assert_eq!(core::mem::align_of::<i32x4>(), 16);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i32x4, _, 4>();
}

#[test]
fn impl_add_for_i32x4() {
  let a = i32x4::from([1, 2, i32::MAX - 1, i32::MAX - 1]);
  let b = i32x4::from([17, 18, 1, 2]);
  let expected = i32x4::from([18, 20, i32::MAX, i32::MIN]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i32x4() {
  let a = i32x4::from([1, 2, i32::MIN + 1, i32::MIN]);
  let b = i32x4::from([17, -18, 1, 1]);
  let expected = i32x4::from([-16, 20, i32::MIN, i32::MAX]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_i32x4() {
  let a = i32x4::from([1, 2, i32::MIN + 1, i32::MIN]);
  let b = i32x4::from([17, -18, 1, 1]);
  let expected = i32x4::from([17, -36, i32::MIN + 1, i32::MIN]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_i32x4() {
  let a = i32x4::from([0, 0, 1, 1]);
  let b = i32x4::from([0, 1, 0, 1]);
  let expected = i32x4::from([0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i32x4() {
  let a = i32x4::from([0, 0, 1, 1]);
  let b = i32x4::from([0, 1, 0, 1]);
  let expected = i32x4::from([0, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i32x4() {
  let a = i32x4::from([0, 0, 1, 1]);
  let b = i32x4::from([0, 1, 0, 1]);
  let expected = i32x4::from([0, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shl_for_i32x4() {
  let a = i32x4::from([1, 2, i32::MAX - 1, i32::MAX - 1]);
  let b = 2;
  let expected =
    i32x4::from([1 << 2, 2 << 2, (i32::MAX - 1) << 2, (i32::MAX - 1) << 2]);
  let actual = a << b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shr_for_i32x4() {
  let a = i32x4::from([1, 2, i32::MAX - 1, i32::MAX - 1]);
  let b = 2;
  let expected =
    i32x4::from([1 >> 2, 2 >> 2, (i32::MAX - 1) >> 2, (i32::MAX - 1) >> 2]);
  let actual = a >> b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_cmp_eq() {
  let a = i32x4::from([1, 2, 3, 4]);
  let b = i32x4::from([2_i32; 4]);
  let expected = i32x4::from([0, -1, 0, 0]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_cmp_gt() {
  let a = i32x4::from([1, 2, 3, 4]);
  let b = i32x4::from([2_i32; 4]);
  let expected = i32x4::from([0, 0, -1, -1]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_cmp_lt() {
  let a = i32x4::from([1, 2, 3, 4]);
  let b = i32x4::from([2_i32; 4]);
  let expected = i32x4::from([-1, 0, 0, 0]);
  let actual = a.cmp_lt(b);
  assert_eq!(expected, actual);

  let expected = i32x4::from([0, 0, 0, 0]);
  let actual = a.cmp_lt(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_blend() {
  let use_t: i32 = -1;
  let t = i32x4::from([1, 2, 3, 4]);
  let f = i32x4::from([17, 18, 19, 20]);
  let mask = i32x4::from([use_t, 0, use_t, 0]);
  let expected = i32x4::from([1, 18, 3, 20]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_abs() {
  let a = i32x4::from([-1, 2, -3, i32::MIN]);
  let expected = i32x4::from([1, 2, 3, i32::MIN]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_unsigned_abs() {
  let a = i32x4::from([-1, 2, -3, i32::MIN]);
  let expected = u32x4::from([1, 2, 3, i32::MIN as u32]);
  let actual = a.unsigned_abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x4_max() {
  let a = i32x4::from([1, 2, i32::MIN + 1, i32::MIN]);
  let b = i32x4::from([17, -18, 1, 1]);
  let expected = i32x4::from([17, 2, 1, 1]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i32x4, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_i32x4_min() {
  let a = i32x4::from([1, 2, i32::MIN + 1, i32::MIN]);
  let b = i32x4::from([17, -18, 1, 1]);
  let expected = i32x4::from([1, -18, i32::MIN + 1, i32::MIN]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i32x4, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_i32x4_round_float() {
  let a = i32x4::from([-1, 30, i32::MIN, i32::MAX]);
  let expected = f32x4::from([-1.0, 30.0, i32::MIN as f32, i32::MAX as f32]);
  let actual = a.round_float();
  assert_eq!(expected, actual);
}

#[test]
fn test_i32x4_move_mask() {
  let a = i32x4::from([-1, 0, -2, -3]);
  let expected = 0b1101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = i32x4::from([i32::MAX, 0, 2, -3]);
  let expected = 0b1000;
  let actual = a.move_mask();
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar_reduce(
    |a: i32x4| a.move_mask(),
    0i32,
    |acc, a, idx| acc | if a < 0 { 1 << idx } else { 0 },
  );
}

#[test]
fn test_i32x4_any() {
  let a = i32x4::from([0, 0, 0, -1]);
  assert!(a.any());
  //
  let a = i32x4::from([0, 0, 0, 0]);
  assert!(!a.any());
}

#[test]
fn test_i32x4_all() {
  let a = i32x4::from([0, 0, 0, -1]);
  assert!(!a.all());
  //
  let a = i32x4::from([-1; 4]);
  assert!(a.all());
}

#[test]
fn test_i32x4_none() {
  let a = i32x4::from([0, 0, 0, -1]);
  assert!(!a.none());
  //
  let a = i32x4::from([0; 4]);
  assert!(a.none());
}

#[test]
fn impl_i32x4_reduce_add() {
  let p = i32x4::from([10000000, 20000000, 30000000, -40000000]);
  assert_eq!(p.reduce_add(), 20000000);
}

#[test]
fn impl_i32x4_reduce_min() {
  for i in 0..4 {
    let mut v = [i32::MAX; 4];
    v[i] = i32::MIN;
    let p = i32x4::from(v);
    assert_eq!(p.reduce_min(), i32::MIN);
  }
}

#[test]
fn impl_i32x4_reduce_max() {
  for i in 0..4 {
    let mut v = [i32::MIN; 4];
    v[i] = i32::MAX;
    let p = i32x4::from(v);
    assert_eq!(p.reduce_max(), i32::MAX);
  }
}

#[test]
fn impl_i32x4_shr_each() {
  let a = i32x4::from([15313, 52322, -1, 4]);
  let shift = i32x4::from([1, 30, 8, 33 /* test masking behavior */]);
  let expected = i32x4::from([7656, 0, -1, 2]);
  let actual = a >> shift;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i32x4, b| a >> b,
    |a, b| a.wrapping_shr(b as u32),
  );
}
#[test]
fn impl_i32x4_shl_each() {
  let a = i32x4::from([15313, 52322, -1, 4]);
  let shift = i32x4::from([1, 30, 8, 33 /* test masking behavior */]);
  let expected = i32x4::from([30626, -2147483648, -256, 8]);
  let actual = a << shift;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i32x4, b| a << b,
    |a, b| a.wrapping_shl(b as u32),
  );
}

#[test]
fn impl_i32x4_mul_widen() {
  let a = i32x4::from([1, 2, 3 * -1000000, i32::MAX]);
  let b = i32x4::from([5, 6, 7 * -1000000, i32::MIN]);
  let expected = i64x4::from([
    1 * 5,
    2 * 6,
    3 * 7 * 1000000 * 1000000,
    i32::MIN as i64 * i32::MAX as i64,
  ]);
  let actual = a.mul_widen(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i32x4, b| a.mul_widen(b),
    |a, b| a as i64 * b as i64,
  );
}

#[test]
fn impl_i32x4_transpose() {
  let a = [
    i32x4::from([1, 2, 3, 4]),
    i32x4::from([5, 6, 7, 8]),
    i32x4::from([9, 10, 11, 12]),
    i32x4::from([13, 14, 15, 16]),
  ];
  let a_t = [
    i32x4::from([1, 5, 9, 13]),
    i32x4::from([2, 6, 10, 14]),
    i32x4::from([3, 7, 11, 15]),
    i32x4::from([4, 8, 12, 16]),
  ];
  assert_eq!(i32x4::transpose(a), a_t);
}

#[cfg(feature = "serde")]
#[test]
fn impl_i32x4_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i32x4::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i32x4::ZERO, deserialized);
}
