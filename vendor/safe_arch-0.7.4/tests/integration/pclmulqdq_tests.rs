use super::*;

#[test]
fn test_mul_i64_carryless_m128i() {
  let x = m128i::from([2_i64, 3]);
  let y = m128i::from([4_i64, 500]);
  //
  let c: [i64; 2] = mul_i64_carryless_m128i::<{ 0 | (0 << 4) }>(x, y).into();
  assert_eq!(c, [8_i64, 0]);
  let c: [i64; 2] = mul_i64_carryless_m128i::<{ 1 | (0 << 4) }>(x, y).into();
  assert_eq!(c, [12_i64, 0]);
  let c: [i64; 2] = mul_i64_carryless_m128i::<{ 0 | (1 << 4) }>(x, y).into();
  assert_eq!(c, [1000_i64, 0]);
  let c: [i64; 2] = mul_i64_carryless_m128i::<{ 1 | (1 << 4) }>(x, y).into();
  assert_eq!(c, [540_i64, 0]); // not 1500 like a normal mul would be!
}
