/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#include "base/memory/stack_allocated.h"

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_H_

#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/substring_set_matcher/substring_set_matcher.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cascade_layer.h"
#include "third_party/blink/renderer/core/css/css_keyframes_rule.h"
#include "third_party/blink/renderer/core/css/css_position_try_rule.h"
#include "third_party/blink/renderer/core/css/media_query_evaluator.h"
#include "third_party/blink/renderer/core/css/resolver/media_query_result.h"
#include "third_party/blink/renderer/core/css/robin_hood_map.h"
#include "third_party/blink/renderer/core/css/rule_feature_set.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/core/css/style_rule_counter_style.h"
#include "third_party/blink/renderer/core/css/style_rule_font_feature_values.h"
#include "third_party/blink/renderer/core/css/style_rule_font_palette_values.h"
#include "third_party/blink/renderer/core/css/style_rule_view_transition.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_stack.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/size_assertions.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class RuleSet;

using AddRuleFlags = unsigned;

enum AddRuleFlag {
  kRuleHasNoSpecialState = 0,
  kRuleIsVisitedDependent = 1 << 0,
  kRuleIsStartingStyle = 1 << 1,
};

// Some CSS properties do not apply to certain pseudo-elements, and need to be
// ignored when resolving styles. Be aware that these values are used in a
// bitfield. Make sure that it's large enough to hold new values.
// See MatchedProperties::Data::valid_property_filter.
enum class ValidPropertyFilter : unsigned {
  // All properties are valid. This is the common case.
  kNoFilter,
  // Defined in a ::cue pseudo-element scope. Only properties listed
  // in https://w3c.github.io/webvtt/#the-cue-pseudo-element are valid.
  kCue,
  // Defined in a ::first-letter pseudo-element scope. Only properties listed in
  // https://drafts.csswg.org/css-pseudo-4/#first-letter-styling are valid.
  kFirstLetter,
  // Defined in a ::first-line pseudo-element scope. Only properties listed in
  // https://drafts.csswg.org/css-pseudo-4/#first-line-styling are valid.
  kFirstLine,
  // Defined in a ::marker pseudo-element scope. Only properties listed in
  // https://drafts.csswg.org/css-pseudo-4/#marker-pseudo are valid.
  kMarker,
  // Defined in a highlight pseudo-element scope like ::selection and
  // ::target-text. Theoretically only properties listed in
  // https://drafts.csswg.org/css-pseudo-4/#highlight-styling should be valid,
  // but for highlight pseudos using originating inheritance instead of
  // highlight inheritance we allow a different set of rules for
  // compatibility reasons.
  kHighlightLegacy,
  // Defined in a highlight pseudo-element scope like ::selection and
  // ::target-text. Only properties listed in
  // https://drafts.csswg.org/css-pseudo-4/#highlight-styling are valid.
  kHighlight,
  // Defined in a @position-try rule. Only properties listed in
  // https://drafts.csswg.org/css-anchor-position-1/#fallback-rule are valid.
  kPositionTry,
  // Defined in an @page rule.
  // See https://drafts.csswg.org/css-page-3/#page-property-list
  kPageContext,
};

class CSSSelector;
class MediaQueryEvaluator;
class StyleSheetContents;

// This is a wrapper around a StyleRule, pointing to one of the N complex
// selectors in the StyleRule. This allows us to treat each selector
// independently but still tie them back to the original StyleRule. If multiple
// selectors from a single rule match the same element we can see that as one
// match for the rule. It computes some information about the wrapped selector
// and makes it accessible cheaply.
class CORE_EXPORT RuleData {
  DISALLOW_NEW();

 public:
  // NOTE: If you move the RuleData to a different RuleSet
  // (and thus a different bloom_hash_backing from what you
  // give to the constructor), you will need to call
  // MovedToDifferentRuleSet() below. Otherwise,
  // DescendantSelectorIdentifierHashes() will return a slice
  // into a nonexistent backing (and GetPosition() will return
  // a bogus value, which cannot be used for Seeker lookups).
  RuleData(StyleRule*,
           unsigned selector_index,
           unsigned position,
           const StyleScope*,
           AddRuleFlags,
           Vector<uint16_t>& bloom_hash_backing);

