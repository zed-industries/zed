// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_ARGUMENT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_ARGUMENT_CONTEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/has_invalidation_flags.h"

namespace blink {

enum CheckPseudoHasArgumentTraversalScope {
  // Case 1: :has() argument selector starts with child or descendant
  //         combinator, and depth is not fixed.
  //         (e.g. :has(.a), :has(.a > .b), :has(.a + .b), :has(> .a .b) ...))
  kSubtree,

  // Case 2: :has() argument selector starts with direct or indirect adjacent
  //         combinator and adjacent distance is not fixed and depth is fixed
  //         and child combinator not exists.
  //         (.e.g. :has(~ .a), :has(~ .a ~ .b), :has(~ .a + .b))
  kAllNextSiblings,

  // Case 3: :has() argument selector starts with direct adjacent combinator
  //         and adjacent distance is fixed and depth is not fixed.
  //         (.e.g. :has(+ .a .b), :has(+ .a > .b .c)), :has(+ .a .b > .c)
  //                :has(+ .a .b ~ .c), :has(+ .a + .b .c))
  kOneNextSiblingSubtree,

  // Case 4: :has() argument selector starts with direct or indirect adjacent
  //         combinator and adjacent distance and depth are not fixed.
  //         (.e.g. :has(~ .a .b), :has(+ .a ~ .b .c))
  kAllNextSiblingSubtrees,

  // Case 5: :has() argument selector starts with direct adjacent combinator
  //         and both adjacent distance and depth are fixed and no child
  //         combinator.
  //          (.e.g. :has(+ .a), :has(+ .a + .b))
  kOneNextSibling,

  // Case 6: :has() argument selector starts with child combinator and depth is
  //         fixed.
  //         (.e.g. :has(> .a), :has(> .a > .b), :has(> .a + .b),
  //                :has(> .a ~ .b))
  kFixedDepthDescendants,

  // Case 7: :has() argument selector starts with direct adjacent combinator
  //         and both adjacent distance and depth are fixed and child combinator
  //         exists.
  //          (.e.g. :has(+ .a > .b), :has(+ .a > .b ~ .c))
  kOneNextSiblingFixedDepthDescendants,

  // Case 8: :has() argument selector starts with direct or indirect adjacent
  //         combinator and adjacent distance is not fixed and depth is fixed
  //         and child combinator exists.
  //            (.e.g. :has(~ .a > .b), :has(+ .a ~ .b > .c),
  //                   :has(~ .a > .b ~ .c), :has(+ .a ~ .b > .c ~ .d),
  kAllNextSiblingsFixedDepthDescendants,

  // Case 9: Same as case 1, but the :has() argument selector needs to match
  //         elements in the shadow tree. (e.g. :host:has(.a))
  kShadowRootSubtree,

  // Case 10: Same as case 6, but the :has() argument selector needs to match
  //          elements in the shadow tree. (e.g. :host:has(> .a))
  kShadowRootFixedDepthDescendants,

  // Case 11: :has() argument selector starts with direct or indirect adjacent
  //          combinator and the selector needs match in the shadow tree of the
  //          :host, in which case the adjacent combinators could never match.
  //          (e.g. :host:has(~ .a), :host:has(+ .a))
  kInvalidShadowRootTraversalScope,

  kTraversalScopeMax = kInvalidShadowRootTraversalScope,
};

// Unique value of each traversal type. The value can be used as a key of
// fast reject filter cache.
//
// These 3 values are stored by dividing the 4-byte field by:
// - depth limit : 0 ~ 13 (14bits)
// - adjacent distance limit : 14 ~ 27 (14 bits)
// - traversal scope : 28 ~ 31 (4 bits)
using CheckPseudoHasArgumentTraversalType = uint32_t;

class CORE_EXPORT CheckPseudoHasArgumentContext {
  STACK_ALLOCATED();

 public:
  explicit CheckPseudoHasArgumentContext(const CSSSelector* selector,
                                         const ContainerNode* scope,
                                         bool match_in_shadow_tree);

