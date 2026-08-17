use super::*;

#[test]
fn test_add_carry_u32() {
  let mut out = 0_u32;
  assert_eq!(add_carry_u32(1, u32::MAX, 5, &mut out), 1);
  assert_eq!(out, 5);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_add_carry_u64() {
  let mut out = 0_u64;
  assert_eq!(add_carry_u64(1, u64::MAX, 5, &mut out), 1);
  assert_eq!(out, 5);
}
