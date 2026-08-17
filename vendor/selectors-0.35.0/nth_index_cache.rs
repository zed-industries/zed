/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::hash::Hash;

use crate::{parser::Selector, tree::OpaqueElement, SelectorImpl};
use rustc_hash::FxHashMap;

/// A cache to speed up matching of nth-index-like selectors.
///
/// See [1] for some discussion around the design tradeoffs.
///
/// [1] https://bugzilla.mozilla.org/show_bug.cgi?id=1401855#c3
#[derive(Default)]
pub struct NthIndexCache {
    nth: NthIndexCacheInner,
    nth_of_selectors: NthIndexOfSelectorsCaches,
    nth_last: NthIndexCacheInner,
    nth_last_of_selectors: NthIndexOfSelectorsCaches,
    nth_of_type: NthIndexCacheInner,
    nth_last_of_type: NthIndexCacheInner,
}

impl NthIndexCache {
    /// Gets the appropriate cache for the given parameters.
    pub fn get<Impl: SelectorImpl>(
        &mut self,
        is_of_type: bool,
        is_from_end: bool,
        selectors: &[Selector<Impl>],
    ) -> &mut NthIndexCacheInner {
        if is_of_type {
            return if is_from_end {
                &mut self.nth_last_of_type
            } else {
                &mut self.nth_of_type
            };
        }
        if !selectors.is_empty() {
            return if is_from_end {
                self.nth_last_of_selectors.lookup(selectors)
            } else {
                self.nth_of_selectors.lookup(selectors)
            };
        }
        if is_from_end {
            &mut self.nth_last
        } else {
            &mut self.nth
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct SelectorListCacheKey(usize);

/// Use the selector list's pointer as the cache key
impl SelectorListCacheKey {
    // :nth-child of selectors are reference-counted with `ThinArc`, so we know their pointers are stable.
    fn new<Impl: SelectorImpl>(selectors: &[Selector<Impl>]) -> Self {
        Self(selectors.as_ptr() as usize)
    }
}

/// Use a different map of cached indices per :nth-child's or :nth-last-child's selector list
#[derive(Default)]
pub struct NthIndexOfSelectorsCaches(FxHashMap<SelectorListCacheKey, NthIndexCacheInner>);

/// Get or insert a map of cached incides for the selector list of this
/// particular :nth-child or :nth-last-child pseudoclass
impl NthIndexOfSelectorsCaches {
    pub fn lookup<Impl: SelectorImpl>(
        &mut self,
        selectors: &[Selector<Impl>],
    ) -> &mut NthIndexCacheInner {
        self.0
            .entry(SelectorListCacheKey::new(selectors))
            .or_default()
    }
}

/// The concrete per-pseudo-class cache.
#[derive(Default)]
pub struct NthIndexCacheInner(FxHashMap<OpaqueElement, i32>);

impl NthIndexCacheInner {
    /// Does a lookup for a given element in the cache.
    pub fn lookup(&mut self, el: OpaqueElement) -> Option<i32> {
        self.0.get(&el).copied()
    }

    /// Inserts an entry into the cache.
    pub fn insert(&mut self, element: OpaqueElement, index: i32) {
        self.0.insert(element, index);
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