  inline bool AdjacentDistanceFixed() const {
    return adjacent_distance_limit_ != kInfiniteAdjacentDistance;
  }
  inline int AdjacentDistanceLimit() const { return adjacent_distance_limit_; }
  inline bool DepthFixed() const { return depth_limit_ != kInfiniteDepth; }
  inline int DepthLimit() const { return depth_limit_; }

  // Returns true if we are matching styles for a shadow host via a :host rule
  // with :has(). In that case :has() is matching descendants in the shadow
  // tree, not the light tree descendants of the host.
  inline bool MatchInShadowTree() const {
    DCHECK(!match_in_shadow_tree_ || traversal_scope_ == kShadowRootSubtree ||
           traversal_scope_ == kShadowRootFixedDepthDescendants ||
           traversal_scope_ == kInvalidShadowRootTraversalScope);
    return match_in_shadow_tree_;
  }

  inline CSSSelector::RelationType LeftmostRelation() const {
    return leftmost_relation_;
  }

  inline bool SiblingCombinatorAtRightmost() const {
    return sibling_combinator_at_rightmost_;
  }
  inline bool SiblingCombinatorBetweenChildOrDescendantCombinator() const {
    return sibling_combinator_between_child_or_descendant_combinator_;
  }

  CheckPseudoHasArgumentTraversalScope TraversalScope() const {
    return traversal_scope_;
  }

  SiblingsAffectedByHasFlags GetSiblingsAffectedByHasFlags() const {
    return siblings_affected_by_has_flags_;
  }

  const CSSSelector* HasArgument() const { return has_argument_; }

  // See SelectorCheckingContext::scope.
  const ContainerNode* Scope() const { return scope_; }

  const Vector<unsigned>& GetPseudoHasArgumentHashes() const {
    return pseudo_has_argument_hashes_;
  }

  CheckPseudoHasArgumentTraversalType TraversalType() const {
    return depth_limit_ | (adjacent_distance_limit_ << kDepthBits) |
           (traversal_scope_ << (kDepthBits + kAdjacentBits));
  }

 private:
  const static size_t kDepthBits = 14;
  const static size_t kAdjacentBits = 14;
  const static size_t kTraversalScopeBits = 4;

  const static int kInfiniteDepth = (1 << kDepthBits) - 1;
  const static int kInfiniteAdjacentDistance = (1 << kAdjacentBits) - 1;

  static_assert(kTraversalScopeMax <= ((1 << kTraversalScopeBits) - 1),
                "traversal scope size check");
  static_assert((kDepthBits + kAdjacentBits + kTraversalScopeBits) <=
                    sizeof(CheckPseudoHasArgumentTraversalType) * 8,
                "traversal type size check");

