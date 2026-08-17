/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::attr::{
    AttrSelectorOperation, AttrSelectorWithOptionalNamespace, CaseSensitivity, NamespaceConstraint,
    ParsedAttrSelectorOperation, ParsedCaseSensitivity,
};
use crate::bloom::{BloomFilter, BLOOM_HASH_MASK};
use crate::kleene_value::KleeneValue;
use crate::parser::{
    AncestorHashes, Combinator, Component, LocalName, MatchesFeaturelessHost, NthSelectorData,
    RelativeSelectorMatchHint,
};
use crate::parser::{
    NonTSPseudoClass, RelativeSelector, Selector, SelectorImpl, SelectorIter, SelectorList,
};
use crate::relative_selector::cache::RelativeSelectorCachedMatch;
use crate::tree::Element;
use bitflags::bitflags;
use debug_unreachable::debug_unreachable;
use log::debug;
use smallvec::SmallVec;
use std::borrow::Borrow;

pub use crate::context::*;

// The bloom filter for descendant CSS selectors will have a <1% false
// positive rate until it has this many selectors in it, then it will
// rapidly increase.
pub static RECOMMENDED_SELECTOR_BLOOM_FILTER_SIZE: usize = 4096;

bitflags! {
    /// Set of flags that are set on either the element or its parent (depending
    /// on the flag) if the element could potentially match a selector.
    #[derive(Clone, Copy)]
    pub struct ElementSelectorFlags: usize {
        /// When a child is added or removed from the parent, all the children
        /// must be restyled, because they may match :nth-last-child,
        /// :last-of-type, :nth-last-of-type, or :only-of-type.
        const HAS_SLOW_SELECTOR = 1 << 0;

        /// When a child is added or removed from the parent, any later
        /// children must be restyled, because they may match :nth-child,
        /// :first-of-type, or :nth-of-type.
        const HAS_SLOW_SELECTOR_LATER_SIBLINGS = 1 << 1;

        /// HAS_SLOW_SELECTOR* was set by the presence of :nth (But not of).
        const HAS_SLOW_SELECTOR_NTH = 1 << 2;

        /// When a DOM mutation occurs on a child that might be matched by
        /// :nth-last-child(.. of <selector list>), earlier children must be
        /// restyled, and HAS_SLOW_SELECTOR will be set (which normally
        /// indicates that all children will be restyled).
        ///
        /// Similarly, when a DOM mutation occurs on a child that might be
        /// matched by :nth-child(.. of <selector list>), later children must be
        /// restyled, and HAS_SLOW_SELECTOR_LATER_SIBLINGS will be set.
        const HAS_SLOW_SELECTOR_NTH_OF = 1 << 3;

        /// When a child is added or removed from the parent, the first and
        /// last children must be restyled, because they may match :first-child,
        /// :last-child, or :only-child.
        const HAS_EDGE_CHILD_SELECTOR = 1 << 4;

        /// The element has an empty selector, so when a child is appended we
        /// might need to restyle the parent completely.
        const HAS_EMPTY_SELECTOR = 1 << 5;

        /// The element may anchor a relative selector.
        const ANCHORS_RELATIVE_SELECTOR = 1 << 6;

        /// The element may anchor a relative selector that is not the subject
        /// of the whole selector.
        const ANCHORS_RELATIVE_SELECTOR_NON_SUBJECT = 1 << 7;

        /// The element is reached by a relative selector search in the sibling direction.
        const RELATIVE_SELECTOR_SEARCH_DIRECTION_SIBLING = 1 << 8;

        /// The element is reached by a relative selector search in the ancestor direction.
        const RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR = 1 << 9;

        // The element is reached by a relative selector search in both sibling and ancestor directions.
        const RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR_SIBLING =
            Self::RELATIVE_SELECTOR_SEARCH_DIRECTION_SIBLING.bits() |
            Self::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR.bits();
    }
}

impl ElementSelectorFlags {
    /// Returns the subset of flags that apply to the element.
    pub fn for_self(self) -> ElementSelectorFlags {
        self & (ElementSelectorFlags::HAS_EMPTY_SELECTOR
            | ElementSelectorFlags::ANCHORS_RELATIVE_SELECTOR
            | ElementSelectorFlags::ANCHORS_RELATIVE_SELECTOR_NON_SUBJECT
            | ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_SIBLING
            | ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR)
    }

    /// Returns the subset of flags that apply to the parent.
    pub fn for_parent(self) -> ElementSelectorFlags {
        self & (ElementSelectorFlags::HAS_SLOW_SELECTOR
            | ElementSelectorFlags::HAS_SLOW_SELECTOR_LATER_SIBLINGS
            | ElementSelectorFlags::HAS_SLOW_SELECTOR_NTH
            | ElementSelectorFlags::HAS_SLOW_SELECTOR_NTH_OF
            | ElementSelectorFlags::HAS_EDGE_CHILD_SELECTOR)
    }
}

/// Holds per-compound-selector data.
struct LocalMatchingContext<'a, 'b: 'a, Impl: SelectorImpl> {
    shared: &'a mut MatchingContext<'b, Impl>,
    rightmost: SubjectOrPseudoElement,
    quirks_data: Option<SelectorIter<'a, Impl>>,
}

#[inline(always)]
pub fn matches_selector_list<E>(
    selector_list: &SelectorList<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
) -> bool
where
    E: Element,
{
    // This is pretty much any(..) but manually inlined because the compiler
    // refuses to do so from querySelector / querySelectorAll.
    for selector in selector_list.slice() {
        let matches = matches_selector(selector, 0, None, element, context);
        if matches {
            return true;
        }
    }

    false
}

/// Given the ancestor hashes from a selector, see if the current element,
/// represented by the bloom filter, has a chance of matching at all.
#[inline(always)]
pub fn selector_may_match(hashes: &AncestorHashes, bf: &BloomFilter) -> bool {
    // Check the first three hashes. Note that we can check for zero before
    // masking off the high bits, since if any of the first three hashes is
    // zero the fourth will be as well. We also take care to avoid the
    // special-case complexity of the fourth hash until we actually reach it,
    // because we usually don't.
    //
    // To be clear: this is all extremely hot.
    for i in 0..3 {
        let packed = hashes.packed_hashes[i];
        if packed == 0 {
            // No more hashes left - unable to fast-reject.
            return true;
        }

        if !bf.might_contain_hash(packed & BLOOM_HASH_MASK) {
            // Hooray! We fast-rejected on this hash.
            return false;
        }
    }

    // Now do the slighty-more-complex work of synthesizing the fourth hash,
    // and check it against the filter if it exists.
    let fourth = hashes.fourth_hash();
    fourth == 0 || bf.might_contain_hash(fourth)
}