  unsigned GetPosition() const { return position_; }
  StyleRule* Rule() const { return rule_.Get(); }
  const CSSSelector& Selector() const {
    return rule_->SelectorAt(selector_index_);
  }
  CSSSelector& MutableSelector() {
    return rule_->MutableSelectorAt(selector_index_);
  }
  unsigned SelectorIndex() const { return selector_index_; }
  bool IsEntirelyCoveredByBucketing() const {
    return is_entirely_covered_by_bucketing_;
  }
  void ComputeEntirelyCoveredByBucketing();
  void ResetEntirelyCoveredByBucketing();
  bool SelectorIsEasy() const { return is_easy_; }
  bool IsStartingStyle() const { return is_starting_style_; }

  unsigned Specificity() const { return specificity_; }
  unsigned LinkMatchType() const { return link_match_type_; }
  ValidPropertyFilter GetValidPropertyFilter() const {
    return static_cast<ValidPropertyFilter>(valid_property_filter_);
  }

  // Member functions related to the descendant Bloom filter.
  const base::span<const uint16_t> DescendantSelectorIdentifierHashes(
      const Vector<uint16_t>& backing) const {
    return UNSAFE_TODO({backing.data() + bloom_hash_pos_, bloom_hash_size_});
  }
  void ComputeBloomFilterHashes(const StyleScope* style_scope,
                                Vector<uint16_t>& backing);
  void MovedToDifferentRuleSet(const Vector<uint16_t>& old_backing,
                               Vector<uint16_t>& new_backing,
                               unsigned new_position);

  void Trace(Visitor*) const;

  // This number is picked fairly arbitrary. If lowered, be aware that there
  // might be sites and extensions using style rules with selector lists
  // exceeding the number of simple selectors to fit in this bitfield.
  // See https://crbug.com/312913 and https://crbug.com/704562
  static constexpr size_t kSelectorIndexBits = 13;

  // This number was picked fairly arbitrarily. We can probably lower it if we
  // need to. Some simple testing showed <100,000 RuleData's on large sites.
  static constexpr size_t kPositionBits = 18;

 private:
  Member<StyleRule> rule_;
  unsigned selector_index_ : kSelectorIndexBits;
  unsigned position_ : kPositionBits;
  unsigned unused_bit_ : 1;
  // 31 bits above (1 free bit).
  unsigned specificity_ : 24;
  unsigned link_match_type_ : 2;
  unsigned valid_property_filter_ : 3;
  unsigned is_entirely_covered_by_bucketing_ : 1;
  unsigned is_easy_ : 1;            // See EasySelectorChecker.
  unsigned is_starting_style_ : 1;  // Inside @starting-style {}.
  // 32 bits above

  // Reference into a slice of bloom_hash_backing_ in the parent RuleSet.
  // We can probably steal a couple of bits here if needed, but if you do,
  // remember to adjust the clamping in ComputeBloomFilterHashes() too.
  unsigned bloom_hash_size_ : 8;
  unsigned bloom_hash_pos_ : 24;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::RuleData)

namespace blink {

struct SameSizeAsRuleData {
  DISALLOW_NEW();
  Member<void*> a;
  unsigned b;
  unsigned c;
  unsigned d;
};

ASSERT_SIZE(RuleData, SameSizeAsRuleData);

// A memory-efficient and (fairly) cache-efficient mapping from bucket key
// (e.g. CSS class, tag name, attribute key, etc.) to a collection of rules
// (RuleData objects). It uses a vector as backing storage, and generally works
// in two phases:
//
//  - During RuleSet setup (before compaction), we simply add rules to the
//    back of the vector, ie., the elements will be in a random order.
//  - Before rule matching, we need to _compact_ the rule map. This is done
//    by grouping/sorting the vector by bucket, so that everything that belongs
//    to the same vector lives together and can easily be picked out.
//
// The normal flow is that you first add all rules, call Compact(), then call
// Find() as many times as you need. (Compact() is a moderately expensive
// operation, which is why we don't want to be doing it too much.) However, in
// certain cases related to UA stylesheets, we may need to insert new rules
// on-the-fly (e.g., when seeing a <video> element for the first time, we
// insert additional rules related to it); if so, you need to call Uncompact()
// before adding them, then Compact() again.
class RuleMap {
  DISALLOW_NEW();

