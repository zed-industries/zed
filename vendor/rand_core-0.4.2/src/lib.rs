// Copyright 2018 Developers of the Rand project.
// Copyright 2017-2018 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Random number generation traits
//!
//! This crate is mainly of interest to crates publishing implementations of
//! [`RngCore`]. Other users are encouraged to use the [`rand`] crate instead
//! which re-exports the main traits and error types.
//!
//! [`RngCore`] is the core trait implemented by algorithmic pseudo-random number
//! generators and external random-number sources.
//!
//! [`SeedableRng`] is an extension trait for construction from fixed seeds and
//! other random number generators.
//!
//! [`Error`] is provided for error-handling. It is safe to use in `no_std`
//! environments.
//!
//! The [`impls`] and [`le`] sub-modules include a few small functions to assist
//! implementation of [`RngCore`].
//!
//! [`rand`]: https://docs.rs/rand

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://rust-random.github.io/rand/")]

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

#![cfg_attr(not(feature="std"), no_std)]
#![cfg_attr(all(feature="alloc", not(feature="std")), feature(alloc))]

#[cfg(feature="std")] extern crate core;
#[cfg(all(feature = "alloc", not(feature="std")))] extern crate alloc;
#[cfg(feature="serde1")] extern crate serde;
#[cfg(feature="serde1")] #[macro_use] extern crate serde_derive;


use core::default::Default;
use core::convert::AsMut;
use core::ptr::copy_nonoverlapping;

#[cfg(all(feature="alloc", not(feature="std")))] use alloc::boxed::Box;

pub use error::{ErrorKind, Error};


mod error;
pub mod block;
pub mod impls;
pub mod le;


/// The core of a random number generator.
///
/// This trait encapsulates the low-level functionality common to all
/// generators, and is the "back end", to be implemented by generators.
/// End users should normally use the `Rng` trait from the [`rand`] crate,
/// which is automatically implemented for every type implementing `RngCore`.
///
/// Three different methods for generating random data are provided since the
/// optimal implementation of each is dependent on the type of generator. There
/// is no required relationship between the output of each; e.g. many
/// implementations of [`fill_bytes`] consume a whole number of `u32` or `u64`
/// values and drop any remaining unused bytes.
///
/// The [`try_fill_bytes`] method is a variant of [`fill_bytes`] allowing error
/// handling; it is not deemed sufficiently useful to add equivalents for
/// [`next_u32`] or [`next_u64`] since the latter methods are almost always used
/// with algorithmic generators (PRNGs), which are normally infallible.
///
/// Algorithmic generators implementing [`SeedableRng`] should normally have
/// *portable, reproducible* output, i.e. fix Endianness when converting values
/// to avoid platform differences, and avoid making any changes which affect
/// output (except by communicating that the release has breaking changes).
///
/// Typically implementators will implement only one of the methods available
/// in this trait directly, then use the helper functions from the
/// [`impls`] module to implement the other methods.
///
/// It is recommended that implementations also implement:
///
/// - `Debug` with a custom implementation which *does not* print any internal
///   state (at least, [`CryptoRng`]s should not risk leaking state through
///   `Debug`).
/// - `Serialize` and `Deserialize` (from Serde), preferably making Serde
///   support optional at the crate level in PRNG libs.
/// - `Clone`, if possible.
/// - *never* implement `Copy` (accidental copies may cause repeated values).
/// - *do not* implement `Default` for pseudorandom generators, but instead
///   implement [`SeedableRng`], to guide users towards proper seeding.
///   External / hardware RNGs can choose to implement `Default`.
/// - `Eq` and `PartialEq` could be implemented, but are probably not useful.
///
/// # Example
///
/// A simple example, obviously not generating very *random* output:
///
/// ```
/// #![allow(dead_code)]
/// use rand_core::{RngCore, Error, impls};
///
/// struct CountingRng(u64);
///
/// impl RngCore for CountingRng {
///     fn next_u32(&mut self) -> u32 {
///         self.next_u64() as u32
///     }
///
///     fn next_u64(&mut self) -> u64 {
///         self.0 += 1;
///         self.0
///     }
///
///     fn fill_bytes(&mut self, dest: &mut [u8]) {
///         impls::fill_bytes_via_next(self, dest)
///     }
///
///     fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
///         Ok(self.fill_bytes(dest))
///     }
/// }
/// ```
///
/// [`rand`]: https://docs.rs/rand
/// [`try_fill_bytes`]: RngCore::try_fill_bytes
/// [`fill_bytes`]: RngCore::fill_bytes
/// [`next_u32`]: RngCore::next_u32
/// [`next_u64`]: RngCore::next_u64
pub trait RngCore {
    /// Return the next random `u32`.
    ///
    /// RNGs must implement at least one method from this trait directly. In
    /// the case this method is not implemented directly, it can be implemented
    /// using `self.next_u64() as u32` or via
    /// [`fill_bytes`](impls::next_u32_via_fill).
    fn next_u32(&mut self) -> u32;

