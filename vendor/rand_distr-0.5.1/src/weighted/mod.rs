// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Weighted (index) sampling
//!
//! This module is a superset of [`rand::distr::weighted`].
//!
//! Multiple implementations of weighted index sampling are provided:
//!
//! -   [`WeightedIndex`] (a re-export from [`rand`]) supports fast construction
//!     and `O(log N)` sampling over `N` weights.
//!     It also supports updating weights with `O(N)` time.
//! -   [`WeightedAliasIndex`] supports `O(1)` sampling, but due to high
//!     construction time many samples are required to outperform [`WeightedIndex`].
//! -   [`WeightedTreeIndex`] supports `O(log N)` sampling and
//!     update/insertion/removal of weights with `O(log N)` time.

mod weighted_alias;
mod weighted_tree;

pub use rand::distr::weighted::*;
pub use weighted_alias::*;
pub use weighted_tree::*;
