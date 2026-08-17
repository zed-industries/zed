// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_VISITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_VISITOR_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/css/invalidation/rule_invalidation_data.h"
#include "third_party/blink/renderer/core/css/invalidation/selector_pre_match.h"

namespace blink {

class CSSSelector;
class StyleScope;

enum class RuleInvalidationDataVisitorType { kBuilder, kTracer };

// This template is used in two different scenarios:
// 1. Builder: Iterates over CSS selectors and constructs InvalidationSets and
//    related data based on their contents. See comments on class RuleFeatureSet
//    for a more detailed breakdown of how these work.
// 2. Tracer: Follows the same logic as Builder while looking at previously
//    constructed RuleInvalidationData, without modifying it. The use case for
//    this is reconstructing the relationships between style rules and
//    invalidation data for developer tooling.
//
// The default assumption, and vast majority of usage, is the Builder scenario.
// Method names reflect this assumption; they are held over from when this was
// the only scenario. So in the Tracer scenario, 'Add' does not actually add
// anything and 'Ensure' does not fill in a missing entry. Rather, they reflect
// steps taken by the Builder that the Tracer is following in.
template <RuleInvalidationDataVisitorType VisitorType>
class RuleInvalidationDataVisitor {
  STACK_ALLOCATED();

 public:
  // Creates invalidation sets for the given CSS selector. This is done as part
  // of creating the RuleSet for the style sheet, i.e., before matching or
  // mutation begins.
  SelectorPreMatch CollectFeaturesFromSelector(const CSSSelector& selector,
                                               const StyleScope* style_scope);

 protected:
  static constexpr bool is_builder() {
    return VisitorType == RuleInvalidationDataVisitorType::kBuilder;
  }

  // To protect the data in the Tracer scenario, we hold it as const.
  // Declare type names that are mutable in Builder and const in Tracer.
  using RuleInvalidationDataType =
      std::conditional<is_builder(),
                       RuleInvalidationData,
                       const RuleInvalidationData>::type;
  using InvalidationSetType = std::
      conditional<is_builder(), InvalidationSet, const InvalidationSet>::type;
  using SiblingInvalidationSetType =
      std::conditional<is_builder(),
                       SiblingInvalidationSet,
                       const SiblingInvalidationSet>::type;
  using InvalidationSetMapType =
      std::conditional<is_builder(),
                       RuleInvalidationData::InvalidationSetMap,
                       const RuleInvalidationData::InvalidationSetMap>::type;
  using PseudoTypeInvalidationSetMapType = std::conditional<
      is_builder(),
      RuleInvalidationData::PseudoTypeInvalidationSetMap,
      const RuleInvalidationData::PseudoTypeInvalidationSetMap>::type;

  explicit RuleInvalidationDataVisitor(RuleInvalidationDataType&);

  struct FeatureMetadata {
    DISALLOW_NEW();
    bool uses_first_line_rules = false;
    bool uses_window_inactive_selector = false;
    unsigned max_direct_adjacent_selectors = 0;
  };
  SelectorPreMatch CollectMetadataFromSelector(
      const CSSSelector&,
      unsigned max_direct_adjacent_selectors,
      FeatureMetadata&);
  void CollectMetadataFromSelectorList(const CSSSelector* selector_list,
                                       unsigned max_direct_adjacent_selectors,
                                       FeatureMetadata&);

  void UpdateInvalidationSets(const CSSSelector&, const StyleScope*);

  // This class is used during collection of features from a given selector;
  // it collects a subset of referenced classes, IDs etc. from the subject
  // of the selector (see e.g. ExtractInvalidationSetFeaturesFromCompound()).
  // It is heavily used during feature set construction (i.e., initial style
  // calculation) but not stored except for the stack, which is why it is
  // fine to have inline sizes on the vectors.
  struct InvalidationSetFeatures {
    STACK_ALLOCATED();

   public:
    void Merge(const InvalidationSetFeatures& other);
    bool HasFeatures() const;
    bool HasIdClassOrAttribute() const;

