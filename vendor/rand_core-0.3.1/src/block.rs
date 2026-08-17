// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The `BlockRngCore` trait and implementation helpers
//!
//! The [`BlockRngCore`] trait exists to assist in the implementation of RNGs
//! which generate a block of data in a cache instead of returning generated
//! values directly.
//!
//! Usage of this trait is optional, but provides two advantages:
//! implementations only need to concern themselves with generation of the
//! block, not the various [`RngCore`] methods (especially [`fill_bytes`], where
//! the optimal implementations are not trivial), and this allows
//! `ReseedingRng` (see [`rand`](https://docs.rs/rand) crate) perform periodic
//! reseeding with very low overhead.
//!
//! # Example
//!
//! ```norun
//! use rand_core::block::{BlockRngCore, BlockRng};
//!
//! struct MyRngCore;
//!
//! impl BlockRngCore for MyRngCore {
//!     type Results = [u32; 16];
//!
//!     fn generate(&mut self, results: &mut Self::Results) {
//!         unimplemented!()
//!     }
//! }
//!
//! impl SeedableRng for MyRngCore {
//!     type Seed = unimplemented!();
//!     fn from_seed(seed: Self::Seed) -> Self {
//!         unimplemented!()
//!     }
//! }
//!
//! // optionally, also implement CryptoRng for MyRngCore
//!
//! // Final RNG.
//! type MyRng = BlockRng<u32, MyRngCore>;
//! ```
//!
//! [`BlockRngCore`]: crate::block::BlockRngCore
//! [`fill_bytes`]: RngCore::fill_bytes

use core::convert::AsRef;
use core::fmt;
use {RngCore, CryptoRng, SeedableRng, Error};
use impls::{fill_via_u32_chunks, fill_via_u64_chunks};

/// A trait for RNGs which do not generate random numbers individually, but in
/// blocks (typically `[u32; N]`). This technique is commonly used by
/// cryptographic RNGs to improve performance.
///
/// See the [module][crate::block] documentation for details.
pub trait BlockRngCore {
    /// Results element type, e.g. `u32`.
    type Item;

    /// Results type. This is the 'block' an RNG implementing `BlockRngCore`
    /// generates, which will usually be an array like `[u32; 16]`.
    type Results: AsRef<[Self::Item]> + AsMut<[Self::Item]> + Default;

    /// Generate a new block of results.
    fn generate(&mut self, results: &mut Self::Results);
}


/// A wrapper type implementing [`RngCore`] for some type implementing
/// [`BlockRngCore`] with `u32` array buffer; i.e. this can be used to implement
/// a full RNG from just a `generate` function.
///
/// The `core` field may be accessed directly but the results buffer may not.
/// PRNG implementations can simply use a type alias
/// (`pub type MyRng = BlockRng<MyRngCore>;`) but might prefer to use a
/// wrapper type (`pub struct MyRng(BlockRng<MyRngCore>);`); the latter must
/// re-implement `RngCore` but hides the implementation details and allows
/// extra functionality to be defined on the RNG
/// (e.g. `impl MyRng { fn set_stream(...){...} }`).
///
/// `BlockRng` has heavily optimized implementations of the [`RngCore`] methods
/// reading values from the results buffer, as well as
/// calling [`BlockRngCore::generate`] directly on the output array when
/// [`fill_bytes`] / [`try_fill_bytes`] is called on a large array. These methods
/// also handle the bookkeeping of when to generate a new batch of values.
///
/// No whole generated `u32` values are thown away and all values are consumed
/// in-order. [`next_u32`] simply takes the next available `u32` value.
/// [`next_u64`] is implemented by combining two `u32` values, least
/// significant first. [`fill_bytes`] and [`try_fill_bytes`] consume a whole
/// number of `u32` values, converting each `u32` to a byte slice in
/// little-endian order. If the requested byte length is not a multiple of 4,
/// some bytes will be discarded.
///
/// See also [`BlockRng64`] which uses `u64` array buffers. Currently there is
/// no direct support for other buffer types.
///
/// For easy initialization `BlockRng` also implements [`SeedableRng`].
///
/// [`next_u32`]: RngCore::next_u32
/// [`next_u64`]: RngCore::next_u64
/// [`fill_bytes`]: RngCore::fill_bytes
/// [`try_fill_bytes`]: RngCore::try_fill_bytes
#[derive(Clone)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct BlockRng<R: BlockRngCore + ?Sized> {
    results: R::Results,
    index: usize,
    /// The *core* part of the RNG, implementing the `generate` function.
    pub core: R,
}

// Custom Debug implementation that does not expose the contents of `results`.
impl<R: BlockRngCore + fmt::Debug> fmt::Debug for BlockRng<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockRng")
           .field("core", &self.core)
           .field("result_len", &self.results.as_ref().len())
           .field("index", &self.index)
           .finish()
    }
}