/// A result of selector matching, includes 3 failure types,
///
///   NotMatchedAndRestartFromClosestLaterSibling
///   NotMatchedAndRestartFromClosestDescendant
///   NotMatchedGlobally
///
/// When NotMatchedGlobally appears, stop selector matching completely since
/// the succeeding selectors never matches.
/// It is raised when
///   Child combinator cannot find the candidate element.
///   Descendant combinator cannot find the candidate element.
///
/// When NotMatchedAndRestartFromClosestDescendant appears, the selector
/// matching does backtracking and restarts from the closest Descendant
/// combinator.
/// It is raised when
///   NextSibling combinator cannot find the candidate element.
///   LaterSibling combinator cannot find the candidate element.
///   Child combinator doesn't match on the found element.
///
/// When NotMatchedAndRestartFromClosestLaterSibling appears, the selector
/// matching does backtracking and restarts from the closest LaterSibling
/// combinator.
/// It is raised when
///   NextSibling combinator doesn't match on the found element.
///
/// For example, when the selector "d1 d2 a" is provided and we cannot *find*
/// an appropriate ancestor element for "d1", this selector matching raises
/// NotMatchedGlobally since even if "d2" is moved to more upper element, the
/// candidates for "d1" becomes less than before and d1 .
///
/// The next example is siblings. When the selector "b1 + b2 ~ d1 a" is
/// provided and we cannot *find* an appropriate brother element for b1,
/// the selector matching raises NotMatchedAndRestartFromClosestDescendant.
/// The selectors ("b1 + b2 ~") doesn't match and matching restart from "d1".
///
/// The additional example is child and sibling. When the selector
/// "b1 + c1 > b2 ~ d1 a" is provided and the selector "b1" doesn't match on
/// the element, this "b1" raises NotMatchedAndRestartFromClosestLaterSibling.
/// However since the selector "c1" raises
/// NotMatchedAndRestartFromClosestDescendant. So the selector
/// "b1 + c1 > b2 ~ " doesn't match and restart matching from "d1".
///
/// There is also the unknown result, which is used during invalidation when
/// specific selector is being tested for before/after comparison. More specifically,
/// selectors that are too expensive to correctly compute during invalidation may
/// return unknown, as the computation will be thrown away and only to be recomputed
/// during styling. For most cases, the unknown result can be treated as matching.
/// This is because a compound of selectors acts like &&, and unknown && matched
/// == matched and unknown && not-matched == not-matched. However, some selectors,
/// like `:is()`, behave like || i.e. `:is(.a, .b)` == a || b. Treating unknown
/// == matching then causes these selectors to always return matching, which undesired
/// for before/after comparison. Coercing to not-matched doesn't work since each
/// inner selector may have compounds: e.g. Toggling `.foo` in `:is(.foo:has(..))`
/// with coersion to not-matched would result in an invalid before/after comparison
/// of not-matched/not-matched.
#[derive(Clone, Copy, Eq, PartialEq)]
enum SelectorMatchingResult {
    Matched,
    NotMatchedAndRestartFromClosestLaterSibling,
    NotMatchedAndRestartFromClosestDescendant,
    NotMatchedGlobally,
    Unknown,
}

impl From<SelectorMatchingResult> for KleeneValue {
    #[inline]
    fn from(value: SelectorMatchingResult) -> Self {
        match value {
            SelectorMatchingResult::Matched => KleeneValue::True,
            SelectorMatchingResult::Unknown => KleeneValue::Unknown,
            SelectorMatchingResult::NotMatchedAndRestartFromClosestLaterSibling
            | SelectorMatchingResult::NotMatchedAndRestartFromClosestDescendant
            | SelectorMatchingResult::NotMatchedGlobally => KleeneValue::False,
        }
    }
}

/// Matches a selector, fast-rejecting against a bloom filter.
///
/// We accept an offset to allow consumers to represent and match against
/// partial selectors (indexed from the right). We use this API design, rather
/// than having the callers pass a SelectorIter, because creating a SelectorIter
/// requires dereferencing the selector to get the length, which adds an
/// unnecessary cache miss for cases when we can fast-reject with AncestorHashes
/// (which the caller can store inline with the selector pointer).
#[inline(always)]
pub fn matches_selector<E>(
    selector: &Selector<E::Impl>,
    offset: usize,
    hashes: Option<&AncestorHashes>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
) -> bool
where
    E: Element,
{
    let result = matches_selector_kleene(selector, offset, hashes, element, context);
    if cfg!(debug_assertions) && result == KleeneValue::Unknown {
        debug_assert!(
            context
                .matching_for_invalidation_comparison()
                .unwrap_or(false),
            "How did we return unknown?"
        );
    }
    result.to_bool(true)
}

/// Same as matches_selector, but returns the Kleene value as-is.
#[inline(always)]
pub fn matches_selector_kleene<E>(
    selector: &Selector<E::Impl>,
    offset: usize,
    hashes: Option<&AncestorHashes>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
) -> KleeneValue
where
    E: Element,
{
    // Use the bloom filter to fast-reject.
    if let Some(hashes) = hashes {
        if let Some(filter) = context.bloom_filter {
            if !selector_may_match(hashes, filter) {
                return KleeneValue::False;
            }
        }
    }
    matches_complex_selector(
        selector.iter_from(offset),
        element,
        context,
        if selector.is_rightmost(offset) {
            SubjectOrPseudoElement::Yes
        } else {
            SubjectOrPseudoElement::No
        },
    )
}

/// Whether a compound selector matched, and whether it was the rightmost
/// selector inside the complex selector.
pub enum CompoundSelectorMatchingResult {
    /// The selector was fully matched.
    FullyMatched,
    /// The compound selector matched, and the next combinator offset is
    /// `next_combinator_offset`.
    Matched { next_combinator_offset: usize },
    /// The selector didn't match.
    NotMatched,
}