  // Indicate the :has argument relative type and subtree traversal scope.
  // If 'adjacent_distance_limit' is integer max, it means that all the
  // adjacent subtrees need to be traversed. otherwise, it means that it is
  // enough to traverse the adjacent subtree at that distance.
  // If 'descendant_traversal_depth_' is integer max, it means that all of the
  // descendant subtree need to be traversed. Otherwise, it means that it is
  // enough to traverse elements at the certain depth.
  //
  // Case 1:  (kDescendant, 0, max)
  //   - Argument selector conditions
  //     - Starts with descendant combinator.
  //   - E.g. ':has(.a)', ':has(.a ~ .b)', ':has(.a ~ .b > .c)'
  //   - Traverse all descendants of the :has() anchor element.
  // Case 2:  (kChild, 0, max)
  //   - Argument selector conditions
  //     - Starts with child combinator.
  //     - At least one descendant combinator.
  //   - E.g. ':has(> .a .b)', ':has(> .a ~ .b .c)', ':has(> .a + .b .c)'
  //   - Traverse all descendants of the :has() anchor element.
  // Case 3:  (kChild, 0, n)
  //   - Argument selector conditions
  //     - Starts with child combinator.
  //     - n number of child combinator. (n > 0)
  //     - No descendant combinator.
  //   - E.g.
  //     - ':has(> .a)'            : (kChild, 0, 1)
  //     - ':has(> .a ~ .b > .c)'  : (kChild, 0, 2)
  //   - Traverse the depth n descendants of the :has() anchor element.
  // Case 4:  (kIndirectAdjacent, max, max)
  //   - Argument selector conditions
  //     - Starts with indirect adjacent combinator.
  //     - At least one descendant combinator.
  //   - E.g. ':has(~ .a .b)', ':has(~ .a + .b > .c ~ .d .e)'
  //   - Traverse all the subsequent sibling subtrees of the :has() anchor
  //     element. (all subsequent siblings and its descendants)
  // Case 5:  (kIndirectAdjacent, max, 0)
  //   - Argument selector conditions
  //     - Starts with indirect adjacent combinator.
  //     - No descendant/child combinator.
  //   - E.g. ':has(~ .a)', ':has(~ .a + .b ~ .c)'
  //   - Traverse all subsequent siblings of the :has() anchor element.
  // Case 6:  (kIndirectAdjacent, max, n)
  //   - Argument selector conditions
  //     - Starts with indirect adjacent combinator.
  //     - n number of child combinator. (n > 0)
  //     - No descendant combinator.
  //   - E.g.
  //     - ':has(~ .a > .b)'                 : (kIndirectAdjacent, max, 1)
  //     - ':has(~ .a + .b > .c ~ .d > .e)'  : (kIndirectAdjacent, max, 2)
  //   - Traverse depth n elements of all subsequent sibling subtree of the
  //     :has() anchor element.
  // Case 7:  (kDirectAdjacent, max, max)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - At least one indirect adjacent combinator to the left of every
  //       descendant or child combinator.
  //     - At least 1 descendant combinator.
  //   - E.g. ':has(+ .a ~ .b .c)', ':has(+ .a ~ .b > .c + .d .e)'
  //   - Traverse all the subsequent sibling subtrees of the :has() anchor
  //     element. (all subsequent siblings and its descendants)
  // Case 8:  (kDirectAdjacent, max, 0)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - At least one indirect adjacent combinator.
  //     - No descendant/child combinator.
  //   - E.g. ':has(+ .a ~ .b)', ':has(+ .a + .b ~ .c)'
  //   - Traverse all subsequent siblings of the :has() anchor element.
  // Case 9:  (kDirectAdjacent, max, n)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - At least one indirect adjacent combinator to the left of every
  //       descendant or child combinator.
  //     - n number of child combinator. (n > 0)
  //     - No descendant combinator.
  //   - E.g.
  //     - ':has(+ .a ~ .b > .c)'            : (kDirectAdjacent, max, 1)
  //     - ':has(+ .a ~ .b > .c + .d >.e)'   : (kDirectAdjacent, max, 2)
  //   - Traverse depth n elements of all subsequent sibling subtree of the
  //     :has() anchor element.
  // Case 10:  (kDirectAdjacent, n, max)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - n number of direct adjacent combinator to the left of the leftmost
  //       child(or descendant) combinator. (n > 0)
  //     - No indirect adjacent combinator to the left of the leftmost child
  //       (or descendant) combinator.
  //     - At least 1 descendant combinator.
  //   - E.g.
  //     - ':has(+ .a .b)'            : (kDirectAdjacent, 1, max)
  //     - ':has(+ .a > .b + .c .d)'  : (kDirectAdjacent, 1, max)
  //     - ':has(+ .a + .b > .c .d)'  : (kDirectAdjacent, 2, max)
  //   - Traverse the distance n sibling subtree of the :has() anchor element.
  //     (sibling element at distance n, and its descendants).
  // Case 11:  (kDirectAdjacent, n, 0)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - n number of direct adjacent combinator. (n > 0)
  //     - No child/descendant/indirect-adjacent combinator.
  //   - E.g.
  //     - ':has(+ .a)'            : (kDirectAdjacent, 1, 0)
  //     - ':has(+ .a + .b + .c)'  : (kDirectAdjacent, 3, 0)
  //   - Traverse the distance n sibling element of the :has() anchor element.
  // Case 12:  (kDirectAdjacent, n, m)
  //   - Argument selector conditions
  //     - Starts with direct adjacent combinator.
  //     - n number of direct adjacent combinator to the left of the leftmost
  //       child combinator. (n > 0)
  //     - No indirect adjacent combinator to the left of the leftmost child
  //       combinator.
  //     - n number of child combinator. (n > 0)
  //     - No descendant combinator.
  //   - E.g.
  //     - ':has(+ .a > .b)'                 : (kDirectAdjacent, 1, 1)
  //     - ':has(+ .a + .b > .c ~ .d > .e)'  : (kDirectAdjacent, 2, 2)
  //   - Traverse the depth m elements of the distance n sibling subtree of
  //     the :has() anchor element. (elements at depth m of the descendant
  //     subtree of the sibling element at distance n)
  CSSSelector::RelationType leftmost_relation_{CSSSelector::kSubSelector};
  int adjacent_distance_limit_;
  int depth_limit_;

