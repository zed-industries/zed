#![allow(clippy::approx_constant)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::nonminimal_bool)]
#![allow(unused_imports)]
#![allow(clippy::precedence)]
#![allow(clippy::eq_op)]
#![allow(clippy::identity_op)]

use core::fmt;
use std::{num::Wrapping, ops::ShlAssign};

mod t_f32x4;
mod t_f32x8;
mod t_f64x2;
mod t_f64x4;
mod t_i16x16;
mod t_i16x8;
mod t_i32x4;
mod t_i32x8;
mod t_i64x2;
mod t_i64x4;
mod t_i8x16;
mod t_i8x32;
mod t_u16x16;
mod t_u16x8;
mod t_u32x4;
mod t_u32x8;
mod t_u64x2;
mod t_u64x4;
mod t_u8x16;
mod t_u8x32;
mod t_usefulness;

/// Generates the next pseudo-random number.
/// Definitely non-cryptographic, just used for generating random test values.
fn next_rand_u64(state: &mut u64) -> u64 {
  // Constants for the LCG
  const A: u64 = 6364136223846793005;
  const C: u64 = 1442695040888963407;

  // Update the state and calculate the next number (rotate to avoid lack of
  // randomness in low bits)
  *state = state.wrapping_mul(A).wrapping_add(C).rotate_left(31);

  *state
}

const RNG_SEED: u64 = 0x123456789abcdef0;

/// Generate a pseudo-random value for a type that implements GenSample.
fn gen_random<T: GenSample>(rng: &mut u64) -> T {
  let r = next_rand_u64(rng);

  // generate special values more often than random chance to test edge cases
  let next = match r & 0xf {
    0 => 0,
    1 => 1,
    2 => u64::MAX,
    _ => next_rand_u64(rng),
  };

  T::get_sample(next)
}

/// Test a vector operation against a pure scalar implementation for random
/// values to make sure that the behavior is the same. This allows for easier
/// for correctness for various values of the vector.
fn test_random_vector_vs_scalar<
  V,
  VR,
  T,
  TR,
  FnVec: Fn(V, V) -> VR,
  FnScalar: Fn(T, T) -> TR,
  const N: usize,
>(
  vector_fn: FnVec,
  scalar_fn: FnScalar,
) where
  V: Copy + From<[T; N]>,
  T: Copy + Default + std::fmt::Debug + GenSample,
  TR: Copy + PartialEq + std::fmt::Debug + Default + GenSample,
  VR: Copy + Into<[TR; N]>,
{
  let mut a_arr = [T::default(); N];
  let mut b_arr: [T; N] = [T::default(); N];

  // use a fixed seed for reproducibility
  let mut rng = RNG_SEED;

  // do 100 iterations
  for _i in 0..100 {
    for i in 0..N {
      a_arr[i] = gen_random(&mut rng);
      b_arr[i] = gen_random(&mut rng);
    }

    let mut expected_arr: [TR; N] = [TR::default(); N];
    for i in 0..N {
      expected_arr[i] = scalar_fn(a_arr[i], b_arr[i]);
    }

    let expected_vec_arr: [TR; N] =
      vector_fn(V::from(a_arr), V::from(b_arr)).into();

    for i in 0..N {
      assert!(
        expected_arr[i].binary_eq(expected_vec_arr[i]),
        "scalar = {:?}\nvec = {:?}\na = {:?}\nb = {:?}",
        expected_arr,
        expected_vec_arr,
        a_arr,
        b_arr
      );
    }
  }
}

/// Test a vector reduce operations that generate a scalar from a vector
/// against a pure scalar implementation for random values to make
/// sure that the behavior is the same. This allows for easier for correctness
/// for various values of the vector.
///
/// The scalar operation uses the same construction as the Rust fold function
/// which takes an accumulator and returns the accumulator after applying the
/// operation.
fn test_random_vector_vs_scalar_reduce<
  V,
  T,
  TR,
  FnVec: Fn(V) -> TR,
  FnScalar: Fn(TR, T, usize) -> TR,
  const N: usize,
>(
  vector_fn: FnVec,
  acc: TR,
  scalar_fn: FnScalar,
) where
  V: From<[T; N]> + Into<[T; N]> + Copy + std::fmt::Debug,
  T: Copy + PartialEq + std::fmt::Debug + Default + GenSample,
  TR: Copy + PartialEq + std::fmt::Debug + Default,
{
  let mut a_arr = [T::default(); N];

  // use a fixed seed for reproducibility
  let mut rng = RNG_SEED;

  // do 100 iterations
  for _i in 0..100 {
    for i in 0..N {
      a_arr[i] = gen_random(&mut rng);
    }

    let mut expected_scalar = acc;
    for i in 0..N {
      expected_scalar = scalar_fn(expected_scalar, a_arr[i], i);
    }

    let expected_vec = vector_fn(V::from(a_arr));
    assert_eq!(
      expected_scalar, expected_vec,
      "scalar = {:?} vec = {:?} source = {:?}",
      expected_scalar, expected_vec, a_arr
    );
  }
}

