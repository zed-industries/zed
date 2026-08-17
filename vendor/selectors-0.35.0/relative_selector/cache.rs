/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use rustc_hash::FxHashMap;
/// Relative selector cache. This is useful for following cases.
/// First case is non-subject relative selector: Imagine `.anchor:has(<..>) ~ .foo`, with DOM
/// `.anchor + .foo + .. + .foo`. Each match on `.foo` triggers `:has()` traversal that
/// yields the same result. This is simple enough, since we just need to store
/// the exact match on that anchor pass/fail.
/// Second case is `querySelectorAll`: Imagine `querySelectorAll(':has(.a)')`, with DOM
/// `div > .. > div > .a`. When the we perform the traversal at the top div,
/// we basically end up evaluating `:has(.a)` for all anchors, which could be reused.
/// Also consider the sibling version: `querySelectorAll(':has(~ .a)')` with DOM
/// `div + .. + div + .a`.
/// TODO(dshin): Second case is not yet handled. That is tracked in Bug 1845291.
use std::hash::Hash;

use crate::parser::{RelativeSelector, SelectorKey};
use crate::{tree::OpaqueElement, SelectorImpl};

/// Match data for a given element and a selector.
#[derive(Clone, Copy)]
pub enum RelativeSelectorCachedMatch {
    /// This selector matches this element.
    Matched,
    /// This selector does not match this element.
    NotMatched,
}

impl RelativeSelectorCachedMatch {
    /// Is the cached result a match?
    pub fn matched(&self) -> bool {
        matches!(*self, Self::Matched)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Key {
    element: OpaqueElement,
    selector: SelectorKey,
}

impl Key {
    pub fn new<Impl: SelectorImpl>(
        element: OpaqueElement,
        selector: &RelativeSelector<Impl>,
    ) -> Self {
        Key {
            element,
            selector: SelectorKey::new(&selector.selector),
        }
    }
}

/// Cache to speed up matching of relative selectors.
#[derive(Default)]
pub struct RelativeSelectorCache {
    cache: FxHashMap<Key, RelativeSelectorCachedMatch>,
}

impl RelativeSelectorCache {
    /// Add a relative selector match into the cache.
    pub fn add<Impl: SelectorImpl>(
        &mut self,
        anchor: OpaqueElement,
        selector: &RelativeSelector<Impl>,
        matched: RelativeSelectorCachedMatch,
    ) {
        self.cache.insert(Key::new(anchor, selector), matched);
    }

    /// Check if we have a cache entry for the element.
    pub fn lookup<Impl: SelectorImpl>(
        &mut self,
        element: OpaqueElement,
        selector: &RelativeSelector<Impl>,
    ) -> Option<RelativeSelectorCachedMatch> {
        self.cache.get(&Key::new(element, selector)).copied()
    }
}