 private:
  // A collection of rules that are in the same bucket. Before compaction,
  // they are scattered around in the bucket vector; after compaction,
  // each bucket will be contiguous.
  struct Extent {
    Extent() : bucket_number(0) {}
    union {
      // [0..num_buckets). Valid before compaction.
      unsigned bucket_number;

      // Into the backing vector. Valid after compaction.
      unsigned start_index;
    };

    // How many rules are in this bucket. Will naturally not change
    // by compaction.
    wtf_size_t length = 0;
  };

 public:
  // Returns false on failure (which should be very rare).
  bool Add(const AtomicString& key, const RuleData& rule_data);
  void AddFilteredRulesFromOtherSet(
      const RuleMap& other,
      const HeapHashSet<Member<StyleRule>>& only_include,
      const RuleSet& old_rule_set,
      RuleSet& new_rule_set);
  base::span<const RuleData> Find(const AtomicString& key) const {
    // Go through all the buckets and check for equality, brute force.
    // Note that we don't check for IsNull() to get an early abort
    // on empty buckets; the comparison of AtomicString is so cheap
    // that it actually benchmarks negatively. This loop typically
    // gets unrolled and inlined, resulting in a very tight lookup.
    const RobinHoodMap<AtomicString, Extent>::Bucket* bucket =
        buckets.Find(key);
    if (bucket == nullptr) {
      return {};
    } else {
      return UNSAFE_TODO(
          {backing.begin() + bucket->value.start_index, bucket->value.length});
    }
  }
  bool IsEmpty() const { return backing.empty(); }
  bool IsCompacted() const { return compacted; }

  void Compact();
  void Uncompact();

  void Trace(Visitor* visitor) const { visitor->Trace(backing); }

  struct ConstIterator {
    STACK_ALLOCATED();

   public:
    RobinHoodMap<AtomicString, Extent>::const_iterator sub_it;
    const RuleMap* rule_map;

    WTF::KeyValuePair<AtomicString, base::span<const RuleData>> operator*()
        const {
      return {sub_it->key, rule_map->GetRulesFromExtent(sub_it->value)};
    }
    bool operator==(const ConstIterator& other) const {
      DCHECK_EQ(rule_map, other.rule_map);
      return sub_it == other.sub_it;
    }
    bool operator!=(const ConstIterator& other) const {
      DCHECK_EQ(rule_map, other.rule_map);
      return sub_it != other.sub_it;
    }
    ConstIterator& operator++() {
      ++sub_it;
      return *this;
    }
  };
  ConstIterator begin() const { return {buckets.begin(), this}; }
  ConstIterator end() const { return {buckets.end(), this}; }

 private:
  base::span<RuleData> GetRulesFromExtent(Extent extent) {
    return UNSAFE_TODO({backing.begin() + extent.start_index, extent.length});
  }
  base::span<const RuleData> GetRulesFromExtent(Extent extent) const {
    return UNSAFE_TODO({backing.begin() + extent.start_index, extent.length});
  }
  base::span<unsigned> GetBucketNumberFromExtent(Extent extent) {
    return UNSAFE_TODO(
        {bucket_number_.begin() + extent.start_index, extent.length});
  }
  base::span<const unsigned> GetBucketNumberFromExtent(Extent extent) const {
    return UNSAFE_TODO(
        {bucket_number_.begin() + extent.start_index, extent.length});
  }

