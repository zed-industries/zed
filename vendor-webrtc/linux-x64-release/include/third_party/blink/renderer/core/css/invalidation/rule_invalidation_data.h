// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_H_

#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/invalidation/invalidation_set.h"
#include "third_party/blink/renderer/platform/wtf/bloom_filter.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

enum class RuleInvalidationDataVisitorType;

// Summarizes and indexes the contents of CSS selectors. It creates
// invalidation sets from them and makes them available via several
// CollectInvalidationSetForFoo methods which use the indices to quickly gather
// the relevant InvalidationSets for a particular DOM mutation.
// Also captures several pieces of metadata that describe the entire set of
// indexed selectors, for example "does any selector use ::first-line".
//
// In addition to complete invalidation (where we just throw up our
// hands and invalidate everything) and :has() (which is described in
// detail below), we have fundamentally five types of invalidation.
// All will be described for a class selector, but apply equally to
// id etc.:
//
//   - Self-invalidation: When an element gets or loses class .c,
//     that element needs to be invalidated (.c exists as a subject in
//     some selector). We represent this by a bit in .c's invalidation
//     set (or by inserting the class name in a Bloom filter; see
//     class_invalidation_sets_).
//
//   - Descendant invalidation: When an element gets or loses class .c,
//     all of its children with class .d need to be invalidated
//     (a selector of the form .c .d or .c > .d exists). We represent
//     this by storing .d in .c's descendant invalidation set.
//
//   - Sibling invalidation: When an element gets or loses class .c,
//     all of its _siblings_ with class .d need to be invalidated
//     (a selector of the form .c ~ .d or .c + .d exists).
//     We represent this by storing .d in c's sibling invalidation set.
//
//   - Universal sibling invalidation: Described below.
//
//   - nth-child invalidation: Described below.
//
// The universal sibling invalidation set is used in cases where compounds left
// of sibling combinators don't have any simple selectors for which we create
// invalidation sets. This includes the following cases:
//   - Tag names to the left of sibling combinators. We do not index
//     invalidation sets on tag names since elements do not change tag names
//     dynamically. However, a selector such as "div + span" necessitates an
//     invalidation on <span> elements when a preceding <div> is added/removed.
//   - Negated selectors to the left of sibling combinators. For example,
//     ":not(.a) + .b" necessitates an invalidation on ".b" when a preceding
//     element not having class "a" is added/removed.
//   - Universal selectors to the left of sibling combinators. For example,
//     "* + .a" necessitates an invalidation on ".a" when it gains or loses a
//     preceding sibling.
// For more detail, see https://crrev.com/1f82047b13f02be39b8104b6afda0615e60a7cee.
//
// nth-child invalidation deals with peculiarities for :nth-child()
// and related selectors (such as :only-of-type). We have a couple
// of distinct strategies for dealing with them:
//
//   - When we add or insert a node in the DOM tree where any child
//     of the parent has matched such a selector, we forcibly schedule
//     the (global) NthSiblingInvalidationSet. In other words, this
//     is hardly related to the normal invalidation mechanism at all.
//
//   - Finally, for :nth_child(... of :has()), we get a signal when
//     a node is affected by :has() subject invalidation
//     (in StyleEngine::InvalidateElementAffectedByHas()), and can
//     forcibly schedule the NthSiblingInvalidationSet, much like
//     the previous point.
//
//   - When we have :nth-child(... of S) as a subject (ie., not just
//     pure :nth-child(), but anything with a selector), we set the
//     invalidates_nth_ bit on all invalidation sets for S. This means
//     that whenever we schedule invalidation sets for anything in S,
//     and any child of the parent has matched any :nth-child() selector,
//     we'll schedule the NthSiblingInvalidationSet.
//
//   - For all ancestors, we go through them recursively to find
//     S within :nth-child(), and set their invalidates_nth_ similarly.
//     This is conceptually the same thing as the previous point,
//     but since we already handle subjects and ancestors differently,
//     it was convenient with some mild code duplication here.
//
//   - When we have sibling selectors against :nth-child, special
//     provisions apply; see comments NthSiblingInvalidationSet.
class CORE_EXPORT RuleInvalidationData {
 public:
  bool operator==(const RuleInvalidationData& other) const;

  void Clear();

  // Collect descendant and sibling invalidation sets, for a given type of
  // change (e.g. “if this element added or removed the given class, what other
  // types of elements need to change?”). This is called during DOM mutations.
  // CollectInvalidationSets* govern self-invalidation and descendant
  // invalidations, while CollectSiblingInvalidationSets* govern sibling
  // invalidations.

  // Note that class invalidations will sometimes return self-invalidation
  // even when it is not necessary; see comment on class_invalidation_sets_.
  void CollectInvalidationSetsForClass(InvalidationLists&,
                                       Element&,
                                       const AtomicString& class_name) const;