    void NarrowToClass(const AtomicString& class_name) {
      if (Size() == 1 && (!ids.empty() || !classes.empty())) {
        return;
      }
      ClearFeatures();
      classes.push_back(class_name);
    }
    void NarrowToAttribute(const AtomicString& attribute) {
      if (Size() == 1 &&
          (!ids.empty() || !classes.empty() || !attributes.empty())) {
        return;
      }
      ClearFeatures();
      attributes.push_back(attribute);
    }
    void NarrowToId(const AtomicString& id) {
      if (Size() == 1 && !ids.empty()) {
        return;
      }
      ClearFeatures();
      ids.push_back(id);
    }
    void NarrowToTag(const AtomicString& tag_name) {
      if (Size() == 1) {
        return;
      }
      ClearFeatures();
      tag_names.push_back(tag_name);
    }
    void NarrowToFeatures(const InvalidationSetFeatures&);
    void ClearFeatures() {
      classes.clear();
      attributes.clear();
      ids.clear();
      tag_names.clear();
      emitted_tag_names.clear();
    }
    unsigned Size() const {
      return classes.size() + attributes.size() + ids.size() +
             tag_names.size() + emitted_tag_names.size();
    }

    Vector<AtomicString, 4> classes;
    Vector<AtomicString, 4> attributes;
    Vector<AtomicString, 4> ids;
    Vector<AtomicString, 4> tag_names;
    Vector<AtomicString, 4> emitted_tag_names;
    unsigned max_direct_adjacent_selectors = 0;

    // descendant_features_depth is used while adding features for logical
    // combinations inside :has() pseudo class to determine whether the current
    // compound selector is in subject position or not.
    //
    // This field stores the number of child and descendant combinators
    // previously evaluated for updating features from combinator. Unlike
    // max_direct_adjacent_selectors field that indicates the max limit,
    // this field simply stores the number of child and descendant combinators.
    //
    // This field is used only for the logical combinations inside :has(), but
    // we need to count all the combinators in the entire selector so that we
    // can correctly determine whether a compound is in the subject position
    // or not.
    // (e.g. For '.a:has(:is(.b ~ .c))) .d', the descendant_features_depth for
    //  compound '.b' is not 0 but 1 since the descendant combinator was
    //  evaludated for updating features when moving from '.d' to '.a:has(...)')
    //
    // How to determine whether a compound is in subject position or not:
    // 1. If descendant_feature.descendant_features_depth > 0, then the compound
    //    is not in subject position.
    // 2. If descendant_feature.descendant_features_depth == 0,
    //   2.1. If sibling_features != nullptr, then the compound is not in
    //        subject position.
    //   2.2. Otherwise, the compound is in subject position.
    unsigned descendant_features_depth = 0;

    InvalidationFlags invalidation_flags;
    bool content_pseudo_crossing = false;
    bool has_nth_pseudo = false;
    bool has_features_for_rule_set_invalidation = false;
  };

  // Siblings which contain nested selectors (e.g. :is) only count as one
  // sibling on the level where the nesting pseudo appears. To calculate
  // the max direct adjacent count correctly for each level, we sometimes
  // need to reset the count at certain boundaries.
  //
  // Example: .a + :is(.b + .c, .d + .e) + .f
  //
  // When processing the above selector, the InvalidationSetFeatures produced
  // from '.f' is eventually passed to both '.b + .c' and '.d + .e' as a mutable
  // reference. Each of those selectors will then increment the max direct
  // adjacent counter, and without a timely reset, changes would leak from one
  // sub-selector to another. It would also leak out of the :is() pseudo,
  // resulting in the wrong count for '.a' as well.
  class AutoRestoreMaxDirectAdjacentSelectors {
    STACK_ALLOCATED();

   public:
    explicit AutoRestoreMaxDirectAdjacentSelectors(
        InvalidationSetFeatures* features)
        : features_(features),
          original_value_(features ? features->max_direct_adjacent_selectors
                                   : 0) {}
    ~AutoRestoreMaxDirectAdjacentSelectors() {
      if (features_) {
        features_->max_direct_adjacent_selectors = original_value_;
      }
    }

   private:
    InvalidationSetFeatures* features_;
    unsigned original_value_ = 0;
  };

