use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i16x8>(), 16);
  assert_eq!(core::mem::align_of::<i16x8>(), 16);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i16x8, _, 8>();
}

#[test]
fn impl_add_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MAX - 1, i16::MAX - 1]);
  let b = i16x8::from([17, 18, 19, 20, 21, 22, 1, 2]);
  let expected = i16x8::from([18, 20, 22, 24, 26, 28, i16::MAX, i16::MIN]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, 1, 1]);
  let expected = i16x8::from([-16, 20, -187, 24, -16, 28, i16::MIN, i16::MAX]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_add_saturating_for_i16x8() {
  let a = i16x8::from([i16::MAX, i16::MIN, 3, 4, -1, -2, -3, -4]);
  let b = i16x8::from([i16::MAX, i16::MIN, 7, 8, -15, -26, -37, 48]);
  let expected = i16x8::from([i16::MAX, i16::MIN, 10, 12, -16, -28, -40, 44]);
  let actual = a.saturating_add(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_scale_i16x8() {
  let a = i16x8::from([100, 200, 300, 400, 500, -600, 700, -800]);
  let b = i16x8::from([900, 1000, 1100, 1200, 1300, -1400, -1500, 1600]);
  let actual = a.mul_scale_round(b);
  let expected = i16x8::from([3, 6, 10, 15, 20, 26, -32, -39]);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_scale_n_i16x8() {
  let a = i16x8::from([100, 200, 300, 400, 500, -600, 700, -800]);
  let actual = a.mul_scale_round_n(0x4000);
  let expected = i16x8::from([50, 100, 150, 200, 250, -300, 350, -400]);
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_saturating_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, i16::MIN, i16::MIN + 1, i16::MAX]);
  let b = i16x8::from([17, -18, 190, -20, 21, -1, 1, -1]);
  let expected =
    i16x8::from([-16, 20, -187, 24, -16, i16::MIN + 1, i16::MIN, i16::MAX]);
  let actual = a.saturating_sub(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, 1, 1]);
  let expected =
    i16x8::from([17, -36, 570, -80, 105, -132, i16::MIN + 1, i16::MIN]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_transpose_for_i16x8() {
  let a = [
    i16x8::new([0, 1, 2, 3, 4, 5, 6, 7]),
    i16x8::new([8, 9, 10, 11, 12, 13, 14, 15]),
    i16x8::new([16, 17, 18, 19, 20, 21, 22, 23]),
    i16x8::new([24, 25, 26, 27, 28, 29, 30, 31]),
    i16x8::new([32, 33, 34, 35, 36, 37, 38, 39]),
    i16x8::new([40, 41, 42, 43, 44, 45, 46, 47]),
    i16x8::new([48, 49, 50, 51, 52, 53, 54, 55]),
    i16x8::new([5600, 5700, 5800, 5900, 6000, 6100, 6200, 6300]),
  ];

  let result = i16x8::transpose(a);

  let expected = [
    i16x8::new([0, 8, 16, 24, 32, 40, 48, 5600]),
    i16x8::new([1, 9, 17, 25, 33, 41, 49, 5700]),
    i16x8::new([2, 10, 18, 26, 34, 42, 50, 5800]),
    i16x8::new([3, 11, 19, 27, 35, 43, 51, 5900]),
    i16x8::new([4, 12, 20, 28, 36, 44, 52, 6000]),
    i16x8::new([5, 13, 21, 29, 37, 45, 53, 6100]),
    i16x8::new([6, 14, 22, 30, 38, 46, 54, 6200]),
    i16x8::new([7, 15, 23, 31, 39, 47, 55, 6300]),
  ];

  assert_eq!(result, expected);
}

#[test]
fn impl_bitand_for_i16x8() {
  let a = i16x8::from([0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i16x8::from([0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i16x8::from([0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i16x8() {
  let a = i16x8::from([0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i16x8::from([0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i16x8::from([0, 1, 1, 1, 0, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i16x8() {
  let a = i16x8::from([0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i16x8::from([0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i16x8::from([0, 1, 1, 0, 0, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shl_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = 2;
  let expected = i16x8::from([
    1 << 2,
    2 << 2,
    3 << 2,
    4 << 2,
    5 << 2,
    6 << 2,
    (i16::MIN + 1) << 2,
    i16::MIN << 2,
  ]);
  let actual = a << b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shr_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = 2;
  let expected = i16x8::from([
    1 >> 2,
    2 >> 2,
    3 >> 2,
    4 >> 2,
    5 >> 2,
    6 >> 2,
    (i16::MIN + 1) >> 2,
    i16::MIN >> 2,
  ]);
  let actual = a >> b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_cmp_eq() {
  let a = i16x8::from([1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i16x8::from([2_i16; 8]);
  let expected = i16x8::from([0, -1, 0, 0, 0, -1, 0, 0]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_cmp_gt() {
  let a = i16x8::from([1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i16x8::from([2_i16; 8]);
  let expected = i16x8::from([0, 0, -1, -1, 0, 0, -1, -1]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_cmp_lt() {
  let a = i16x8::from([1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i16x8::from([2_i16; 8]);
  let expected = i16x8::from([-1, 0, 0, 0, -1, 0, 0, 0]);
  let actual = a.cmp_lt(b);
  assert_eq!(expected, actual);

  let expected = i16x8::from([0, 0, 0, 0, 0, 0, 0, 0]);
  let actual = a.cmp_lt(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_blend() {
  let use_t: i16 = -1;
  let t = i16x8::from([1, 2, 3, 4, 5, 6, 7, 8]);
  let f = i16x8::from([17, 18, 19, 20, 21, 22, 23, 24]);
  let mask = i16x8::from([use_t, 0, use_t, 0, use_t, 0, use_t, 0]);
  let expected = i16x8::from([1, 18, 3, 20, 5, 22, 7, 24]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_abs() {
  let a = i16x8::from([1, -2, 3, -4, 5, -6, -7, i16::MIN]);
  let expected = i16x8::from([1, 2, 3, 4, 5, 6, 7, i16::MIN]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_unsigned_abs() {
  let a = i16x8::from([1, -2, 3, -4, 5, -6, -7, i16::MIN]);
  let expected = u16x8::from([1, 2, 3, 4, 5, 6, 7, i16::MIN as u16]);
  let actual = a.unsigned_abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_max() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, 1, 1]);
  let expected = i16x8::from([17, 2, 190, 4, 21, 6, 1, 1]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i16x8, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_i16x8_min() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, 1, 1]);
  let expected = i16x8::from([1, -18, 3, -20, 5, -22, i16::MIN + 1, i16::MIN]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i16x8, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn test_from_u8x16_low() {
  let bytes =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, u8::MAX, 9, 10, 11, 12, 13, 14, 15, 16]);
  let expected = i16x8::from([1, 2, 3, 4, 5, 6, 7, u8::MAX as i16]);
  let actual = i16x8::from_u8x16_low(bytes);
  assert_eq!(expected, actual);
}

#[test]
fn test_from_u8x16_high() {
  let a =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 255, 128]);
  let expected = i16x8::from([9, 10, 11, 12, 13, 14, 255, 128]);
  let actual = i16x8::from_u8x16_high(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_from_i32x8_truncate() {
  let src = i32x8::new([10000, 1001, 2, 3, 4, 5, -65536, 65536]);

  let expected = i16x8::new([10000, 1001, 2, 3, 4, 5, 0, 0]);

  let result = i16x8::from_i32x8_truncate(src);

  assert_eq!(result, expected);
}

#[test]
fn impl_from_i32x8_saturate() {
  let src = i32x8::new([10000, 1001, 2, 3, 4, 5, -65535, 65536]);

  let expected = i16x8::new([10000, 1001, 2, 3, 4, 5, -32768, 32767]);

  let result = i16x8::from_i32x8_saturate(src);

  assert_eq!(result, expected);
}

#[test]
fn impl_from_i16_slice() {
  let src = [0, 1_i16, 2, 3, 4, 5, 6, 7, 8];

  let result = i16x8::from_slice_unaligned(&src[1..9]);

  let expected = i16x8::new([1_i16, 2, 3, 4, 5, 6, 7, 8]);
  assert_eq!(result, expected);
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
}

#[test]
fn test_i16x8_any() {
  let a = i16x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(a.any());
  //
  let a = i16x8::from([0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(!a.any());
}

#[test]
fn test_i16x8_all() {
  let a = i16x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(!a.all());
  //
  let a = i16x8::from([-1; 8]);
  assert!(a.all());
}

#[test]
fn test_i16x8_none() {
  let a = i16x8::from([0, 0, 0, -1, 0, 0, 0, 0]);
  assert!(!a.none());
  //
  let a = i16x8::from([0; 8]);
  assert!(a.none());
}

#[test]
fn impl_i16x8_reduce_add() {
  let p = i16x8::from([1, 2, 3, 4, 5, 6, 7, 9]);
  assert_eq!(p.reduce_add(), 37);
}

#[test]
fn impl_dot_for_i16x8() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN + 1, i16::MIN]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, 3, 2]);
  let expected = i32x4::from([-19, 490, -27, -163837]);
  let actual = a.dot(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x8_reduce_min() {
  for i in 0..8 {
    let mut v = [i16::MAX; 8];
    v[i] = i16::MIN;
    let p = i16x8::from(v);
    assert_eq!(p.reduce_min(), i16::MIN);
  }
}

#[test]
fn impl_i16x8_reduce_max() {
  for i in 0..8 {
    let mut v = [i16::MIN; 8];
    v[i] = i16::MAX;
    let p = i16x8::from(v);
    assert_eq!(p.reduce_min(), i16::MIN);
  }
}

#[test]
fn impl_mul_keep_high() {
  let a = i16x8::from([i16::MAX, 200, 300, 4568, -1, -2, -3, -4]);
  let b = i16x8::from([i16::MIN, 600, 700, 8910, -15, -26, -37, 48]);
  let c: [i16; 8] = i16x8::mul_keep_high(a, b).into();
  assert_eq!(
    c,
    [
      (i32::from(i16::MAX) * i32::from(i16::MIN) >> 16) as i16,
      1,
      3,
      621,
      0,
      0,
      0,
      -1
    ]
  );

  crate::test_random_vector_vs_scalar(
    |a: i16x8, b| i16x8::mul_keep_high(a, b),
    |a, b| ((i32::from(a) * i32::from(b)) >> 16) as i16,
  );
}

#[test]
fn impl_i16x8_mul_widen() {
  let a = i16x8::from([1, 2, 3, 4, 5, 6, i16::MIN, i16::MAX]);
  let b = i16x8::from([17, -18, 190, -20, 21, -22, i16::MAX, i16::MAX]);
  let expected = i32x8::from([
    17,
    -36,
    570,
    -80,
    105,
    -132,
    (i16::MIN as i32) * (i16::MAX as i32),
    (i16::MAX as i32) * (i16::MAX as i32),
  ]);
  let actual = a.mul_widen(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: i16x8, b| a.mul_widen(b),
    |a, b| i32::from(a) * i32::from(b),
  );
}

#[cfg(feature = "serde")]
#[test]
fn impl_i16x8_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i16x8::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i16x8::ZERO, deserialized);
}
