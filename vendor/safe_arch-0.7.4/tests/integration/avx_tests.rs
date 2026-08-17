use super::*;

#[test]
fn test_add_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([5.0, 6.0, 7.0, 8.5]);
  let c = add_m256d(a, b).to_array();
  assert_eq!(c, [6.0, 8.0, 10.0, 12.5]);
}

#[test]
fn test_add_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
  let b = m256::from_array([5.0, 6.0, 7.0, 8.5, 90.0, 100.0, 110.0, 51.0]);
  let c = add_m256(a, b).to_array();
  assert_eq!(c, [6.0, 8.0, 10.0, 12.5, 110.0, 130.0, 150.0, 101.0]);
}

#[test]
fn test_addsub_m256d() {
  let a = m256d::from_array([10.0, 20.0, 30.0, 40.0]);
  let b = m256d::from_array([100.0, 200.0, 300.0, 400.0]);
  let c = addsub_m256d(a, b).to_array();
  assert_eq!(c, [-90.0, 220.0, -270.0, 440.0]);
}

#[test]
fn test_addsub_m256() {
  let a = m256::from_array([10.0, 20.0, 30.0, 40.0, 1.0, 2.0, 3.0, 4.0]);
  let b = m256::from_array([1.0, 20.0, 3.0, 40.0, 11.0, 12.0, 13.0, 14.0]);
  let c = addsub_m256(a, b).to_array();
  assert_eq!(c, [9.0, 40.0, 27.0, 80.0, -10.0, 14.0, -10.0, 18.0]);
}