  RobinHoodMap<AtomicString, Extent> buckets;

  // Contains all the rules from all the buckets; after compaction,
  // they will be contiguous in memory and you can do easily lookups
  // on them through Find(); before, they are identified
  // by having the group number in bucket_data_.
  //
  // The inline size is, perhaps surprisingly, to reduce GC pressure
  // for _large_ vectors. Setting an inline size (other than zero)
  // causes Vector, and by extension HeapVector, to grow more
  // aggressively on push_back(), leading to fewer memory allocations
  // that need freeing. We call ShrinkToFit() on compaction, so the
  // excess increase (which would normally be the downside of this
  // strategy) is not a big problem for us.
  //
  // Of course, we also save a few allocations for the rule sets
  // that are tiny. Most RuleMaps are either ~1–2 entries or in
  // the hundreds/thousands.
  HeapVector<RuleData, 4> backing;

  // Used by RuleMap before compaction, to hold what bucket the corresponding
  // RuleData (by index) is to be sorted into (this member is 1:1 with backing).
  // After compaction, the vector is emptied to save memory.
  Vector<unsigned> bucket_number_;

  wtf_size_t num_buckets = 0;
  bool compacted = false;
};

// Holds RuleData objects. It partitions them into various indexed groups,
// e.g. it stores separately rules that match against id, class, tag, shadow
// host, etc. It indexes these by some key where possible, e.g. rules that match
// against tag name are indexed by that tag. Rules that don't fall into a
// specific group are appended to the "universal" rules. The grouping is done to
// optimize finding what rules apply to an element under consideration by
// ElementRuleCollector::CollectMatchingRules.
class CORE_EXPORT RuleSet final : public GarbageCollected<RuleSet> {
 public:
  RuleSet() = default;
  RuleSet(const RuleSet&) = delete;
  RuleSet& operator=(const RuleSet&) = delete;

  void AddRulesFromSheet(const StyleSheetContents*,
                         const MediaQueryEvaluator&,
                         CascadeLayer* = nullptr,
                         const StyleScope* = nullptr);

  // “within_mixin” means that we are currently adding this rule
  // as part of @apply in a mixin, and all rules we add must be
  // duplicated and reparented. This is also propagated through
  // AddChildRules().
  void AddStyleRule(StyleRule* style_rule,
                    StyleRule* parent_rule,
                    const MediaQueryEvaluator& medium,
                    AddRuleFlags add_rule_flags,
                    bool within_mixin,
                    const ContainerQuery* container_query = nullptr,
                    CascadeLayer* cascade_layer = nullptr,
                    const StyleScope* style_scope = nullptr);

  // Adds RuleDatas (and only RuleDatas) from the other set, but only if they
  // correspond to rules in “only_include”. This is used when creating diff
  // rulesets for invalidation, and the resulting RuleSets are not usable
  // for anything else. In particular, cascade layers are not copied
  // and RuleData offsets are not adjusted (so CascadePriority would be wrong
  // if merging RuleDatas from different RuleSets). This means that the only
  // thing you can really do with this RuleSet afterwards is
  // ElementRuleCollector's CheckIfAnyRuleMatches(); the regular
  // Collect*Rules() functions are bound to give you trouble.
  void AddFilteredRulesFromOtherSet(
      const RuleSet& other,
      const HeapHashSet<Member<StyleRule>>& only_include);

  void AddFilteredRulesFromOtherBucket(
      const RuleSet& other,
      const HeapVector<RuleData>& src,
      const HeapHashSet<Member<StyleRule>>& only_include,
      HeapVector<RuleData>* dst);

  const RuleFeatureSet& Features() const { return features_; }

