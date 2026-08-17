use super::*;

#[test]
fn test_bitandnot_u32() {
  let a = [1, 0, 1, 0];
  let b = [1, 1, 0, 0];
  let mut c = [0_u32; 4];
  for i in 0..4 {
    c[i] = bitandnot_u32(a[i], b[i]);
  }
  assert_eq!(c, [0, 1, 0, 0]);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bitandnot_u64() {
  let a = [1_u64, 0, 1, 0];
  let b = [1_u64, 1, 0, 0];
  let mut c = [0_u64; 4];
  for i in 0..4 {
    c[i] = bitandnot_u64(a[i], b[i]);
  }
  assert_eq!(c, [0_u64, 1, 0, 0]);
}

#[test]
fn test_bit_extract_u32() {
  assert_eq!(bit_extract_u32(0b0110, 0, 3), 0b110);
  assert_eq!(bit_extract_u32(0b0110, 0, 2), 0b10);
  assert_eq!(bit_extract_u32(0b0110, 1, 2), 0b11);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_extract_u64() {
  assert_eq!(bit_extract_u64(0b0110, 0, 3), 0b110);
  assert_eq!(bit_extract_u64(0b0110, 0, 2), 0b10);
  assert_eq!(bit_extract_u64(0b0110, 1, 2), 0b11);
}

#[test]
fn test_bit_extract2_u32() {
  assert_eq!(bit_extract2_u32(0b0110, (3 << 8) | 0), 0b110);
  assert_eq!(bit_extract2_u32(0b0110, (2 << 8) | 0), 0b10);
  assert_eq!(bit_extract2_u32(0b0110, (2 << 8) | 1), 0b11);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_extract2_u64() {
  assert_eq!(bit_extract2_u64(0b0110, (3 << 8) | 0), 0b110);
  assert_eq!(bit_extract2_u64(0b0110, (2 << 8) | 0), 0b10);
  assert_eq!(bit_extract2_u64(0b0110, (2 << 8) | 1), 0b11);
}

#[test]
fn test_bit_lowest_set_value_u32() {
  assert_eq!(bit_lowest_set_value_u32(0b0), 0);
  assert_eq!(bit_lowest_set_value_u32(0b1), 1);
  assert_eq!(bit_lowest_set_value_u32(0b10), 2);
  assert_eq!(bit_lowest_set_value_u32(0b100), 4);
  assert_eq!(bit_lowest_set_value_u32(0b111100), 4);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_lowest_set_value_u64() {
  assert_eq!(bit_lowest_set_value_u64(0b0), 0);
  assert_eq!(bit_lowest_set_value_u64(0b1), 1);
  assert_eq!(bit_lowest_set_value_u64(0b10), 2);
  assert_eq!(bit_lowest_set_value_u64(0b100), 4);
  assert_eq!(bit_lowest_set_value_u64(0b111100), 4);
}

#[test]
fn test_bit_lowest_set_mask_u32() {
  assert_eq!(bit_lowest_set_mask_u32(0b0), u32::MAX);
  assert_eq!(bit_lowest_set_mask_u32(0b1), 0b1);
  assert_eq!(bit_lowest_set_mask_u32(0b10), 0b11);
  assert_eq!(bit_lowest_set_mask_u32(0b100), 0b111);
  assert_eq!(bit_lowest_set_mask_u32(0b111100), 0b111);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_lowest_set_mask_u64() {
  assert_eq!(bit_lowest_set_mask_u64(0b0), u64::MAX);
  assert_eq!(bit_lowest_set_mask_u64(0b1), 0b1);
  assert_eq!(bit_lowest_set_mask_u64(0b10), 0b11);
  assert_eq!(bit_lowest_set_mask_u64(0b100), 0b111);
  assert_eq!(bit_lowest_set_mask_u64(0b111100), 0b111);
}

#[test]
fn test_bit_lowest_set_reset_u32() {
  assert_eq!(bit_lowest_set_reset_u32(0b0), 0);
  assert_eq!(bit_lowest_set_reset_u32(0b1), 0b0);
  assert_eq!(bit_lowest_set_reset_u32(0b10), 0b00);
  assert_eq!(bit_lowest_set_reset_u32(0b100), 0b000);
  assert_eq!(bit_lowest_set_reset_u32(0b111100), 0b111000);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_bit_lowest_set_reset_u64() {
  assert_eq!(bit_lowest_set_reset_u64(0b0), 0);
  assert_eq!(bit_lowest_set_reset_u64(0b1), 0b0);
  assert_eq!(bit_lowest_set_reset_u64(0b10), 0b00);
  assert_eq!(bit_lowest_set_reset_u64(0b100), 0b000);
  assert_eq!(bit_lowest_set_reset_u64(0b111100), 0b111000);
}

#[test]
fn test_trailing_zero_count_u32() {
  assert_eq!(trailing_zero_count_u32(0b0), 32);
  assert_eq!(trailing_zero_count_u32(0b1), 0);
  assert_eq!(trailing_zero_count_u32(0b10), 1);
  assert_eq!(trailing_zero_count_u32(0b100), 2);
  assert_eq!(trailing_zero_count_u32(0b111100), 2);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_trailing_zero_count_u64() {
  assert_eq!(trailing_zero_count_u64(0b0), 64);
  assert_eq!(trailing_zero_count_u64(0b1), 0);
  assert_eq!(trailing_zero_count_u64(0b10), 1);
  assert_eq!(trailing_zero_count_u64(0b100), 2);
  assert_eq!(trailing_zero_count_u64(0b111100), 2);
}
