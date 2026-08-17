use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i64x2>(), 16);
  assert_eq!(core::mem::align_of::<i64x2>(), 16);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i64x2, _, 2>();
}

#[test]
fn impl_add_for_i64x2() {
  let a = i64x2::from([i64::MAX - 1, i64::MAX - 1]);
  let b = i64x2::from([1, 2]);
  let expected = i64x2::from([i64::MAX, i64::MIN]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i64x2() {
  let a = i64x2::from([i64::MIN + 1, i64::MIN]);
  let b = i64x2::from([1, 1]);
  let expected = i64x2::from([i64::MIN, i64::MAX]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_i64x2() {
  let a = i64x2::from([i64::MIN + 1, 24]);
  let b = i64x2::from([1, -26]);
  let expected = i64x2::from([i64::MIN + 1, 24 * -26]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_i64x2() {
  let a = i64x2::from([1, 1]);
  let b = i64x2::from([0, 1]);
  let expected = i64x2::from([0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i64x2() {
  let a = i64x2::from([1, 1]);
  let b = i64x2::from([0, 1]);
  let expected = i64x2::from([1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i64x2() {
  let a = i64x2::from([1, 1]);
  let b = i64x2::from([0, 1]);
  let expected = i64x2::from([1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shl_for_i64x2() {
  let a = i64x2::from([i64::MAX - 1, i64::MAX - 1]);
  let b = 2;
  let expected = i64x2::from([(i64::MAX - 1) << 2, (i64::MAX - 1) << 2]);
  let actual = a << b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i64x2_blend() {
  let use_t: i64 = -1;
  let t = i64x2::from([1, 2]);
  let f = i64x2::from([17, 18]);
  let mask = i64x2::from([use_t, 0]);
  let expected = i64x2::from([1, 18]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i64x2_abs() {
  let a = i64x2::from([-1, i64::MIN]);
  let expected = i64x2::from([1, i64::MIN]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i64x2_unsigned_abs() {
  let a = i64x2::from([-1, i64::MIN]);
  let expected = u64x2::from([1, i64::MIN as u64]);
  let actual = a.unsigned_abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i64x2_cmp_eq() {
  let a = i64x2::from([1_i64, 4]);
  let b = i64x2::from([3_i64, 4]);
  let expected = i64x2::from([0, -1]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i64x2_cmp_gt() {
  let a = i64x2::from([3_i64, 4]);
  let b = i64x2::from([1_i64, 4]);
  let expected = i64x2::from([-1, 0]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn test_i64x2_any() {
  let a = i64x2::from([3, -1]);
  assert!(a.any());
  //
  let a = i64x2::from([1, 0]);
  assert!(!a.any());
}

#[test]
fn test_i64x2_all() {
  let a = i64x2::from([-1, -1]);
  assert!(a.all(), "{:?}", a);
  //
  let a = i64x2::from([1, -1]);
  assert!(!a.all());
}

#[test]
fn test_i64x2_none() {
  let a = i64x2::from([1, 0]);
  assert!(a.none());
  //
  let a = i64x2::from([1, -1]);
  assert!(!a.none());
}

#[test]
fn test_i64x2_move_mask() {
  let a = i64x2::from([-1, 0]);
  let expected = 0b01;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
  //
  let a = i64x2::from([1, -1]);
  let expected = 0b10;
  let actual = a.move_mask();
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar_reduce(
    |a: i64x2| a.move_mask(),
    0i32,
    |acc, a, idx| acc | if a < 0 { 1 << idx } else { 0 },
  );
}

#[cfg(feature = "serde")]
#[test]
fn impl_i64x2_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i64x2::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i64x2::ZERO, deserialized);
}