  base::span<const RuleData> IdRules(const AtomicString& key) const {
    return id_rules_.Find(key);
  }
  base::span<const RuleData> ClassRules(const AtomicString& key) const {
    return class_rules_.Find(key);
  }
  bool HasAnyAttrRules() const { return !attr_rules_.IsEmpty(); }
  base::span<const RuleData> AttrRules(const AtomicString& key) const {
    return attr_rules_.Find(key);
  }
  bool CanIgnoreEntireList(base::span<const RuleData> list,
                           const AtomicString& key,
                           const AtomicString& value) const;
  base::span<const RuleData> TagRules(const AtomicString& key) const {
    return tag_rules_.Find(key);
  }
  base::span<const RuleData> UAShadowPseudoElementRules(
      const AtomicString& key) const {
    return ua_shadow_pseudo_element_rules_.Find(key);
  }
  base::span<const RuleData> LinkPseudoClassRules() const {
    return link_pseudo_class_rules_;
  }
  base::span<const RuleData> CuePseudoRules() const {
    return cue_pseudo_rules_;
  }
  base::span<const RuleData> FocusPseudoClassRules() const {
    return focus_pseudo_class_rules_;
  }
  base::span<const RuleData> FocusVisiblePseudoClassRules() const {
    return focus_visible_pseudo_class_rules_;
  }
  base::span<const RuleData> RootElementRules() const {
    return root_element_rules_;
  }
  base::span<const RuleData> UniversalRules() const { return universal_rules_; }
  base::span<const RuleData> ShadowHostRules() const {
    return shadow_host_rules_;
  }
  base::span<const RuleData> PartPseudoRules() const {
    return part_pseudo_rules_;
  }
  base::span<const RuleData> SelectorFragmentAnchorRules() const {
    return selector_fragment_anchor_rules_;
  }
  const HeapVector<Member<StyleRulePage>>& PageRules() const {
    return page_rules_;
  }
  const HeapVector<Member<StyleRuleFontFace>>& FontFaceRules() const {
    return font_face_rules_;
  }
  const HeapVector<Member<StyleRuleKeyframes>>& KeyframesRules() const {
    return keyframes_rules_;
  }
  const HeapVector<Member<StyleRuleProperty>>& PropertyRules() const {
    return property_rules_;
  }
  const HeapVector<Member<StyleRuleCounterStyle>>& CounterStyleRules() const {
    return counter_style_rules_;
  }
  const HeapVector<Member<StyleRuleFontPaletteValues>>& FontPaletteValuesRules()
      const {
    return font_palette_values_rules_;
  }
  const HeapVector<Member<StyleRuleFontFeatureValues>>& FontFeatureValuesRules()
      const {
    return font_feature_values_rules_;
  }
  const HeapVector<Member<StyleRuleViewTransition>>& ViewTransitionRules()
      const {
    return view_transition_rules_;
  }
  const HeapVector<Member<StyleRulePositionTry>>& PositionTryRules() const {
    return position_try_rules_;
  }
  const HeapVector<Member<StyleRuleFunction>>& FunctionRules() const {
    return function_rules_;
  }
  base::span<const RuleData> SlottedPseudoElementRules() const {
    return slotted_pseudo_element_rules_;
  }

  bool HasCascadeLayers() const { return implicit_outer_layer_ != nullptr; }
  const CascadeLayer& CascadeLayers() const {
    DCHECK(implicit_outer_layer_);
    return *implicit_outer_layer_;
  }

  unsigned RuleCount() const { return rule_count_; }

  void CompactRulesIfNeeded() {
    if (need_compaction_) {
      CompactRules();
    }
  }

  void AssertCompacted() const { DCHECK(!need_compaction_); }

  bool HasSlottedRules() const {
    return !slotted_pseudo_element_rules_.empty();
  }

  bool HasPartPseudoRules() const { return !part_pseudo_rules_.empty(); }

  bool HasBucketForStyleAttribute() const { return has_bucket_for_style_attr_; }

  bool MustCheckUniversalBucketForShadowHost() const {
    return must_check_universal_bucket_for_shadow_host_;
  }

  bool HasUAShadowPseudoElementRules() const {
    return !ua_shadow_pseudo_element_rules_.IsEmpty();
  }