  void CollectInvalidationSetsForId(InvalidationLists&,
                                    Element&,
                                    const AtomicString& id) const;
  void CollectInvalidationSetsForAttribute(
      InvalidationLists&,
      Element&,
      const QualifiedName& attribute_name) const;
  void CollectInvalidationSetsForPseudoClass(InvalidationLists&,
                                             Element&,
                                             CSSSelector::PseudoType) const;

  void CollectSiblingInvalidationSetForClass(
      InvalidationLists&,
      Element&,
      const AtomicString& class_name,
      unsigned min_direct_adjacent) const;
  void CollectSiblingInvalidationSetForId(InvalidationLists&,
                                          Element&,
                                          const AtomicString& id,
                                          unsigned min_direct_adjacent) const;
  void CollectSiblingInvalidationSetForAttribute(
      InvalidationLists&,
      Element&,
      const QualifiedName& attribute_name,
      unsigned min_direct_adjacent) const;

  void CollectUniversalSiblingInvalidationSet(
      InvalidationLists&,
      unsigned min_direct_adjacent) const;

  void CollectNthInvalidationSet(InvalidationLists&) const;

  // TODO: Document.
  void CollectPartInvalidationSet(InvalidationLists&) const;

  // Quick tests for whether we need to consider :has() invalidation.
  bool NeedsHasInvalidationForClass(const AtomicString& class_name) const;
  bool NeedsHasInvalidationForAttribute(
      const QualifiedName& attribute_name) const;
  bool NeedsHasInvalidationForId(const AtomicString& id) const;
  bool NeedsHasInvalidationForTagName(const AtomicString& tag_name) const;
  bool NeedsHasInvalidationForInsertedOrRemovedElement(Element&) const;
  bool NeedsHasInvalidationForPseudoClass(
      CSSSelector::PseudoType pseudo_type) const;

  inline bool NeedsHasInvalidationForClassChange() const {
    return !classes_in_has_argument.empty();
  }
  inline bool NeedsHasInvalidationForAttributeChange() const {
    return !attributes_in_has_argument.empty();
  }
  inline bool NeedsHasInvalidationForIdChange() const {
    return !ids_in_has_argument.empty();
  }
  inline bool NeedsHasInvalidationForPseudoStateChange() const {
    return !pseudos_in_has_argument.empty();
  }
  inline bool NeedsHasInvalidationForInsertionOrRemoval() const {
    return not_pseudo_in_has_argument || universal_in_has_argument ||
           !tag_names_in_has_argument.empty() ||
           NeedsHasInvalidationForClassChange() ||
           NeedsHasInvalidationForAttributeChange() ||
           NeedsHasInvalidationForIdChange() ||
           NeedsHasInvalidationForPseudoStateChange();
  }

  bool HasSelectorForId(const AtomicString& id_value) const {
    return id_invalidation_sets.Contains(id_value);
  }
  bool HasIdsInSelectors() const { return id_invalidation_sets.size() > 0; }
  bool InvalidatesParts() const { return invalidates_parts; }
  // Returns true if we have :nth-child(... of S) selectors where S contains a
  // :has() selector.
  bool UsesHasInsideNth() const { return uses_has_inside_nth; }
  bool UsesFirstLineRules() const { return uses_first_line_rules; }
  bool UsesWindowInactiveSelector() const {
    return uses_window_inactive_selector;
  }
  unsigned MaxDirectAdjacentSelectors() const {
    return max_direct_adjacent_selectors;
  }

  // Format the RuleInvalidationData for debugging purposes.
  //
  //  [>] Means descendant invalidation set.
  //  [+] Means sibling invalidation set.
  //  [>+] Means sibling descendant invalidation set.
  //
  // Examples:
  //
  //      .a[>] { ... } - Descendant invalidation set class |a|.
  //      #a[+] { ... } - Sibling invalidation set for id |a|
  //  [name][>] { ... } - Descendant invalidation set for attribute |name|.
  //  :hover[>] { ... } - Descendant set for pseudo-class |hover|.
  //       *[+] { ... } - Universal sibling invalidation set.
  //    nth[+>] { ... } - Nth sibling descendant invalidation set.
  //    type[>] { ... } - Type rule invalidation set.
  //
  // META flags (omitted if false):
  //
  //  F - Uses first line rules.
  //  W - Uses window inactive selector.
  //  R - Needs full recalc for ruleset invalidation.
  //  P - Invalidates parts.
  //  ~ - Max direct siblings is kDirectAdjacentMax.
  //  <integer> - Max direct siblings is specified number (omitted if 0).
  //
  // See InvalidationSet::ToString for more information.
  String ToString() const;

 private:
  static void ExtractInvalidationSets(InvalidationSet* invalidation_set,
                                      DescendantInvalidationSet*& descendants,
                                      SiblingInvalidationSet*& siblings);

