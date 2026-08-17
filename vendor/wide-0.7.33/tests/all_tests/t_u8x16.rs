use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<u8x16>(), 16);
  assert_eq!(core::mem::align_of::<u8x16>(), 16);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<u8x16, _, 16>();
}

#[test]
fn impl_add_for_u8x16() {
  let a =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 250, 250]);
  let b =
    u8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 5, 6]);
  let expected = u8x16::from([
    18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 255, 0,
  ]);
  let actual = a + b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u8x16, b| a + b,
    |a, b| a.wrapping_add(b),
  );
}

#[test]
fn impl_sub_for_u8x16() {
  let a = u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1, 0]);
  let b =
    u8x16::from([170, 18, 10, 200, 241, 2, 93, 4, 12, 8, 27, 28, 29, 30, 1, 1]);
  let expected = u8x16::from([
    87,
    240,
    249,
    60,
    20,
    4,
    170,
    4,
    253,
    2,
    240,
    240,
    240,
    240,
    0,
    u8::MAX,
  ]);
  let actual = a - b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u8x16, b| a - b,
    |a, b| a.wrapping_sub(b),
  );
}

#[test]
fn impl_saturating_add_for_u8x16() {
  let a =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 250, 250]);
  let b =
    u8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 5, 6]);
  let expected = u8x16::from([
    18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 255, 255,
  ]);
  let actual = a.saturating_add(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u8x16, b| a.saturating_add(b),
    |a, b| a.saturating_add(b),
  );
}

#[test]
fn impl_saturating_sub_for_u8x16() {
  let a = u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1, 0]);
  let b =
    u8x16::from([170, 18, 10, 200, 241, 2, 93, 4, 12, 8, 27, 28, 29, 30, 1, 1]);
  let expected = u8x16::from([0, 0, 0, 0, 0, 4, 0, 4, 0, 2, 0, 0, 0, 0, 0, 0]);
  let actual = a.saturating_sub(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u8x16, b| a.saturating_sub(b),
    |a, b| a.saturating_sub(b),
  );
}

#[test]
fn impl_bitand_for_u8x16() {
  let a = u8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = u8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = u8x16::from([0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u8x16, b| a & b, |a, b| a & b);
}

#[test]
fn impl_bitor_for_u8x16() {
  let a = u8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = u8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = u8x16::from([0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u8x16, b| a | b, |a, b| a | b);
}

#[test]
fn impl_bitxor_for_u8x16() {
  let a = u8x16::from([0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1]);
  let b = u8x16::from([0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]);
  let expected = u8x16::from([0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u8x16, b| a ^ b, |a, b| a ^ b);
}

#[test]
fn impl_u8x16_cmp_eq() {
  let a = u8x16::from([1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
  let b = u8x16::from([2_u8; 16]);
  let expected = u8x16::from([
    0,
    u8::MAX,
    0,
    0,
    0,
    u8::MAX,
    0,
    0,
    0,
    u8::MAX,
    0,
    0,
    0,
    u8::MAX,
    0,
    0,
  ]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_u8x16_blend() {
  let use_t: u8 = u8::MAX;
  let t =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 126, 127]);
  let f =
    u8x16::from([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 1, 1]);
  let mask = u8x16::from([
    use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0, use_t, 0,
    use_t, 0,
  ]);
  let expected =
    u8x16::from([1, 18, 3, 20, 5, 22, 7, 24, 9, 26, 11, 28, 13, 30, 126, 1]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_u8x16_max() {
  let a =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 250, 250]);
  let b =
    u8x16::from([17, 18, 19, 20, 2, 2, 2, 24, 25, 26, 27, 28, 29, 30, 5, 6]);
  let expected = u8x16::from([
    17, 18, 19, 20, 5, 6, 7, 24, 25, 26, 27, 28, 29, 30, 250, 250,
  ]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u8x16, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_u8x16_min() {
  let a =
    u8x16::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 250, 250]);
  let b =
    u8x16::from([17, 18, 19, 20, 2, 2, 2, 24, 25, 26, 27, 28, 29, 30, 5, 6]);
  let expected =
    u8x16::from([1, 2, 3, 4, 2, 2, 2, 8, 9, 10, 11, 12, 13, 14, 5, 6]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u8x16, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_unpack_low_u8() {
  let a = u8x16::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let b =
    u8x16::from([12, 11, 22, 13, 99, 15, 16, 17, 8, 19, 2, 21, 22, 3, 24, 127]);
  let c: [u8; 16] = u8x16::unpack_low(a, b).into();
  assert_eq!(c, [0, 12, 1, 11, 2, 22, 3, 13, 4, 99, 5, 15, 6, 16, 7, 17]);
}

#[test]
fn impl_unpack_high_u8() {
  let a = u8x16::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let b =
    u8x16::from([12, 11, 22, 13, 99, 15, 16, 17, 8, 19, 2, 21, 22, 3, 24, 127]);
  let c: [u8; 16] = u8x16::unpack_high(a, b).into();
  assert_eq!(c, [8, 8, 9, 19, 10, 2, 11, 21, 12, 22, 13, 3, 14, 24, 15, 127]);
}

#[test]
fn impl_narrow_i16x8() {
  let a = i16x8::from([-1, 2, -3, 4, -5, 6, -7, 8]);
  let b = i16x8::from([9, 10, 11, 12, 13, -14, 15, -16]);
  let c: [u8; 16] = u8x16::narrow_i16x8(a, b).into();
  assert_eq!(c, [0, 2, 0, 4, 0, 6, 0, 8, 9, 10, 11, 12, 13, 0, 15, 0]);
}

#[cfg(feature = "serde")]
#[test]
fn impl_u8x16_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&u8x16::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(u8x16::ZERO, deserialized);
}