  // If a single @scope rule covers all rules in this RuleSet,
  // returns the corresponding StyleScope rule, or returns nullptr otherwise.
  //
  // This is useful for rejecting entire RuleSets early when implicit @scopes
  // aren't in scope.
  //
  // See ElementRuleCollector::CanRejectScope.
  const StyleScope* SingleScope() const {
    if (scope_intervals_.size() == 1u) {
      const Interval<StyleScope>& interval = scope_intervals_.front();
      if (interval.start_position == 0) {
        return interval.value.Get();
      }
    }
    return nullptr;
  }

  bool DidMediaQueryResultsChange(const MediaQueryEvaluator& evaluator) const;

  // We use a vector of Interval<T> to represent that rules with positions
  // between start_position (inclusive) and the next Interval<T>'s
  // start_position (exclusive) share some property:
  //
  //   - If T = CascadeLayer, belong to the given layer.
  //   - If T = ContainerQuery, are predicated on the given container query.
  //   - If T = StyleScope, are declared in the given @style scope.
  //
  // We do this instead of putting the data directly onto the RuleData,
  // because most rules don't need these fields and websites can have a large
  // number of RuleData objects (30k+). Since neighboring rules tend to have the
  // same values for these (often nullptr), we save memory and cache space at
  // the cost of a some extra seeking through these lists when matching rules.
  template <class T>
  class Interval {
    DISALLOW_NEW();

   public:
    Interval(const T* passed_value, unsigned passed_position)
        : value(passed_value), start_position(passed_position) {}
    const Member<const T> value;
    const unsigned start_position = 0;

    void Trace(Visitor*) const;
  };

  const HeapVector<Interval<CascadeLayer>>& LayerIntervals() const {
    return layer_intervals_;
  }
  const HeapVector<Interval<ContainerQuery>>& ContainerQueryIntervals() const {
    return container_query_intervals_;
  }
  const HeapVector<Interval<StyleScope>>& ScopeIntervals() const {
    return scope_intervals_;
  }
  const Vector<uint16_t>& BloomHashBacking() const {
    return bloom_hash_backing_;
  }

#if DCHECK_IS_ON()
  void Show() const;
  const HeapVector<RuleData>& AllRulesForTest() const { return all_rules_; }
#endif  // DCHECK_IS_ON()

  void Trace(Visitor*) const;

 private:
  FRIEND_TEST_ALL_PREFIXES(RuleSetTest, RuleCountNotIncreasedByInvalidRuleData);
  FRIEND_TEST_ALL_PREFIXES(RuleSetTest, RuleDataPositionLimit);
  friend class RuleSetCascadeLayerTest;
  friend class RuleMap;  // For scope_intervals_ and
                         // NewlyAddedFromDifferentRuleSet().

  using SubstringMatcherMap =
      HashMap<AtomicString, std::unique_ptr<base::SubstringSetMatcher>>;

  void AddToRuleSet(const AtomicString& key, RuleMap&, const RuleData&);
  void AddToRuleSet(HeapVector<RuleData>&, const RuleData&);
  void AddPageRule(StyleRulePage*);
  void AddFontFaceRule(StyleRuleFontFace*);
  void AddKeyframesRule(StyleRuleKeyframes*);
  void AddPropertyRule(StyleRuleProperty*);
  void AddCounterStyleRule(StyleRuleCounterStyle*);
  void AddFontPaletteValuesRule(StyleRuleFontPaletteValues*);
  void AddFontFeatureValuesRule(StyleRuleFontFeatureValues*);
  void AddPositionTryRule(StyleRulePositionTry*);
  void AddFunctionRule(StyleRuleFunction*);
  void AddViewTransitionRule(StyleRuleViewTransition*);