  // While adding features to the invalidation sets for the complex selectors
  // in :is() inside :has(), we need to differentiate whether the :has() is in
  // subject position or not if there is no sibling_features.
  //
  // - case 1) .a:has(:is(.b ~ .c))     : Add features as if we have .b ~ .a
  // - case 2) .a:has(:is(.b ~ .c)) .d  : add features as if we have .b ~ .a .d
  //
  // For .b in case 1, we need to use descendant_features as sibling_features.
  // But for .b in case 2, we need to extract sibling features from the compound
  // selector containing the :has() pseudo class.
  //
  // By maintaining a descendant depth information to descendant_features
  // object, we can determine whether the current compound is in subject
  // position or not. The descendant features depth will be increased when
  // RuleFeatureSet meets descendant or child combinator while adding features.
  //
  // Example)
  // - .a:has(:is(.b ~ .c))         : At .b, the descendant_features_depth is 0
  // - .a:has(:is(.b ~ .c)) .d      : At .b, the descendant_features_depth is 1
  // - .a:has(:is(.b ~ .c)) .d ~ .e : At .b, the descendant_features_depth is 1
  // - .a:has(:is(.b ~ .c)) .d > .e : At .b, the descendant_features_depth is 2
  //
  // To keep the correct depth in the descendant_features object for each level
  // of nested logical combinations, this class is used.
  class AutoRestoreDescendantFeaturesDepth {
    STACK_ALLOCATED();

   public:
    explicit AutoRestoreDescendantFeaturesDepth(
        InvalidationSetFeatures* features)
        : features_(features),
          original_value_(features ? features->descendant_features_depth : 0) {}
    ~AutoRestoreDescendantFeaturesDepth() {
      if (features_) {
        features_->descendant_features_depth = original_value_;
      }
    }

   private:
    InvalidationSetFeatures* features_;
    unsigned original_value_ = 0;
  };

  // For .a :has(:is(.b .c)).d, the invalidation set for .b is marked as whole-
  // subtree-invalid because :has() is in subject position and evaluated before
  // .b. But the invalidation set for .a can have descendant class .d. In this
  // case, the descendant_features for the same compound selector can have two
  // different state of WholeSubtreeInvalid flag. To keep the correct flag,
  // this class is used.
  class AutoRestoreWholeSubtreeInvalid {
    STACK_ALLOCATED();

   public:
    explicit AutoRestoreWholeSubtreeInvalid(InvalidationSetFeatures& features)
        : features_(features),
          original_value_(features.invalidation_flags.WholeSubtreeInvalid()) {}
    ~AutoRestoreWholeSubtreeInvalid() {
      features_.invalidation_flags.SetWholeSubtreeInvalid(original_value_);
    }

   private:
    InvalidationSetFeatures& features_;
    bool original_value_;
  };

  // For :is(:host(.a), .b) .c, the invalidation set for .a should be marked
  // as tree-crossing, but the invalidation set for .b should not.
  class AutoRestoreTreeBoundaryCrossingFlag {
    STACK_ALLOCATED();

   public:
    explicit AutoRestoreTreeBoundaryCrossingFlag(
        InvalidationSetFeatures& features)
        : features_(features),
          original_value_(features.invalidation_flags.TreeBoundaryCrossing()) {}
    ~AutoRestoreTreeBoundaryCrossingFlag() {
      features_.invalidation_flags.SetTreeBoundaryCrossing(original_value_);
    }

   private:
    InvalidationSetFeatures& features_;
    bool original_value_;
  };

  // For :is(.a, :host-context(.b), .c) .d, the invalidation set for .c should
  // not be marked as insertion point crossing.
  class AutoRestoreInsertionPointCrossingFlag {
    STACK_ALLOCATED();

   public:
    explicit AutoRestoreInsertionPointCrossingFlag(
        InvalidationSetFeatures& features)
        : features_(features),
          original_value_(
              features.invalidation_flags.InsertionPointCrossing()) {}
    ~AutoRestoreInsertionPointCrossingFlag() {
      features_.invalidation_flags.SetInsertionPointCrossing(original_value_);
    }

   private:
    InvalidationSetFeatures& features_;
    bool original_value_;
  };

