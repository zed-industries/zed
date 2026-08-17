use wide::*;

#[test]
fn size_align() {
  assert_eq!(core::mem::size_of::<i16x16>(), 32);
  assert_eq!(core::mem::align_of::<i16x16>(), 32);
}

#[test]
fn basic_traits() {
  crate::test_basic_traits::<i16x16, _, 16>();
}

#[test]
fn impl_add_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    15,
    20,
    5000,
    2990,
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    15,
    20,
    5000,
    2990,
  ]);
  let b = i16x16::from([
    17, 18, 1, 2, 20, 5, 900, 900, 17, 18, 1, 2, 20, 5, 900, 900,
  ]);
  let expected = i16x16::from([
    18,
    20,
    i16::MAX,
    i16::MIN,
    35,
    25,
    5900,
    3890,
    18,
    20,
    i16::MAX,
    i16::MIN,
    35,
    25,
    5900,
    3890,
  ]);
  let actual = a + b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_sub_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    15,
    20,
    5000,
    2990,
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    15,
    20,
    5000,
    2990,
  ]);
  let b = i16x16::from([
    17, -18, 1, 1, 20, 5, 900, 900, 17, -18, 1, 1, 20, 5, 900, 900,
  ]);
  let expected = i16x16::from([
    -16,
    20,
    i16::MIN,
    i16::MAX,
    -5,
    15,
    4100,
    2090,
    -16,
    20,
    i16::MIN,
    i16::MAX,
    -5,
    15,
    4100,
    2090,
  ]);
  let actual = a - b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_saturating_add_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    15,
    20,
    5000,
    2990,
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    15,
    20,
    5000,
    2990,
  ]);
  let b = i16x16::from([
    17, 18, 1, 2, 20, 5, 900, 900, 17, 18, 1, 2, 20, 5, 900, 900,
  ]);
  let expected = i16x16::from([
    18,
    20,
    i16::MAX,
    i16::MAX,
    35,
    25,
    5900,
    3890,
    18,
    20,
    i16::MAX,
    i16::MAX,
    35,
    25,
    5900,
    3890,
  ]);
  let actual = a.saturating_add(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_saturating_sub_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    15,
    20,
    5000,
    2990,
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    15,
    20,
    5000,
    2990,
  ]);
  let b = i16x16::from([
    17, -18, 1, 1, 20, 5, 900, 900, 17, -18, 1, 1, 20, 5, 900, 900,
  ]);
  let expected = i16x16::from([
    -16,
    20,
    i16::MIN,
    i16::MIN,
    -5,
    15,
    4100,
    2090,
    -16,
    20,
    i16::MIN,
    i16::MIN,
    -5,
    15,
    4100,
    2090,
  ]);
  let actual = a.saturating_sub(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_scale_i16x16() {
  let a = i16x16::from([
    0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300,
    1400, 1500,
  ]);
  let b = i16x16::from([
    0, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000,
    2100, 2200, 2300,
  ]);
  let actual = a.mul_scale_round(b);
  let expected = i16x16::from([
    0, 3, 6, 10, 15, 20, 26, 32, 39, 47, 55, 64, 73, 83, 94, 105,
  ]);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_scale_n_i16x16() {
  let a = i16x16::from([
    0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300,
    1400, 1500,
  ]);
  let actual = a.mul_scale_round_n(16400); // slightly higher than 0.5 to test rounding
  let expected = i16x16::from([
    0, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 551, 601, 651, 701, 751,
  ]);
  assert_eq!(expected, actual);
}

#[test]
fn impl_mul_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    2,
    3,
    4,
    5,
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    2,
    3,
    4,
    5,
  ]);
  let b =
    i16x16::from([17, -18, 1, 1, -1, -2, -6, 3, 17, -18, 1, 1, -1, -2, -6, 3]);
  let expected = i16x16::from([
    17,
    -36,
    i16::MIN + 1,
    i16::MIN,
    -2,
    -6,
    -24,
    15,
    17,
    -36,
    i16::MIN + 1,
    i16::MIN,
    -2,
    -6,
    -24,
    15,
  ]);
  let actual = a * b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitand_for_i16x16() {
  let a = i16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i16x16::from([0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1]);
  let actual = a & b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitor_for_i16x16() {
  let a = i16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i16x16::from([0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1]);
  let actual = a | b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_bitxor_for_i16x16() {
  let a = i16x16::from([0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1]);
  let b = i16x16::from([0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1]);
  let expected = i16x16::from([0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0]);
  let actual = a ^ b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shl_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    128,
    255,
    590,
    5667,
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    128,
    255,
    590,
    5667,
  ]);
  let b = 2;
  let expected = i16x16::from([
    1 << 2,
    2 << 2,
    (i16::MAX - 1) << 2,
    (i16::MAX - 1) << 2,
    128 << 2,
    255 << 2,
    590 << 2,
    5667 << 2,
    1 << 2,
    2 << 2,
    (i16::MAX - 1) << 2,
    (i16::MAX - 1) << 2,
    128 << 2,
    255 << 2,
    590 << 2,
    5667 << 2,
  ]);
  let actual = a << b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_shr_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    128,
    255,
    590,
    5667,
    1,
    2,
    i16::MAX - 1,
    i16::MAX - 1,
    128,
    255,
    590,
    5667,
  ]);
  let b = 2;
  let expected = i16x16::from([
    1 >> 2,
    2 >> 2,
    (i16::MAX - 1) >> 2,
    (i16::MAX - 1) >> 2,
    128 >> 2,
    255 >> 2,
    590 >> 2,
    5667 >> 2,
    1 >> 2,
    2 >> 2,
    (i16::MAX - 1) >> 2,
    (i16::MAX - 1) >> 2,
    128 >> 2,
    255 >> 2,
    590 >> 2,
    5667 >> 2,
  ]);
  let actual = a >> b;
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_cmp_eq() {
  let a = i16x16::from([1, 2, 3, 4, 2, 1, 8, 2, 1, 2, 3, 4, 2, 1, 8, 2]);
  let b = i16x16::from([2_i16; 16]);
  let expected =
    i16x16::from([0, -1, 0, 0, -1, 0, 0, -1, 0, -1, 0, 0, -1, 0, 0, -1]);
  let actual = a.cmp_eq(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_cmp_gt() {
  let a = i16x16::from([1, 2, 9, 4, 1, 2, 8, 10, 1, 2, 9, 4, 1, 2, 8, 10]);
  let b = i16x16::from([5_i16; 16]);
  let expected =
    i16x16::from([0, 0, -1, 0, 0, 0, -1, -1, 0, 0, -1, 0, 0, 0, -1, -1]);
  let actual = a.cmp_gt(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_cmp_lt() {
  let a = i16x16::from([1, 2, 9, 4, 1, 2, 8, 10, 1, 2, 9, 4, 1, 2, 8, 10]);
  let b = i16x16::from([5_i16; 16]);
  let expected =
    i16x16::from([-1, -1, 0, -1, -1, -1, 0, 0, -1, -1, 0, -1, -1, -1, 0, 0]);
  let actual = a.cmp_lt(b);
  assert_eq!(expected, actual);

  let expected = i16x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  let actual = a.cmp_lt(a);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_blend() {
  let use_t: i16 = -1;
  let t = i16x16::from([1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
  let f = i16x16::from([
    17, 18, 19, 20, 25, 30, 50, 90, 17, 18, 19, 20, 25, 30, 50, 90,
  ]);
  let mask = i16x16::from([
    use_t, 0, use_t, 0, 0, 0, 0, use_t, use_t, 0, use_t, 0, 0, 0, 0, use_t,
  ]);
  let expected =
    i16x16::from([1, 18, 3, 20, 25, 30, 50, 8, 1, 18, 3, 20, 25, 30, 50, 8]);
  let actual = mask.blend(t, f);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_abs() {
  let a = i16x16::from([
    -1,
    2,
    -3,
    i16::MIN,
    6,
    -15,
    -19,
    9,
    -1,
    2,
    -3,
    i16::MIN,
    6,
    -15,
    -19,
    9,
  ]);
  let expected = i16x16::from([
    1,
    2,
    3,
    i16::MIN,
    6,
    15,
    19,
    9,
    1,
    2,
    3,
    i16::MIN,
    6,
    15,
    19,
    9,
  ]);
  let actual = a.abs();
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_max() {
  let a = i16x16::from([
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    12,
    9,
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    12,
    9,
  ]);
  let b = i16x16::from([
    17, -18, 1, 1, 19, -5, -1, -9, 17, -18, 1, 1, 19, -5, -1, -9,
  ]);
  let expected =
    i16x16::from([17, 2, 1, 1, 19, -5, 12, 9, 17, 2, 1, 1, 19, -5, 12, 9]);
  let actual = a.max(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i16x16, b| a.max(b), |a, b| a.max(b));
}

#[test]
fn impl_i16x16_min() {
  let a = i16x16::from([
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    12,
    9,
    1,
    2,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    12,
    9,
  ]);
  let b = i16x16::from([
    17, -18, 1, 1, 19, -5, -1, -9, 17, -18, 1, 1, 19, -5, -1, -9,
  ]);
  let expected = i16x16::from([
    1,
    -18,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    -1,
    -9,
    1,
    -18,
    i16::MIN + 1,
    i16::MIN,
    6,
    -8,
    -1,
    -9,
  ]);
  let actual = a.min(b);
  assert_eq!(expected, actual);

  crate::test_random_vector_vs_scalar(|a: i16x16, b| a.min(b), |a, b| a.min(b));
}

#[test]
fn impl_from_i8x16() {
  let a = i8x16::from([
    10,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    7,
    i8::MAX,
    12,
    13,
    6,
    55,
    i8::MIN,
  ]);

  let actual = i16x16::from_i8x16(a);

  let expected = i16x16::from([
    10,
    2,
    -3,
    4,
    5,
    -6,
    7,
    8,
    9,
    7,
    i8::MAX as i16,
    12,
    13,
    6,
    55,
    i8::MIN as i16,
  ]);

  assert_eq!(expected, actual);
}

#[test]
fn test_i16x16_move_mask() {
  let indexes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

  for x in 0..256 {
    // multiply by prime number to mix bits a bit
    let i = x * 251;

    let a =
      i16x16::from(indexes.map(|x| if i & (1 << x) != 0 { -1 } else { 0 }));

    assert_eq!(a.move_mask(), i);
  }

  let a =
    i16x16::from([-1, 0, -2, -3, -1, 0, -2, -3, -1, 0, -1, 0, -1, 0, -1, 0]);

  let expected = 0b0101010111011101;
  let actual = a.move_mask();
  assert_eq!(expected, actual);
}

#[test]
fn test_i16x16_any() {
  let a = i16x16::from([0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(a.any());

  let a = i16x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(a.any());

  //
  let a = i16x16::from([0; 16]);
  assert!(!a.any());
}

#[test]
fn test_i16x16_all() {
  let a = i16x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(!a.all());
  //
  let a = i16x16::from([-1; 16]);
  assert!(a.all());
}

#[test]
fn test_i16x16_none() {
  let a = i16x16::from([0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  assert!(!a.none());

  let a = i16x16::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0]);
  assert!(!a.none());

  //
  let a = i16x16::from([0; 16]);
  assert!(a.none());
}

#[test]
fn impl_i16x16_reduce_add() {
  let p =
    i16x16::from([1, 2, 3, 4, 5, 6, 7, 9, 10, 20, 30, 40, 50, 60, 70, 90]);
  assert_eq!(p.reduce_add(), 407);
}

#[test]
fn impl_dot_for_i16x16() {
  let a = i16x16::from([
    1,
    2,
    3,
    4,
    5,
    6,
    i16::MIN + 1,
    i16::MIN,
    10,
    20,
    30,
    40,
    50,
    60,
    i16::MAX - 1,
    i16::MAX,
  ]);
  let b = i16x16::from([
    17, -18, 190, -20, 21, -22, 3, 2, 170, -180, 1900, -200, 210, -220, 30, 20,
  ]);
  let expected =
    i32x8::from([-19, 490, -27, -163837, -1900, 49000, -2700, 1638320]);
  let actual = a.dot(b);
  assert_eq!(expected, actual);
}

#[test]
fn impl_i16x16_reduce_min() {
  for i in 0..8 {
    let mut v = [i16::MAX; 16];
    v[i] = i16::MIN;
    let p = i16x16::from(v);
    assert_eq!(p.reduce_min(), i16::MIN);
  }
}

#[test]
fn impl_i16x16_reduce_max() {
  for i in 0..8 {
    let mut v = [i16::MIN; 16];
    v[i] = i16::MAX;
    let p = i16x16::from(v);
    assert_eq!(p.reduce_min(), i16::MIN);
  }
}

#[cfg(feature = "serde")]
#[test]
fn impl_i16x16_ser_de_roundtrip() {
  let serialized =
    bincode::serialize(&i16x16::ZERO).expect("serialization failed");
  let deserialized =
    bincode::deserialize(&serialized).expect("deserializaion failed");
  assert_eq!(i16x16::ZERO, deserialized);
}
