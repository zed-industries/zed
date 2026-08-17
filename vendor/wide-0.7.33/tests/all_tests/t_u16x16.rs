use std::num::Wrapping;

use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<u16x16>(), 32);
  assert_eq!(core::mem::align_of::<u16x16>(), 32);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<u16x16, _, 16>();
}

#[test]
fn impl_add_for_u16x16() {
  let a = u16x16::from([
    1,
    2,
    i16::MAX as u16 - 1,
    i16::MAX as u16 - 1,
    15,
    20,
    5000,
    2990,
    1,
    2,
    i16::MAX as u16 - 1,
    i16::MAX as u16 - 1,
    15,
    20,
    5000,
    2990,
  ]);
  let b = u16x16::from([
    17, 18, 1, 2, 20, 5, 900, 900, 17, 18, 1, 2, 20, 5, 900, 900,
  ]);
  let expected = u16x16::from([
    18,
    20,
    i16::MAX as u16,
    i16::MIN as u16,
    35,
    25,
    5900,
    3890,
    18,
    20,
    i16::MAX as u16,
    i16::MIN as u16,
    35,
    25,
    5900,
    3890,
  ]);
  let actual = a + b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u16x16, b| a + b,
    |a, b| a.wrapping_add(b),
  );
}

#[test]
fn impl_sub_for_u16x16() {
  let a = u16x16::from([
    1,
    2,
    1,
    2,
    15,
    20,
    5000,
    2990,
    1,
    2,
    u16::MAX,
    u16::MAX - 1,
    15,
    20,
    5000,
    2990,
  ]);
  let b = u16x16::from([
    17, 18, 1, 1, 20, 5, 900, 900, 17, 18, 1, 1, 20, 5, 900, 900,
  ]);
  let expected = u16x16::from([
    65520, 65520, 0, 1, 65531, 15, 4100, 2090, 65520, 65520, 65534, 65533,
    65531, 15, 4100, 2090,
  ]);
  let actual = a - b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u16x16, b| a - b,
    |a, b| a.wrapping_sub(b),
  );
}

#[test]
fn impl_saturating_add_for_u16x16() {
  let a = u16x16::from([
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    15,
    20,
    5000,
    2990,
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    15,
    20,
    5000,
    2990,
  ]);
  let b = u16x16::from([
    17, 18, 1, 2, 20, 5, 900, 900, 17, 18, 1, 2, 20, 5, 900, 900,
  ]);
  let expected = u16x16::from([
    18,
    20,
    u16::MAX,
    u16::MAX,
    35,
    25,
    5900,
    3890,
    18,
    20,
    u16::MAX,
    u16::MAX,
    35,
    25,
    5900,
    3890,
  ]);
  let actual = a.saturating_add(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u16x16, b| a.saturating_add(b),
    |a, b| a.saturating_add(b),
  );
}

#[test]
fn impl_saturating_sub_for_u16x16() {
  let a = u16x16::from([
    1, 2, 1, 0, 15, 20, 5000, 2990, 1, 2, 1, 0, 15, 20, 5000, 2990,
  ]);
  let b = u16x16::from([
    17, 18, 1, 1, 20, 5, 900, 900, 17, 18, 1, 1, 20, 5, 900, 900,
  ]);
  let expected = u16x16::from([
    0, 0, 0, 0, 0, 15, 4100, 2090, 0, 0, 0, 0, 0, 15, 4100, 2090,
  ]);
  let actual = a.saturating_sub(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u16x16, b| a.saturating_sub(b),
    |a, b| a.saturating_sub(b),
  );
}