  // Extracts features for the given complex selector, and adds those features
  // the appropriate invalidation sets.
  //
  // The returned InvalidationSetFeatures contain the descendant features,
  // extracted from the rightmost compound selector.
  //
  // The PositionType indicates whether or not the complex selector resides
  // in the rightmost compound (kSubject), or anything to the left of that
  // (kAncestor). For example, for ':is(.a .b) :is(.c .d)', the nested
  // complex selector '.c .d' should be called with kSubject, and the '.a .b'
  // should be called with kAncestor.
  //
  // The PseudoType indicates whether or not we are inside a nested complex
  // selector. For example, for :is(.a .b), this function is called with
  // CSSSelector equal to '.a .b', and PseudoType equal to kPseudoIs.
  // For top-level complex selectors, the PseudoType is kPseudoUnknown.
  enum PositionType { kSubject, kAncestor };
  enum FeatureInvalidationType {
    kNormalInvalidation,
    kRequiresSubtreeInvalidation
  };
  FeatureInvalidationType UpdateInvalidationSetsForComplex(
      const CSSSelector&,
      bool in_nth_child,
      const StyleScope*,
      InvalidationSetFeatures&,
      PositionType,
      CSSSelector::PseudoType);

  void UpdateFeaturesFromCombinator(
      CSSSelector::RelationType,
      const CSSSelector* last_compound_selector_in_adjacent_chain,
      InvalidationSetFeatures& last_compound_in_adjacent_chain_features,
      InvalidationSetFeatures*& sibling_features,
      InvalidationSetFeatures& descendant_features,
      bool for_logical_combination_in_has,
      bool in_nth_child);
  void UpdateFeaturesFromStyleScope(
      const StyleScope&,
      InvalidationSetFeatures& descendant_features);
  void ExtractInvalidationSetFeaturesFromSimpleSelector(
      const CSSSelector&,
      InvalidationSetFeatures&);
  const CSSSelector* ExtractInvalidationSetFeaturesFromCompound(
      const CSSSelector&,
      InvalidationSetFeatures&,
      PositionType,
      bool for_logical_combination_in_has,
      bool in_nth_child);
  void ExtractInvalidationSetFeaturesFromSelectorList(const CSSSelector&,
                                                      bool in_nth_child,
                                                      InvalidationSetFeatures&,
                                                      PositionType);

  void AddFeaturesToInvalidationSets(
      const CSSSelector&,
      bool in_nth_child,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features);
  const CSSSelector* AddFeaturesToInvalidationSetsForCompoundSelector(
      const CSSSelector&,
      bool in_nth_child,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features);
  void AddFeaturesToInvalidationSetsForSimpleSelector(
      const CSSSelector& simple_selector,
      const CSSSelector& compound,
      bool in_nth_child,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features);
  void AddFeaturesToInvalidationSetsForSelectorList(
      const CSSSelector&,
      bool in_nth_child,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features);
  void AddFeaturesToInvalidationSetsForStyleScope(
      const StyleScope&,
      InvalidationSetFeatures& descendant_features);
  void AddFeaturesToUniversalSiblingInvalidationSet(
      const InvalidationSetFeatures& sibling_features,
      const InvalidationSetFeatures& descendant_features);
  void AddValuesInComplexSelectorInsideIsWhereNot(
      const CSSSelector* selector_first);
  bool AddValueOfSimpleSelectorInHasArgument(
      const CSSSelector& has_pseudo_class);

  void CollectValuesInHasArgument(const CSSSelector& has_pseudo_class);

  // The logical combinations like ':is()', ':where()' and ':not()' can cause
  // a compound selector in ':has()' to match an element outside of the ':has()'
  // argument checking scope. (:has() anchor element, its ancestors, its
  // previous siblings or its ancestor previous siblings)
  // To support invalidation for a mutation on the elements, we can add features
  // in invalidation sets only for the complex selectors in :is() inside :has()
  // as if we have another rule with simple selector.
  //
  // Example 1) '.a:has(:is(.b .c)) {}'
  //   - For class 'b' change, invalidate descendant '.a' ('.b .a {}')
  //
  // Example 2) '.a:has(~ :is(.b ~ .c)) {}'
  //   - For class 'b' change, invalidate sibling '.a' ('.b ~ .a {}')
  //
  // Example 3) '.a:has(~ :is(.b ~ .c)) .d {}'
  //   - For class 'b' change, invalidate descendant '.d' of sibling '.a'.
  //     ('.b ~ .a .d {}')
  //
  // Example 4) '.a:has(:is(.b ~ .c .d)) {}'
  //   - For class 'b' change, invalidate descendant '.a' of sibling '.c'
  //     ('.b ~ .c .a {}'), and invalidate sibling '.a' ('.b ~ .a {}').
  void AddFeaturesToInvalidationSetsForHasPseudoClass(
      const CSSSelector& has_pseudo_class,
      const CSSSelector* compound_containing_has,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features,
      bool in_nth_child);