  // Indicates the selector's combinator information which can be used for
  // sibling traversal after the :has() argument selector matched.
  bool sibling_combinator_at_rightmost_{false};
  bool sibling_combinator_between_child_or_descendant_combinator_{false};
  CheckPseudoHasArgumentTraversalScope traversal_scope_;
  SiblingsAffectedByHasFlags siblings_affected_by_has_flags_;
  const CSSSelector* has_argument_;
  const ContainerNode* scope_;
  bool match_in_shadow_tree_;

  Vector<unsigned> pseudo_has_argument_hashes_;

  friend class CheckPseudoHasArgumentContextTest;
};

// Subtree traversal iterator class for :has() argument checking. To solve the
// following issues, this traversal uses the reversed DOM tree order, and
// provides a functionality to limit the traversal depth.
//
// 1. Cache 'Matched' and 'NotMatched' candidate elements while checking the
//    :has() argument selector.
//
// SelectorChecker::CheckPseudoHas() can get all 'Matched' candidates (elements
// that can be a :has() anchor element) while checking the :has() argument
// selector on an element in the traversal range. And when it found the
// elements, it caches those as 'Matched' candidates.
// By following the reversed DOM tree order, we can get these two advantages.
// - Maximize the number of 'Matched' candidates that can be cached while
//   checking :has() argument selector.
// - Can cache 'NotMatched' candidates (elements that cannot be a :has() anchor
//   element) in case of these 4 traversal scope types:
//   - kSubtree
//   - kAllNextSiblings
//   - kOneNextSiblingSubtree
//   - kAllNextSiblingSubtrees
//   While traversing, we can cache an element as 'NotMatched' if the element is
//   not cached as 'Matched' because it must be cached as 'Matched' previously
//   if it is a :has() anchor element. (Reversed DOM tree order guarantees that
//   all the descendants, next siblings and next sibling subtrees were already
//   traversed)
//
// 2. Prevent unnecessary subtree traversal when it can be limited with
//    child combinator or direct adjacent combinator.
//
// We can limit the tree traversal range when we count the leftmost combinators
// of a :has() argument selector. For example, when we check ':has(> .a > .b)'
// on an element, instead of traversing all the descendants of the :has() anchor
// element, we can limit the traversal only for the elements at depth 2 of the
// :has() anchor element. When we check ':has(+ .a > .b)', we can limit the
// traversal only for the child elements of the direct adjacent sibling of the
// :has() anchor element. To implement this, we need a way to limit the
// traversal depth and a way to check whether the iterator is currently at the
// fixed depth or not.
class CORE_EXPORT CheckPseudoHasArgumentTraversalIterator {
  STACK_ALLOCATED();

 public:
  CheckPseudoHasArgumentTraversalIterator(Element&,
                                          CheckPseudoHasArgumentContext&);
  void operator++();
  Element* CurrentElement() const { return current_element_; }
  bool AtEnd() const { return !current_element_; }
  inline int CurrentDepth() const { return current_depth_; }
  inline Element* ScopeElement() const { return has_anchor_element_; }

 private:
  inline Element* LastWithin(ContainerNode*);

  Element* const has_anchor_element_;
  bool match_in_shadow_tree_;
  int depth_limit_;
  Element* last_element_{nullptr};
  Element* sibling_at_fixed_distance_{nullptr};
  Element* current_element_{nullptr};
  int current_depth_{0};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_ARGUMENT_CONTEXT_H_