    /// Return the next random `u64`.
    ///
    /// RNGs must implement at least one method from this trait directly. In
    /// the case this method is not implemented directly, it can be implemented
    /// via [`next_u32`](impls::next_u64_via_u32) or via
    /// [`fill_bytes`](impls::next_u64_via_fill).
    fn next_u64(&mut self) -> u64;

    /// Fill `dest` with random data.
    ///
    /// RNGs must implement at least one method from this trait directly. In
    /// the case this method is not implemented directly, it can be implemented
    /// via [`next_u*`](impls::fill_bytes_via_next) or
    /// via [`try_fill_bytes`](RngCore::try_fill_bytes); if this generator can
    /// fail the implementation must choose how best to handle errors here
    /// (e.g. panic with a descriptive message or log a warning and retry a few
    /// times).
    ///
    /// This method should guarantee that `dest` is entirely filled
    /// with new data, and may panic if this is impossible
    /// (e.g. reading past the end of a file that is being used as the
    /// source of randomness).
    fn fill_bytes(&mut self, dest: &mut [u8]);

    /// Fill `dest` entirely with random data.
    ///
    /// This is the only method which allows an RNG to report errors while
    /// generating random data thus making this the primary method implemented
    /// by external (true) RNGs (e.g. `OsRng`) which can fail. It may be used
    /// directly to generate keys and to seed (infallible) PRNGs.
    ///
    /// Other than error handling, this method is identical to [`fill_bytes`];
    /// thus this may be implemented using `Ok(self.fill_bytes(dest))` or
    /// `fill_bytes` may be implemented with
    /// `self.try_fill_bytes(dest).unwrap()` or more specific error handling.
    ///
    /// [`fill_bytes`]: RngCore::fill_bytes
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error>;
}

/// A marker trait used to indicate that an [`RngCore`] or [`BlockRngCore`]
/// implementation is supposed to be cryptographically secure.
///
/// *Cryptographically secure generators*, also known as *CSPRNGs*, should
/// satisfy an additional properties over other generators: given the first
/// *k* bits of an algorithm's output
/// sequence, it should not be possible using polynomial-time algorithms to
/// predict the next bit with probability significantly greater than 50%.
///
/// Some generators may satisfy an additional property, however this is not
/// required by this trait: if the CSPRNG's state is revealed, it should not be
/// computationally-feasible to reconstruct output prior to this. Some other
/// generators allow backwards-computation and are consided *reversible*.
///
/// Note that this trait is provided for guidance only and cannot guarantee
/// suitability for cryptographic applications. In general it should only be
/// implemented for well-reviewed code implementing well-regarded algorithms.
///
/// Note also that use of a `CryptoRng` does not protect against other
/// weaknesses such as seeding from a weak entropy source or leaking state.
///
/// [`BlockRngCore`]: block::BlockRngCore
pub trait CryptoRng {}

