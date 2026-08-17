//! AHash is a hashing algorithm is intended to be a high performance, (hardware specific), keyed hash function.
//! This can be seen as a DOS resistant alternative to `FxHash`, or a fast equivalent to `SipHash`.
//! It provides a high speed hash algorithm, but where the result is not predictable without knowing a Key.
//! This allows it to be used in a `HashMap` without allowing for the possibility that an malicious user can
//! induce a collision.
//!
//! # How aHash works
//!
//! aHash uses the hardware AES instruction on x86 processors to provide a keyed hash function.
//! aHash is not a cryptographically secure hash.
//!
//! # Example
//! ```
//! use ahash::{AHasher, RandomState};
//! use std::collections::HashMap;
//!
//! let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
//! map.insert(12, 34);
//! ```
//! For convinence wrappers called `AHashMap` and `AHashSet` are also provided.
//! These to the same thing with slightly less typing.
//! ```ignore
//! use ahash::AHashMap;
//!
//! let mut map: AHashMap<i32, i32> = AHashMap::with_capacity(4);
//! map.insert(12, 34);
//! map.insert(56, 78);
//! ```
#![deny(clippy::correctness, clippy::complexity, clippy::perf)]
#![allow(clippy::pedantic, clippy::cast_lossless, clippy::unreadable_literal)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(feature = "specialize", feature(min_specialization))]
#![cfg_attr(feature = "stdsimd", feature(stdsimd))]

#[macro_use]
mod convert;

#[cfg(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
))]
mod aes_hash;
mod fallback_hash;
#[cfg(test)]
mod hash_quality_test;

#[cfg(feature = "std")]
mod hash_map;
#[cfg(feature = "std")]
mod hash_set;
mod operations;
mod random_state;
mod specialize;

#[cfg(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
))]
pub use crate::aes_hash::AHasher;

#[cfg(not(any(
    all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)),
    all(any(target_arch = "arm", target_arch = "aarch64"), target_feature = "crypto", not(miri), feature = "stdsimd")
)))]
pub use crate::fallback_hash::AHasher;
pub use crate::random_state::RandomState;

pub use crate::specialize::CallHasher;

#[cfg(feature = "std")]
pub use crate::hash_map::AHashMap;
#[cfg(feature = "std")]
pub use crate::hash_set::AHashSet;
use core::hash::BuildHasher;
use core::hash::Hash;
use core::hash::Hasher;

/// Provides a default [Hasher] with fixed keys.
/// This is typically used in conjunction with [BuildHasherDefault] to create
/// [AHasher]s in order to hash the keys of the map.
///
/// Generally it is preferable to use [RandomState] instead, so that different
/// hashmaps will have different keys. However if fixed keys are desireable this
/// may be used instead.
///
/// # Example
/// ```
/// use std::hash::BuildHasherDefault;
/// use ahash::{AHasher, RandomState};
/// use std::collections::HashMap;
///
/// let mut map: HashMap<i32, i32, BuildHasherDefault<AHasher>> = HashMap::default();
/// map.insert(12, 34);
/// ```
///
/// [BuildHasherDefault]: std::hash::BuildHasherDefault
/// [Hasher]: std::hash::Hasher
/// [HashMap]: std::collections::HashMap
impl Default for AHasher {
    /// Constructs a new [AHasher] with fixed keys.
    /// If `std` is enabled these will be generated upon first invocation.
    /// Otherwise if the `compile-time-rng`feature is enabled these will be generated at compile time.
    /// If neither of these features are available, hardcoded constants will be used.
    ///
    /// Because the values are fixed, different hashers will all hash elements the same way.
    /// This could make hash values predictable, if DOS attacks are a concern. If this behaviour is
    /// not required, it may be preferable to use [RandomState] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use ahash::AHasher;
    /// use std::hash::Hasher;
    ///
    /// let mut hasher_1 = AHasher::default();
    /// let mut hasher_2 = AHasher::default();
    ///
    /// hasher_1.write_u32(1234);
    /// hasher_2.write_u32(1234);
    ///
    /// assert_eq!(hasher_1.finish(), hasher_2.finish());
    /// ```
    #[inline]
    fn default() -> AHasher {
        RandomState::with_fixed_keys().build_hasher()
    }
}