  bool MatchMediaForAddRules(const MediaQueryEvaluator& evaluator,
                             const MediaQuerySet* media_queries);
  void AddChildRules(StyleRule* parent_rule,
                     base::span<const Member<StyleRuleBase>>,
                     const MediaQueryEvaluator& medium,
                     AddRuleFlags,
                     const ContainerQuery*,
                     CascadeLayer*,
                     const StyleScope*,
                     bool within_mixin);

  // Determines whether or not CSSSelector::is_covered_by_bucketing_ should
  // be computed during calls to FindBestRuleSetAndAdd.
  enum class BucketCoverage {
    kIgnore,
    kCompute,
  };

  template <BucketCoverage bucket_coverage>
  void FindBestRuleSetAndAdd(CSSSelector&, const RuleData&, const StyleScope*);

  void AddRule(StyleRule*,
               unsigned selector_index,
               AddRuleFlags,
               const ContainerQuery*,
               const CascadeLayer*,
               const StyleScope*);

  // Must be called when a RuleData has been added to this RuleSet
  // through some form that does not go through AddRule();
  // used during creation of diff rulesets (AddFilteredRulesFromOtherSet()).
  // In particular, it will adjust the position of new_rule_data,
  // add it to the necessary intervals for diff rulesets, and adjust
  // rule_count_.
  //
  // Used only by RuleSet itself, and RuleMap (through a friend declaration).
  void NewlyAddedFromDifferentRuleSet(const RuleData& old_rule_data,
                                      const StyleScope* style_scope,
                                      const RuleSet& old_rule_set,
                                      RuleData& new_rule_data);

  void SortKeyframesRulesIfNeeded();

  void CompactRules();
  static void CreateSubstringMatchers(
      RuleMap& attr_map,
      const HeapVector<Interval<StyleScope>>& scope_intervals,
      SubstringMatcherMap& substring_matcher_map);

#if DCHECK_IS_ON()
  void AssertRuleListsSorted() const;
#endif

  CascadeLayer* EnsureImplicitOuterLayer() {
    if (!implicit_outer_layer_) {
      implicit_outer_layer_ = MakeGarbageCollected<CascadeLayer>();
    }
    return implicit_outer_layer_.Get();
  }

  CascadeLayer* GetOrAddSubLayer(CascadeLayer*,
                                 const StyleRuleBase::LayerName&);
  void AddRuleToLayerIntervals(const CascadeLayer*, unsigned position);

  // May return nullptr for the implicit outer layer.
  const CascadeLayer* GetLayerForTest(const RuleData&) const;

  RuleMap id_rules_;
  RuleMap class_rules_;
  RuleMap attr_rules_;
  // A structure for quickly rejecting an entire attribute rule set
  // (from attr_rules_). If we have many rules in the same bucket,
  // we build up a case-insensitive substring-matching structure of all
  // the values we can match on (all attribute selectors are either substring,
  // or something stricter than substring). We can then use that structure
  // to see in linear time (of the length of the attribute value in the DOM)
  // whether we can have any matches at all.
  //
  // If we find any matches, we need to recheck each rule, because the rule in
  // question may actually be case-sensitive, or we might want e.g. a prefix
  // match instead of a substring match. (We could solve prefix/suffix by
  // means of inserting special start-of-string and end-of-string tokens,
  // but we keep it simple for now.) Also, the way we use the
  // SubstringSetMatcher, we don't actually get back which rules matched.
  //
  // This element does not exist, if there are few enough rules that we don't
  // deem this step worth it, or if the build of the tree failed. (In
  // particular, if there is only a single rule in this bucket, it's
  // pointless to run the entire Aho-Corasick algorithm instead of just
  // doing a simple match.) Check GetMinimumRulesetSizeForSubstringMatcher()
  // before looking up for a cheaper test.
  SubstringMatcherMap attr_substring_matchers_;
  RuleMap tag_rules_;
  RuleMap ua_shadow_pseudo_element_rules_;
  HeapVector<RuleData> link_pseudo_class_rules_;
  HeapVector<RuleData> cue_pseudo_rules_;
  HeapVector<RuleData> focus_pseudo_class_rules_;
  HeapVector<RuleData> focus_visible_pseudo_class_rules_;
  HeapVector<RuleData> universal_rules_;
  HeapVector<RuleData> shadow_host_rules_;
  HeapVector<RuleData> part_pseudo_rules_;
  HeapVector<RuleData> slotted_pseudo_element_rules_;
  HeapVector<RuleData> selector_fragment_anchor_rules_;
  HeapVector<RuleData> root_element_rules_;
  RuleFeatureSet features_;
  HeapVector<Member<StyleRulePage>> page_rules_;
  HeapVector<Member<StyleRuleFontFace>> font_face_rules_;
  HeapVector<Member<StyleRuleFontPaletteValues>> font_palette_values_rules_;
  HeapVector<Member<StyleRuleFontFeatureValues>> font_feature_values_rules_;
  HeapVector<Member<StyleRuleViewTransition>> view_transition_rules_;
  HeapVector<Member<StyleRuleKeyframes>> keyframes_rules_;
  HeapVector<Member<StyleRuleProperty>> property_rules_;
  HeapVector<Member<StyleRuleCounterStyle>> counter_style_rules_;
  HeapVector<Member<StyleRulePositionTry>> position_try_rules_;
  HeapVector<MediaQuerySetResult> media_query_set_results_;
  HeapVector<Member<StyleRuleFunction>> function_rules_;
  HeapHashMap<AtomicString, Member<StyleRuleMixin>> mixins_;

