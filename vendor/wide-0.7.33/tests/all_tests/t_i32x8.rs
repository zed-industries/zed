use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i32x8>(), 32);
  assert_eq!(core::mem::align_of::<i32x8>(), 32);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i32x8, _, 8>();
}

#[test]
fn impl_add_for_i32x8() {
  let a = i32x8::from([1, 2, i32::MAX - 1, i32::MAX - 1, 15, 20, 5000, 2990]);
  let b = i32x8::from([17, 18, 1, 2, 20, 5, 900, 900]);
  let expected = i32x8::from([18, 20, i32::MAX, i32::MIN, 35, 25, 5900, 3890]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i32x8() {
  let a = i32x8::from([1, 2, i32::MIN + 1, i32::MIN, 15, 20, 5000, 2990]);
  let b = i32x8::from([17, -18, 1, 1, 20, 5, 900, 900]);
  let expected = i32x8::from([-16, 20, i32::MIN, i32::MAX, -5, 15, 4100, 2090]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_i32x8() {
  let a = i32x8::from([1, 2, i32::MIN + 1, i32::MIN, 2, 3, 4, 5]);
  let b = i32x8::from([17, -18, 1, 1, -1, -2, -6, 3]);
  let expected =
    i32x8::from([17, -36, i32::MIN + 1, i32::MIN, -2, -6, -24, 15]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_i32x8() {
  let a = i32x8::from([0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i32x8::from([0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i32x8::from([0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i32x8() {
  let a = i32x8::from([0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i32x8::from([0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i32x8::from([0, 1, 1, 1, 1, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i32x8() {
  let a = i32x8::from([0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i32x8::from([0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i32x8::from([0, 1, 1, 0, 1, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shl_for_i32x8() {
  let a = i32x8::from([1, 2, i32::MAX - 1, i32::MAX - 1, 128, 255, 590, 5667]);
  let b = 2;
  let expected = i32x8::from([
    1 << 2,
    2 << 2,
    (i32::MAX - 1) << 2,
    (i32::MAX - 1) << 2,
    128 << 2,
    255 << 2,
    590 << 2,
    5667 << 2,
  ]);
  let actual = a << b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shr_for_i32x8() {
  let a = i32x8::from([1, 2, i32::MAX - 1, i32::MAX - 1, 128, 255, 590, 5667]);
  let b = 2;
  let expected = i32x8::from([
    1 >> 2,
    2 >> 2,
    (i32::MAX - 1) >> 2,
    (i32::MAX - 1) >> 2,
    128 >> 2,
    255 >> 2,
    590 >> 2,
    5667 >> 2,
  ]);
  let actual = a >> b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_cmp_eq() {
  let a = i32x8::from([1, 2, 3, 4, 2, 1, 8, 2]);
  let b = i32x8::from([2_i32; 8]);
  let expected = i32x8::from([0, -1, 0, 0, -1, 0, 0, -1]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_cmp_gt() {
  let a = i32x8::from([1, 2, 9, 4, 1, 2, 8, 10]);
  let b = i32x8::from([5_i32; 8]);
  let expected = i32x8::from([0, 0, -1, 0, 0, 0, -1, -1]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_cmp_lt() {
  let a = i32x8::from([1, 2, 9, 4, 1, 2, 8, 10]);
  let b = i32x8::from([5_i32; 8]);
  let expected = i32x8::from([-1, -1, 0, -1, -1, -1, 0, 0]);
  let actual = a.cmp_lt(b);
  assert_eq!(expected, actual);

  let expected = i32x8::from([0, 0, 0, 0, 0, 0, 0, 0]);
  let actual = a.cmp_lt(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_blend() {
  let use_t: i32 = -1;
  let t = i32x8::from([1, 2, 3, 4, 5, 6, 7, 8]);
  let f = i32x8::from([17, 18, 19, 20, 25, 30, 50, 90]);
  let mask = i32x8::from([use_t, 0, use_t, 0, 0, 0, 0, use_t]);
  let expected = i32x8::from([1, 18, 3, 20, 25, 30, 50, 8]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_abs() {
  let a = i32x8::from([-1, 2, -3, i32::MIN, 6, -15, -19, 9]);
  let expected = i32x8::from([1, 2, 3, i32::MIN, 6, 15, 19, 9]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_unsigned_abs() {
  let a = i32x8::from([-1, 2, -3, i32::MIN, 6, -15, -19, 9]);
  let expected = u32x8::from([1, 2, 3, i32::MIN as u32, 6, 15, 19, 9]);
  let actual = a.unsigned_abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i32x8_max() {
  let a = i32x8::from([1, 2, i32::MIN + 1, i32::MIN, 6, -8, 12, 9]);
  let b = i32x8::from([17, -18, 1, 1, 19, -5, -1, -9]);
  let expected = i32x8::from([17, 2, 1, 1, 19, -5, 12, 9]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i32x8, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_i32x8_min() {
  let a = i32x8::from([1, 2, i32::MIN + 1, i32::MIN, 6, -8, 12, 9]);
  let b = i32x8::from([17, -18, 1, 1, 19, -5, -1, -9]);
  let expected = i32x8::from([1, -18, i32::MIN + 1, i32::MIN, 6, -8, -1, -9]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i32x8, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_i32x8_round_float() {
  let a = i32x8::from([-1, 30, i32::MIN, i32::MAX, 29, 35, -8, 0]);
  let expected = f32x8::from([
    -1.0,
    30.0,
    i32::MIN as f32,
    i32::MAX as f32,
    29.0,
    35.0,
    -8.0,
    0.0,
  ]);
  let actual = a.round_float();
  assert_eq!(expected, actual);
}

#[test]
fn impl_transpose_for_i32x8() {
  let a = [
    i32x8::new([0, 1, 2, 3, 4, 5, 6, 7]),
    i32x8::new([8, 9, 10, 11, 12, 13, 14, 15]),
    i32x8::new([16, 17, 18, 19, 20, 21, 22, 23]),
    i32x8::new([24, 25, 26, 27, 28, 29, 30, 31]),
    i32x8::new([32, 33, 34, 35, 36, 37, 38, 39]),
    i32x8::new([40, 41, 42, 43, 44, 45, 46, 47]),
    i32x8::new([48, 49, 50, 51, 52, 53, 54, 55]),
    i32x8::new([
      5600000, 5700000, 5800000, 5900000, 6000000, 6100000, 6200000, 6300000,
    ]),
  ];

  let result = i32x8::transpose(a);

  let expected = [
    i32x8::new([0, 8, 16, 24, 32, 40, 48, 5600000]),
    i32x8::new([1, 9, 17, 25, 33, 41, 49, 5700000]),
    i32x8::new([2, 10, 18, 26, 34, 42, 50, 5800000]),
    i32x8::new([3, 11, 19, 27, 35, 43, 51, 5900000]),
    i32x8::new([4, 12, 20, 28, 36, 44, 52, 6000000]),
    i32x8::new([5, 13, 21, 29, 37, 45, 53, 6100000]),
    i32x8::new([6, 14, 22, 30, 38, 46, 54, 6200000]),
    i32x8::new([7, 15, 23, 31, 39, 47, 55, 6300000]),
  ];

  assert_eq!(result, expected);
}

#[test]
fn impl_from_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let actual = i32x8::from_i16x8(a);
  let expected =
    i32x8::from([1, 2, 3, 4, 5, 6, (i16::MIN + 1) as i32, i16::MIN as i32]);

  assert_eq!(actual, expected);
}

#[test]
fn impl_from_u16x8() {
  let a = u16x8::from([1, 2, 3, 4, 5, i16::MAX as u16, u16::MAX - 1, u16::MAX]);
  let actual = i32x8::from_u16x8(a);
  let expected = i32x8::from([
    1,
    2,
    3,
    4,
    5,
    i16::MAX as i32,
    (u16::MAX - 1) as i32,
    u16::MAX as i32,
  ]);

  assert_eq!(actual, expected);

  crate::test_random_vector_vs_scalar(
    |a: u16x8, _b| i32x8::from_u16x8(a),
    |a, _b| a as u32 as i32,
  );
}

#[test]
fn test_i16x8_move_mask() {
  let a = i16x8::from([-1, 0, -2, -3, -1, 0, -2, -3]);
  let expected = 0b11011101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = i16x8::from([1, 0, 2, -3, 1, 0, 2, -3]);
  let expected = 0b10001000;
  let actual = a.move_mask();
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar_reduce(
    |a: i32x8| a.move_mask(),
    0i32,
    |acc, a, idx| acc | if a < 0 { 1 << idx } else { 0 },
  );
}

#[test]
fn test_i32x8_any() {
  let a = i32x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(a.any());
  //
  let a = i32x8::from([0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(!a.any());

  crate::test_random_vector_vs_scalar_reduce(
    |a: i32x8| a.any(),
    false,
    |acc, a, _idx| acc | (a < 0),
  );
}

#[test]
fn test_i32x8_all() {
  let a = i32x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(!a.all());
  //
  let a = i32x8::from([-1; 8]);
  assert!(a.all());

  crate::test_random_vector_vs_scalar_reduce(
    |a: i32x8| a.all(),
    true,
    |acc, a, _idx| acc & (a < 0),
  );
}

#[test]
fn test_i32x8_none() {
  let a = i32x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(!a.none());
  //
  let a = i32x8::from([0; 8]);
  assert!(a.none());

  crate::test_random_vector_vs_scalar_reduce(
    |a: i32x8| a.none(),
    true,
    |acc, a, _idx| acc & !(a < 0),
  );
}

#[test]
fn impl_i32x8_reduce_add() {
  let p = i32x8::from([
    10000000, 20000000, 30000000, 40000000, 50000000, 60000000, 70000000,
    90000000,
  ]);
  assert_eq!(p.reduce_add(), 370000000);
}

#[test]
fn impl_i32x8_reduce_min() {
  for i in 0..8 {
    let mut v = [i32::MAX; 8];
    v[i] = i32::MIN;
    let p = i32x8::from(v);
    assert_eq!(p.reduce_min(), i32::MIN);
  }
}

#[test]
fn impl_i32x8_reduce_max() {
  for i in 0..8 {
    let mut v = [i32::MIN; 8];
    v[i] = i32::MAX;
    let p = i32x8::from(v);
    assert_eq!(p.reduce_max(), i32::MAX);
  }
}

#[test]
fn impl_i32x4_shr_each() {
  let a = u32x8::from([15313, 52322, u32::MAX, 4, 10, 20, 30, 40]);
  let shift =
    u32x8::from([1, 30, 8, 33 /* test masking behavior */, 1, 2, 3, 4]);
  let expected = u32x8::from([7656, 0, 16777215, 2, 5, 5, 3, 2]);
  let actual = a >> shift;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i32x8, b| a >> b,
    |a, b| a.wrapping_shr(b as u32),
  );
}
#[test]
fn impl_i32x8_shl_each() {
  let a = i32x8::from([15313, 52322, -1, 4, 1, 2, 3, 4]);
  let shift =
    i32x8::from([1, 30, 8, 33 /* test masking behavior */, 1, 2, 3, 4]);
  let expected = i32x8::from([30626, -2147483648, -256, 8, 2, 8, 24, 64]);
  let actual = a << shift;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i32x8, b| a << b,
    |a, b| a.wrapping_shl(b as u32),
  );
}

#[cfg(feature = "serde")]
#[test]
fn impl_i32x8_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i32x8::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i32x8::ZERO, deserialized);
}
