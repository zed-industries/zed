use super::*;

#[test]
fn test_blend_imm_i16_m128i() {
  let a = m128i::from([0_i16, 1, 2, 3, 4, 5, 6, 7]);
  let b = m128i::from([0_i16, -1, -2, -3, -4, -5, -6, -7]);
  //
  let c: [i16; 8] = blend_imm_i16_m128i::<0b1111_0110>(a, b).into();
  assert_eq!(c, [0_i16, -1, -2, 3, -4, -5, -6, -7]);
}

#[test]
fn test_blend_imm_m128d() {
  let a = m128d::from_array([0.0, 1.0]);
  let b = m128d::from_array([2.0, 3.0]);
  let c = blend_imm_m128d::<0b10>(a, b).to_array();
  assert_eq!(c, [0.0, 3.0]);
}

#[test]
fn test_blend_imm_m128() {
  let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
  let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
  let c = blend_imm_m128::<0b0110>(a, b).to_array();
  assert_eq!(c, [0.0, 5.0, 6.0, 3.0]);
}

#[test]
fn test_blend_varying_i8_m128i() {
  let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let b = m128i::from([0_i8, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15]);
  let mask = m128i::from([0_i8, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0]);
  let c: [i8; 16] = blend_varying_i8_m128i(a, b, mask).into();
  assert_eq!(c, [0, -1, -2, 3, 4, 5, -6, -7, -8, 9, 10, 11, -12, -13, -14, 15]);
}

#[test]
fn test_blend_varying_m128d() {
  let a = m128d::from_array([0.0, 1.0]);
  let b = m128d::from_array([2.0, 3.0]);
  let mask = m128d::from_array([-1.0, 0.0]);
  let c = blend_varying_m128d(a, b, mask).to_array();
  assert_eq!(c, [2.0, 1.0]);
}

#[test]
fn test_blend_varying_m128() {
  let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
  let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
  let mask = m128::from_array([-1.0, 0.0, -1.0, 0.0]);
  let c = blend_varying_m128(a, b, mask).to_array();
  assert_eq!(c, [4.0, 1.0, 6.0, 3.0]);
}

#[test]
fn test_ceil_m128d() {
  let a = m128d::from_array([-0.1, 1.8]);
  assert_eq!(ceil_m128d(a).to_array(), [0.0, 2.0]);
}

#[test]
fn test_ceil_m128() {
  let a = m128::from_array([-0.1, 1.8, 2.5, 3.0]);
  assert_eq!(ceil_m128(a).to_array(), [0.0, 2.0, 3.0, 3.0]);
}

#[test]
fn test_ceil_m128d_s() {
  let a = m128d::from_array([-0.1, 1.8]);
  let b = m128d::from_array([2.5, 3.0]);
  assert_eq!(ceil_m128d_s(a, b).to_array(), [3.0, 1.8]);
}

#[test]
fn test_ceil_m128_s() {
  let a = m128::from_array([-0.1, 1.8, 5.0, 6.0]);
  let b = m128::from_array([2.5, 3.0, 10.0, 20.0]);
  assert_eq!(ceil_m128_s(a, b).to_array(), [3.0, 1.8, 5.0, 6.0]);
}

#[test]
fn test_cmp_eq_mask_i64_m128i() {
  let a = m128i::from([5_i64, 6_i64]);
  let b = m128i::from([5_i64, 7_i64]);
  let c: [i64; 2] = cmp_eq_mask_i64_m128i(a, b).into();
  assert_eq!(c, [-1_i64, 0]);
}

#[test]
fn test_convert_to_i32_m128i_from_lower4_i16_m128i() {
  let a = m128i::from([0_i16, -1, 2, -3, 4, 5, 6, 7]);
  let c: [i32; 4] = convert_to_i32_m128i_from_lower4_i16_m128i(a).into();
  assert_eq!(c, [0, -1, 2, -3]);
}

#[test]
fn test_convert_to_i16_m128i_from_lower2_i16_m128i() {
  let a = m128i::from([0_i16, -1, 2, -3, 4, 5, 6, 7]);
  let c: [i64; 2] = convert_to_i16_m128i_from_lower2_i16_m128i(a).into();
  assert_eq!(c, [0, -1]);
}

#[test]
fn test_convert_to_i64_m128i_from_lower2_i32_m128i() {
  let a = m128i::from([0, -1, 2, -3]);
  let c: [i64; 2] = convert_to_i64_m128i_from_lower2_i32_m128i(a).into();
  assert_eq!(c, [0, -1]);
}

