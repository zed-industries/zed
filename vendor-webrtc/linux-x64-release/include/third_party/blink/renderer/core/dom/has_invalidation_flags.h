// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_HAS_INVALIDATION_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_HAS_INVALIDATION_FLAGS_H_

namespace blink {

// Flags for :has() invalidation.
//
// The flags can be categorized 3 types.
//
// 1. Flags for the :has() anchor elements.
//    - AffectedBySubjectHas
//        Indicates that this element may match a subject :has() selector, which
//        means we need to invalidate the element when the :has() state changes.
//    - AffectedByNonSubjectHas
//        Indicates that this element may match a non-subject :has() selector,
//        which means we need to schedule descendant and sibling invalidation
//        sets on this element when the :has() state changes.
//    - AffectedByPseudosInHas
//        Indicates that this element can be affected by the state change of the
//        pseudo class in the :has() argument selector. For every pseudo state
//        change mutation, if an element doesn't have the flag set, the element
//        will not be invalidated or scheduled on even if the element has the
//        AffectedBySubjectHas or AffectedByNonSubjectHas flag set.
//    - AffectedByMultipleHas
//        Indicate that this element can be affected by multiple :has() pseudo
//        classes.
//        SelectorChecker uses CheckPseudoHasFastRejectFilter to preemtively
//        skip non-matching :has() pseudo class checks only if there are
//        multiple :has() to check on the same anchor element. SelectorChecker
//        would not use the reject filter for a single :has() because it would
//        have worse performance caused by the bloom filter memory allocation
//        and the tree traversal for collecting element identifier hashes.
//        To avoid the unnecessary overhead, bloom filter creation and element
//        identifier hash collection are performed on the second check, and at
//        this time AffectedByMultipleHas flag is set.
//        This flag is used to determine whether SelectorChecker can use the
//        reject filter even if on the first check since the flag indicates that
//        there can be additional checks on the same anchor element.
//
//    SelectorChecker::CheckPseudoClass() set the flags on an element when it
//    checks a :has() pseudo class on the element.
//
// 2. Flags for the elements that a :has() argument selector can be tested on.
//    (The elements that can affect a :has() pseudo class state)
//
//    - SiblingsAffectedByHas :
//        Indicates that this element possibly matches any of the :has()
//        argument selectors, and we need to traverse siblings to find the
//        subject or non-subject :has() anchor element.
//        The SiblingsAffectedByHas consists of two flags.
//        - SiblingsAffectedByHasForSiblingRelationship
//            Indicates that the `:has()` argument selector is to check the
//            sibling relationship. The argument selector starts with a direct
//            or indirect adjacent combinator and it doesn't have any descendant
//            or child combinator(s).
//        - SiblingsAffectedByHasForSiblingDescendantRelationship
//            Indicates that the `:has()` argument selector is to check the
//            sibling-descendant relationship. The argument selector starts with
//            a direct or indirect adjacent combinator and it has descendant or
//            child combinator(s).
//    - AncestorsOrAncestorSiblingsAffectedByHas :
//        Indicates that this element possibly matches any of the :has()
//        argument selectors, and we need to traverse ancestors or siblings of
//        ancestors to find the subject or non-subject :has() anchor element.
//
//    SelectorChecker::CheckPseudoHas() set the flags on some elements when it
//    checks the :has() argument selectors. (StyleEngine also set the flags
//    on the elements to be inserted if the inserted elements possibly affecting
//    a :has() state change)
//
//    Before starting the subtree traversal for checking the :has() argument
//    selector, the SelectorChecker::CheckPseudoHas() set the flags on the
//    :has() anchor element or its next siblings (The :has() anchor element
//    should have the flags set so that the StyleEngine can determine whether an
//    inserted element is possibly affecting :has() state).
//
//    If the :has() argument selector starts with child or descendant
//    combinator, the :has() anchor element will have the
//    AncestorsOrAncestorSiblingsAffectedByHas flag set. If the :has() argument
//    starts with adjacent combinators, the :has() anchor element and its next
//    siblings will have the SiblingsAffectedByHas flag set.
//
//    If the :has() argument selector checks descendant or sibling descendant
//    relationship (child or descendant combinator exists in the argument), for
//    every elements in the argument checking traversal, the
//    AncestorsOrAncestorSiblingsAffectedByHas flag will be set so that the
//    StyleEngine can traverse to ancestors for :has() invalidation.
//
//    StyleEngine tries to find the :has() anchor elements by traversing
//    siblings or ancestors of a mutated element only when an element has the
//    xxx-affected-by-has flags set. If an element doesn't have those flags set,
//    then the StyleEngine will stop the traversal at the element.
//
//    CheckPseudoHasArgumentTraversalIterator traverses the subtree in the
//    reversed DOM tree order to prevent duplicated subtree traversal caused by
//    the multiple :has() anchor elements. If there is an argument matched
//    element in the traversal, it returns early because the :has() pseudo class
//    matches.
//
//    Due to the traversal order and the early returning, the :has()
//    invalidation traversal can be broken when the :has() argument selector
//    matches on an element because the ancestors or previous siblings of the
//    element will not have the AncestorsOrAncestorSiblingsAffectedByHas flag
//    set.
//
//    To prevent the problem, when the :has() argument matches on an element,
//    the SelectorChecker::CheckPseudoHas traverses to siblings, ancestors or
//    ancestor siblings of the argument matched element and set the
//    AncestorsOrAncestorSiblingsAffectedByHas flag on the elements until reach
//    to the :has() anchor element or sibling of :has() anchor element.
//
// 3. Flags for the elements that the particular pseudo classes in the :has()
//    argument selector can be tested on.
//    (The elements that can affect a :has() pseudo class state by their own
//     state change for the particular pseudo classes)
//
//    - AncestorsOrSiblingsAffectedByHoverInHas :
//        Indicates that this element may matched a :hover inside :has().
//    - AncestorsOrSiblingsAffectedByActiveInHas :
//        Indicates that this element may matched a :active inside :has().
//    - AncestorsOrSiblingsAffectedByFocusInHas :
//        Indicates that this element may matched a :focus inside :has().
//    - AncestorsOrSiblingsAffectedByFocusVisibleInHas :
//        Indicates that this element may matched a :focus-visible inside
//        :has().
//
//    SelectorChecker::CheckPseudoClass check the flags on an element when it
//    checks the pseudo classes on the element.
//
// Similar to the DynamicRestyleFlags in the ContainerNode, these flags will
// never be reset. (except the AffectedBySubjectHas flag which is defined at
// the computed style extra flags)
//
// Example 1) Subject :has() (has only descendant relationship)
//  <style> .a:has(.b) {...} </style>
//  <div>
//    <div class=a>  <!-- AffectedBySubjectHas (computed style extra flag) -->
//      <div>           <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//        <div></div>   <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//        <div></div>   <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//      </div>
//    </div>
//  </div>
//
//
// Example 2) Non-subject :has()
//  <style> .a:has(.b) .c {...} </style>
//  <div>
//    <div class=a>          <!-- AffectedByNonSubjectHas -->
//      <div>                <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//        <div></div>        <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//        <div class=c></div><!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//      </div>
//    </div>
//  </div>
//
//
// Example 3) Subject :has() (has only sibling relationship)
//  <style> .a:has(~ .b) {...} </style>
//  <div>
//    <div></div>
//    <div class=a>  <!-- AffectedBySubjectHas (computed style extra flag) -->
//      <div></div>
//    </div>
//    <div></div>    <!-- SiblingsAffectedByHasForSiblingRelationship -->
//    <div></div>    <!-- SiblingsAffectedByHasForSiblingRelationship -->
//  </div>
//
//
// Example 4) Subject :has() (has both sibling and descendant relationship)
//  <style> .a:has(~ .b .c) {...} </style>
//  <div>
//    <div></div>
//    <div class=a>  <!-- AffectedBySubjectHas (computed style extra flag) -->
//    </div>
//    <div>     <!-- SiblingsAffectedByHasForSiblingDescendantRelationship -->
//      <div></div>  <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//      <div></div>  <!-- AncestorsOrAncestorSiblingsAffectedByHas -->
//    </div>
//  </div>

enum SiblingsAffectedByHasFlags : unsigned {
  kFlagForSiblingRelationship = 1 << 0,
  kFlagForSiblingDescendantRelationship = 1 << 1,