/// A random number generator that can be explicitly seeded.
///
/// This trait encapsulates the low-level functionality common to all
/// pseudo-random number generators (PRNGs, or algorithmic generators).
///
/// The `FromEntropy` trait from the [`rand`] crate is automatically
/// implemented for every type implementing `SeedableRng`, providing
/// a convenient `from_entropy()` constructor.
///
/// [`rand`]: https://docs.rs/rand
pub trait SeedableRng: Sized {
    /// Seed type, which is restricted to types mutably-dereferencable as `u8`
    /// arrays (we recommend `[u8; N]` for some `N`).
    ///
    /// It is recommended to seed PRNGs with a seed of at least circa 100 bits,
    /// which means an array of `[u8; 12]` or greater to avoid picking RNGs with
    /// partially overlapping periods.
    ///
    /// For cryptographic RNG's a seed of 256 bits is recommended, `[u8; 32]`.
    ///
    ///
    /// # Implementing `SeedableRng` for RNGs with large seeds
    ///
    /// Note that the required traits `core::default::Default` and
    /// `core::convert::AsMut<u8>` are not implemented for large arrays
    /// `[u8; N]` with `N` > 32. To be able to implement the traits required by
    /// `SeedableRng` for RNGs with such large seeds, the newtype pattern can be
    /// used:
    ///
    /// ```
    /// use rand_core::SeedableRng;
    ///
    /// const N: usize = 64;
    /// pub struct MyRngSeed(pub [u8; N]);
    /// pub struct MyRng(MyRngSeed);
    ///
    /// impl Default for MyRngSeed {
    ///     fn default() -> MyRngSeed {
    ///         MyRngSeed([0; N])
    ///     }
    /// }
    ///
    /// impl AsMut<[u8]> for MyRngSeed {
    ///     fn as_mut(&mut self) -> &mut [u8] {
    ///         &mut self.0
    ///     }
    /// }
    ///
    /// impl SeedableRng for MyRng {
    ///     type Seed = MyRngSeed;
    ///
    ///     fn from_seed(seed: MyRngSeed) -> MyRng {
    ///         MyRng(seed)
    ///     }
    /// }
    /// ```
    type Seed: Sized + Default + AsMut<[u8]>;

    /// Create a new PRNG using the given seed.
    ///
    /// PRNG implementations are allowed to assume that bits in the seed are
    /// well distributed. That means usually that the number of one and zero
    /// bits are about equal, and values like 0, 1 and (size - 1) are unlikely.
    ///
    /// PRNG implementations are recommended to be reproducible. A PRNG seeded
    /// using this function with a fixed seed should produce the same sequence
    /// of output in the future and on different architectures (with for example
    /// different endianness).
    ///
    /// It is however not required that this function yield the same state as a
    /// reference implementation of the PRNG given equivalent seed; if necessary
    /// another constructor replicating behaviour from a reference
    /// implementation can be added.
    ///
    /// PRNG implementations should make sure `from_seed` never panics. In the
    /// case that some special values (like an all zero seed) are not viable
    /// seeds it is preferable to map these to alternative constant value(s),
    /// for example `0xBAD5EEDu32` or `0x0DDB1A5E5BAD5EEDu64` ("odd biases? bad
    /// seed"). This is assuming only a small number of values must be rejected.
    fn from_seed(seed: Self::Seed) -> Self;

    /// Create a new PRNG using a `u64` seed.
    ///
    /// This is a convenience-wrapper around `from_seed` to allow construction
    /// of any `SeedableRng` from a simple `u64` value. It is designed such that
    /// low Hamming Weight numbers like 0 and 1 can be used and should still
    /// result in good, independent seeds to the PRNG which is returned.
    ///
    /// This **is not suitable for cryptography**, as should be clear given that
    /// the input size is only 64 bits.
    ///
    /// Implementations for PRNGs *may* provide their own implementations of
    /// this function, but the default implementation should be good enough for
    /// all purposes. *Changing* the implementation of this function should be
    /// considered a value-breaking change.
    fn seed_from_u64(mut state: u64) -> Self {
        // We use PCG32 to generate a u32 sequence, and copy to the seed
        const MUL: u64 = 6364136223846793005;
        const INC: u64 = 11634580027462260723;

        let mut seed = Self::Seed::default();
        for chunk in seed.as_mut().chunks_mut(4) {
            // We advance the state first (to get away from the input value,
            // in case it has low Hamming Weight).
            state = state.wrapping_mul(MUL).wrapping_add(INC);

            // Use PCG output function with to_le to generate x:
            let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
            let rot = (state >> 59) as u32;
            let x = xorshifted.rotate_right(rot).to_le();

            unsafe {
                let p = &x as *const u32 as *const u8;
                copy_nonoverlapping(p, chunk.as_mut_ptr(), chunk.len());
            }
        }

        Self::from_seed(seed)
    }

