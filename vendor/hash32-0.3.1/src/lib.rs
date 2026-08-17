//! 32-bit hashing algorithms
//!
//! # Why?
//!
//! Because 32-bit architectures are a thing (e.g. ARM Cortex-M) and you don't want your hashing
//! function to pull in a bunch of slow 64-bit compiler intrinsics (software implementations of
//! 64-bit operations).
//!
//! # Relationship to `core::hash`
//!
//! This crate extends [`core::hash`] with a 32-bit version of `Hasher`, which extends
//! `core::hash::Hasher`. It requires that the hasher only performs 32-bit operations when computing
//! the hash, and adds [`finish32`] to get the hasher's result as a `u32`. The standard `finish`
//! method should just zero-extend this result.
//!
//! Since it extends `core::hash::Hasher`, `Hasher` can be used with any type which implements the
//! standard `Hash` trait.
//!
//! This crate also adds a version of `BuildHasherDefault` with a const constructor, to work around
//! the `core` version's lack of one.
//!
//! [`core::hash`]: https://doc.rust-lang.org/std/hash/index.html
//! [`finish32`]: crate::Hasher::finish32
//!
//! # Hashers
//!
//! This crate provides implementations of the following 32-bit hashing algorithms:
//!
//! - [Fowler-Noll-Vo](struct.FnvHasher.html)
//! - [MurmurHash3](struct.Murmur3Hasher.html)
//!
//! # Generic code
//!
//! In generic code, the trait bound `H: core::hash::Hasher` accepts *both* 64-bit hashers like
//! `std::collections::hash_map::DefaultHasher`; and 32-bit hashers like the ones defined in this
//! crate (`hash32::FnvHasher` and `hash32::Murmur3Hasher`)
//!
//! The trait bound `H: hash32::Hasher` is *more* restrictive as it only accepts 32-bit hashers.
//!
//! The `BuildHasherDefault<H>` type implements the `core::hash::BuildHasher` trait so it can
//! construct both 32-bit and 64-bit hashers. To constrain the type to only produce 32-bit hasher
//! you can add the trait bound `H::Hasher: hash32::Hasher`
//!
//! # MSRV
//!
//! This crate is guaranteed to compile on latest stable Rust. It *might* compile on older
//! versions but that may change in any new patch release.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate byteorder;

use core::fmt;
use core::hash::BuildHasher;
use core::marker::PhantomData;

pub use fnv::Hasher as FnvHasher;
pub use murmur3::Hasher as Murmur3Hasher;

mod fnv;
mod murmur3;

/// A copy of [`core::hash::BuildHasherDefault`][0], but with a const constructor.
///
/// This will eventually be deprecated once the version in `core` becomes const-constructible
/// (presumably using `const Default`).
///
/// [0]: https://doc.rust-lang.org/core/hash/struct.BuildHasherDefault.html
pub struct BuildHasherDefault<H> {
    _marker: PhantomData<H>,
}

impl<H> Default for BuildHasherDefault<H> {
    fn default() -> Self {
        BuildHasherDefault {
            _marker: PhantomData,
        }
    }
}

impl<H> Clone for BuildHasherDefault<H> {
    fn clone(&self) -> Self {
        BuildHasherDefault::default()
    }
}

impl<H> PartialEq for BuildHasherDefault<H> {
    fn eq(&self, _other: &BuildHasherDefault<H>) -> bool {
        true
    }
}

impl<H> Eq for BuildHasherDefault<H> {}

impl<H> fmt::Debug for BuildHasherDefault<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("BuildHasherDefault")
    }
}

impl<H> BuildHasherDefault<H> {
    /// `const` constructor
    pub const fn new() -> Self {
        BuildHasherDefault {
            _marker: PhantomData,
        }
    }
}

impl<H> BuildHasher for BuildHasherDefault<H>
where
    H: Default + core::hash::Hasher,
{
    type Hasher = H;

    fn build_hasher(&self) -> Self::Hasher {
        H::default()
    }
}

/// An extension of [core::hash::Hasher][0] for hashers which use 32 bits.
///
/// For hashers which implement this trait, the standard `finish` method should just return a
/// zero-extended version of the result of `finish32`.
///
/// [0]: https://doc.rust-lang.org/core/hash/trait.Hasher.html
///
/// # Contract
///
/// Implementers of this trait must *not* perform any 64-bit (or 128-bit) operation while computing
/// the hash.
pub trait Hasher: core::hash::Hasher {
    /// The equivalent of [`core::hash::Hasher.finish`][0] for 32-bit hashers.
    ///
    /// This returns the hash directly; `finish` zero-extends it to 64 bits for compatibility.
    ///
    /// [0]: https://doc.rust-lang.org/std/hash/trait.Hasher.html#tymethod.finish
    fn finish32(&self) -> u32;
}