impl<R: BlockRngCore> BlockRng<R> {
    /// Create a new `BlockRng` from an existing RNG implementing
    /// `BlockRngCore`. Results will be generated on first use.
    pub fn new(core: R) -> BlockRng<R>{
        let results_empty = R::Results::default();
        BlockRng {
            core,
            index: results_empty.as_ref().len(),
            results: results_empty,
        }
    }

    /// Get the index into the result buffer.
    ///
    /// If this is equal to or larger than the size of the result buffer then
    /// the buffer is "empty" and `generate()` must be called to produce new
    /// results.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Reset the number of available results.
    /// This will force a new set of results to be generated on next use.
    pub fn reset(&mut self) {
        self.index = self.results.as_ref().len();
    }

    /// Generate a new set of results immediately, setting the index to the
    /// given value.
    pub fn generate_and_set(&mut self, index: usize) {
        assert!(index < self.results.as_ref().len());
        self.core.generate(&mut self.results);
        self.index = index;
    }
}

impl<R: BlockRngCore<Item=u32>> RngCore for BlockRng<R>
where <R as BlockRngCore>::Results: AsRef<[u32]> + AsMut<[u32]>
{
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        if self.index >= self.results.as_ref().len() {
            self.generate_and_set(0);
        }

        let value = self.results.as_ref()[self.index];
        self.index += 1;
        value
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let read_u64 = |results: &[u32], index| {
            if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
                // requires little-endian CPU supporting unaligned reads:
                unsafe { *(&results[index] as *const u32 as *const u64) }
            } else {
                let x = u64::from(results[index]);
                let y = u64::from(results[index + 1]);
                (y << 32) | x
            }
        };

        let len = self.results.as_ref().len();

        let index = self.index;
        if index < len-1 {
            self.index += 2;
            // Read an u64 from the current index
            read_u64(self.results.as_ref(), index)
        } else if index >= len {
            self.generate_and_set(2);
            read_u64(self.results.as_ref(), 0)
        } else {
            let x = u64::from(self.results.as_ref()[len-1]);
            self.generate_and_set(1);
            let y = u64::from(self.results.as_ref()[0]);
            (y << 32) | x
        }
    }

    // As an optimization we try to write directly into the output buffer.
    // This is only enabled for little-endian platforms where unaligned writes
    // are known to be safe and fast.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut filled = 0;

        // Continue filling from the current set of results
        if self.index < self.results.as_ref().len() {
            let (consumed_u32, filled_u8) =
                fill_via_u32_chunks(&self.results.as_ref()[self.index..],
                                    dest);

            self.index += consumed_u32;
            filled += filled_u8;
        }

        let len_remainder =
            (dest.len() - filled) % (self.results.as_ref().len() * 4);
        let end_direct = dest.len() - len_remainder;

        while filled < end_direct {
            let dest_u32: &mut R::Results = unsafe {
                &mut *(dest[filled..].as_mut_ptr() as
                *mut <R as BlockRngCore>::Results)
            };
            self.core.generate(dest_u32);
            filled += self.results.as_ref().len() * 4;
            self.index = self.results.as_ref().len();
        }

        if len_remainder > 0 {
            self.core.generate(&mut self.results);
            let (consumed_u32, _) =
                fill_via_u32_chunks(self.results.as_ref(),
                                    &mut dest[filled..]);

            self.index = consumed_u32;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        while read_len < dest.len() {
            if self.index >= self.results.as_ref().len() {
                self.generate_and_set(0);
            }
            let (consumed_u32, filled_u8) =
                fill_via_u32_chunks(&self.results.as_ref()[self.index..],
                                    &mut dest[read_len..]);

            self.index += consumed_u32;
            read_len += filled_u8;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl<R: BlockRngCore + SeedableRng> SeedableRng for BlockRng<R> {
    type Seed = R::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }

    fn seed_from_u64(seed: u64) -> Self {
        Self::new(R::seed_from_u64(seed))
    }

    fn from_rng<S: RngCore>(rng: S) -> Result<Self, Error> {
        Ok(Self::new(R::from_rng(rng)?))
    }
}



/// A wrapper type implementing [`RngCore`] for some type implementing
/// [`BlockRngCore`] with `u64` array buffer; i.e. this can be used to implement
/// a full RNG from just a `generate` function.
///
/// This is similar to [`BlockRng`], but specialized for algorithms that operate
/// on `u64` values.
///
/// No whole generated `u64` values are thrown away and all values are consumed
/// in-order. [`next_u64`] simply takes the next available `u64` value.
/// [`next_u32`] is however a bit special: half of a `u64` is consumed, leaving
/// the other half in the buffer. If the next function called is [`next_u32`]
/// then the other half is then consumed, however both [`next_u64`] and
/// [`fill_bytes`] discard the rest of any half-consumed `u64`s when called.
///
/// [`fill_bytes`] and [`try_fill_bytes`] consume a whole number of `u64`
/// values. If the requested length is not a multiple of 8, some bytes will be
/// discarded.
///
/// [`next_u32`]: RngCore::next_u32
/// [`next_u64`]: RngCore::next_u64
/// [`fill_bytes`]: RngCore::fill_bytes
/// [`try_fill_bytes`]: RngCore::try_fill_bytes
#[derive(Clone)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
pub struct BlockRng64<R: BlockRngCore + ?Sized> {
    results: R::Results,
    index: usize,
    half_used: bool, // true if only half of the previous result is used
    /// The *core* part of the RNG, implementing the `generate` function.
    pub core: R,
}

// Custom Debug implementation that does not expose the contents of `results`.
impl<R: BlockRngCore + fmt::Debug> fmt::Debug for BlockRng64<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockRng64")
           .field("core", &self.core)
           .field("result_len", &self.results.as_ref().len())
           .field("index", &self.index)
           .field("half_used", &self.half_used)
           .finish()
    }
}

