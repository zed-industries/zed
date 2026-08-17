use super::*;

#[test]
fn test_bit_zero_high_index_u32() {
  assert_eq!(bit_zero_high_index_u32(0b1111, 0), 0b0000);
  assert_eq!(bit_zero_high_index_u32(0b1111, 1), 0b0001);
  assert_eq!(bit_zero_high_index_u32(0b1111, 2), 0b0011);
  assert_eq!(bit_zero_high_index_u32(0b1111, 3), 0b0111);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_zero_high_index_u64() {
  assert_eq!(bit_zero_high_index_u64(0b1111, 0), 0b0000);
  assert_eq!(bit_zero_high_index_u64(0b1111, 1), 0b0001);
  assert_eq!(bit_zero_high_index_u64(0b1111, 2), 0b0011);
  assert_eq!(bit_zero_high_index_u64(0b1111, 3), 0b0111);
}

#[test]
fn test_mul_extended_u32() {
  let mut x = 0_u32;
  assert_eq!(mul_extended_u32(u32::MAX, 17, &mut x), 4294967279);
  assert_eq!(x, 16);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_mul_extended_u64() {
  let mut x = 0_u64;
  assert_eq!(mul_extended_u64(u64::MAX, 17, &mut x), 18446744073709551599);
  assert_eq!(x, 16);
}

#[test]
fn test_population_deposit_u32() {
  assert_eq!(population_deposit_u32(0b1001, 0b1111), 0b1001);
  assert_eq!(population_deposit_u32(0b1001, 0b1110), 0b0010);
  assert_eq!(population_deposit_u32(0b1001, 0b1100), 0b0100);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_population_deposit_u64() {
  assert_eq!(population_deposit_u64(0b1001, 0b1111), 0b1001);
  assert_eq!(population_deposit_u64(0b1001, 0b1110), 0b0010);
  assert_eq!(population_deposit_u64(0b1001, 0b1100), 0b0100);
}

#[test]
fn test_population_extract_u32() {
  assert_eq!(population_extract_u32(0b1001, 0b1111), 0b1001);
  assert_eq!(population_extract_u32(0b1001, 0b1110), 0b0100);
  assert_eq!(population_extract_u32(0b1001, 0b1100), 0b0010);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_population_extract_u64() {
  assert_eq!(population_extract_u64(0b1001, 0b1111), 0b1001);
  assert_eq!(population_extract_u64(0b1001, 0b1110), 0b0100);
  assert_eq!(population_extract_u64(0b1001, 0b1100), 0b0010);
}
