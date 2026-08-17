//! Utilities for hashmap initialization based on random sources.

use core::hash::{BuildHasher, Hasher};

/// Wasmparser's hashing state stored per-map.
///
/// This is DoS-resistant when the `std` feature is activated and still somewhat
/// resistant when it's not active but not as secure.
#[derive(Clone, Debug)]
pub struct RandomState(RandomStateImpl);

impl Default for RandomState {
    #[inline]
    fn default() -> RandomState {
        RandomState(RandomStateImpl::default())
    }
}

impl BuildHasher for RandomState {
    type Hasher = RandomStateHasher;

    #[inline]
    fn build_hasher(&self) -> RandomStateHasher {
        RandomStateHasher(self.0.build_hasher())
    }
}

/// Wasmparser's hasher type used with [`RandomState`].
pub struct RandomStateHasher(<RandomStateImpl as BuildHasher>::Hasher);

impl Hasher for RandomStateHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.0.write_u8(i)
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.0.write_u16(i)
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.0.write_u32(i)
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0.write_u64(i)
    }
    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.0.write_u128(i)
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.0.write_usize(i)
    }
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.0.write_i8(i)
    }
    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.0.write_i16(i)
    }
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.0.write_i32(i)
    }
    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.0.write_i64(i)
    }
    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.0.write_i128(i)
    }
    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.0.write_isize(i)
    }
}

// When the `std` feature is active reuse the standard library's implementation
// of hash state and hasher.
#[cfg(feature = "std")]
use std::collections::hash_map::RandomState as RandomStateImpl;

// When the `std` feature is NOT active then rely on `ahash::RandomState`. That
// relies on ASLR by default for randomness.
#[derive(Clone, Debug)]
#[cfg(not(feature = "std"))]
struct RandomStateImpl {
    state: hashbrown::DefaultHashBuilder,
}

#[cfg(not(feature = "std"))]
impl Default for RandomStateImpl {
    fn default() -> RandomStateImpl {
        RandomStateImpl {
            state: hashbrown::DefaultHashBuilder::default(),
        }
    }
}

#[cfg(not(feature = "std"))]
impl BuildHasher for RandomStateImpl {
    type Hasher = <hashbrown::DefaultHashBuilder as BuildHasher>::Hasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        self.state.build_hasher()
    }
}