fn test_basic_traits<V, T, const N: usize>()
where
  V: Copy
    + From<[T; N]>
    + Into<[T; N]>
    + std::ops::Add<Output = V>
    + std::ops::Sub<Output = V>
    + std::ops::BitXor<Output = V>
    + std::ops::BitOr<Output = V>
    + std::ops::BitAnd<Output = V>
    + std::ops::Not<Output = V>
    + std::ops::Neg<Output = V>
    + wide::CmpEq<Output = V>
    + PartialEq
    + Eq
    + fmt::Debug,
  T: Copy
    + Default
    + std::fmt::Debug
    + GenSample
    + PartialEq
    + Eq
    + std::ops::BitXor<Output = T>
    + std::ops::BitOr<Output = T>
    + std::ops::BitAnd<Output = T>
    + std::ops::Not<Output = T>,
  Wrapping<T>:
    std::ops::Add<Output = Wrapping<T>> + std::ops::Sub<Output = Wrapping<T>>,
{
  // test add
  test_random_vector_vs_scalar(
    |a: V, b| a + b,
    |a, b| (Wrapping::<T>(a) + Wrapping::<T>(b)).0,
  );

  // test sub
  test_random_vector_vs_scalar(
    |a: V, b| a - b,
    |a, b| (Wrapping::<T>(a) - Wrapping::<T>(b)).0,
  );

  // test neg
  test_random_vector_vs_scalar(
    |a: V, b| a - (-b),
    |a, b| (Wrapping::<T>(a) + Wrapping::<T>(b)).0,
  );

  test_random_vector_vs_scalar(|a: V, b| a ^ b, |a, b| a ^ b);

  test_random_vector_vs_scalar(|a: V, b| a & b, |a, b| a & b);

  test_random_vector_vs_scalar(|a: V, b| a | b, |a, b| a | b);

  test_random_vector_vs_scalar(
    |a: V, b| a.cmp_eq(b),
    |a, b| if a == b { !T::default() } else { T::default() },
  );

  let a = V::from([T::default(); N]);
  let b = V::from([!T::default(); N]);

  assert!(a != b);
  assert!(a == a);
  assert!(b == a.not());
}

/// trait to reduce a 64 bit pseudo-random number to a random sample value
trait GenSample
where
  Self: PartialEq + Copy,
{
  fn get_sample(v: u64) -> Self;
  fn binary_eq(self, b: Self) -> bool {
    self == b
  }
}

impl GenSample for u64 {
  fn get_sample(v: u64) -> Self {
    v
  }
}

impl GenSample for u32 {
  fn get_sample(v: u64) -> Self {
    v as u32
  }
}

impl GenSample for u16 {
  fn get_sample(v: u64) -> Self {
    v as u16
  }
}

impl GenSample for u8 {
  fn get_sample(v: u64) -> Self {
    v as u8
  }
}

impl GenSample for i64 {
  fn get_sample(v: u64) -> Self {
    v as i64
  }
}

impl GenSample for i32 {
  fn get_sample(v: u64) -> Self {
    v as i32
  }
}

impl GenSample for i16 {
  fn get_sample(v: u64) -> Self {
    v as i16
  }
}

impl GenSample for i8 {
  fn get_sample(v: u64) -> Self {
    v as i8
  }
}

impl GenSample for f32 {
  fn get_sample(v: u64) -> Self {
    // generate special float values more often than random
    // chance to test edge cases
    let m = (v >> 8) & 15;

    match m {
      1 => f32::NAN,
      2 => f32::INFINITY,
      3 => f32::NEG_INFINITY,
      _ => ((v as i64) as f32) / 7.0,
    }
  }

  /// floating points Nan always fails equality so we need to special case it
  fn binary_eq(self, b: Self) -> bool {
    if self.is_nan() {
      b.is_nan()
    } else if self.is_infinite() {
      b.is_infinite() && self.is_sign_positive() == b.is_sign_positive()
    } else {
      (self - b).abs() < 0.000001
    }
  }
}

impl GenSample for f64 {
  // generate special float values more often than random
  // chance to test edge cases
  fn get_sample(v: u64) -> Self {
    let m = (v >> 8) & 15;

    match m {
      1 => f64::NAN,
      2 => f64::INFINITY,
      3 => f64::NEG_INFINITY,
      _ => ((v as i64) as f64) / 7.0,
    }
  }

  /// floating points Nan always fails equality so we need to special case it
  fn binary_eq(self, b: Self) -> bool {
    if self.is_nan() {
      b.is_nan()
    } else if self.is_infinite() {
      b.is_infinite() && self.is_sign_positive() == b.is_sign_positive()
    } else {
      (self - b).abs() < 0.000001
    }
  }
}
