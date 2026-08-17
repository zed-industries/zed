#![allow(bad_style)]
#![allow(unused_imports)]
#![allow(clippy::identity_op)]

use safe_arch::*;

#[cfg(target_feature = "adx")]
mod adx_tests;

#[cfg(target_feature = "avx")]
mod avx_tests;

#[cfg(target_feature = "bmi1")]
mod bmi1_tests;

#[cfg(target_feature = "bmi2")]
mod bmi2_tests;

#[cfg(target_feature = "lzcnt")]
mod lzcnt_tests;

#[cfg(target_feature = "pclmulqdq")]
mod pclmulqdq_tests;

#[cfg(target_feature = "popcnt")]
mod popcnt_tests;

#[cfg(target_feature = "rdrand")]
mod rdrand_tests;

#[cfg(target_feature = "rdseed")]
mod rdseed_tests;

#[cfg(target_feature = "sse2")]
mod sse2_tests;

#[cfg(target_feature = "sse3")]
mod sse3_tests;

#[cfg(target_feature = "ssse3")]
mod ssse3_tests;

#[cfg(target_feature = "sse4.1")]
mod sse4_1_tests;

#[cfg(target_feature = "sse4.2")]
mod sse4_2_tests;

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m128_size_align() {
  assert_eq!(core::mem::size_of::<m128>(), 16);
  assert_eq!(core::mem::align_of::<m128>(), 16);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m128d_size_align() {
  assert_eq!(core::mem::size_of::<m128d>(), 16);
  assert_eq!(core::mem::align_of::<m128d>(), 16);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m128i_size_align() {
  assert_eq!(core::mem::size_of::<m128i>(), 16);
  assert_eq!(core::mem::align_of::<m128i>(), 16);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m256_size_align() {
  assert_eq!(core::mem::size_of::<m256>(), 32);
  assert_eq!(core::mem::align_of::<m256>(), 32);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m256d_size_align() {
  assert_eq!(core::mem::size_of::<m256d>(), 32);
  assert_eq!(core::mem::align_of::<m256d>(), 32);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m256i_size_align() {
  assert_eq!(core::mem::size_of::<m256i>(), 32);
  assert_eq!(core::mem::align_of::<m256i>(), 32);
}

#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn test_m128_fmt() {
  let f = format!("{:?}", m128::default());
  assert_eq!(&f, "m128(0.0, 0.0, 0.0, 0.0)");

  let f = format!("{}", m128::default());
  assert_eq!(&f, "(0, 0, 0, 0)");

  let f = format!("{:b}", m128::default());
  assert_eq!(&f, "(0, 0, 0, 0)");

  let f = format!("{:e}", m128::default());
  assert_eq!(&f, "(0e0, 0e0, 0e0, 0e0)");

  let f = format!("{:E}", m128::default());
  assert_eq!(&f, "(0E0, 0E0, 0E0, 0E0)");

  let f = format!("{:x}", m128::default());
  assert_eq!(&f, "(0, 0, 0, 0)");

  let f = format!("{:X}", m128::default());
  assert_eq!(&f, "(0, 0, 0, 0)");

  let f = format!("{:o}", m128::default());
  assert_eq!(&f, "(0, 0, 0, 0)");
}

#[allow(dead_code)]
fn approx_eq_f32(a: f32, b: f32) -> bool {
  (a - b).abs() < 0.00000001
}

#[allow(dead_code)]
fn approx_eq_f64(a: f64, b: f64) -> bool {
  (a - b).abs() < 0.00000000001
}
