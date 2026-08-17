/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

/// Bloom filter for relative selectors.
use rustc_hash::FxHashMap;

use crate::bloom::BloomFilter;
use crate::context::QuirksMode;
use crate::parser::{collect_selector_hashes, RelativeSelector, RelativeSelectorMatchHint};
use crate::tree::{Element, OpaqueElement};
use crate::SelectorImpl;

enum Entry {
    /// Filter lookup happened once. Construction of the filter is expensive,
    /// so this is set when the element for subtree traversal is encountered.
    Lookup,
    /// Filter lookup happened more than once, and the filter for this element's
    /// subtree traversal is constructed. Could use special handlings for pseudo-classes
    /// such as `:hover` and `:focus`, see Bug 1845503.
    HasFilter(Box<BloomFilter>),
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum TraversalKind {
    Children,
    Descendants,
}

fn add_to_filter<E: Element>(element: &E, filter: &mut BloomFilter, kind: TraversalKind) -> bool {
    let mut child = element.first_element_child();
    while let Some(e) = child {
        if !e.add_element_unique_hashes(filter) {
            return false;
        }
        if kind == TraversalKind::Descendants {
            if !add_to_filter(&e, filter, kind) {
                return false;
            }
        }
        child = e.next_sibling_element();
    }
    true
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Key(OpaqueElement, TraversalKind);

/// Map of bloom filters for fast-rejecting relative selectors.
#[derive(Default)]
pub struct RelativeSelectorFilterMap {
    map: FxHashMap<Key, Entry>,
}

fn fast_reject<Impl: SelectorImpl>(
    selector: &RelativeSelector<Impl>,
    quirks_mode: QuirksMode,
    filter: &BloomFilter,
) -> bool {
    let mut hashes = [0u32; 4];
    let mut len = 0;
    // For inner selectors, we only collect from the single rightmost compound.
    // This is because inner selectors can cause breakouts: e.g. `.anchor:has(:is(.a .b) .c)`
    // can match when `.a` is the ancestor of `.anchor`. Including `.a` would possibly fast
    // reject the subtree for not having `.a`, even if the selector would match.
    // Technically, if the selector's traversal is non-sibling subtree, we can traverse
    // inner selectors up to the point where a descendant/child combinator is encountered
    // (e.g. In `.anchor:has(:is(.a ~ .b) .c)`, `.a` is guaranteed to be the a descendant
    // of `.anchor`). While that can be separately handled, well, this is simpler.
    collect_selector_hashes(
        selector.selector.iter(),
        quirks_mode,
        &mut hashes,
        &mut len,
        |s| s.iter(),
    );
    for i in 0..len {
        if !filter.might_contain_hash(hashes[i]) {
            // Definitely rejected.
            return true;
        }
    }
    false
}

impl RelativeSelectorFilterMap {
    fn get_filter<E: Element>(&mut self, element: &E, kind: TraversalKind) -> Option<&BloomFilter> {
        // Insert flag to indicate that we looked up the filter once, and
        // create the filter if and only if that flag is there.
        let key = Key(element.opaque(), kind);
        let entry = self
            .map
            .entry(key)
            .and_modify(|entry| {
                if !matches!(entry, Entry::Lookup) {
                    return;
                }
                let mut filter = BloomFilter::new();
                // Go through all children/descendants of this element and add their hashes.
                if add_to_filter(element, &mut filter, kind) {
                    *entry = Entry::HasFilter(Box::new(filter));
                }
            })
            .or_insert(Entry::Lookup);
        match entry {
            Entry::Lookup => None,
            Entry::HasFilter(ref filter) => Some(filter.as_ref()),
        }
    }

    /// Potentially reject the given selector for this element.
    /// This may seem redundant in presence of the cache, but the cache keys into the
    /// selector-element pair specifically, while this filter keys to the element
    /// and the traversal kind, so it is useful for handling multiple selectors
    /// that effectively end up looking at the same(-ish, for siblings) subtree.
    pub fn fast_reject<Impl: SelectorImpl, E: Element>(
        &mut self,
        element: &E,
        selector: &RelativeSelector<Impl>,
        quirks_mode: QuirksMode,
    ) -> bool {
        if matches!(
            selector.match_hint,
            RelativeSelectorMatchHint::InNextSibling
        ) {
            // Don't bother.
            return false;
        }
        let is_sibling = matches!(
            selector.match_hint,
            RelativeSelectorMatchHint::InSibling
                | RelativeSelectorMatchHint::InNextSiblingSubtree
                | RelativeSelectorMatchHint::InSiblingSubtree
        );
        let is_subtree = matches!(
            selector.match_hint,
            RelativeSelectorMatchHint::InSubtree
                | RelativeSelectorMatchHint::InNextSiblingSubtree
                | RelativeSelectorMatchHint::InSiblingSubtree
        );
        let kind = if is_subtree {
            TraversalKind::Descendants
        } else {
            TraversalKind::Children
        };
        if is_sibling {
            // Contain the entirety of the parent's children/subtree in the filter, and use that.
            // This is less likely to reject, especially for sibling subtree matches; however, it's less
            // expensive memory-wise, compared to storing filters for each sibling.
            element.parent_element().map_or(false, |parent| {
                self.get_filter(&parent, kind)
                    .map_or(false, |filter| fast_reject(selector, quirks_mode, filter))
            })
        } else {
            self.get_filter(element, kind)
                .map_or(false, |filter| fast_reject(selector, quirks_mode, filter))
        }
    }
}