  // There are two methods to add features for logical combinations in :has().
  // - kForAllNonRightmostCompounds:
  //     Add features as if the non-subject part of the logical combination
  //     argument is prepended to the compound containing :has().
  //     (e.g. In the above example, Example 1, 2, 3 and '.b ~ .c .a' of
  //      Example 4)
  // - kForCompoundImmediatelyFollowsAdjacentRelation:
  //     Add features as if an adjacent combinator and its next compound
  //     selector are prepended to the compound containing :has().
  //     (e.g. In the above example, '.b ~ .a' of Example 4)
  //
  // Due to the difference between the two methods (how the features are
  // updated from combinators), sibling features or descendant features for
  // a certain compound can be different per the method.
  // - For '.a:has(:is(.b ~ .c .d)) ~ .e',
  //   - At '.b' when kForAllNonRightmostCompounds:
  //     - sibling_features == '.c' / descendant_features == '.e'
  //   - At '.b' when kForCompoundImmediatelyFollowsAdjacentRelation:
  //     - sibling_features == descendant_features == '.e'
  //
  // To avoid maintaining multiple 'sibling_features' and 'descendant_features'
  // for each compound selector, features are added separately for each method.
  // (Call AddFeaturesToInvalidationSetsForLogicalCombinationInHas() for each
  //  method in AddFeaturesToInvalidationSetsForHasPseudoClass())
  enum AddFeaturesMethodForLogicalCombinationInHas {
    kForAllNonRightmostCompounds,
    kForCompoundImmediatelyFollowsAdjacentRelation
  };

  // AddFeaturesToInvalidationSetsForLogicalCombinationInHas() is invoked for
  // each logical combination inside :has(). Same as the usual feature adding
  // logic, sibling features and descendant features extracted from the
  // previous compounds are passed though 'sibling_features' and
  // 'descendant_features' arguments.
  //
  // The rightmost compound of a non-nested logical combinations is always
  // in the :has() argument checking scope.
  // - '.c' in '.a:has(:is(.b .c) .d)' is always a descendant of :has() anchor
  //   element.
  //
  // But the rightmost compound of a nested logical combinations can be or
  // cannot be in the :has() argument checking scope.
  // - '.c' in '.a:has(:is(:is(.b .c) .d))' can be a :has() anchor element or
  //   its ancestor.
  // - '.d' in '.a:has(:is(.b :is(.c .d)))' is always a descendant of :has()
  //   anchor element.
  //
  // To differentiate between the two cases, this method has an argument
  // 'previous_combinator' that represents the previous combinator evaluated
  // for updating features for logical combination inside :has().
  // The argument is always kSubSelector when the method is called for the
  // non-nested logical combinations inside :has() (when the method is called
  // in AddFeaturesToInvalidationSetsForHasPseudoClass()).
  // For the rest compounds, after the rightmost compound is skipped, the value
  // is changed to the combinator at the left of the compound.
  void AddFeaturesToInvalidationSetsForLogicalCombinationInHas(
      const CSSSelector& logical_combination,
      const CSSSelector* compound_containing_has,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features,
      CSSSelector::RelationType previous_combinator,
      AddFeaturesMethodForLogicalCombinationInHas);

  void UpdateFeaturesFromCombinatorForLogicalCombinationInHas(
      CSSSelector::RelationType combinator,
      const CSSSelector* last_compound_selector_in_adjacent_chain,
      InvalidationSetFeatures& last_compound_in_adjacent_chain_features,
      InvalidationSetFeatures*& sibling_features,
      InvalidationSetFeatures& descendant_features);
  const CSSSelector* SkipAddingAndGetLastInCompoundForLogicalCombinationInHas(
      const CSSSelector* compound_in_logical_combination,
      const CSSSelector* compound_containing_has,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features,
      CSSSelector::RelationType previous_combinator,
      AddFeaturesMethodForLogicalCombinationInHas);
  const CSSSelector* AddFeaturesAndGetLastInCompoundForLogicalCombinationInHas(
      const CSSSelector* compound_in_logical_combination,
      const CSSSelector* compound_containing_has,
      InvalidationSetFeatures* sibling_features,
      InvalidationSetFeatures& descendant_features,
      CSSSelector::RelationType previous_combinator,
      AddFeaturesMethodForLogicalCombinationInHas);
  // Go recursively through everything in the given selector
  // (which is typically an ancestor; see the class comment)
  // and mark the invalidation sets of any simple selectors within it
  // for Nth-child invalidation.
  void MarkInvalidationSetsWithinNthChild(const CSSSelector& selector,
                                          bool in_nth_child);