    /// Create a new PRNG seeded from another `Rng`.
    ///
    /// This is the recommended way to initialize PRNGs with fresh entropy. The
    /// `FromEntropy` trait from the [`rand`] crate provides a convenient
    /// `from_entropy` method based on `from_rng`.
    ///
    /// Usage of this method is not recommended when reproducibility is required
    /// since implementing PRNGs are not required to fix Endianness and are
    /// allowed to modify implementations in new releases.
    ///
    /// It is important to use a good source of randomness to initialize the
    /// PRNG. Cryptographic PRNG may be rendered insecure when seeded from a
    /// non-cryptographic PRNG or with insufficient entropy.
    /// Many non-cryptographic PRNGs will show statistical bias in their first
    /// results if their seed numbers are small or if there is a simple pattern
    /// between them.
    ///
    /// Prefer to seed from a strong external entropy source like `OsRng` from
    /// the [`rand_os`] crate or from a cryptographic PRNG; if creating a new
    /// generator for cryptographic uses you *must* seed from a strong source.
    ///
    /// Seeding a small PRNG from another small PRNG is possible, but
    /// something to be careful with. An extreme example of how this can go
    /// wrong is seeding an Xorshift RNG from another Xorshift RNG, which
    /// will effectively clone the generator. In general seeding from a
    /// generator which is hard to predict is probably okay.
    ///
    /// PRNG implementations are allowed to assume that a good RNG is provided
    /// for seeding, and that it is cryptographically secure when appropriate.
    ///
    /// [`rand`]: https://docs.rs/rand
    /// [`rand_os`]: https://docs.rs/rand_os
    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut seed = Self::Seed::default();
        rng.try_fill_bytes(seed.as_mut())?;
        Ok(Self::from_seed(seed))
    }
}

// Implement `RngCore` for references to an `RngCore`.
// Force inlining all functions, so that it is up to the `RngCore`
// implementation and the optimizer to decide on inlining.
impl<'a, R: RngCore + ?Sized> RngCore for &'a mut R {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        (**self).next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        (**self).next_u64()
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        (**self).try_fill_bytes(dest)
    }
}

// Implement `RngCore` for boxed references to an `RngCore`.
// Force inlining all functions, so that it is up to the `RngCore`
// implementation and the optimizer to decide on inlining.
#[cfg(feature="alloc")]
impl<R: RngCore + ?Sized> RngCore for Box<R> {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        (**self).next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        (**self).next_u64()
    }

    #[inline(always)]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        (**self).fill_bytes(dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        (**self).try_fill_bytes(dest)
    }
}

#[cfg(feature="std")]
impl std::io::Read for RngCore {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.try_fill_bytes(buf)?;
        Ok(buf.len())
    }
}

// Implement `CryptoRng` for references to an `CryptoRng`.
impl<'a, R: CryptoRng + ?Sized> CryptoRng for &'a mut R {}

// Implement `CryptoRng` for boxed references to an `CryptoRng`.
#[cfg(feature="alloc")]
impl<R: CryptoRng + ?Sized> CryptoRng for Box<R> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seed_from_u64() {
        struct SeedableNum(u64);
        impl SeedableRng for SeedableNum {
            type Seed = [u8; 8];
            fn from_seed(seed: Self::Seed) -> Self {
                let mut x = [0u64; 1];
                le::read_u64_into(&seed, &mut x);
                SeedableNum(x[0])
            }
        }

        const N: usize = 8;
        const SEEDS: [u64; N] = [0u64, 1, 2, 3, 4, 8, 16, -1i64 as u64];
        let mut results = [0u64; N];
        for (i, seed) in SEEDS.iter().enumerate() {
            let SeedableNum(x) = SeedableNum::seed_from_u64(*seed);
            results[i] = x;
        }

        for (i1, r1) in results.iter().enumerate() {
            let weight = r1.count_ones();
            // This is the binomial distribution B(64, 0.5), so chance of
            // weight < 20 is binocdf(19, 64, 0.5) = 7.8e-4, and same for
            // weight > 44.
            assert!(weight >= 20 && weight <= 44);

            for (i2, r2) in results.iter().enumerate() {
                if i1 == i2 { continue; }
                let diff_weight = (r1 ^ r2).count_ones();
                assert!(diff_weight >= 20);
            }
        }

        // value-breakage test:
        assert_eq!(results[0], 5029875928683246316);
    }
}