fn complex_selector_early_reject_by_local_name<E: Element>(
    list: &SelectorList<E::Impl>,
    element: &E,
) -> bool {
    list.slice()
        .iter()
        .all(|s| early_reject_by_local_name(s, 0, element))
}

/// Returns true if this compound would not match the given element by due
/// to a local name selector (If one exists).
pub fn early_reject_by_local_name<E: Element>(
    selector: &Selector<E::Impl>,
    from_offset: usize,
    element: &E,
) -> bool {
    let iter = selector.iter_from(from_offset);
    for component in iter {
        if match component {
            Component::LocalName(name) => !matches_local_name(element, name),
            Component::Is(list) | Component::Where(list) => {
                complex_selector_early_reject_by_local_name(list, element)
            },
            _ => continue,
        } {
            return true;
        }
    }
    false
}

/// Matches a compound selector belonging to `selector`, starting at offset
/// `from_offset`, matching left to right.
///
/// Requires that `from_offset` points to a `Combinator`.
///
/// NOTE(emilio): This doesn't allow to match in the leftmost sequence of the
/// complex selector, but it happens to be the case we don't need it.
pub fn matches_compound_selector_from<E>(
    selector: &Selector<E::Impl>,
    mut from_offset: usize,
    context: &mut MatchingContext<E::Impl>,
    element: &E,
) -> CompoundSelectorMatchingResult
where
    E: Element,
{
    debug_assert!(
        !context
            .matching_for_invalidation_comparison()
            .unwrap_or(false),
        "CompoundSelectorMatchingResult doesn't support unknown"
    );
    if cfg!(debug_assertions) && from_offset != 0 {
        selector.combinator_at_parse_order(from_offset - 1); // This asserts.
    }

    let mut local_context = LocalMatchingContext {
        shared: context,
        // We have no info if this is an outer selector. This function is called in
        // an invalidation context, which only calls this for non-subject (i.e.
        // Non-rightmost) positions.
        rightmost: SubjectOrPseudoElement::No,
        quirks_data: None,
    };

    // Find the end of the selector or the next combinator, then match
    // backwards, so that we match in the same order as
    // matches_complex_selector, which is usually faster.
    let start_offset = from_offset;
    for component in selector.iter_raw_parse_order_from(from_offset) {
        if matches!(*component, Component::Combinator(..)) {
            debug_assert_ne!(from_offset, 0, "Selector started with a combinator?");
            break;
        }

        from_offset += 1;
    }

    debug_assert!(from_offset >= 1);
    debug_assert!(from_offset <= selector.len());

    let iter = selector.iter_from(selector.len() - from_offset);
    debug_assert!(
        iter.clone().next().is_some() || from_offset != selector.len(),
        "Got the math wrong: {:?} | {:?} | {} {}",
        selector,
        selector.iter_raw_match_order().as_slice(),
        from_offset,
        start_offset
    );

    debug_assert!(
        !local_context.shared.featureless(),
        "Invalidating featureless element somehow?"
    );

    for component in iter {
        let result = matches_simple_selector(component, element, &mut local_context);
        debug_assert!(
            result != KleeneValue::Unknown,
            "Returned unknown in non invalidation context?"
        );
        if !result.to_bool(true) {
            return CompoundSelectorMatchingResult::NotMatched;
        }
    }

    if from_offset != selector.len() {
        return CompoundSelectorMatchingResult::Matched {
            next_combinator_offset: from_offset,
        };
    }

    CompoundSelectorMatchingResult::FullyMatched
}