/// Used for specialization. (Sealed)
pub(crate) trait BuildHasherExt: BuildHasher {
    #[doc(hidden)]
    fn hash_as_u64<T: Hash + ?Sized>(&self, value: &T) -> u64;

    #[doc(hidden)]
    fn hash_as_fixed_length<T: Hash + ?Sized>(&self, value: &T) -> u64;

    #[doc(hidden)]
    fn hash_as_str<T: Hash + ?Sized>(&self, value: &T) -> u64;
}

impl<B: BuildHasher> BuildHasherExt for B {
    #[inline]
    #[cfg(feature = "specialize")]
    default fn hash_as_u64<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[inline]
    #[cfg(not(feature = "specialize"))]
    fn hash_as_u64<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[inline]
    #[cfg(feature = "specialize")]
    default fn hash_as_fixed_length<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[inline]
    #[cfg(not(feature = "specialize"))]
    fn hash_as_fixed_length<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[inline]
    #[cfg(feature = "specialize")]
    default fn hash_as_str<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
    #[inline]
    #[cfg(not(feature = "specialize"))]
    fn hash_as_str<T: Hash + ?Sized>(&self, value: &T) -> u64 {
        let mut hasher = self.build_hasher();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

// #[inline(never)]
// #[doc(hidden)]
// pub fn hash_test(input: &[u8]) -> u64 {
//     let a = RandomState::with_seeds(11, 22, 33, 44);
//     <[u8]>::get_hash(input, &a)
// }

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use crate::convert::Convert;
    use crate::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn test_default_builder() {
        use core::hash::BuildHasherDefault;

        let mut map = HashMap::<u32, u64, BuildHasherDefault<AHasher>>::default();
        map.insert(1, 3);
    }

    #[test]
    fn test_builder() {
        let mut map = HashMap::<u32, u64, RandomState>::default();
        map.insert(1, 3);
    }

    #[test]
    fn test_conversion() {
        let input: &[u8] = b"dddddddd";
        let bytes: u64 = as_array!(input, 8).convert();
        assert_eq!(bytes, 0x6464646464646464);
    }


    #[test]
    fn test_non_zero() {
        let mut hasher1 = AHasher::new_with_keys(0, 0);
        let mut hasher2 = AHasher::new_with_keys(0, 0);
        "foo".hash(&mut hasher1);
        "bar".hash(&mut hasher2);
        assert_ne!(hasher1.finish(), 0);
        assert_ne!(hasher2.finish(), 0);
        assert_ne!(hasher1.finish(), hasher2.finish());

        let mut hasher1 = AHasher::new_with_keys(0, 0);
        let mut hasher2 = AHasher::new_with_keys(0, 0);
        3_u64.hash(&mut hasher1);
        4_u64.hash(&mut hasher2);
        assert_ne!(hasher1.finish(), 0);
        assert_ne!(hasher2.finish(), 0);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_non_zero_specialized() {
        let hasher_build = RandomState::with_seeds(0,0,0,0);

        let h1 = str::get_hash("foo", &hasher_build);
        let h2 = str::get_hash("bar", &hasher_build);
        assert_ne!(h1, 0);
        assert_ne!(h2, 0);
        assert_ne!(h1, h2);

        let h1 = u64::get_hash(&3_u64, &hasher_build);
        let h2 = u64::get_hash(&4_u64, &hasher_build);
        assert_ne!(h1, 0);
        assert_ne!(h2, 0);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_ahasher_construction() {
        let _ = AHasher::new_with_keys(1234, 5678);
    }
}
