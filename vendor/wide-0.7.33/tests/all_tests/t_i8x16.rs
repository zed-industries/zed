use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i8x16>(), 16);
  assert_eq!(core::mem::align_of::<i8x16>(), 16);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i8x16, _, 16>();
}

#[test]
fn impl_add_for_i8x16() {
  let a =
    i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 126, 127]);
  let b =
    i8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 1]);
  let expected = i8x16::from([
    18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 127, -128,
  ]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i8x16() {
  let a = i8x16::from([
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    i8::MIN + 1,
    i8::MIN,
  ]);
  let b =
    i8x16::from([17, 27, -1, 20, 21, -8, 23, 0, 1, 2, -9, 28, 64, 30, 1, 1]);
  let expected = i8x16::from([
    -16,
    -25,
    4,
    -16,
    -16,
    14,
    -16,
    8,
    8,
    8,
    20,
    -16,
    -51,
    -16,
    i8::MIN,
    i8::MAX,
  ]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_saturating_add_for_i8x16() {
  let a =
    i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 126, 127]);
  let b =
    i8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 1]);
  let expected = i8x16::from([
    18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 127, 127,
  ]);
  let actual = a.saturating_add(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_saturating_sub_for_i8x16() {
  let a = i8x16::from([
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    i8::MIN + 1,
    i8::MIN,
  ]);
  let b =
    i8x16::from([17, 27, -1, 20, 21, -8, 23, 0, 1, 2, -9, 28, 64, 30, 1, 1]);
  let expected = i8x16::from([
    -16,
    -25,
    4,
    -16,
    -16,
    14,
    -16,
    8,
    8,
    8,
    20,
    -16,
    -51,
    -16,
    i8::MIN,
    i8::MIN,
  ]);
  let actual = a.saturating_sub(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_i8x16() {
  let a = i8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i8x16::from([0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i8x16() {
  let a = i8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i8x16::from([0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i8x16() {
  let a = i8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = i8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = i8x16::from([0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_cmp_eq() {
  let a = i8x16::from([1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i8x16::from([2_i8; 16]);
  let expected =
    i8x16::from([0, -1, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_cmp_gt() {
  let a = i8x16::from([1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i8x16::from([2_i8; 16]);
  let expected =
    i8x16::from([0, 0, -1, -1, 0, 0, -1, -1, 0, 0, -1, -1, 0, 0, -1, -1]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_cmp_lt() {
  let a = i8x16::from([1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
  let b = i8x16::from([2_i8; 16]);
  let expected =
    i8x16::from([-1, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0]);
  let actual = a.cmp_lt(b);
  assert_eq!(expected, actual);

  let expected = i8x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  let actual = a.cmp_lt(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_blend() {
  let use_t: i8 = -1;
  let t =
    i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 126, 127]);
  let f =
    i8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 1]);
  let mask = i8x16::from([
    use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0,
    use_t, 0,
  ]);
  let expected =
    i8x16::from([1, 18, 3, 20, 5, 22, 7, 24, 9, 26, 11, 28, 13, 30, 126, 1]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_abs() {
  let a = i8x16::from([
    -1,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    -10,
    -11,
    12,
    13,
    -14,
    -126,
    i8::MIN,
  ]);
  let expected =
    i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 126, i8::MIN]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_unsigned_abs() {
  let a = i8x16::from([
    -1,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    -10,
    -11,
    12,
    13,
    -14,
    -126,
    i8::MIN,
  ]);
  let expected = u8x16::from([
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    126,
    i8::MIN as u8,
  ]);
  let actual = a.unsigned_abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i8x16_max() {
  let a =
    i8x16::from([10, 2, -3, 4, 5, -6, 7, 8, 9, 7, -11, 12, 13, 6, 55, i8::MIN]);
  let b = i8x16::from([
    -1,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    -10,
    -11,
    12,
    13,
    -14,
    -126,
    i8::MIN + 1,
  ]);
  let expected =
    i8x16::from([10, 2, -3, 4, 5, -6, 7, 8, 9, 7, -11, 12, 13, 6, 55, -127]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i8x16, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_i8x16_min() {
  let a =
    i8x16::from([10, 2, -3, 4, 5, -6, 7, 8, 9, 7, -11, 12, 13, 6, 55, i8::MIN]);
  let b = i8x16::from([
    -1,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    -10,
    -11,
    12,
    13,
    -14,
    -126,
    i8::MIN + 1,
  ]);
  let expected = i8x16::from([
    -1,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    -10,
    -11,
    12,
    13,
    -14,
    -126,
    i8::MIN,
  ]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i8x16, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_from_i16x16_truncate() {
  let src = i16x16::new([
    10000, 1001, 2, 3, 4, 5, 6, 32767, 10000, 1001, 2, 128, -129, -128, 127,
    255,
  ]);

  let expected = i8x16::new([
    16, -23, 2, 3, 4, 5, 6, -1, 16, -23, 2, -128, 127, -128, 127, -1,
  ]);

  let result = i8x16::from_i16x16_truncate(src);

  assert_eq!(result, expected);
}

#[test]
fn impl_from_i16x16_saturate() {
  let src = i16x16::new([
    10000, 1001, 2, 3, 4, 5, 6, 32767, 10000, 1001, 2, 128, -129, -128, 127,
    255,
  ]);

  let expected = i8x16::new([
    127, 127, 2, 3, 4, 5, 6, 127, 127, 127, 2, 127, -128, -128, 127, 127,
  ]);

  let result = i8x16::from_i16x16_saturate(src);

  assert_eq!(result, expected);
}

#[test]
fn test_i8x16_move_mask() {
  let a =
    i8x16::from([-1, 0, -2, -3, -1, 0, -2, -3, -1, 0, -1, 0, -1, 0, -1, 0]);
  let expected = 0b0101010111011101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
}

#[test]
fn test_i8x16_any() {
  let a = i8x16::from([0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(a.any());

  let a = i8x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(a.any());

  //
  let a = i8x16::from([0; 16]);
  assert!(!a.any());
}

#[test]
fn test_i8x16_all() {
  let a = i8x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(!a.all());
  //
  let a = i8x16::from([-1; 16]);
  assert!(a.all());
}

#[test]
fn test_i8x16_none() {
  let a = i8x16::from([0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(!a.none());

  let a = i8x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(!a.none());

  //
  let a = i8x16::from([0; 16]);
  assert!(a.none());
}

#[test]
fn impl_from_i8_slice() {
  let src = [0, 1_i8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

  let result = i8x16::from_slice_unaligned(&src[1..17]);

  let expected =
    i8x16::new([1_i8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
  assert_eq!(result, expected);
}

#[test]
fn test_i8x16_swizzle() {
  let a = i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
  let b = i8x16::from([15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
  let expected =
    i8x16::from([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
  let actual = a.swizzle(b);
  assert_eq!(expected, actual);

  let b = i8x16::from([15, 17, -13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, -1, 0]);
  let expected =
    i8x16::from([16, 0, 0, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 0, 1]);
  let actual = a.swizzle(b);
  assert_eq!(expected, actual);
}

#[test]
fn test_i8x16_swizzle_relaxed() {
  let a = i8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
  let b = i8x16::from([15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
  let expected =
    i8x16::from([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
  let actual = a.swizzle_relaxed(b);
  assert_eq!(expected, actual);

  let b =
    i8x16::from([15, -17, -13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, -1, 0]);
  let expected =
    i8x16::from([16, 0, 0, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 0, 1]);
  let actual = a.swizzle_relaxed(b);
  assert_eq!(expected, actual);
}

#[cfg(feature = "serde")]
#[test]
fn impl_i8x16_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i8x16::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i8x16::ZERO, deserialized);
}