#[test]
fn test_bitand_m256d() {
  let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
  let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
  let c = bitand_m256d(a, b).to_array();
  assert_eq!(c, [1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_bitand_m256() {
  let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
  let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
  let c = bitand_m256(a, b).to_array();
  assert_eq!(c, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_bitandnot_m256d() {
  let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
  let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
  let c = bitandnot_m256d(a, b).to_array();
  assert_eq!(c, [0.0, 1.0, 0.0, 0.0]);
}

#[test]
fn test_bitandnot_m256() {
  let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
  let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
  let c = bitandnot_m256(a, b).to_array();
  assert_eq!(c, [0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
}

#[test]
fn test_blend_m256d() {
  let a = m256d::from_array([10.0, 20.0, 30.0, 40.0]);
  let b = m256d::from_array([100.0, 200.0, 300.0, 400.0]);
  //
  let c = blend_m256d::<0b0110>(a, b).to_array();
  assert_eq!(c, [10.0, 200.0, 300.0, 40.0]);
}

#[test]
fn test_blend_m256() {
  let a = m256::from_array([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
  let b = m256::from_array([100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0]);
  //
  let c = blend_m256::<0b0011_0110>(a, b).to_array();
  assert_eq!(c, [10.0, 200.0, 300.0, 40.0, 500.0, 600.0, 70.0, 80.0]);
}

#[test]
fn test_blend_varying_m256d() {
  let a = m256d::from_array([0.0, 1.0, 20.0, 30.0]);
  let b = m256d::from_array([2.0, 3.0, 70.0, 80.0]);
  let mask = m256d::from_array([-1.0, 0.0, 0.0, -1.0]);
  let c = blend_varying_m256d(a, b, mask).to_array();
  assert_eq!(c, [2.0, 1.0, 20.0, 80.0]);
}

#[test]
fn test_blend_varying_m256() {
  let a = m256::from_array([0.0, 1.0, 2.0, 3.0, 8.0, 9.0, 10.0, 11.0]);
  let b = m256::from_array([4.0, 5.0, 6.0, 7.0, -4.0, -5.0, -6.0, -7.0]);
  let mask = m256::from_array([-1.0, 0.0, -1.0, 0.0, -1.0, -1.0, 0.0, 0.0]);
  let c = blend_varying_m256(a, b, mask).to_array();
  assert_eq!(c, [4.0, 1.0, 6.0, 3.0, -4.0, -5.0, 10.0, 11.0]);
}

#[test]
fn test_load_m128d_splat_m256d() {
  let a = m128d::from_array([0.0, 1.0]);
  let b = load_m128d_splat_m256d(&a).to_array();
  assert_eq!(b, [0.0, 1.0, 0.0, 1.0]);
}

#[test]
fn test_load_m128_splat_m256() {
  let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
  let b = load_m128_splat_m256(&a).to_array();
  assert_eq!(b, [0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_load_f64_splat_m256d() {
  let a = 1.0;
  let b = load_f64_splat_m256d(&a).to_array();
  assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_load_f32_splat_m256() {
  let a = 1.0;
  let b = load_f32_splat_m256(&a).to_array();
  assert_eq!(b, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_cast_to_m256_from_m256d() {
  let a = load_f64_splat_m256d(&1.0);
  assert_eq!(cast_to_m256_from_m256d(a).to_bits(), [0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000]);
}

#[test]
fn test_cast_to_m256i_from_m256d() {
  let a = load_f64_splat_m256d(&1.0);
  let b: [u64; 4] = cast_to_m256i_from_m256d(a).into();
  assert_eq!(b, [0x3FF00000_00000000_u64; 4]);
}

#[test]
fn test_cast_to_m256d_from_m256i() {
  let a = m256i::from([1.0_f64.to_bits(); 4]);
  let b = cast_to_m256d_from_m256i(a).to_array();
  assert_eq!(b, [1.0; 4]);
}

#[test]
fn test_cast_to_m256_from_m256i() {
  let a = m256i::from([1.0_f32.to_bits(); 8]);
  let b = cast_to_m256_from_m256i(a).to_array();
  assert_eq!(b, [1.0; 8]);
}

#[test]
fn test_cast_to_m128_from_m256() {
  let a = m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = cast_to_m128_from_m256(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_cast_to_m128d_from_m256d() {
  let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
  let b = cast_to_m128d_from_m256d(a).to_array();
  assert_eq!(b, [1.0, 2.0]);
}

#[test]
fn test_cast_to_m128i_from_m256i() {
  let a = m256i::from([1, 2, 3, 4, 5, 6, 7, 8]);
  let b: [i32; 4] = cast_to_m128i_from_m256i(a).into();
  assert_eq!(b, [1, 2, 3, 4]);
}

#[test]
fn test_ceil_m256d() {
  let a = m256d::from([1.1, 2.5, 3.8, 5.0]);
  let b = ceil_m256d(a).to_array();
  assert_eq!(b, [2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn test_ceil_m256() {
  let a = m256::from([1.1, 2.5, 3.8, 5.0, -0.5, -1.1, -2.7, -3.0]);
  let b = ceil_m256(a).to_array();
  assert_eq!(b, [2.0, 3.0, 4.0, 5.0, 0.0, -1.0, -2.0, -3.0]);
}

#[test]
fn test_cmp_op_mask_m128() {
  let a = m128::from_array([2.0, 0.0, -2.0, 0.0]);
  let b = m128::from_array([1.0, 1.0, -1.0, -1.0]);
  let c = cmp_op_mask_m128::<{ cmp_op!(GreaterThanOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u32::MAX, 0, 0, u32::MAX]);
}

#[test]
fn test_cmp_op_mask_m128_s() {
  let a = m128::from_array([2.0, 0.0, -2.0, 0.0]);
  let b = m128::from_array([1.0, 1.0, -1.0, -1.0]);
  let c = cmp_op_mask_m128_s::<{ cmp_op!(GreaterThanOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u32::MAX, 0, (-2_f32).to_bits(), 0]);
}

#[test]
fn test_cmp_op_mask_m256() {
  let a = m256::from_array([1.0, 5.0, 0.0, 7.0, 5.0, 6.0, 7.0, -20.0]);
  let b = m256::from_array([2.0, 1.0, 3.0, 4.0, 1.0, -2.0, -3.0, -4.0]);
  let c = cmp_op_mask_m256::<{ cmp_op!(LessThanOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u32::MAX, 0, u32::MAX, 0, 0, 0, 0, u32::MAX]);
}

#[test]
fn test_cmp_op_mask_m128d() {
  let a = m128d::from_array([1.0, 0.0]);
  let b = m128d::from_array([1.0, 1.0]);
  let c = cmp_op_mask_m128d::<{ cmp_op!(EqualOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u64::MAX, 0]);
}

#[test]
fn test_cmp_op_mask_m128d_s() {
  let a = m128d::from_array([1.0, 7.0]);
  let b = m128d::from_array([1.0, 1.0]);
  let c = cmp_op_mask_m128d_s::<{ cmp_op!(EqualOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u64::MAX, 7_f64.to_bits()]);
}

#[test]
fn test_cmp_op_mask_m256d() {
  let a = m256d::from_array([1.0, 5.0, 0.0, 7.0]);
  let b = m256d::from_array([2.0, 1.0, 3.0, 4.0]);
  let c = cmp_op_mask_m256d::<{ cmp_op!(LessThanOrdered) }>(a, b).to_bits();
  assert_eq!(c, [u64::MAX, 0, u64::MAX, 0]);
}

#[test]
fn test_convert_to_m256d_from_i32_m128i() {
  let a = m128i::from([4, 5, 6, 7]);
  let b = convert_to_m256d_from_i32_m128i(a).to_array();
  assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn test_convert_to_m256_from_i32_m256i() {
  let a = m256i::from([4, 5, 6, 7, 8, -9, 1, 0]);
  let b = convert_to_m256_from_i32_m256i(a).to_array();
  assert_eq!(b, [4.0, 5.0, 6.0, 7.0, 8.0, -9.0, 1.0, 0.0]);
}

#[test]
fn test_convert_to_i32_m128i_from_m256d() {
  let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
  let b: [i32; 4] = convert_to_i32_m128i_from_m256d(a).into();
  assert_eq!(b, [4, 5, 6, 7]);
}

#[test]
fn test_convert_to_m128_from_m256d() {
  let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
  let b = convert_to_m128_from_m256d(a).to_array();
  assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn test_convert_to_i32_m256i_from_m256() {
  let a = m256::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
  let b: [i32; 8] = convert_to_i32_m256i_from_m256(a).into();
  assert_eq!(b, [4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn test_convert_to_m256d_from_m128() {
  let a = m128::from([4.0, 5.0, 6.0, 7.0]);
  let b = convert_to_m256d_from_m128(a).to_array();
  assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn test_convert_to_f64_from_m256d_s() {
  let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
  let b = convert_to_f64_from_m256d_s(a);
  assert_eq!(b, 4.0);
}

#[test]
fn test_convert_to_i32_from_m256i_s() {
  let a = m256i::from([4, 5, 6, 7, 8, 9, 10, 11]);
  let b = convert_to_i32_from_m256i_s(a);
  assert_eq!(b, 4);
}

#[test]
fn test_convert_to_f32_from_m256_s() {
  let a = m256::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
  let b = convert_to_f32_from_m256_s(a);
  assert_eq!(b, 4.0);
}

#[test]
fn test_div_m256d() {
  let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
  let b = m256d::from([2.0, 2.0, 3.0, 7.0]);
  let c = div_m256d(a, b).to_array();
  assert_eq!(c, [2.0, 2.5, 2.0, 1.0]);
}

#[test]
fn test_div_m256() {
  let a = m256::from_array([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
  let b = m256::from_array([2.0, 2.0, 3.0, 7.0, 2.0, 3.0, 4.0, 11.0]);
  let c = div_m256(a, b).to_array();
  assert_eq!(c, [2.0, 2.5, 2.0, 1.0, 4.0, 3.0, 2.5, 1.0]);
}

#[test]
fn test_dot_product_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  let c = dot_product_m256::<0b1111_1111>(a, b).to_array();
  assert_eq!(c, [110.0, 110.0, 110.0, 110.0, 382.0, 382.0, 382.0, 382.0]);
}

#[test]
fn test_extract_i32_from_m256i() {
  let a = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
  assert_eq!(extract_i32_from_m256i::<3>(a), 12);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_extract_i64_from_m256i() {
  let a = m256i::from([9_i64, 10, 11, 12]);
  assert_eq!(extract_i64_from_m256i::<1>(a), 10_i64);
}

#[test]
fn test_extract_m128d_from_m256d() {
  let a = m256d::from([13.0, 14.0, 15.0, 16.0]);
  let b = m128d::from([15.0, 16.0]).to_array();
  let c = extract_m128d_from_m256d::<1>(a).to_array();
  assert_eq!(b, c);
}

#[test]
fn test_extract_m128_from_m256() {
  let a = m256::from([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  let b = m128::from([13.0, 14.0, 15.0, 16.0]).to_array();
  let c = extract_m128_from_m256::<1>(a).to_array();
  assert_eq!(b, c);
}

#[test]
fn test_extract_m128i_from_m256i() {
  let a = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
  let b: [i32; 4] = m128i::from([13, 14, 15, 16]).into();
  let c: [i32; 4] = extract_m128i_from_m256i::<1>(a).into();
  assert_eq!(b, c);
}

#[test]
fn test_floor_m256d() {
  let a = m256d::from([1.1, 2.5, 3.8, 5.0]);
  let b = floor_m256d(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 5.0]);
}

#[test]
fn test_floor_m256() {
  let a = m256::from([1.1, 2.5, 3.8, 5.0, -0.5, -1.1, -2.7, -3.0]);
  let b = floor_m256(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 5.0, -1.0, -2.0, -3.0, -3.0]);
}

#[test]
fn test_add_horizontal_m256d() {
  let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from([1.0, 3.0, 5.0, 7.0]);
  let c = add_horizontal_m256d(a, b).to_array();
  assert_eq!(c, [3.0, 4.0, 7.0, 12.0]);
}

#[test]
fn test_add_horizontal_m256() {
  let a = m256::from([8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
  let b = m256::from([0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0]);
  let c = add_horizontal_m256(a, b).to_array();
  assert_eq!(c, [15.0, 11.0, 2.0, 12.0, 7.0, 3.0, 48.0, 192.0]);
}

#[test]
fn test_sub_horizontal_m256d() {
  let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from([1.0, 3.0, 5.0, 70.0]);
  let c = sub_horizontal_m256d(a, b).to_array();
  assert_eq!(c, [-1.0, -2.0, -1.0, -65.0]);
}

#[test]
fn test_sub_horizontal_m256() {
  let a = m256::from([8.0, 17.0, 6.0, 5.0, 4.0, 23.0, 2.0, 1.0]);
  let b = m256::from([0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0]);
  let c = sub_horizontal_m256(a, b).to_array();
  assert_eq!(c, [-9.0, 1.0, -2.0, -4.0, -19.0, 1.0, -16.0, -64.0]);
}

#[test]
fn test_insert_i8_to_m256i() {
  let a = m256i::from([0_i8; 32]);
  let b: [i8; 32] = insert_i8_to_m256i::<3>(a, 5).into();
  let c: [i8; 32] = m256i::from([0_i8, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).into();
  assert_eq!(b, c);
}

#[test]
fn test_insert_i16_to_m256i() {
  let a = m256i::from([0_i16; 16]);
  let b: [i16; 16] = insert_i16_to_m256i::<3>(a, 5).into();
  let c: [i16; 16] = m256i::from([0_i16, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).into();
  assert_eq!(b, c);
}

#[test]
fn test_insert_i32_to_m256i() {
  let a = m256i::from([0_i32; 8]);
  let b: [i32; 8] = insert_i32_to_m256i::<3>(a, 5).into();
  let c: [i32; 8] = m256i::from([0, 0, 0, 5, 0, 0, 0, 0]).into();
  assert_eq!(b, c);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_insert_i64_to_m256i() {
  let a = m256i::from([0_i64; 4]);
  let b: [i64; 4] = insert_i64_to_m256i::<3>(a, 5).into();
  let c: [i64; 4] = m256i::from([0, 0, 0, 5_i64]).into();
  assert_eq!(b, c);
}

#[test]
fn test_insert_m128d_to_m256d() {
  let a = m256d::from([0.0; 4]);
  let b: [f64; 4] = insert_m128d_to_m256d::<1>(a, m128d::from([3.0, 4.0])).to_array();
  assert_eq!(b, [0.0, 0.0, 3.0, 4.0]);
}

#[test]
fn test_insert_m128_to_m256() {
  let a = m256::from([0.0; 8]);
  let b: [f32; 8] = insert_m128_to_m256::<1>(a, m128::from([1.0, 2.0, 3.0, 4.0])).to_array();
  assert_eq!(b, [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_insert_m128i_to_m256i_slow_avx() {
  let a = m256i::from([0_i32; 8]);
  let b: [i32; 8] = insert_m128i_to_m256i_slow_avx::<1>(a, m128i::from([1, 2, 3, 4])).into();
  assert_eq!(b, [0, 0, 0, 0, 1, 2, 3, 4]);
}

#[test]
fn test_load_m256d() {
  let a = m256d::from([8.0, 17.0, 6.0, 5.0]);
  let b = load_m256d(&a);
  assert_eq!(a.to_array(), b.to_array());
}

#[test]
fn test_load_m256() {
  let a = m256::from([8.0, 17.0, 6.0, 5.0, 4.0, 23.0, 2.0, 1.0]);
  let b = load_m256(&a);
  assert_eq!(a.to_array(), b.to_array());
}

#[test]
fn test_load_m256i() {
  let a = m256i::from([8, 17, 6, 5, 4, 23, 2, 1]);
  let b = load_m256i(&a);
  assert_eq!(<[i32; 8]>::from(a), <[i32; 8]>::from(b));
}

#[test]
fn test_load_unaligned_m256d() {
  assert_eq!(load_unaligned_m256d(&[8.0, 17.0, 6.0, 5.0]).to_array(), [8.0, 17.0, 6.0, 5.0]);
}

#[test]
fn test_load_unaligned_m256() {
  assert_eq!(load_unaligned_m256(&[8.0, 17.0, 6.0, 5.0, 1.0, 2.0, 3.0, 4.0]).to_array(), [8.0, 17.0, 6.0, 5.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_load_unaligned_m256i() {
  assert_eq!(<[i8; 32]>::from(load_unaligned_m256i(&[7_i8; 32])), [7_i8; 32]);
}

#[test]
fn test_load_unaligned_hi_lo_m256d() {
  assert_eq!(load_unaligned_hi_lo_m256d(&[3.0, 4.0], &[1.0, 2.0]).to_array(), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_load_unaligned_hi_lo_m256() {
  assert_eq!(load_unaligned_hi_lo_m256(&[5.0, 6.0, 7.0, 8.0], &[1.0, 2.0, 3.0, 4.0]).to_array(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_load_unaligned_hi_lo_m256i() {
  assert_eq!(<[i8; 32]>::from(load_unaligned_hi_lo_m256i(&[7_i8; 16], &[9_i8; 16])), [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,]);
}

#[test]
fn test_load_masked_m128d() {
  let a = m128d::from([8.0, 17.0]);
  let b = load_masked_m128d(&a, m128i::from([0_i64, -1])).to_array();
  assert_eq!(b, [0.0, 17.0]);
}

#[test]
fn test_load_masked_m256d() {
  let a = m256d::from([8.0, 17.0, 16.0, 20.0]);
  let b = load_masked_m256d(&a, m256i::from([0_i64, -1, -1, 0])).to_array();
  assert_eq!(b, [0.0, 17.0, 16.0, 0.0]);
}

#[test]
fn test_load_masked_m128() {
  let a = m128::from([8.0, 17.0, 16.0, 12.0]);
  let b = load_masked_m128(&a, m128i::from([0, -1, -1, 0])).to_array();
  assert_eq!(b, [0.0, 17.0, 16.0, 0.0]);
}

#[test]
fn test_load_masked_m256() {
  let a = m256::from([8.0, 17.0, 16.0, 20.0, 80.0, 1.0, 2.0, 3.0]);
  let b = load_masked_m256(&a, m256i::from([0, -1, -1, 0, -1, -1, 0, 0])).to_array();
  assert_eq!(b, [0.0, 17.0, 16.0, 0.0, 80.0, 1.0, 0.0, 0.0]);
}

#[test]
fn test_store_masked_m128d() {
  let mut a = m128d::default();
  store_masked_m128d(&mut a, m128i::from([0_i64, -1]), m128d::from([8.0, 17.0]));
  assert_eq!(a.to_array(), [0.0, 17.0]);
}

#[test]
fn test_store_masked_m256d() {
  let mut a = m256d::default();
  store_masked_m256d(&mut a, m256i::from([0_i64, -1, -1, 0]), m256d::from([8.0, 17.0, 16.0, 20.0]));
  assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0]);
}

#[test]
fn test_store_masked_m128() {
  let mut a = m128::default();
  store_masked_m128(&mut a, m128i::from([0, -1, -1, 0]), m128::from([8.0, 17.0, 16.0, 20.0]));
  assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0]);
}

#[test]
fn test_store_masked_m256() {
  let mut a = m256::default();
  store_masked_m256(&mut a, m256i::from([0, -1, -1, 0, -1, -1, 0, 0]), m256::from([8.0, 17.0, 16.0, 20.0, 80.0, 1.0, 2.0, 3.0]));
  assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0, 80.0, 1.0, 0.0, 0.0]);
}

#[test]
fn test_max_m256d() {
  let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
  let b = m256d::from_array([5.0, 6.0, -0.5, 2.2]);
  let c = max_m256d(a, b).to_array();
  assert_eq!(c, [5.0, 12.0, -0.5, 3.0]);
}

#[test]
fn test_max_m256() {
  let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 10.0, 0.0, 1.0, 2.0]);
  let b = m256::from_array([5.0, 6.0, -0.5, 2.2, 5.0, 6.0, 7.0, 8.0]);
  let c = max_m256(a, b).to_array();
  assert_eq!(c, [5.0, 12.0, -0.5, 3.0, 10.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_min_m256d() {
  let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
  let b = m256d::from_array([5.0, 6.0, -0.5, 2.2]);
  let c = min_m256d(a, b).to_array();
  assert_eq!(c, [1.0, 6.0, -1.0, 2.2]);
}

#[test]
fn test_min_m256() {
  let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 10.0, 0.0, 1.0, 2.0]);
  let b = m256::from_array([5.0, 6.0, -0.5, 2.2, 5.0, 6.0, 7.0, 8.0]);
  let c = min_m256(a, b).to_array();
  assert_eq!(c, [1.0, 6.0, -1.0, 2.2, 5.0, 0.0, 1.0, 2.0]);
}

#[test]
fn test_duplicate_odd_lanes_m256d() {
  let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
  let c = duplicate_odd_lanes_m256d(a).to_array();
  assert_eq!(c, [1.0, 1.0, -1.0, -1.0]);
}

#[test]
fn test_duplicate_even_lanes_m256() {
  let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 0.0, 7.0, 2.0, 50.0]);
  let c = duplicate_even_lanes_m256(a).to_array();
  assert_eq!(c, [12.0, 12.0, 3.0, 3.0, 7.0, 7.0, 50.0, 50.0]);
}

#[test]
fn test_duplicate_odd_lanes_m256() {
  let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 0.0, 7.0, 2.0, 50.0]);
  let c = duplicate_odd_lanes_m256(a).to_array();
  assert_eq!(c, [1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 2.0, 2.0]);
}

#[test]
fn test_move_mask_m256d() {
  assert_eq!(0b0100, move_mask_m256d(m256d::from([1.0, 12.0, -1.0, 3.0])));
}

#[test]
fn test_move_mask_m256() {
  assert_eq!(0b00110100, move_mask_m256(m256::from([1.0, 12.0, -1.0, 3.0, -1.0, -2.0, 3.0, 4.0])));
}

#[test]
fn test_mul_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([5.0, 6.0, 7.0, 8.5]);
  let c = mul_m256d(a, b).to_array();
  assert_eq!(c, [5.0, 12.0, 21.0, 34.0]);
}

#[test]
fn test_mul_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
  let b = m256::from_array([5.0, 6.0, 7.0, 8.5, 90.0, 100.0, 110.0, 51.0]);
  let c = mul_m256(a, b).to_array();
  assert_eq!(c, [5.0, 12.0, 21.0, 34.0, 1800.0, 3000.0, 4400.0, 2550.0]);
}

#[test]
fn test_bitor_m256d() {
  let a = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
  let b = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
  let c = bitor_m256d(a, b).to_array();
  assert_eq!(c, [1.0, 1.0, 1.0, 0.0]);
}

#[test]
fn test_bitor_m256() {
  let a = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
  let b = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
  let c = bitor_m256(a, b).to_array();
  assert_eq!(c, [1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
}

#[test]
fn test_permute_m128d() {
  let a = m128d::from_array([1.0, 2.0]);
  //
  let b = permute_m128d::<0b_0_1>(a).to_array();
  assert_eq!(b, [2.0, 1.0]);
}

#[test]
fn test_permute_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  //
  let b = permute_m256d::<0b_0_1_0_1>(a).to_array();
  assert_eq!(b, [2.0, 1.0, 4.0, 3.0]);
}

#[test]
fn test_permute_m128() {
  let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
  //
  let b = permute_m128::<0b_00_00_00_00>(a).to_array();
  assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
  //
  let b = permute_m128::<0b_11_00_01_00>(a).to_array();
  assert_eq!(b, [1.0, 2.0, 1.0, 4.0]);
  //
  let b = permute_m128::<0b_10_10_00_00>(a).to_array();
  assert_eq!(b, [1.0, 1.0, 3.0, 3.0]);
}

#[test]
fn test_permute_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  //
  let b = permute_m256::<0b_00_10_01_11>(a).to_array();
  assert_eq!(b, [4.0, 2.0, 3.0, 1.0, 8.0, 6.0, 7.0, 5.0]);
}

#[test]
fn test_permute2z_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
  //
  let c = permute2z_m256d::<0b1000_0010>(a, b).to_array();
  assert_eq!(c, [5.0, 6.0, 0.0, 0.0]);
  //
  let c = permute2z_m256d::<0b0001_1000>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 3.0, 4.0]);
}

#[test]
fn test_permute2z_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  //
  let c = permute2z_m256::<0b1000_0010>(a, b).to_array();
  assert_eq!(c, [9.0, 10.0, 11.0, 12.0, 0.0, 0.0, 0.0, 0.0]);
  //
  let c = permute2z_m256::<0b0001_1000>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 0.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_permute2z_m256i() {
  let a = m256i::from([1, 2, 3, 4, 5, 6, 7, 8]);
  let b = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
  //
  let c: [i32; 8] = permute2z_m256i::<0b1000_0010>(a, b).into();
  assert_eq!(c, [9, 10, 11, 12, 0, 0, 0, 0]);
  //
  let c: [i32; 8] = permute2z_m256i::<0b0001_1000>(a, b).into();
  assert_eq!(c, [0, 0, 0, 0, 5, 6, 7, 8]);
}

#[test]
fn test_shuffle_av_f64_all_m128d() {
  let a = m128d::from_array([2.0, 3.0]);
  let v = m128i::from([1_i64 << 1, 0 << 1]);
  let c = shuffle_av_f64_all_m128d(a, v).to_array();
  assert_eq!(c, [3.0, 2.0]);
}

#[test]
fn test_shuffle_av_f64_half_m256d() {
  let a = m256d::from_array([2.0, 3.0, 7.0, 8.0]);
  let v = m256i::from([1_i64 << 1, 0 << 1, 1 << 1, 1 << 1]);
  let c = shuffle_av_f64_half_m256d(a, v).to_array();
  assert_eq!(c, [3.0, 2.0, 8.0, 8.0]);
}

#[test]
fn test_shuffle_av_f32_all_m128() {
  let a = m128::from_array([5.0, 6.0, 7.0, 8.0]);
  let v = m128i::from([0, 2, 3, 1]);
  let c = shuffle_av_f32_all_m128(a, v).to_array();
  assert_eq!(c, [5.0, 7.0, 8.0, 6.0]);
}

#[test]
fn test_shuffle_av_f32_half_m256() {
  let a = m256::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
  let v = m256i::from([0, 2, 3, 1, 0, 3, 2, 2]);
  let c = shuffle_av_f32_half_m256(a, v).to_array();
  assert_eq!(c, [0.0, 2.0, 3.0, 1.0, 4.0, 7.0, 6.0, 6.0]);
}

#[test]
fn test_reciprocal_m256() {
  let a = m256::from_array([1.0, 2.0, 4.0, 8.0, 0.5, 2.0, 8.0, 16.0]);
  let b = reciprocal_m256(a).to_array();
  let expected = [1.0, 0.5, 0.25, 0.125, 2.0, 0.5, 0.125, 0.0625];
  for i in 0..4 {
    assert!((b[i] - expected[i]).abs() < 0.001);
  }
}

#[test]
fn test_round_m256d() {
  let a = m256d::from_array([-0.1, 1.6, 2.5, 3.1]);
  //
  assert_eq!(round_m256d::<{ round_op!(Nearest) }>(a).to_array(), [0.0, 2.0, 2.0, 3.0]);
  //
  assert_eq!(round_m256d::<{ round_op!(NegInf) }>(a).to_array(), [-1.0, 1.0, 2.0, 3.0]);
  //
  assert_eq!(round_m256d::<{ round_op!(PosInf) }>(a).to_array(), [0.0, 2.0, 3.0, 4.0]);
  //
  assert_eq!(round_m256d::<{ round_op!(Zero) }>(a).to_array(), [0.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_round_m256() {
  let a = m256::from_array([-0.1, 1.6, 3.3, 4.5, 5.1, 6.5, 7.2, 8.0]);
  //
  assert_eq!(round_m256::<{ round_op!(Nearest) }>(a).to_array(), [0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  //
  assert_eq!(round_m256::<{ round_op!(NegInf) }>(a).to_array(), [-1.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  //
  assert_eq!(round_m256::<{ round_op!(PosInf) }>(a).to_array(), [0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0]);
  //
  assert_eq!(round_m256::<{ round_op!(Zero) }>(a).to_array(), [0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_reciprocal_sqrt_m256() {
  let a = m256::from_array([16.0, 9.0, 4.0, 25.0, 16.0, 9.0, 4.0, 25.0]);
  let b = reciprocal_sqrt_m256(a).to_array();
  let expected = [0.25, 0.33333, 0.5, 0.2, 0.25, 0.33333, 0.5, 0.2];
  for i in 0..8 {
    assert!((b[i] - expected[i]).abs() < 0.001);
  }
}

#[test]
fn test_set_i8_m256i() {
  let a: [i8; 32] = set_i8_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31).into();
  assert_eq!(a, [31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_set_i16_m256i() {
  let a: [i16; 16] = set_i16_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15).into();
  assert_eq!(a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_set_i32_m256i() {
  let a: [i32; 8] = set_i32_m256i(0, 1, 2, 3, 4, 5, 6, 7).into();
  assert_eq!(a, [7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_set_i64_m256i() {
  let a: [i64; 4] = set_i64_m256i(0, 1, 2, 3).into();
  assert_eq!(a, [3, 2, 1, 0]);
}

#[test]
fn test_set_m128_m256() {
  let a = set_m128_m256(m128::from([4.0, 5.0, 6.0, 7.0]), m128::from([0.0, 1.0, 2.0, 3.0])).to_array();
  assert_eq!(a, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn test_set_m128d_m256d() {
  let a = set_m128d_m256d(m128d::from([2.0, 3.0]), m128d::from([0.0, 1.0])).to_array();
  assert_eq!(a, [0.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_set_m128i_m256i() {
  let a: [i64; 4] = set_m128i_m256i(set_i64_m128i(3_i64, 2), set_i64_m128i(1_i64, 0)).into();
  assert_eq!(a, [0_i64, 1, 2, 3]);
}

#[test]
fn test_set_m256d() {
  let a = set_m256d(0.0, 1.0, 2.0, 3.0).to_array();
  assert_eq!(a, [3.0, 2.0, 1.0, 0.0]);
}

#[test]
fn test_set_m256() {
  let a = set_m256(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0).to_array();
  assert_eq!(a, [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
}

#[test]
fn test_set_splat_i8_m256i() {
  let a: [i8; 32] = set_splat_i8_m256i(56).into();
  assert_eq!(a, [56_i8; 32]);
}

#[test]
fn test_set_splat_i16_m256i() {
  let a: [i16; 16] = set_splat_i16_m256i(56).into();
  assert_eq!(a, [56_i16; 16]);
}

#[test]
fn test_set_splat_i32_m256i() {
  let a: [i32; 8] = set_splat_i32_m256i(56).into();
  assert_eq!(a, [56_i32; 8]);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_set_splat_i64_m256i() {
  let a: [i64; 4] = set_splat_i64_m256i(56).into();
  assert_eq!(a, [56_i64; 4]);
}

#[test]
fn test_set_splat_m256d() {
  let a = set_splat_m256d(56.0).to_array();
  assert_eq!(a, [56.0; 4]);
}

#[test]
fn test_set_splat_m256() {
  let a = set_splat_m256(56.0).to_array();
  assert_eq!(a, [56.0; 8]);
}

#[test]
fn test_set_reversed_i8_m256i() {
  let a: [i8; 32] = set_reversed_i8_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31).into();
  assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
}

#[test]
fn test_set_reversed_i16_m256i() {
  let a: [i16; 16] = set_reversed_i16_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15).into();
  assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
}

#[test]
fn test_set_reversed_i32_m256i() {
  let a: [i32; 8] = set_reversed_i32_m256i(0, 1, 2, 3, 4, 5, 6, 7).into();
  assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_set_reversed_i64_m256i() {
  let a: [i64; 4] = set_reversed_i64_m256i(0, 1, 2, 3).into();
  assert_eq!(a, [0, 1, 2, 3]);
}

#[test]
fn test_set_reversed_m128_m256() {
  let a = set_reversed_m128_m256(set_reversed_m128(7.0, 6.0, 5.0, 4.0), set_reversed_m128(3.0, 2.0, 1.0, 0.0)).to_array();
  assert_eq!(a, [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
}

#[test]
fn test_set_reversed_m128d_m256d() {
  let a = set_reversed_m128d_m256d(set_reversed_m128d(3.0, 2.0), set_reversed_m128d(1.0, 0.0)).to_array();
  assert_eq!(a, [3.0, 2.0, 1.0, 0.0]);
}

#[test]
fn test_set_reversed_m128i_m256i() {
  let a: [i64; 4] = set_reversed_m128i_m256i(m128i::from([0_i64, 1]), m128i::from([2_i64, 3])).into();
  assert_eq!(a, [0_i64, 1, 2, 3]);
}

#[test]
fn test_set_reversed_m256d() {
  let a = set_reversed_m256d(0.0, 1.0, 2.0, 3.0).to_array();
  assert_eq!(a, [0.0, 1.0, 2.0, 3.0]);
}

#[test]
fn test_set_reversed_m256() {
  let a = set_reversed_m256(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0).to_array();
  assert_eq!(a, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn test_zeroed_m256d() {
  let a = zeroed_m256d().to_array();
  assert_eq!(a, [0.0; 4]);
}

#[test]
fn test_zeroed_m256() {
  let a = zeroed_m256().to_array();
  assert_eq!(a, [0.0; 8]);
}

#[test]
fn test_zeroed_m256i() {
  let a: [i32; 8] = zeroed_m256i().into();
  assert_eq!(a, [0; 8]);
}

#[test]
fn test_shuffle_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
  //
  let c = shuffle_m256d::<0b_0_0_0_0>(a, b).to_array();
  assert_eq!(c, [1.0, 5.0, 3.0, 7.0]);
  //
  let c = shuffle_m256d::<0b_0_0_0_1>(a, b).to_array();
  assert_eq!(c, [2.0, 5.0, 3.0, 7.0]);
  //
  let c = shuffle_m256d::<0b_0_0_1_0>(a, b).to_array();
  assert_eq!(c, [1.0, 6.0, 3.0, 7.0]);
  //
  let c = shuffle_m256d::<0b_0_0_1_1>(a, b).to_array();
  assert_eq!(c, [2.0, 6.0, 3.0, 7.0]);
  //
  let c = shuffle_m256d::<0b_1_0_0_1>(a, b).to_array();
  assert_eq!(c, [2.0, 5.0, 3.0, 8.0]);
  //
  let c = shuffle_m256d::<0b_0_1_0_1>(a, b).to_array();
  assert_eq!(c, [2.0, 5.0, 4.0, 7.0]);
  //
  let c = shuffle_m256d::<0b_1_1_1_1>(a, b).to_array();
  assert_eq!(c, [2.0, 6.0, 4.0, 8.0]);
}

#[test]
fn test_shuffle_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  //
  let c = shuffle_m256::<0b_00_10_11_01>(a, b).to_array();
  assert_eq!(c, [2.0, 4.0, 11.0, 9.0, 6.0, 8.0, 15.0, 13.0]);
}

#[test]
fn test_sqrt_m256d() {
  let a = m256d::from_array([1.0, 4.0, 9.0, 16.0]);
  let b = sqrt_m256d(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_sqrt_m256() {
  let a = m256::from_array([1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 0.0, 49.0]);
  let b = sqrt_m256(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 7.0]);
}

#[test]
fn test_store_m256d() {
  let mut addr = m256d::from([0.0; 4]);
  store_m256d(&mut addr, m256d::from([1.0, 2.0, 3.0, 4.0]));
  assert_eq!(addr.to_array(), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_store_m256() {
  let mut addr = m256::from([0.0; 8]);
  store_m256(&mut addr, m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
  assert_eq!(addr.to_array(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_store_m256i() {
  let mut addr = m256i::from([0_i32; 8]);
  store_m256i(&mut addr, m256i::from([1, 2, 3, 4, 5, 6, 7, 8]));
  assert_eq!(<[i32; 8]>::from(addr), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_store_unaligned_m256d() {
  let mut addr = [0.0; 4];
  store_unaligned_m256d(&mut addr, m256d::from([1.0, 2.0, 3.0, 4.0]));
  assert_eq!(addr, [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_store_unaligned_m256() {
  let mut addr = [0.0; 8];
  store_unaligned_m256(&mut addr, m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
  assert_eq!(addr, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_store_unaligned_m256i() {
  let mut addr = [0_i8; 32];
  store_unaligned_m256i(&mut addr, m256i::from([12_i8; 32]));
  assert_eq!(addr, [12_i8; 32]);
}

#[test]
fn test_store_unaligned_hi_lo_m256d() {
  let mut hi_addr = [0.0; 2];
  let mut lo_addr = [0.0; 2];
  store_unaligned_hi_lo_m256d(&mut hi_addr, &mut lo_addr, m256d::from([1.0, 2.0, 3.0, 4.0]));
  assert_eq!(hi_addr, [3.0, 4.0]);
  assert_eq!(lo_addr, [1.0, 2.0]);
}

#[test]
fn test_store_unaligned_hi_lo_m256() {
  let mut hi_addr = [0.0; 4];
  let mut lo_addr = [0.0; 4];
  store_unaligned_hi_lo_m256(&mut hi_addr, &mut lo_addr, m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
  assert_eq!(hi_addr, [5.0, 6.0, 7.0, 8.0]);
  assert_eq!(lo_addr, [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_store_unaligned_hi_lo_m256i() {
  let mut hi_addr = [0_i8; 16];
  let mut lo_addr = [0_i8; 16];
  store_unaligned_hi_lo_m256i(&mut hi_addr, &mut lo_addr, m256i::from([56_i8; 32]));
  assert_eq!(hi_addr, [56_i8; 16]);
  assert_eq!(lo_addr, [56_i8; 16]);
}

#[test]
fn test_sub_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([5.0, 60.0, 712.0, 8.5]);
  let c = sub_m256d(a, b).to_array();
  assert_eq!(c, [-4.0, -58.0, -709.0, -4.5]);
}

#[test]
fn test_sub_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
  let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
  let c = sub_m256(a, b).to_array();
  assert_eq!(c, [-58.0, -59.0, -76.0, -77.5, -70.0, -70.0, -70.0, -1.0]);
}

#[test]
fn test_unpack_hi_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([59.0, 61.0, 79.0, 81.5]);
  let c = unpack_hi_m256d(a, b).to_array();
  assert_eq!(c, [2.0, 61.0, 4.0, 81.5]);
}

#[test]
fn test_unpack_hi_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
  let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
  let c = unpack_hi_m256(a, b).to_array();
  assert_eq!(c, [3.0, 79.0, 4.0, 81.5, 40.0, 110.0, 50.0, 51.0]);
}

#[test]
fn test_unpack_lo_m256d() {
  let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let b = m256d::from_array([59.0, 61.0, 79.0, 81.5]);
  let c = unpack_lo_m256d(a, b).to_array();
  assert_eq!(c, [1.0, 59.0, 3.0, 79.0]);
}

#[test]
fn test_unpack_lo_m256() {
  let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
  let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
  let c = unpack_lo_m256(a, b).to_array();
  assert_eq!(c, [1.0, 59.0, 2.0, 61.0, 20.0, 90.0, 30.0, 100.0]);
}

#[test]
fn test_bitxor_m256d() {
  let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
  let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
  let c = bitxor_m256d(a, b).to_array();
  assert_eq!(c, [0.0, 1.0, 1.0, 0.0]);
}

#[test]
fn test_bitxor_m256() {
  let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
  let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
  let c = bitxor_m256(a, b).to_array();
  assert_eq!(c, [0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0]);
}

#[test]
fn test_zero_extend_m128d() {
  let a = zero_extend_m128d(m128d::from_array([1.0, 2.0])).to_array();
  assert_eq!(a, [1.0, 2.0, 0.0, 0.0]);
}

#[test]
fn test_zero_extend_m128() {
  let a = zero_extend_m128(m128::from_array([1.0, 2.0, 3.0, 4.0])).to_array();
  assert_eq!(a, [1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn test_zero_extend_m128i() {
  let a: [i32; 8] = zero_extend_m128i(m128i::from([1, 2, 3, 4])).into();
  assert_eq!(a, [1, 2, 3, 4, 0, 0, 0, 0]);
}

#[test]
fn test_ptest_m256() {
  let a = m256::from([-1.0, 5.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0]);
  let b = m256::from([0.0, -1.0, 0.0, -1.0, 0.0, -1.0, 0.0, -1.0]);

  assert_eq!(testz_m256(a, a), 0);
  assert_eq!(testz_m256(a, b), 1);
  assert_eq!(testz_m256(b, b), 0);

  assert_eq!(testc_m256(a, a), 1);
  assert_eq!(testc_m256(a, b), 0);
  assert_eq!(testc_m256(b, b), 1);
}

#[test]
fn test_ptest_m128() {
  let a = m128::from([-1.0, 5.0, -1.0, 0.0]);
  let b = m128::from([0.0, -1.0, 0.0, -1.0]);

  assert_eq!(testz_m128(a, a), 0);
  assert_eq!(testz_m128(a, b), 1);
  assert_eq!(testz_m128(b, b), 0);

  assert_eq!(testc_m128(a, a), 1);
  assert_eq!(testc_m128(a, b), 0);
  assert_eq!(testc_m128(b, b), 1);
}

#[test]
fn test_ptest_m256d() {
  let a = m256d::from([-1.0, 5.0, -1.0, 0.0]);
  let b = m256d::from([0.0, -1.0, 0.0, -1.0]);

  assert_eq!(testz_m256d(a, a), 0);
  assert_eq!(testz_m256d(a, b), 1);
  assert_eq!(testz_m256d(b, b), 0);

  assert_eq!(testc_m256d(a, a), 1);
  assert_eq!(testc_m256d(a, b), 0);
  assert_eq!(testc_m256d(b, b), 1);
}

#[test]
fn test_ptest_m128d() {
  let a = m128d::from([-1.0, 5.0]);
  let b = m128d::from([0.0, -1.0]);

  assert_eq!(testz_m128d(a, a), 0);
  assert_eq!(testz_m128d(a, b), 1);
  assert_eq!(testz_m128d(b, b), 0);

  assert_eq!(testc_m128d(a, a), 1);
  assert_eq!(testc_m128d(a, b), 0);
  assert_eq!(testc_m128d(b, b), 1);
}

#[test]
fn test_ptest_i256() {
  let a = m256i::from([1, 0, 1, 0, 1, 0, 1, 0]);
  let b = m256i::from([0, 1, 0, 1, 0, 1, 0, 1]);

  assert_eq!(testz_m256i(a, a), 0);
  assert_eq!(testz_m256i(a, b), 1);
  assert_eq!(testz_m256i(b, b), 0);

  assert_eq!(testc_m256i(a, a), 1);
  assert_eq!(testc_m256i(a, b), 0);
  assert_eq!(testc_m256i(b, b), 1);
}