#[test]
fn impl_bitand_for_u16x16() {
  let a = u16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = u16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = u16x16::from([0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, b| a & b, |a, b| a & b);
}

#[test]
fn impl_bitor_for_u16x16() {
  let a = u16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = u16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = u16x16::from([0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, b| a | b, |a, b| a | b);
}

#[test]
fn impl_bitxor_for_u16x16() {
  let a = u16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = u16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = u16x16::from([0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, b| a ^ b, |a, b| a ^ b);
}

#[test]
fn impl_shl_for_u16x16() {
  let a = u16x16::from([
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    128,
    255,
    590,
    5667,
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    128,
    255,
    590,
    5667,
  ]);
  let b = 2;
  let expected = u16x16::from([
    1 << 2,
    2 << 2,
    (u16::MAX - 1) << 2,
    (u16::MAX - 1) << 2,
    128 << 2,
    255 << 2,
    590 << 2,
    5667 << 2,
    1 << 2,
    2 << 2,
    (u16::MAX - 1) << 2,
    (u16::MAX - 1) << 2,
    128 << 2,
    255 << 2,
    590 << 2,
    5667 << 2,
  ]);
  let actual = a << b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, _b| a << 3, |a, _b| a << 3);
}

#[test]
fn impl_shr_for_u16x16() {
  let a = u16x16::from([
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    128,
    255,
    590,
    5667,
    1,
    2,
    u16::MAX - 1,
    u16::MAX - 1,
    128,
    255,
    590,
    5667,
  ]);
  let b = 2;
  let expected = u16x16::from([
    1 >> 2,
    2 >> 2,
    (u16::MAX - 1) >> 2,
    (u16::MAX - 1) >> 2,
    128 >> 2,
    255 >> 2,
    590 >> 2,
    5667 >> 2,
    1 >> 2,
    2 >> 2,
    (u16::MAX - 1) >> 2,
    (u16::MAX - 1) >> 2,
    128 >> 2,
    255 >> 2,
    590 >> 2,
    5667 >> 2,
  ]);
  let actual = a >> b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, _b| a >> 3, |a, _b| a >> 3);
}

#[test]
fn impl_u16x16_cmp_eq() {
  let a = u16x16::from([1, 2, 3, 4, 2, 1, 8, 2, 1, 2, 3, 4, 2, 1, 8, 2]);
  let b = u16x16::from([2_u16; 16]);
  let expected = u16x16::from([
    0,
    u16::MAX,
    0,
    0,
    u16::MAX,
    0,
    0,
    u16::MAX,
    0,
    u16::MAX,
    0,
    0,
    u16::MAX,
    0,
    0,
    u16::MAX,
  ]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_u16x16_blend() {
  let use_t: u16 = u16::MAX;
  let t = u16x16::from([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
  let f = u16x16::from([
    17, 18, 19, 20, 25, 30, 50, 90, 17, 18, 19, 20, 25, 30, 50, 90,
  ]);
  let mask = u16x16::from([
    use_t, 0, use_t, 0, 0, 0, 0, use_t, use_t, 0, use_t, 0, 0, 0, 0, use_t,
  ]);
  let expected =
    u16x16::from([1, 18, 3, 20, 25, 30, 50, 8, 1, 18, 3, 20, 25, 30, 50, 8]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_u16x16_max() {
  let a =
    u16x16::from([u16::MAX, 2, 1, 0, 6, 8, 12, 9, 1, 2, 1, 0, 6, 8, 12, 9]);
  let b = u16x16::from([17, 0, 1, 1, 19, 0, 0, 0, 17, 0, 1, 1, 19, 0, 0, 0]);
  let expected =
    u16x16::from([u16::MAX, 2, 1, 1, 19, 8, 12, 9, 17, 2, 1, 1, 19, 8, 12, 9]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_u16x16_from_u8x16() {
  let v = [10u8, 2, 3, 4, 5, 6, 7, 8, 9, 7, 127, 12, 13, 6, 55, 255];

  assert_eq!(
    u16x16::from(v.map(|a| u16::from(a))),
    u16x16::from(u8x16::from(v))
  );

  crate::test_random_vector_vs_scalar(
    |a: u8x16, _b| u16x16::from(a),
    |a, _b| u16::from(a),
  );
}

#[test]
fn impl_u16x16_min() {
  let a = u16x16::from([1, 2, 1, 0, 6, 8, 12, 9, 1, 2, 1, 0, 6, 8, 12, 9]);
  let b =
    u16x16::from([u16::MAX, 0, 1, 1, 19, 0, 0, 0, 17, 0, 1, 1, 19, 0, 0, 0]);
  let expected = u16x16::from([1, 0, 1, 0, 6, 0, 0, 0, 1, 0, 1, 0, 6, 0, 0, 0]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: u16x16, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_mul_for_u16x16() {
  let a = u16x16::from([
    2,
    2,
    i16::MAX as u16,
    4,
    5,
    6,
    u16::MIN + 1,
    u16::MIN,
    1,
    2,
    i16::MAX as u16,
    4,
    5,
    6,
    u16::MIN + 1,
    u16::MIN,
  ]);
  let b = u16x16::from([
    17, 18, 190, 20, 21, 22, 1, 1, 17, 18, 190, 20, 21, 22, 1, 1,
  ]);
  let expected = u16x16::from([
    2 * 17,
    36,
    (Wrapping(i16::MAX as u16) * Wrapping(190)).0,
    80,
    105,
    132,
    u16::MIN + 1,
    u16::MIN,
    17,
    36,
    (Wrapping(i16::MAX as u16) * Wrapping(190)).0,
    80,
    105,
    132,
    u16::MIN + 1,
    u16::MIN,
  ]);
  let actual = a * b;
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(
    |a: u16x16, b| a * b,
    |a, b| a.wrapping_mul(b),
  );
}

#[cfg(feature = "serde")]
#[test]
fn impl_u16x16_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&u16x16::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(u16x16::ZERO, deserialized);
}
