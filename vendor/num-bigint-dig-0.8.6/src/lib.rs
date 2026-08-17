// Copyright 2018 Stichting Organism
//
// Copyright 2018 Friedel Ziegelmayer
//
// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Big integer (signed version: `BigInt`, unsigned version: `BigUint`).
//!
//! A `BigUint` is represented as a vector of `BigDigit`s.
//! A `BigInt` is a combination of `BigUint` and `Sign`.
//!
//! Common numerical operations are overloaded, so we can treat them
//! the same way we treat other numbers.
//!
//! ## Example
//!
//! ```rust
//! extern crate num_bigint_dig as num_bigint;
//! extern crate num_traits;
//!
//! # fn main() {
//! use num_bigint::BigUint;
//! use num_traits::{Zero, One};
//! use std::mem::replace;
//!
//! // Calculate large fibonacci numbers.
//! fn fib(n: usize) -> BigUint {
//!     let mut f0: BigUint = Zero::zero();
//!     let mut f1: BigUint = One::one();
//!     for _ in 0..n {
//!         let f2 = f0 + &f1;
//!         // This is a low cost way of swapping f0 with f1 and f1 with f2.
//!         f0 = replace(&mut f1, f2);
//!     }
//!     f0
//! }
//!
//! // This is a very large number.
//! //println!("fib(1000) = {}", fib(1000));
//! # }
//! ```
//!
//! It's easy to generate large random numbers:
//!
#![cfg_attr(feature = "std", doc = " ```")]
#![cfg_attr(not(feature = "std"), doc = " ```ignore")]
//!
//! # #[cfg(feature = "rand")]
//! extern crate rand;
//! extern crate num_bigint_dig as bigint;
//!
//! # #[cfg(feature = "rand")]
//! # fn main() {
//! use bigint::{ToBigInt, RandBigInt};
//!
//! let mut rng = rand::thread_rng();
//! let a = rng.gen_bigint(1000);
//!
//! let low = -10000.to_bigint().unwrap();
//! let high = 10000.to_bigint().unwrap();
//! let b = rng.gen_bigint_range(&low, &high);
//!
//! // Probably an even larger number.
//! //println!("{}", a * b);
//! # }
//!
//! # #[cfg(not(feature = "rand"))]
//! # fn main() {
//! # }
//! ```
//!
//! ## Compatibility
//!
//! The `num-bigint-dig` crate is tested for rustc 1.56 and greater.
//!
//! ## `no_std` compatibility
//!
//! This crate is compatible with `no_std` environments.
//!
//! Note however that it still requires the `alloc` crate, so the user should
//! ensure that they set a `global_allocator`.
//!
//! To use in no_std environment, add the crate as such in your `Cargo.toml`
//! file:
//!
//! ```toml
//! [dependencies]
//! num-bigint-dig = { version = "0.8", default-features=false }
//! ```
//!
//! Every features should be compatible with no_std environment, so feel free to
//! add features like `prime`, `i128`, etc...

#![doc(html_root_url = "https://docs.rs/num-bigint/0.2")]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate smallvec;

#[cfg(feature = "prime")]
#[macro_use]
extern crate lazy_static;

extern crate num_integer as integer;

use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

#[macro_use]
mod macros;

mod bigint;
mod biguint;

#[cfg(feature = "prime")]
pub mod prime;

pub mod algorithms;
pub mod traits;

pub use crate::traits::*;

#[cfg(feature = "rand")]
mod bigrand;

#[cfg(target_pointer_width = "32")]
type UsizePromotion = u32;
#[cfg(target_pointer_width = "64")]
type UsizePromotion = u64;

#[cfg(target_pointer_width = "32")]
type IsizePromotion = i32;
#[cfg(target_pointer_width = "64")]
type IsizePromotion = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBigIntError {
    kind: BigIntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BigIntErrorKind {
    Empty,
    InvalidDigit,
}

impl ParseBigIntError {
    fn __description(&self) -> &str {
        use crate::BigIntErrorKind::*;
        match self.kind {
            Empty => "cannot parse integer from empty string",
            InvalidDigit => "invalid digit found in string",
        }
    }

    fn empty() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::Empty,
        }
    }

    fn invalid() -> Self {
        ParseBigIntError {
            kind: BigIntErrorKind::InvalidDigit,
        }
    }
}

impl fmt::Display for ParseBigIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

#[cfg(feature = "std")]
impl Error for ParseBigIntError {
    fn description(&self) -> &str {
        self.__description()
    }
}

pub use crate::biguint::BigUint;
pub use crate::biguint::IntoBigUint;
pub use crate::biguint::ToBigUint;

pub use crate::bigint::negate_sign;
pub use crate::bigint::BigInt;
pub use crate::bigint::IntoBigInt;
pub use crate::bigint::Sign;
pub use crate::bigint::ToBigInt;

#[cfg(feature = "rand")]
pub use crate::bigrand::{RandBigInt, RandomBits, UniformBigInt, UniformBigUint};

#[cfg(feature = "prime")]
pub use bigrand::RandPrime;

#[cfg(not(feature = "u64_digit"))]
pub const VEC_SIZE: usize = 8;

#[cfg(feature = "u64_digit")]
pub const VEC_SIZE: usize = 4;

mod big_digit {
    /// A `BigDigit` is a `BigUint`'s composing element.
    #[cfg(not(feature = "u64_digit"))]
    pub type BigDigit = u32;
    #[cfg(feature = "u64_digit")]
    pub type BigDigit = u64;

    /// A `DoubleBigDigit` is the internal type used to do the computations.  Its
    /// size is the double of the size of `BigDigit`.
    #[cfg(not(feature = "u64_digit"))]
    pub type DoubleBigDigit = u64;
    #[cfg(feature = "u64_digit")]
    pub type DoubleBigDigit = u128;

    /// A `SignedDoubleBigDigit` is the signed version of `DoubleBigDigit`.
    #[cfg(not(feature = "u64_digit"))]
    pub type SignedDoubleBigDigit = i64;
    #[cfg(feature = "u64_digit")]
    pub type SignedDoubleBigDigit = i128;

    // `DoubleBigDigit` size dependent
    #[cfg(not(feature = "u64_digit"))]
    pub const BITS: usize = 32;
    #[cfg(feature = "u64_digit")]
    pub const BITS: usize = 64;

    #[cfg(not(feature = "u64_digit"))]
    const LO_MASK: DoubleBigDigit = (-1i32 as DoubleBigDigit) >> BITS;
    #[cfg(feature = "u64_digit")]
    const LO_MASK: DoubleBigDigit = (-1i64 as DoubleBigDigit) >> BITS;

    #[inline]
    fn get_hi(n: DoubleBigDigit) -> BigDigit {
        (n >> BITS) as BigDigit
    }
    #[inline]
    fn get_lo(n: DoubleBigDigit) -> BigDigit {
        (n & LO_MASK) as BigDigit
    }

    /// Split one `DoubleBigDigit` into two `BigDigit`s.
    #[inline]
    pub fn from_doublebigdigit(n: DoubleBigDigit) -> (BigDigit, BigDigit) {
        (get_hi(n), get_lo(n))
    }

    /// Join two `BigDigit`s into one `DoubleBigDigit`
    #[inline]
    pub fn to_doublebigdigit(hi: BigDigit, lo: BigDigit) -> DoubleBigDigit {
        (DoubleBigDigit::from(lo)) | ((DoubleBigDigit::from(hi)) << BITS)
    }
}
