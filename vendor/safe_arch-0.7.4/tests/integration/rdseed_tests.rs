use super::*;

// Note(Lokathor): It's technically possible, and valid, that these could fail
// when run just once. However, if they fail across multiple test runs then
// *that* is when we have a problem.

#[test]
fn test_rdseed_u16() {
  let mut val = 0_u16;
  let _it_worked = rdseed_u16(&mut val);
}

#[test]
fn test_rdseed_u32() {
  let mut val = 0_u32;
  let _it_worked = rdseed_u32(&mut val);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_rdseed_u64() {
  let mut val = 0_u64;
  let _it_worked = rdseed_u64(&mut val);
}