impl<R: BlockRngCore> BlockRng64<R> {
    /// Create a new `BlockRng` from an existing RNG implementing
    /// `BlockRngCore`. Results will be generated on first use.
    pub fn new(core: R) -> BlockRng64<R>{
        let results_empty = R::Results::default();
        BlockRng64 {
            core,
            index: results_empty.as_ref().len(),
            half_used: false,
            results: results_empty,
        }
    }

    /// Get the index into the result buffer.
    ///
    /// If this is equal to or larger than the size of the result buffer then
    /// the buffer is "empty" and `generate()` must be called to produce new
    /// results.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Reset the number of available results.
    /// This will force a new set of results to be generated on next use.
    pub fn reset(&mut self) {
        self.index = self.results.as_ref().len();
        self.half_used = false;
    }

    /// Generate a new set of results immediately, setting the index to the
    /// given value.
    pub fn generate_and_set(&mut self, index: usize) {
        assert!(index < self.results.as_ref().len());
        self.core.generate(&mut self.results);
        self.index = index;
        self.half_used = false;
    }
}

impl<R: BlockRngCore<Item=u64>> RngCore for BlockRng64<R>
where <R as BlockRngCore>::Results: AsRef<[u64]> + AsMut<[u64]>
{
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        let mut index = self.index * 2 - self.half_used as usize;
        if index >= self.results.as_ref().len() * 2 {
            self.core.generate(&mut self.results);
            self.index = 0;
            // `self.half_used` is by definition `false`
            self.half_used = false;
            index = 0;
        }

        self.half_used = !self.half_used;
        self.index += self.half_used as usize;

        // Index as if this is a u32 slice.
        unsafe {
            let results =
                &*(self.results.as_ref() as *const [u64] as *const [u32]);
            if cfg!(target_endian = "little") {
                *results.get_unchecked(index)
            } else {
                *results.get_unchecked(index ^ 1)
            }
        }
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        if self.index >= self.results.as_ref().len() {
            self.core.generate(&mut self.results);
            self.index = 0;
        }

        let value = self.results.as_ref()[self.index];
        self.index += 1;
        self.half_used = false;
        value
    }

    // As an optimization we try to write directly into the output buffer.
    // This is only enabled for little-endian platforms where unaligned writes
    // are known to be safe and fast.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut filled = 0;
        self.half_used = false;

        // Continue filling from the current set of results
        if self.index < self.results.as_ref().len() {
            let (consumed_u64, filled_u8) =
                fill_via_u64_chunks(&self.results.as_ref()[self.index..],
                                    dest);

            self.index += consumed_u64;
            filled += filled_u8;
        }

        let len_remainder =
            (dest.len() - filled) % (self.results.as_ref().len() * 8);
        let end_direct = dest.len() - len_remainder;

        while filled < end_direct {
            let dest_u64: &mut R::Results = unsafe {
                ::core::mem::transmute(dest[filled..].as_mut_ptr())
            };
            self.core.generate(dest_u64);
            filled += self.results.as_ref().len() * 8;
            self.index = self.results.as_ref().len();
        }

        if len_remainder > 0 {
            self.core.generate(&mut self.results);
            let (consumed_u64, _) =
                fill_via_u64_chunks(&mut self.results.as_ref(),
                                    &mut dest[filled..]);

            self.index = consumed_u64;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut read_len = 0;
        self.half_used = false;
        while read_len < dest.len() {
            if self.index as usize >= self.results.as_ref().len() {
                self.core.generate(&mut self.results);
                self.index = 0;
            }

            let (consumed_u64, filled_u8) =
                fill_via_u64_chunks(&self.results.as_ref()[self.index as usize..],
                                    &mut dest[read_len..]);

            self.index += consumed_u64;
            read_len += filled_u8;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl<R: BlockRngCore + SeedableRng> SeedableRng for BlockRng64<R> {
    type Seed = R::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }

    fn seed_from_u64(seed: u64) -> Self {
        Self::new(R::seed_from_u64(seed))
    }

    fn from_rng<S: RngCore>(rng: S) -> Result<Self, Error> {
        Ok(Self::new(R::from_rng(rng)?))
    }
}

impl<R: BlockRngCore + CryptoRng> CryptoRng for BlockRng<R> {}