  // Each map entry is either a DescendantInvalidationSet or
  // SiblingInvalidationSet.
  // When both are needed, we store the SiblingInvalidationSet, and use it to
  // hold the DescendantInvalidationSet.
  using InvalidationSetMap =
      HashMap<AtomicString, scoped_refptr<InvalidationSet>>;
  using PseudoTypeInvalidationSetMap =
      HashMap<CSSSelector::PseudoType,
              scoped_refptr<InvalidationSet>,
              IntWithZeroKeyHashTraits<unsigned>>;
  using ValuesInHasArgument = HashSet<AtomicString>;
  using PseudosInHasArgument = HashSet<CSSSelector::PseudoType>;

  // Class and ID invalidation have a special rule that is different from the
  // other sets; we do not store self-invalidation entries directly, but as a
  // Bloom filter (which can have false positives) keyed on the class/ID name's
  // AtomicString hash (multiplied with kClassSalt or kIdSalt).
  //
  // The reason is that some pages have huge amounts of simple rules of the type
  // “.foo { ...rules... }”, which would cause one such entry (consisting of the
  // self-invalidation bit only) per class rule. Dropping them and making them
  // implicit saves a lot of memory for such sites; the downside is that we can
  // get false positives. (For our 2 kB Bloom filter with two hash functions
  // and 16384 slots, we can store about 2000 such classes with a 95% rejection
  // rate. For 10000 classes, the rejection rate drops to 50%.)
  //
  // In particular, if you have an element and set class="bar" and there is no
  // rule for .bar, you may still get self-invalidation for the element. Worse,
  // when inserting a new style sheet or inserting/deleting rules, _any_ element
  // with class="" can get self-invalidated unless the Bloom filter stops it
  // (which depends strongly on how many such classes there are). So this is a
  // tradeoff. We could perhaps be more intelligent about not inserting into the
  // Bloom filter if we had to insert sibling or descendant sets too, but this
  // seems a bit narrow in practice.
  InvalidationSetMap class_invalidation_sets;

  std::unique_ptr<WTF::BloomFilter<14>> names_with_self_invalidation;
  static constexpr int kClassSalt = 13;
  static constexpr int kIdSalt = 29;

  // We don't create the Bloom filter right away; there may be so few of
  // them that we don't really bother. This number counts the times we've
  // inserted something that could go in there; once it reaches 50
  // (for this style sheet), we create the Bloom filter and start
  // inserting there instead. Note that we don't _remove_ any of the sets,
  // though; they will remain. This also means that when merging the
  // RuleFeatureSets into the global one, we can go over 50 such entries
  // in total.
  unsigned num_candidates_for_names_bloom_filter = 0;

  InvalidationSetMap attribute_invalidation_sets;
  InvalidationSetMap
      id_invalidation_sets;  // See comment on class_invalidation_sets_.
  PseudoTypeInvalidationSetMap pseudo_invalidation_sets;
  scoped_refptr<SiblingInvalidationSet> universal_sibling_invalidation_set;
  scoped_refptr<NthSiblingInvalidationSet> nth_invalidation_set;

  ValuesInHasArgument classes_in_has_argument;
  ValuesInHasArgument attributes_in_has_argument;
  ValuesInHasArgument ids_in_has_argument;
  ValuesInHasArgument tag_names_in_has_argument;
  PseudosInHasArgument pseudos_in_has_argument;

  // Metadata collected from rules, during either selector pre-match or
  // invaidation set construction.
  unsigned max_direct_adjacent_selectors = 0;
  bool uses_first_line_rules = false;
  bool uses_window_inactive_selector = false;
  bool universal_in_has_argument = false;
  // We always need to invalidate on insertion/removal when we have :not()
  // inside :has().
  bool not_pseudo_in_has_argument = false;

  bool invalidates_parts = false;
  // If we have a selector on the form :nth-child(... of :has(S)), any element
  // changing S will trigger that the element is "affected by subject of
  // :has", and in turn, will look up the tree for anything affected by
  // :nth-child to invalidate its siblings. However, this mechanism is
  // frequently too broad; if we have two separate rules :nth-child(an+b)
  // (without complex selector) and :has(S), such :has() invalidation will not
  // be able to distinguish between an element actually having a
  // :nth-child(... of :has(S)), and just being affected by
  // (forward-)positional rules for entirely different reasons. Thus, we will
  // over-invalidate a lot (crbug.com/1426750). Since this combination is
  // fairly rare, we make a per-stylesheet stop gap solution: We note whether
  // there are any such has-inside-nth-child rules. If not, we don't need to
  // do :nth-child() invalidation of elements affected by :has(). Of course,
  // this means that the existence of a single such rule will potentially send
  // us over the performance cliff, so we may have to revisit this solution in
  // the future.
  bool uses_has_inside_nth = false;

  friend class RuleFeatureSet;
  friend class RuleFeatureSetTest;
  friend class RuleInvalidationDataBuilder;
  template <RuleInvalidationDataVisitorType VisitorType>
  friend class RuleInvalidationDataVisitor;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_H_