  InvalidationSetType* InvalidationSetForSimpleSelector(const CSSSelector&,
                                                        InvalidationType,
                                                        PositionType,
                                                        bool in_nth_child);

  InvalidationSetType* EnsureClassInvalidationSet(
      const AtomicString& class_name,
      InvalidationType,
      PositionType,
      bool in_nth_child);
  InvalidationSetType* EnsureAttributeInvalidationSet(
      const AtomicString& attribute_name,
      InvalidationType,
      PositionType,
      bool in_nth_child);
  InvalidationSetType* EnsureIdInvalidationSet(const AtomicString& id,
                                               InvalidationType,
                                               PositionType,
                                               bool in_nth_child);
  InvalidationSetType* EnsurePseudoInvalidationSet(CSSSelector::PseudoType,
                                                   InvalidationType,
                                                   PositionType,
                                                   bool in_nth_child);

  InvalidationSetType* EnsureInvalidationSet(InvalidationSetMapType&,
                                             const AtomicString& key,
                                             InvalidationType,
                                             PositionType,
                                             bool in_nth_child);
  InvalidationSetType* EnsureInvalidationSet(PseudoTypeInvalidationSetMapType&,
                                             CSSSelector::PseudoType key,
                                             InvalidationType,
                                             PositionType,
                                             bool in_nth_child);
  SiblingInvalidationSetType* EnsureUniversalSiblingInvalidationSet();
  SiblingInvalidationSetType* EnsureNthInvalidationSet();

  void AddFeaturesToInvalidationSet(InvalidationSetType*,
                                    const InvalidationSetFeatures&);
  static void SetWholeSubtreeInvalid(InvalidationSetType* invalidation_set);
  static void SetInvalidatesSelf(InvalidationSetType* invalidation_set);
  static void SetInvalidatesNth(InvalidationSetType* invalidation_set);
  static void UpdateMaxDirectAdjacentSelectors(
      SiblingInvalidationSetType* invalidation_set,
      unsigned value);

  // Inserts the given value as a key for self-invalidation.
  // Return true if the insertion was successful. (It may fail because
  // there is no Bloom filter yet.)
  bool InsertIntoSelfInvalidationBloomFilter(const AtomicString& value,
                                             int salt);

  static InvalidationSetType* EnsureSiblingDescendantInvalidationSet(
      SiblingInvalidationSetType* invalidation_set);

  // Make sure that the pointer in `invalidation_set` has a single
  // reference that can be modified safely. (This is done through
  // copy-on-write, if needed, so that it can be modified without
  // disturbing unrelated invalidation sets that shared the pointer.)
  // If invalidation_set is nullptr, a new one is created. If an existing
  // InvalidationSet is used as base, it is extended to the right type
  // (descendant, sibling, self -- n-th sibling is treated as sibling)
  // if needed.
  //
  // The return value is the invalidation set to be modified. This is
  // identical to the new value of invalidation_set in all cases _except_
  // if the existing invalidation was a sibling invalidation set and
  // you requested a descendant invalidation set -- if so, it is a reference
  // to the DescendantInvalidationSet embedded within that set.
  // In other words, you must ignore the value of invalidation_set
  // after this function, since it is not what you requested.
  static InvalidationSet& EnsureMutableInvalidationSet(
      InvalidationType type,
      PositionType position,
      bool in_nth_child,
      scoped_refptr<InvalidationSet>& invalidation_set);

  struct AddFeaturesToInvalidationSetsForLogicalCombinationInHasContext;

  RuleInvalidationDataType& rule_invalidation_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_INVALIDATION_RULE_INVALIDATION_DATA_VISITOR_H_
