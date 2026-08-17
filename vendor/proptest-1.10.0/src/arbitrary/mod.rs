//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Defines the `Arbitrary` trait and related free functions
//! and type aliases.
//!
//! See the [`Arbitrary`] trait for more information.
//!
//! [`Arbitrary`]: trait.Arbitrary.html

use crate::strategy::statics;
use crate::strategy::{Map, Strategy};

//==============================================================================
// Trait and impls
//==============================================================================

mod traits;

#[macro_use]
pub mod functor;

#[macro_use]
mod macros;

mod arrays;
mod primitives;
mod sample;
mod tuples;

mod _core;

#[cfg(any(feature = "std", feature = "alloc"))]
mod _alloc;

#[cfg(feature = "std")]
mod _std;

pub use self::traits::*;

//==============================================================================
// SMapped + Mapped aliases to make documentation clearer.
//==============================================================================

pub(crate) type SFnPtrMap<S, O> =
    statics::Map<S, fn(<S as Strategy>::Value) -> O>;

/// A static map from a strategy of `I` to `O`.
///
/// # Stability
///
/// This is provided to make documentation more readable.
/// Do not rely on it existing in your own code.
pub type SMapped<I, O> = statics::Map<StrategyFor<I>, fn(I) -> O>;

/// A normal map from a strategy of `I` to `O`.
///
/// # Stability
///
/// This is provided to make documentation more readable.
/// Do not rely on it existing in your own code.
pub type Mapped<I, O> = Map<StrategyFor<I>, fn(I) -> O>;
