// Copyright 2015-2016 Brian Smith.
// SPDX-License-Identifier: ISC
// Modifications copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

//! Cryptographic pseudo-random number generation.
//!
//! An application should create a single `SystemRandom` and then use it for
//! all randomness generation. See `SystemRandom`'s documentation for more
//! details.

//! # Example
//! ```
//! use aws_lc_rs::{rand, rand::SecureRandom};
//!
//! //  Using `rand::fill`
//! let mut rand_bytes = [0u8; 32];
//! rand::fill(&mut rand_bytes).unwrap();
//!
//! // Using `SystemRandom`
//! let rng = rand::SystemRandom::new();
//! rng.fill(&mut rand_bytes).unwrap();
//!
//! // Using `rand::generate`
//! let random_array = rand::generate(&rng).unwrap();
//! let more_rand_bytes: [u8; 64] = random_array.expose();
//! ```
use crate::aws_lc::RAND_bytes;
use crate::error::Unspecified;
use crate::fips::indicator_check;
use core::fmt::Debug;

/// Re-exports of sealed traits for development testing.
///
/// This module is only available when the `dev-tests-only` feature is enabled.
/// It exposes the [`SecureRandom`](unsealed::SecureRandom) trait, allowing consumers
/// to provide their own implementations (e.g., a deterministic RNG) for testing purposes.
///
/// # Example
///
/// ```ignore
/// use aws_lc_rs::rand::{unsealed, SecureRandom};
/// use aws_lc_rs::error::Unspecified;
///
/// #[derive(Debug)]
/// struct DeterministicRandom(u8);
///
/// impl unsealed::SecureRandom for DeterministicRandom {
///     fn fill_impl(&self, dest: &mut [u8]) -> Result<(), Unspecified> {
///         for (i, byte) in dest.iter_mut().enumerate() {
///             *byte = self.0.wrapping_add(i as u8);
///         }
///         Ok(())
///     }
/// }
/// ```
#[cfg(any(dev_tests_only, aws_lc_rs_docsrs))]
#[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "dev-tests-only")))]
#[allow(unused_imports)]
pub mod unsealed {
    pub use super::sealed::*;
}

/// A secure random number generator.
pub trait SecureRandom: sealed::SecureRandom {
    /// Fills `dest` with random bytes.
    ///
    /// # Errors
    /// `error::Unspecified` if unable to fill `dest`.
    fn fill(&self, dest: &mut [u8]) -> Result<(), Unspecified>;

    /// Fills `dest` with random bytes.
    ///
    /// This method is only available when the `dev-tests-only` feature is enabled.
    ///
    /// # Errors
    /// `error::Unspecified` if unable to fill `dest`.
    #[cfg(any(test, dev_tests_only, aws_lc_rs_docsrs))]
    #[cfg_attr(aws_lc_rs_docsrs, doc(cfg(feature = "dev-tests-only")))]
    fn mut_fill(&mut self, dest: &mut [u8]) -> Result<(), Unspecified>;
}

impl<T> SecureRandom for T
where
    T: sealed::SecureRandom,
{
    #[inline]
    fn fill(&self, dest: &mut [u8]) -> Result<(), Unspecified> {
        self.fill_impl(dest)
    }

    #[inline]
    #[cfg(any(test, dev_tests_only, aws_lc_rs_docsrs))]
    fn mut_fill(&mut self, dest: &mut [u8]) -> Result<(), Unspecified> {
        self.fill_impl(dest)
    }
}

/// A random value constructed from a `SecureRandom` that hasn't been exposed
/// through any safe Rust interface.
///
/// Intentionally does not implement any traits other than `Sized`.
pub struct Random<T: RandomlyConstructable>(T);

impl<T: RandomlyConstructable> Random<T> {
    /// Expose the random value.
    #[inline]
    pub fn expose(self) -> T {
        self.0
    }
}

/// Generate the new random value using `rng`.
///
/// # Errors
/// `error::Unspecified` if unable to fill buffer.
#[inline]
pub fn generate<T: RandomlyConstructable>(
    rng: &dyn SecureRandom,
) -> Result<Random<T>, Unspecified> {
    let mut r = T::zero();
    rng.fill(r.as_mut_bytes())?;
    Ok(Random(r))
}

pub(crate) mod sealed {
    use crate::error;