#[test]
fn test_convert_to_i16_m128i_from_lower8_i8_m128i() {
  let a = m128i::from([0_i8, -1, 2, -3, 4, -5, 6, -7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let c: [i16; 8] = convert_to_i16_m128i_from_lower8_i8_m128i(a).into();
  assert_eq!(c, [0_i16, -1, 2, -3, 4, -5, 6, -7]);
}

#[test]
fn test_convert_to_i32_m128i_from_lower4_i8_m128i() {
  let a = m128i::from([0_i8, -1, 2, -3, 4, -5, 6, -7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let c: [i32; 4] = convert_to_i32_m128i_from_lower4_i8_m128i(a).into();
  assert_eq!(c, [0, -1, 2, -3]);
}

#[test]
fn test_convert_to_u32_m128i_from_lower4_u16_m128i() {
  let a = m128i::from([u16::MAX, 1, 2, 3, 4, 5, 6, 7]);
  let c: [u32; 4] = convert_to_u32_m128i_from_lower4_u16_m128i(a).into();
  assert_eq!(c, [u16::MAX as u32, 1, 2, 3]);
}

#[test]
fn test_convert_to_u64_m128i_from_lower2_u16_m128i() {
  let a = m128i::from([u16::MAX, 1, 2, 3, 4, 5, 6, 7]);
  let c: [u64; 2] = convert_to_u64_m128i_from_lower2_u16_m128i(a).into();
  assert_eq!(c, [u16::MAX as u64, 1]);
}

#[test]
fn test_convert_to_u64_m128i_from_lower2_u32_m128i() {
  let a = m128i::from([u32::MAX, 1, 2, 3]);
  let c: [u64; 2] = convert_to_u64_m128i_from_lower2_u32_m128i(a).into();
  assert_eq!(c, [u32::MAX as u64, 1]);
}

#[test]
fn test_convert_to_u16_m128i_from_lower8_u8_m128i() {
  let a = m128i::from([u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let c: [u16; 8] = convert_to_u16_m128i_from_lower8_u8_m128i(a).into();
  assert_eq!(c, [u8::MAX as u16, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_convert_to_u32_m128i_from_lower4_u8_m128i() {
  let a = m128i::from([u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let c: [u32; 4] = convert_to_u32_m128i_from_lower4_u8_m128i(a).into();
  assert_eq!(c, [u8::MAX as u32, 1, 2, 3]);
}

#[test]
fn test_dot_product_m128d() {
  let a = m128d::from_array([1.0, 2.0]);
  let b = m128d::from_array([3.0, 4.0]);

  //

  let c = dot_product_m128d::<0b0000_0011>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0]); // no mul
  let c = dot_product_m128d::<0b0001_0011>(a, b).to_array();
  assert_eq!(c, [3.0, 3.0]); // mul lane 0 (1 * 3)
  let c = dot_product_m128d::<0b0010_0011>(a, b).to_array();
  assert_eq!(c, [8.0, 8.0]); // mul lane 1 (2 * 4)
  let c = dot_product_m128d::<0b0011_0011>(a, b).to_array();
  assert_eq!(c, [11.0, 11.0]); // mul both lanes (and summed in the next step)

  // After here we have two temp lanes, which get added to form `sum`.

  let c = dot_product_m128d::<0b0011_0000>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0]); // never use sum
  let c = dot_product_m128d::<0b0011_0001>(a, b).to_array();
  assert_eq!(c, [11.0, 0.0]); // sum in output lane 0
  let c = dot_product_m128d::<0b0011_0010>(a, b).to_array();
  assert_eq!(c, [0.0, 11.0]); // sum in output lane 1
  let c = dot_product_m128d::<0b0011_0011>(a, b).to_array();
  assert_eq!(c, [11.0, 11.0]); // sum in both output lanes.
}

#[test]
fn test_dot_product_m128() {
  let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);

  //

  let c = dot_product_m128::<0b0000_1111>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 0.0]); // no mul
  let c = dot_product_m128::<0b0001_1111>(a, b).to_array();
  assert_eq!(c, [5.0, 5.0, 5.0, 5.0]); // mul temp lane 0 (1 * 5)
  let c = dot_product_m128::<0b0010_1111>(a, b).to_array();
  assert_eq!(c, [12.0, 12.0, 12.0, 12.0]); // mul temp lane 1 (2 * 6)
  let c = dot_product_m128::<0b0100_1111>(a, b).to_array();
  assert_eq!(c, [21.0, 21.0, 21.0, 21.0]); // mul temp lane 2 (3 * 7)
  let c = dot_product_m128::<0b1000_1111>(a, b).to_array();
  assert_eq!(c, [32.0, 32.0, 32.0, 32.0]); // mul temp lane 3 (4 * 8)
  let c = dot_product_m128::<0b1111_1111>(a, b).to_array();
  assert_eq!(c, [70.0, 70.0, 70.0, 70.0]); // mul all lanes (and summed in the next step)

  // After here we have four temp lanes, which get added to form `sum`.

  let c = dot_product_m128::<0b1111_0000>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 0.0]); // never use sum

  let c = dot_product_m128::<0b1111_0001>(a, b).to_array();
  assert_eq!(c, [70.0, 0.0, 0.0, 0.0]); // sum in output lane 0

  let c = dot_product_m128::<0b1111_0010>(a, b).to_array();
  assert_eq!(c, [0.0, 70.0, 0.0, 0.0]); // sum in output lane 1

  let c = dot_product_m128::<0b1111_0100>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 70.0, 0.0]); // sum in output lane 2

  let c = dot_product_m128::<0b1111_1000>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 70.0]); // sum in output lane 3

  let c = dot_product_m128::<0b1111_1111>(a, b).to_array();
  assert_eq!(c, [70.0, 70.0, 70.0, 70.0]); // sum in all output lanes
}

#[test]
fn test_extract_i32_imm_m128i() {
  let a = m128i::from([5, 6, 7, 8]);
  assert_eq!(extract_i32_imm_m128i::<1>(a), 6);
}

#[test]
fn test_ptest_i128() {
  let a = m128i::from([1, 0, 1, 0]);
  let b = m128i::from([0, 1, 0, 0]);

  assert_eq!(testz_m128i(a, a), 0);
  assert_eq!(testz_m128i(a, b), 1);
  assert_eq!(testz_m128i(b, b), 0);

  assert_eq!(testc_m128i(a, a), 1);
  assert_eq!(testc_m128i(a, b), 0);
  assert_eq!(testc_m128i(b, b), 1);
}