/// Matches a complex selector.
#[inline(always)]
fn matches_complex_selector<E>(
    mut iter: SelectorIter<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue
where
    E: Element,
{
    // If this is the special pseudo-element mode, consume the ::pseudo-element
    // before proceeding, since the caller has already handled that part.
    if context.matching_mode() == MatchingMode::ForStatelessPseudoElement && !context.is_nested() {
        // Consume the pseudo.
        match *iter.next().unwrap() {
            Component::PseudoElement(ref pseudo) => {
                if let Some(ref f) = context.pseudo_element_matching_fn {
                    if !f(pseudo) {
                        return KleeneValue::False;
                    }
                }
            },
            ref other => {
                debug_assert!(
                    false,
                    "Used MatchingMode::ForStatelessPseudoElement \
                     in a non-pseudo selector {:?}",
                    other
                );
                return KleeneValue::False;
            },
        }

        if !iter.matches_for_stateless_pseudo_element() {
            return KleeneValue::False;
        }

        // Advance to the non-pseudo-element part of the selector.
        let next_sequence = iter.next_sequence().unwrap();
        debug_assert_eq!(next_sequence, Combinator::PseudoElement);
    }

    matches_complex_selector_internal(
        iter,
        element,
        context,
        rightmost,
        SubjectOrPseudoElement::Yes,
    )
    .into()
}

/// Matches each selector of a list as a complex selector
fn matches_complex_selector_list<E: Element>(
    list: &[Selector<E::Impl>],
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue {
    KleeneValue::any(list.iter(), |selector| {
        matches_complex_selector(selector.iter(), element, context, rightmost)
    })
}

fn matches_relative_selector<E: Element>(
    relative_selector: &RelativeSelector<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> bool {
    // Overall, we want to mark the path that we've traversed so that when an element
    // is invalidated, we early-reject unnecessary relative selector invalidations.
    if relative_selector.match_hint.is_descendant_direction() {
        if context.needs_selector_flags() {
            element.apply_selector_flags(
                ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR,
            );
        }
        let mut next_element = element.first_element_child();
        while let Some(el) = next_element {
            if context.needs_selector_flags() {
                el.apply_selector_flags(
                    ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR,
                );
            }
            let mut matched = matches_complex_selector(
                relative_selector.selector.iter(),
                &el,
                context,
                rightmost,
            )
            .to_bool(true);
            if !matched && relative_selector.match_hint.is_subtree() {
                matched = matches_relative_selector_subtree(
                    &relative_selector.selector,
                    &el,
                    context,
                    rightmost,
                );
            }
            if matched {
                return true;
            }
            next_element = el.next_sibling_element();
        }
    } else {
        debug_assert!(
            matches!(
                relative_selector.match_hint,
                RelativeSelectorMatchHint::InNextSibling
                    | RelativeSelectorMatchHint::InNextSiblingSubtree
                    | RelativeSelectorMatchHint::InSibling
                    | RelativeSelectorMatchHint::InSiblingSubtree
            ),
            "Not descendant direction, but also not sibling direction?"
        );
        if context.needs_selector_flags() {
            element.apply_selector_flags(
                ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_SIBLING,
            );
        }
        let sibling_flag = if relative_selector.match_hint.is_subtree() {
            ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR_SIBLING
        } else {
            ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_SIBLING
        };
        let mut next_element = element.next_sibling_element();
        while let Some(el) = next_element {
            if context.needs_selector_flags() {
                el.apply_selector_flags(sibling_flag);
            }
            let matched = if relative_selector.match_hint.is_subtree() {
                matches_relative_selector_subtree(
                    &relative_selector.selector,
                    &el,
                    context,
                    rightmost,
                )
            } else {
                matches_complex_selector(relative_selector.selector.iter(), &el, context, rightmost)
                    .to_bool(true)
            };
            if matched {
                return true;
            }
            if relative_selector.match_hint.is_next_sibling() {
                break;
            }
            next_element = el.next_sibling_element();
        }
    }
    return false;
}

fn relative_selector_match_early<E: Element>(
    selector: &RelativeSelector<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
) -> Option<bool> {
    // See if we can return a cached result.
    if let Some(cached) = context
        .selector_caches
        .relative_selector
        .lookup(element.opaque(), selector)
    {
        return Some(cached.matched());
    }
    // See if we can fast-reject.
    if context
        .selector_caches
        .relative_selector_filter_map
        .fast_reject(element, selector, context.quirks_mode())
    {
        // Alright, add as unmatched to cache.
        context.selector_caches.relative_selector.add(
            element.opaque(),
            selector,
            RelativeSelectorCachedMatch::NotMatched,
        );
        return Some(false);
    }
    None
}

fn match_relative_selectors<E: Element>(
    selectors: &[RelativeSelector<E::Impl>],
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue {
    if context.relative_selector_anchor().is_some() {
        // FIXME(emilio): This currently can happen with nesting, and it's not fully
        // correct, arguably. But the ideal solution isn't super-clear either. For now,
        // cope with it and explicitly reject it at match time. See [1] for discussion.
        //
        // [1]: https://github.com/w3c/csswg-drafts/issues/9600
        return KleeneValue::False;
    }
    if let Some(may_return_unknown) = context.matching_for_invalidation_comparison() {
        // In the context of invalidation, :has is expensive, especially because we
        // can't use caching/filtering due to now/then matches. DOM structure also
        // may have changed.
        return if may_return_unknown {
            KleeneValue::Unknown
        } else {
            KleeneValue::from(!context.in_negation())
        };
    }
    context
        .nest_for_relative_selector(element.opaque(), |context| {
            do_match_relative_selectors(selectors, element, context, rightmost)
        })
        .into()
}

/// Matches a relative selector in a list of relative selectors.
fn do_match_relative_selectors<E: Element>(
    selectors: &[RelativeSelector<E::Impl>],
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> bool {
    // Due to style sharing implications (See style sharing code), we mark the current styling context
    // to mark elements considered for :has matching. Additionally, we want to mark the elements themselves,
    // since we don't want to indiscriminately invalidate every element as a potential anchor.
    if rightmost == SubjectOrPseudoElement::Yes {
        if context.needs_selector_flags() {
            element.apply_selector_flags(ElementSelectorFlags::ANCHORS_RELATIVE_SELECTOR);
        }
    } else {
        if context.needs_selector_flags() {
            element
                .apply_selector_flags(ElementSelectorFlags::ANCHORS_RELATIVE_SELECTOR_NON_SUBJECT);
        }
    }

    for relative_selector in selectors.iter() {
        if let Some(result) = relative_selector_match_early(relative_selector, element, context) {
            if result {
                return true;
            }
            // Early return indicates no match, continue to next selector.
            continue;
        }

        let matched = matches_relative_selector(relative_selector, element, context, rightmost);
        context.selector_caches.relative_selector.add(
            element.opaque(),
            relative_selector,
            if matched {
                RelativeSelectorCachedMatch::Matched
            } else {
                RelativeSelectorCachedMatch::NotMatched
            },
        );
        if matched {
            return true;
        }
    }

    false
}

fn matches_relative_selector_subtree<E: Element>(
    selector: &Selector<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> bool {
    let mut current = element.first_element_child();

    while let Some(el) = current {
        if context.needs_selector_flags() {
            el.apply_selector_flags(
                ElementSelectorFlags::RELATIVE_SELECTOR_SEARCH_DIRECTION_ANCESTOR,
            );
        }
        if matches_complex_selector(selector.iter(), &el, context, rightmost).to_bool(true) {
            return true;
        }

        if matches_relative_selector_subtree(selector, &el, context, rightmost) {
            return true;
        }

        current = el.next_sibling_element();
    }

    false
}

/// Whether the :hover and :active quirk applies.
///
/// https://quirks.spec.whatwg.org/#the-active-and-hover-quirk
fn hover_and_active_quirk_applies<Impl: SelectorImpl>(
    selector_iter: &SelectorIter<Impl>,
    context: &MatchingContext<Impl>,
    rightmost: SubjectOrPseudoElement,
) -> bool {
    debug_assert_eq!(context.quirks_mode(), QuirksMode::Quirks);

    if context.is_nested() {
        return false;
    }

    // This compound selector had a pseudo-element to the right that we
    // intentionally skipped.
    if rightmost == SubjectOrPseudoElement::Yes
        && context.matching_mode() == MatchingMode::ForStatelessPseudoElement
    {
        return false;
    }

    selector_iter.clone().all(|simple| match *simple {
        Component::NonTSPseudoClass(ref pseudo_class) => pseudo_class.is_active_or_hover(),
        _ => false,
    })
}

#[derive(Clone, Copy, PartialEq)]
enum SubjectOrPseudoElement {
    Yes,
    No,
}

fn host_for_part<E>(element: &E, context: &MatchingContext<E::Impl>) -> Option<E>
where
    E: Element,
{
    let scope = context.current_host;
    let mut curr = element.containing_shadow_host()?;
    if scope == Some(curr.opaque()) {
        return Some(curr);
    }
    loop {
        let parent = curr.containing_shadow_host();
        if parent.as_ref().map(|h| h.opaque()) == scope {
            return Some(curr);
        }
        curr = parent?;
    }
}

fn assigned_slot<E>(element: &E, context: &MatchingContext<E::Impl>) -> Option<E>
where
    E: Element,
{
    debug_assert!(element
        .assigned_slot()
        .map_or(true, |s| s.is_html_slot_element()));
    let scope = context.current_host?;
    let mut current_slot = element.assigned_slot()?;
    while current_slot.containing_shadow_host().unwrap().opaque() != scope {
        current_slot = current_slot.assigned_slot()?;
    }
    Some(current_slot)
}

struct NextElement<E> {
    next_element: Option<E>,
    featureless: bool,
}

impl<E> NextElement<E> {
    #[inline(always)]
    fn new(next_element: Option<E>, featureless: bool) -> Self {
        Self {
            next_element,
            featureless,
        }
    }
}

#[inline(always)]
fn next_element_for_combinator<E>(
    element: &E,
    combinator: Combinator,
    context: &MatchingContext<E::Impl>,
) -> NextElement<E>
where
    E: Element,
{
    match combinator {
        Combinator::NextSibling | Combinator::LaterSibling => {
            NextElement::new(element.prev_sibling_element(), false)
        },
        Combinator::Child | Combinator::Descendant => {
            if let Some(parent) = element.parent_element() {
                return NextElement::new(Some(parent), false);
            }

            let element = if element.parent_node_is_shadow_root() {
                element.containing_shadow_host()
            } else {
                None
            };
            NextElement::new(element, true)
        },
        Combinator::Part => NextElement::new(host_for_part(element, context), false),
        Combinator::SlotAssignment => NextElement::new(assigned_slot(element, context), false),
        Combinator::PseudoElement => {
            NextElement::new(element.pseudo_element_originating_element(), false)
        },
    }
}

fn matches_complex_selector_internal<E>(
    mut selector_iter: SelectorIter<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    mut rightmost: SubjectOrPseudoElement,
    mut first_subject_compound: SubjectOrPseudoElement,
) -> SelectorMatchingResult
where
    E: Element,
{
    debug!(
        "Matching complex selector {:?} for {:?}",
        selector_iter, element
    );

    let matches_compound_selector =
        matches_compound_selector(&mut selector_iter, element, context, rightmost);

    let Some(combinator) = selector_iter.next_sequence() else {
        return match matches_compound_selector {
            KleeneValue::True => SelectorMatchingResult::Matched,
            KleeneValue::Unknown => SelectorMatchingResult::Unknown,
            KleeneValue::False => {
                SelectorMatchingResult::NotMatchedAndRestartFromClosestLaterSibling
            },
        };
    };

    let is_pseudo_combinator = combinator.is_pseudo_element();
    if context.featureless() && !is_pseudo_combinator {
        // A featureless element shouldn't match any further combinator.
        // TODO(emilio): Maybe we could avoid the compound matching more eagerly.
        return SelectorMatchingResult::NotMatchedGlobally;
    }

    let is_sibling_combinator = combinator.is_sibling();
    if is_sibling_combinator && context.needs_selector_flags() {
        // We need the flags even if we don't match.
        element.apply_selector_flags(ElementSelectorFlags::HAS_SLOW_SELECTOR_LATER_SIBLINGS);
    }

    if matches_compound_selector == KleeneValue::False {
        // We don't short circuit unknown here, since the rest of the selector
        // to the left of this compound may still return false.
        return SelectorMatchingResult::NotMatchedAndRestartFromClosestLaterSibling;
    }

    if !is_pseudo_combinator {
        rightmost = SubjectOrPseudoElement::No;
        first_subject_compound = SubjectOrPseudoElement::No;
    }

    // Stop matching :visited as soon as we find a link, or a combinator for
    // something that isn't an ancestor.
    let mut visited_handling = if is_sibling_combinator {
        VisitedHandlingMode::AllLinksUnvisited
    } else {
        context.visited_handling()
    };

    let candidate_not_found = if is_sibling_combinator {
        SelectorMatchingResult::NotMatchedAndRestartFromClosestDescendant
    } else {
        SelectorMatchingResult::NotMatchedGlobally
    };

    let mut element = element.clone();
    loop {
        if element.is_link() {
            visited_handling = VisitedHandlingMode::AllLinksUnvisited;
        }

        let NextElement {
            next_element,
            featureless,
        } = next_element_for_combinator(&element, combinator, &context);
        element = match next_element {
            None => return candidate_not_found,
            Some(e) => e,
        };

        let result = context.with_visited_handling_mode(visited_handling, |context| {
            context.with_featureless(featureless, |context| {
                matches_complex_selector_internal(
                    selector_iter.clone(),
                    &element,
                    context,
                    rightmost,
                    first_subject_compound,
                )
            })
        });

        // Return the status immediately if it is one of the global states.
        match result {
            SelectorMatchingResult::Matched => {
                debug_assert!(
                    matches_compound_selector.to_bool(true),
                    "Compound didn't match?"
                );
                if !matches_compound_selector.to_bool(false) {
                    return SelectorMatchingResult::Unknown;
                }
                return result;
            },
            SelectorMatchingResult::Unknown | SelectorMatchingResult::NotMatchedGlobally => {
                return result
            },
            _ => {},
        }

        match combinator {
            Combinator::Descendant => {
                // The Descendant combinator and the status is
                // NotMatchedAndRestartFromClosestLaterSibling or
                // NotMatchedAndRestartFromClosestDescendant, or the LaterSibling combinator and
                // the status is NotMatchedAndRestartFromClosestDescendant, we can continue to
                // matching on the next candidate element.
            },
            Combinator::Child => {
                // Upgrade the failure status to NotMatchedAndRestartFromClosestDescendant.
                return SelectorMatchingResult::NotMatchedAndRestartFromClosestDescendant;
            },
            Combinator::LaterSibling => {
                // If the failure status is NotMatchedAndRestartFromClosestDescendant and combinator is
                // LaterSibling, give up this LaterSibling matching and restart from the closest
                // descendant combinator.
                if matches!(
                    result,
                    SelectorMatchingResult::NotMatchedAndRestartFromClosestDescendant
                ) {
                    return result;
                }
            },
            Combinator::NextSibling
            | Combinator::PseudoElement
            | Combinator::Part
            | Combinator::SlotAssignment => {
                // NOTE(emilio): Conceptually, PseudoElement / Part / SlotAssignment should return
                // `candidate_not_found`, but it doesn't matter in practice since they don't have
                // sibling / descendant combinators to the right of them. This hopefully saves one
                // branch.
                return result;
            },
        }

        if featureless {
            // A featureless element didn't match the selector, we can stop matching now rather
            // than looking at following elements for our combinator.
            return candidate_not_found;
        }
    }
}

#[inline]
fn matches_local_name<E>(element: &E, local_name: &LocalName<E::Impl>) -> bool
where
    E: Element,
{
    let name = select_name(element, &local_name.name, &local_name.lower_name).borrow();
    element.has_local_name(name)
}

fn matches_part<E>(
    element: &E,
    parts: &[<E::Impl as SelectorImpl>::Identifier],
    context: &mut MatchingContext<E::Impl>,
) -> bool
where
    E: Element,
{
    let mut hosts = SmallVec::<[E; 4]>::new();

    let mut host = match element.containing_shadow_host() {
        Some(h) => h,
        None => return false,
    };

    let current_host = context.current_host;
    if current_host != Some(host.opaque()) {
        loop {
            let outer_host = host.containing_shadow_host();
            if outer_host.as_ref().map(|h| h.opaque()) == current_host {
                break;
            }
            let outer_host = match outer_host {
                Some(h) => h,
                None => return false,
            };
            // TODO(emilio): if worth it, we could early return if
            // host doesn't have the exportparts attribute.
            hosts.push(host);
            host = outer_host;
        }
    }

    // Translate the part into the right scope.
    parts.iter().all(|part| {
        let mut part = part.clone();
        for host in hosts.iter().rev() {
            part = match host.imported_part(&part) {
                Some(p) => p,
                None => return false,
            };
        }
        element.is_part(&part)
    })
}

fn matches_host<E>(
    element: &E,
    selector: Option<&Selector<E::Impl>>,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue
where
    E: Element,
{
    let host = match context.shadow_host() {
        Some(h) => h,
        None => return KleeneValue::False,
    };
    if host != element.opaque() {
        return KleeneValue::False;
    }
    let Some(selector) = selector else {
        return KleeneValue::True;
    };
    context.nest(|context| {
        context.with_featureless(false, |context| {
            matches_complex_selector(selector.iter(), element, context, rightmost)
        })
    })
}

fn matches_slotted<E>(
    element: &E,
    selector: &Selector<E::Impl>,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue
where
    E: Element,
{
    // <slots> are never flattened tree slottables.
    if element.is_html_slot_element() {
        return KleeneValue::False;
    }
    context.nest(|context| matches_complex_selector(selector.iter(), element, context, rightmost))
}

fn matches_rare_attribute_selector<E>(
    element: &E,
    attr_sel: &AttrSelectorWithOptionalNamespace<E::Impl>,
) -> bool
where
    E: Element,
{
    let empty_string;
    let namespace = match attr_sel.namespace() {
        Some(ns) => ns,
        None => {
            empty_string = crate::parser::namespace_empty_string::<E::Impl>();
            NamespaceConstraint::Specific(&empty_string)
        },
    };
    element.attr_matches(
        &namespace,
        select_name(element, &attr_sel.local_name, &attr_sel.local_name_lower),
        &match attr_sel.operation {
            ParsedAttrSelectorOperation::Exists => AttrSelectorOperation::Exists,
            ParsedAttrSelectorOperation::WithValue {
                operator,
                case_sensitivity,
                ref value,
            } => AttrSelectorOperation::WithValue {
                operator,
                case_sensitivity: to_unconditional_case_sensitivity(case_sensitivity, element),
                value,
            },
        },
    )
}

/// There are relatively few selectors in a given compound that may match a featureless element.
/// Instead of adding a check to every selector that may not match, we handle it here in an out of
/// line path.
pub(crate) fn compound_matches_featureless_host<Impl: SelectorImpl>(
    iter: &mut SelectorIter<Impl>,
    scope_matches_featureless_host: bool,
) -> MatchesFeaturelessHost {
    let mut matches = MatchesFeaturelessHost::Only;
    for component in iter {
        match component {
            Component::Scope | Component::ImplicitScope if scope_matches_featureless_host => {},
            // :host only matches featureless elements.
            Component::Host(..) => {},
            // Pseudo-elements are allowed to match as well.
            Component::PseudoElement(..) => {},
            // We allow logical pseudo-classes, but we'll fail matching of the inner selectors if
            // necessary.
            Component::Is(ref l) | Component::Where(ref l) => {
                let mut any_yes = false;
                let mut any_no = false;
                for selector in l.slice() {
                    match selector.matches_featureless_host(scope_matches_featureless_host) {
                        MatchesFeaturelessHost::Never => {
                            any_no = true;
                        },
                        MatchesFeaturelessHost::Yes => {
                            any_yes = true;
                            any_no = true;
                        },
                        MatchesFeaturelessHost::Only => {
                            any_yes = true;
                        },
                    }
                }
                if !any_yes {
                    return MatchesFeaturelessHost::Never;
                }
                if any_no {
                    // Potentially downgrade since we might match non-featureless elements too.
                    matches = MatchesFeaturelessHost::Yes;
                }
            },
            Component::Negation(ref l) => {
                // For now preserving behavior, see
                // https://github.com/w3c/csswg-drafts/issues/10179 for existing resolutions that
                // tweak this behavior.
                for selector in l.slice() {
                    if selector.matches_featureless_host(scope_matches_featureless_host)
                        != MatchesFeaturelessHost::Only
                    {
                        return MatchesFeaturelessHost::Never;
                    }
                }
            },
            // Other components don't match the host scope.
            _ => return MatchesFeaturelessHost::Never,
        }
    }
    matches
}

/// Determines whether the given element matches the given compound selector.
#[inline]
fn matches_compound_selector<E>(
    selector_iter: &mut SelectorIter<E::Impl>,
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue
where
    E: Element,
{
    if context.featureless()
        && compound_matches_featureless_host(
            &mut selector_iter.clone(),
            /* scope_matches_featureless_host = */ true,
        ) == MatchesFeaturelessHost::Never
    {
        return KleeneValue::False;
    }
    let quirks_data = if context.quirks_mode() == QuirksMode::Quirks {
        Some(selector_iter.clone())
    } else {
        None
    };
    let mut local_context = LocalMatchingContext {
        shared: context,
        rightmost,
        quirks_data,
    };
    KleeneValue::any_false(selector_iter, |simple| {
        matches_simple_selector(simple, element, &mut local_context)
    })
}

/// Determines whether the given element matches the given single selector.
fn matches_simple_selector<E>(
    selector: &Component<E::Impl>,
    element: &E,
    context: &mut LocalMatchingContext<E::Impl>,
) -> KleeneValue
where
    E: Element,
{
    debug_assert!(context.shared.is_nested() || !context.shared.in_negation());
    let rightmost = context.rightmost;
    KleeneValue::from(match *selector {
        Component::ID(ref id) => {
            element.has_id(id, context.shared.classes_and_ids_case_sensitivity())
        },
        Component::Class(ref class) => {
            element.has_class(class, context.shared.classes_and_ids_case_sensitivity())
        },
        Component::LocalName(ref local_name) => matches_local_name(element, local_name),
        Component::AttributeInNoNamespaceExists {
            ref local_name,
            ref local_name_lower,
        } => element.has_attr_in_no_namespace(select_name(element, local_name, local_name_lower)),
        Component::AttributeInNoNamespace {
            ref local_name,
            ref value,
            operator,
            case_sensitivity,
        } => element.attr_matches(
            &NamespaceConstraint::Specific(&crate::parser::namespace_empty_string::<E::Impl>()),
            local_name,
            &AttrSelectorOperation::WithValue {
                operator,
                case_sensitivity: to_unconditional_case_sensitivity(case_sensitivity, element),
                value,
            },
        ),
        Component::AttributeOther(ref attr_sel) => {
            matches_rare_attribute_selector(element, attr_sel)
        },
        Component::Part(ref parts) => matches_part(element, parts, &mut context.shared),
        Component::Slotted(ref selector) => {
            return matches_slotted(element, selector, &mut context.shared, rightmost);
        },
        Component::PseudoElement(ref pseudo) => {
            element.match_pseudo_element(pseudo, context.shared)
        },
        Component::ExplicitUniversalType | Component::ExplicitAnyNamespace => true,
        Component::Namespace(_, ref url) | Component::DefaultNamespace(ref url) => {
            element.has_namespace(&url.borrow())
        },
        Component::ExplicitNoNamespace => {
            let ns = crate::parser::namespace_empty_string::<E::Impl>();
            element.has_namespace(&ns.borrow())
        },
        Component::NonTSPseudoClass(ref pc) => {
            if let Some(ref iter) = context.quirks_data {
                if pc.is_active_or_hover()
                    && !element.is_link()
                    && hover_and_active_quirk_applies(iter, context.shared, context.rightmost)
                {
                    return KleeneValue::False;
                }
            }
            element.match_non_ts_pseudo_class(pc, &mut context.shared)
        },
        Component::Root => element.is_root(),
        Component::Empty => {
            if context.shared.needs_selector_flags() {
                element.apply_selector_flags(ElementSelectorFlags::HAS_EMPTY_SELECTOR);
            }
            element.is_empty()
        },
        Component::Host(ref selector) => {
            return matches_host(element, selector.as_ref(), &mut context.shared, rightmost);
        },
        Component::ParentSelector => match context.shared.scope_element {
            Some(ref scope_element) => element.opaque() == *scope_element,
            None => element.is_root(),
        },
        Component::Scope | Component::ImplicitScope => {
            if let Some(may_return_unknown) = context.shared.matching_for_invalidation_comparison()
            {
                return if may_return_unknown {
                    KleeneValue::Unknown
                } else {
                    KleeneValue::from(!context.shared.in_negation())
                };
            } else {
                match context.shared.scope_element {
                    Some(ref scope_element) => element.opaque() == *scope_element,
                    None => element.is_root(),
                }
            }
        },
        Component::Nth(ref nth_data) => {
            return matches_generic_nth_child(element, context.shared, nth_data, &[], rightmost);
        },
        Component::NthOf(ref nth_of_data) => {
            return context.shared.nest(|context| {
                matches_generic_nth_child(
                    element,
                    context,
                    nth_of_data.nth_data(),
                    nth_of_data.selectors(),
                    rightmost,
                )
            })
        },
        Component::Is(ref list) | Component::Where(ref list) => {
            return context.shared.nest(|context| {
                matches_complex_selector_list(list.slice(), element, context, rightmost)
            })
        },
        Component::Negation(ref list) => {
            return context.shared.nest_for_negation(|context| {
                !matches_complex_selector_list(list.slice(), element, context, rightmost)
            })
        },
        Component::Has(ref relative_selectors) => {
            return match_relative_selectors(
                relative_selectors,
                element,
                context.shared,
                rightmost,
            );
        },
        Component::Combinator(_) => unsafe {
            debug_unreachable!("Shouldn't try to selector-match combinators")
        },
        Component::RelativeSelectorAnchor => {
            let anchor = context.shared.relative_selector_anchor();
            // We may match inner relative selectors, in which case we want to always match.
            anchor.map_or(true, |a| a == element.opaque())
        },
        Component::Invalid(..) => false,
    })
}

#[inline(always)]
pub fn select_name<'a, E: Element, T: PartialEq>(
    element: &E,
    local_name: &'a T,
    local_name_lower: &'a T,
) -> &'a T {
    if local_name == local_name_lower || element.is_html_element_in_html_document() {
        local_name_lower
    } else {
        local_name
    }
}

#[inline(always)]
pub fn to_unconditional_case_sensitivity<'a, E: Element>(
    parsed: ParsedCaseSensitivity,
    element: &E,
) -> CaseSensitivity {
    match parsed {
        ParsedCaseSensitivity::CaseSensitive | ParsedCaseSensitivity::ExplicitCaseSensitive => {
            CaseSensitivity::CaseSensitive
        },
        ParsedCaseSensitivity::AsciiCaseInsensitive => CaseSensitivity::AsciiCaseInsensitive,
        ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => {
            if element.is_html_element_in_html_document() {
                CaseSensitivity::AsciiCaseInsensitive
            } else {
                CaseSensitivity::CaseSensitive
            }
        },
    }
}

fn matches_generic_nth_child<E>(
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    nth_data: &NthSelectorData,
    selectors: &[Selector<E::Impl>],
    rightmost: SubjectOrPseudoElement,
) -> KleeneValue
where
    E: Element,
{
    if element.ignores_nth_child_selectors() {
        return KleeneValue::False;
    }
    let has_selectors = !selectors.is_empty();
    let selectors_match = !has_selectors
        || matches_complex_selector_list(selectors, element, context, rightmost).to_bool(true);
    if let Some(may_return_unknown) = context.matching_for_invalidation_comparison() {
        // Skip expensive indexing math in invalidation.
        return if selectors_match && may_return_unknown {
            KleeneValue::Unknown
        } else {
            KleeneValue::from(selectors_match && !context.in_negation())
        };
    }

    let NthSelectorData { ty, an_plus_b, .. } = *nth_data;
    let is_of_type = ty.is_of_type();
    if ty.is_only() {
        debug_assert!(
            !has_selectors,
            ":only-child and :only-of-type cannot have a selector list!"
        );
        return KleeneValue::from(
            matches_generic_nth_child(
                element,
                context,
                &NthSelectorData::first(is_of_type),
                selectors,
                rightmost,
            )
            .to_bool(true)
                && matches_generic_nth_child(
                    element,
                    context,
                    &NthSelectorData::last(is_of_type),
                    selectors,
                    rightmost,
                )
                .to_bool(true),
        );
    }

    let is_from_end = ty.is_from_end();

    // It's useful to know whether this can only select the first/last element
    // child for optimization purposes, see the `HAS_EDGE_CHILD_SELECTOR` flag.
    let is_edge_child_selector = nth_data.is_simple_edge() && !has_selectors;

    if context.needs_selector_flags() {
        let mut flags = if is_edge_child_selector {
            ElementSelectorFlags::HAS_EDGE_CHILD_SELECTOR
        } else if is_from_end {
            ElementSelectorFlags::HAS_SLOW_SELECTOR
        } else {
            ElementSelectorFlags::HAS_SLOW_SELECTOR_LATER_SIBLINGS
        };
        flags |= if has_selectors {
            ElementSelectorFlags::HAS_SLOW_SELECTOR_NTH_OF
        } else {
            ElementSelectorFlags::HAS_SLOW_SELECTOR_NTH
        };
        element.apply_selector_flags(flags);
    }

    if !selectors_match {
        return KleeneValue::False;
    }

    // :first/last-child are rather trivial to match, don't bother with the
    // cache.
    if is_edge_child_selector {
        return if is_from_end {
            element.next_sibling_element()
        } else {
            element.prev_sibling_element()
        }
        .is_none()
        .into();
    }

    // Lookup or compute the index.
    let index = if let Some(i) = context
        .nth_index_cache(is_of_type, is_from_end, selectors)
        .lookup(element.opaque())
    {
        i
    } else {
        let i = nth_child_index(
            element,
            context,
            selectors,
            is_of_type,
            is_from_end,
            /* check_cache = */ true,
            rightmost,
        );
        context
            .nth_index_cache(is_of_type, is_from_end, selectors)
            .insert(element.opaque(), i);
        i
    };
    debug_assert_eq!(
        index,
        nth_child_index(
            element,
            context,
            selectors,
            is_of_type,
            is_from_end,
            /* check_cache = */ false,
            rightmost,
        ),
        "invalid cache"
    );

    an_plus_b.matches_index(index).into()
}

#[inline]
fn nth_child_index<E>(
    element: &E,
    context: &mut MatchingContext<E::Impl>,
    selectors: &[Selector<E::Impl>],
    is_of_type: bool,
    is_from_end: bool,
    check_cache: bool,
    rightmost: SubjectOrPseudoElement,
) -> i32
where
    E: Element,
{
    // The traversal mostly processes siblings left to right. So when we walk
    // siblings to the right when computing NthLast/NthLastOfType we're unlikely
    // to get cache hits along the way. As such, we take the hit of walking the
    // siblings to the left checking the cache in the is_from_end case (this
    // matches what Gecko does). The indices-from-the-left is handled during the
    // regular look further below.
    if check_cache
        && is_from_end
        && !context
            .nth_index_cache(is_of_type, is_from_end, selectors)
            .is_empty()
    {
        let mut index: i32 = 1;
        let mut curr = element.clone();
        while let Some(e) = curr.prev_sibling_element() {
            curr = e;
            let matches = if is_of_type {
                element.is_same_type(&curr)
            } else if !selectors.is_empty() {
                matches_complex_selector_list(selectors, &curr, context, rightmost).to_bool(true)
            } else {
                true
            };
            if !matches {
                continue;
            }
            if let Some(i) = context
                .nth_index_cache(is_of_type, is_from_end, selectors)
                .lookup(curr.opaque())
            {
                return i - index;
            }
            index += 1;
        }
    }

    let mut index: i32 = 1;
    let mut curr = element.clone();
    let next = |e: E| {
        if is_from_end {
            e.next_sibling_element()
        } else {
            e.prev_sibling_element()
        }
    };
    while let Some(e) = next(curr) {
        curr = e;
        let matches = if is_of_type {
            element.is_same_type(&curr)
        } else if !selectors.is_empty() {
            matches_complex_selector_list(selectors, &curr, context, rightmost).to_bool(true)
        } else {
            true
        };
        if !matches {
            continue;
        }
        // If we're computing indices from the left, check each element in the
        // cache. We handle the indices-from-the-right case at the top of this
        // function.
        if !is_from_end && check_cache {
            if let Some(i) = context
                .nth_index_cache(is_of_type, is_from_end, selectors)
                .lookup(curr.opaque())
            {
                return i + index;
            }
        }
        index += 1;
    }

    index
}