    /// A sealed trait for secure random number generation.
    pub trait SecureRandom: core::fmt::Debug {
        /// Fills `dest` with random bytes.
        ///
        /// # Errors
        /// Returns `error::Unspecified` if unable to fill `dest`.
        fn fill_impl(&self, dest: &mut [u8]) -> Result<(), error::Unspecified>;
    }

    /// A sealed trait for types that can be randomly constructed.
    pub trait RandomlyConstructable: Sized {
        /// Returns a zeroed instance of the type.
        fn zero() -> Self;
        /// Returns a mutable byte slice of the value.
        fn as_mut_bytes(&mut self) -> &mut [u8];
    }

    impl<const T: usize> RandomlyConstructable for [u8; T] {
        #[inline]
        fn zero() -> Self {
            [0; T]
        }

        #[inline]
        fn as_mut_bytes(&mut self) -> &mut [u8] {
            &mut self[..]
        }
    }
}

/// A type that can be returned by `aws_lc_rs::rand::generate()`.
pub trait RandomlyConstructable: sealed::RandomlyConstructable {}

impl<T> RandomlyConstructable for T where T: sealed::RandomlyConstructable {}

/// A secure random number generator where the random values come from the
/// underlying *AWS-LC* libcrypto.
///
/// A single `SystemRandom` may be shared across multiple threads safely.
//
// # FIPS
// Use this implementation for retrieving random bytes.
#[derive(Clone, Debug)]
pub struct SystemRandom(());

const SYSTEM_RANDOM: SystemRandom = SystemRandom(());

impl SystemRandom {
    /// Constructs a new `SystemRandom`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SystemRandom {
    fn default() -> Self {
        SYSTEM_RANDOM
    }
}

impl sealed::SecureRandom for SystemRandom {
    #[inline]
    fn fill_impl(&self, dest: &mut [u8]) -> Result<(), Unspecified> {
        fill(dest)
    }
}

/// Fills `dest` with random bytes.
///
// # FIPS
// Use this for retrieving random bytes or [`SystemRandom`].
//
/// # Errors
/// `error::Unspecified` if unable to fill `dest`.
pub fn fill(dest: &mut [u8]) -> Result<(), Unspecified> {
    if 1 != indicator_check!(unsafe { RAND_bytes(dest.as_mut_ptr(), dest.len()) }) {
        return Err(Unspecified);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::rand;
    use core::array::IntoIter;

    use crate::rand::{generate, SecureRandom, SystemRandom};

    #[test]
    fn test_secure_random_fill() {
        // Collect enough random values so that the assertions below should never fail again
        let mut random_array = [0u8; 1009];
        let rng = SystemRandom::new();
        rng.fill(&mut random_array).unwrap();

        let (mean, variance) = mean_variance(&mut random_array.into_iter());
        assert!((106f64..150f64).contains(&mean), "Mean: {mean}");
        assert!(variance > 8f64);
        println!("Mean: {mean} Variance: {variance}");
    }

    #[test]
    fn test_rand_fill() {
        // Collect enough random values so that the assertions below should never fail again
        let mut random_array = [0u8; 1009];
        rand::fill(&mut random_array).unwrap();

        let (mean, variance) = mean_variance(&mut random_array.into_iter());
        assert!((106f64..150f64).contains(&mean), "Mean: {mean}");
        assert!(variance > 8f64);
        println!("Mean: {mean} Variance: {variance}");
    }

    #[test]
    fn test_randomly_constructable() {
        let rando = SystemRandom::new();
        let random_array = generate(&rando).unwrap();
        // Collect enough random values so that the assertions below should never fail again
        let random_array: [u8; 1009] = random_array.expose();
        let (mean, variance) = mean_variance(&mut random_array.into_iter());
        assert!((106f64..150f64).contains(&mean), "Mean: {mean}");
        assert!(variance > 8f64);
        println!("Mean: {mean} Variance: {variance}");
    }

    fn mean_variance<T: Into<f64>, const N: usize>(iterable: &mut IntoIter<T, N>) -> (f64, f64) {
        let iter = iterable;
        let mean: Option<T> = iter.next();
        let mut mean = mean.unwrap().into();
        let mut var_squared = 0f64;
        let mut count = 1f64;
        for value in iter.by_ref() {
            count += 1f64;
            let value = value.into();
            let prev_mean = mean;
            mean = prev_mean + (value - prev_mean) / count;
            var_squared =
                var_squared + ((value - prev_mean) * (value - mean) - var_squared) / count;
        }

        (mean, var_squared.sqrt())
    }
}
