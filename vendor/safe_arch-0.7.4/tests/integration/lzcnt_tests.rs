use super::*;

#[test]
fn test_leading_zero_count_u32() {
  assert_eq!(leading_zero_count_u32(u32::MAX), 0);
  assert_eq!(leading_zero_count_u32(u32::MAX >> 3), 3);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_leading_zero_count_u64() {
  assert_eq!(leading_zero_count_u64(u64::MAX), 0);
  assert_eq!(leading_zero_count_u64(u64::MAX >> 3), 3);
}