  kNoSiblingsAffectedByHasFlags = 0,
};

struct HasInvalidationFlags {
  unsigned affected_by_subject_has : 1;
  unsigned affected_by_non_subject_has : 1;
  unsigned affected_by_pseudos_in_has : 1;

  unsigned siblings_affected_by_has : 2;
  unsigned ancestors_or_ancestor_siblings_affected_by_has : 1;

  unsigned ancestors_or_siblings_affected_by_hover_in_has : 1;
  unsigned ancestors_or_siblings_affected_by_active_in_has : 1;
  unsigned ancestors_or_siblings_affected_by_focus_in_has : 1;
  unsigned ancestors_or_siblings_affected_by_focus_visible_in_has : 1;
  unsigned affected_by_logical_combinations_in_has : 1;

  unsigned affected_by_multiple_has : 1;

  HasInvalidationFlags()
      : affected_by_subject_has(false),
        affected_by_non_subject_has(false),
        affected_by_pseudos_in_has(false),
        siblings_affected_by_has(0),
        ancestors_or_ancestor_siblings_affected_by_has(false),
        ancestors_or_siblings_affected_by_hover_in_has(false),
        ancestors_or_siblings_affected_by_active_in_has(false),
        ancestors_or_siblings_affected_by_focus_in_has(false),
        ancestors_or_siblings_affected_by_focus_visible_in_has(false),
        affected_by_multiple_has(false) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_HAS_INVALIDATION_FLAGS_H_
