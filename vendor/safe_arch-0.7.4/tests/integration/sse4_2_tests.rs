use super::*;

#[test]
fn test_cmp_gt_mask_i64_m128i() {
  let a = m128i::from([1_i64, 3]);
  let b = m128i::from([0_i64, 3]);
  let c: [i64; 2] = cmp_gt_mask_i64_m128i(a, b).into();
  assert_eq!(c, [-1_i64, 0]);
}

#[test]
fn test_crc32_u8() {
  assert_eq!(crc32_u8(u32::MAX, u8::MAX), 16777215_u32);
}

#[test]
fn test_crc32_u16() {
  assert_eq!(crc32_u16(u32::MAX, u16::MAX), 65535_u32);
}

#[test]
fn test_crc32_u32() {
  assert_eq!(crc32_u32(u32::MAX, u32::MAX), 0_u32);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_crc32_u64() {
  assert_eq!(crc32_u64(u64::MAX, u64::MAX), 3080238136_u64);
}

#[test]
#[rustfmt::skip]
#[cfg(target_arch = "x86_64")]
fn test_search_implicit_str_for_index() {

  // Eq Any
  let needle: m128i = m128i::from(*b"e_______________");
  let hay: m128i = m128i::from(*b"some test words.");

  assert_eq!(3, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_FIRST_MATCH}>(needle, hay));
  assert_eq!(6, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_LAST_MATCH}>(needle, hay));

  assert_eq!(3, search_explicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_FIRST_MATCH}>(needle, 1, hay, 16));
  assert_eq!(6, search_explicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_LAST_MATCH}>(needle, 1, hay, 16));

  // more than one needle character will match any of them, though we
  // don't get info about _which_ needle character matched.
  let needle: m128i = m128i::from(*b"et\0_____________");
  assert_eq!(3, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_FIRST_MATCH}>(needle, hay));
  assert_eq!(8, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_LAST_MATCH}>(needle, hay));

  // Cmp Ranges
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"vz\0_____________");
  assert_eq!(10, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_RANGES|STR_CMP_FIRST_MATCH}>(needle, hay));

  // Cmp Eq Each
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"_____test_______");
  assert_eq!(5, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_EACH|STR_CMP_FIRST_MATCH}>(needle, hay));
  assert_eq!(8, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_EACH|STR_CMP_LAST_MATCH}>(needle, hay));

  // Cmp Eq Ordered
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"words\0__________");
  assert_eq!(10, search_implicit_str_for_index::<{STR_CMP_U8|STR_CMP_EQ_ORDERED|STR_CMP_FIRST_MATCH}>(needle, hay));

}

#[test]
#[rustfmt::skip]
#[cfg(target_arch = "x86_64")]
fn test_search_implicit_str_for_mask() {
  // EqAny
  let hay: m128i = m128i::from(*b"some test words.");

  // explicit needle length
  let needle: m128i = m128i::from(*b"e_______________");
  let i: u128 = search_explicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_BIT_MASK}>(needle, 1, hay, 16).into();
  assert_eq!(i, 0b0000000001001000);
  let i: [i8; 16] = search_explicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_UNIT_MASK}>(needle, 1, hay, 16).into();
  assert_eq!(i, [0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

  // implicit needle length
  let needle: m128i = m128i::from(*b"e\0______________");
  let i: u128 = search_implicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_BIT_MASK}>(needle, hay).into();
  assert_eq!(i, 0b0000000001001000);

  // more than one needle character will match any of them, though we
  // don't get info about _which_ needle character matched.
  let needle: m128i = m128i::from(*b"et\0_____________");
  let i: u128 = search_implicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_ANY|STR_CMP_BIT_MASK}>(needle, hay).into();
  assert_eq!(i, 0b0000000101101000);

  // Cmp Ranges
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"am\0_____________");
  let i: u128 = search_implicit_str_for_mask::<{STR_CMP_U8|STR_CMP_RANGES|STR_CMP_BIT_MASK}>(needle, hay).into();
  assert_eq!(i, 0b0010000001001100);

  //Cmp Eq Each
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"_____test_______");
  let i: u128 = search_implicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_EACH|STR_CMP_BIT_MASK}>(needle, hay).into();
  assert_eq!(i, 0b0000000111100000);

  // Cmp Eq Ordered
  let hay: m128i = m128i::from(*b"some test words.");
  let needle: m128i = m128i::from(*b"words\0__________");
  let i: u128 = search_implicit_str_for_mask::<{STR_CMP_U8|STR_CMP_EQ_ORDERED|STR_CMP_BIT_MASK}>(needle, hay).into();
  assert_eq!(i, 0b00000010000000000); // one bit at the start of the match
}