  // Whether there is a ruleset bucket for rules with a selector on
  // the style attribute (which is rare, but allowed). If so, the caller
  // may need to take extra steps to synchronize the style attribute on
  // an element before looking for appropriate buckets.
  bool has_bucket_for_style_attr_ = false;

  // Whether we need to check the universal bucket for rules when calculating
  // style for the shadow host. There are two reasons why this may need to
  // be checked:
  //
  // 1. Since the :scope pseudo-class can match a shadow host when that host
  //    is the scoping root,
  //    ElementRuleCollector::CollectMatchingShadowHostRules also needs to
  //    collect rules from the universal bucket, but this is only required when
  //    :scope is actually present.
  //
  // 2. Combination rules such as :is(:host, .foo) will be bucketed into the
  //    universal bucket and not into the bucket for :host. If this happens,
  //    we will need to check the universal bucket, too.
  //
  // Nothing else in the universal bucket can match the host from inside
  // the shadow tree.
  bool must_check_universal_bucket_for_shadow_host_ = false;

  unsigned rule_count_ = 0;
  bool need_compaction_ = false;

  // nullptr if the stylesheet doesn't explicitly declare any layer.
  Member<CascadeLayer> implicit_outer_layer_;
  // Empty vector if the stylesheet doesn't explicitly declare any layer.
  HeapVector<Interval<CascadeLayer>> layer_intervals_;
  // Empty vector if the stylesheet doesn't use any container queries.
  HeapVector<Interval<ContainerQuery>> container_query_intervals_;
  // Empty vector if the stylesheet doesn't use any @scopes.
  HeapVector<Interval<StyleScope>> scope_intervals_;

  // Backing store for the Bloom filter hashes for each RuleData.
  // It is stored here so that we can have a variable number of them
  // (without the overhead of a Vector in each RuleData).
  Vector<uint16_t> bloom_hash_backing_;

#if DCHECK_IS_ON()
  HeapVector<RuleData> all_rules_;

  // If true, we don't DCHECK that these are unsorted, since they
  // came from merged+filtered rulesets, which only happens when
  // making diff rulesets for invalidation. Those do not care
  // about the ordering, since they do not use the CascadeLayerSeeker.
  bool allow_unsorted_ = false;
#endif  // DCHECK_IS_ON()
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::RuleSet::Interval<blink::CascadeLayer>)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::RuleSet::Interval<blink::ContainerQuery>)
WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::RuleSet::Interval<blink::StyleScope>)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_H_
