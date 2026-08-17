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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ELEMENT_RULE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ELEMENT_RULE_COLLECTOR_H_

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/container_selector.h"
#include "third_party/blink/renderer/core/css/css_rule_list.h"
#include "third_party/blink/renderer/core/css/part_names.h"
#include "third_party/blink/renderer/core/css/resolver/match_request.h"
#include "third_party/blink/renderer/core/css/resolver/match_result.h"
#include "third_party/blink/renderer/core/css/selector_checker.h"
#include "third_party/blink/renderer/core/css/style_recalc_context.h"
#include "third_party/blink/renderer/core/css/style_request.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Element;
class ElementResolveContext;
class ElementRuleCollector;
class RuleData;
class SelectorFilter;
class StyleRuleUsageTracker;

class MatchedRule {
  DISALLOW_NEW();

  // Everything in this class is private to ElementRuleCollector, since it
  // contains non-owned references to RuleData (see the constructor), but we
  // cannot make the class itself private, since
  // WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS() and Vector::emplace_back()
  // need it to be visible from the outside.
  static const unsigned kBitsForPositionInRuleData = 18;

 public:
  // Does not take overship of rule_data (it is owned by the appropriate
  // bucket in RuleSet), so the RuleData must live for at least as long as
  // the MatchedRule, ie., those buckets must not be modified (which would
  // invalidate the RuleData pointers). This is fine, because MatchedRule
  // is only used during matching (in ElementRuleCollector), and the
  // RuleData itself never escapes SortAndTransferMatchedRules() -- only
  // the other elements that it points to.
  MatchedRule(const RuleData* rule_data,
              uint16_t layer_order,
              unsigned proximity,
              unsigned style_sheet_index)
      : sort_key_((static_cast<uint64_t>(layer_order) << 48) |
                  (static_cast<uint64_t>(rule_data->Specificity()) << 16) |
                  (65535 - ClampTo<uint16_t>(proximity))),
        position_((static_cast<uint64_t>(style_sheet_index)
                   << kBitsForPositionInRuleData) +
                  rule_data->GetPosition()),
        rule_(rule_data->Rule()),
        link_match_type_(rule_data->LinkMatchType()),
        valid_property_filter_(
            static_cast<unsigned>(rule_data->GetValidPropertyFilter())),
        selector_index_(rule_data->SelectorIndex()) {}

  void Trace(Visitor* visitor) const { visitor->Trace(rule_); }

 private:
  StyleRule* Rule() const { return rule_; }
  uint16_t LayerOrder() const { return sort_key_ >> 48; }
  uint64_t SortKey() const { return sort_key_; }
  uint64_t GetPosition() const { return position_; }  // Secondary sort key.
  unsigned LinkMatchType() const { return link_match_type_; }
  ValidPropertyFilter GetValidPropertyFilter(bool is_matching_ua_rules) const {
    return is_matching_ua_rules
               ? ValidPropertyFilter::kNoFilter
               : static_cast<ValidPropertyFilter>(valid_property_filter_);
  }
  unsigned SelectorIndex() const { return selector_index_; }

  // Used for tests only.
  const CSSSelector& Selector() const {
    return rule_->SelectorAt(selector_index_);
  }

 private:
  uint64_t sort_key_;
  uint64_t position_;

  Member<StyleRule> rule_;

  // NOTE: If we need some more spare bits, we can probably move some bits
  // in position_ upwards and use some of the bottom. Right now, though,
  // packing these better wouldn't make the struct any smaller, due to
  // alignment/padding.
  uint8_t link_match_type_;        // 2 bits needed.
  uint8_t valid_property_filter_;  // ValidPropertyFilter, 3 bits needed.
  uint16_t selector_index_;  // RuleData::kSelectorIndexBits (13) bits needed.

  friend class ElementRuleCollector;
  FRIEND_TEST_ALL_PREFIXES(ElementRuleCollectorTest, DirectNesting);
  FRIEND_TEST_ALL_PREFIXES(ElementRuleCollectorTest,
                           RuleNotStartingWithAmpersand);
  FRIEND_TEST_ALL_PREFIXES(ElementRuleCollectorTest, NestedRulesInMediaQuery);
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::MatchedRule)

namespace blink {

using StyleRuleList = GCedHeapVector<Member<StyleRule>>;

// Manages the process of finding what rules in a RuleSet apply to a given
// Element. These tend to be used several times in different contexts and should
// have ClearMatchedRules called before use.
//
// ElementRuleCollector is designed to be used as a stack object.
// Create one, ask what rules the ElementResolveContext matches
// and then let it go out of scope. In particular, do not change
// values in the RuleSet buckets (which would invalidate the RuleData
// pointers) before you have extracted the results, typically with
// SortAndTransferMatchedRules().
//
// FIXME: Currently it modifies the ComputedStyle but should not!
class CORE_EXPORT ElementRuleCollector {
  STACK_ALLOCATED();

 public:
  ElementRuleCollector(const ElementResolveContext&,
                       const StyleRecalcContext&,
                       const SelectorFilter&,
                       MatchResult&,
                       EInsideLink);
  ElementRuleCollector(const ElementRuleCollector&) = delete;
  ElementRuleCollector& operator=(const ElementRuleCollector&) = delete;
  ~ElementRuleCollector();

  void SetMode(SelectorChecker::Mode mode) { mode_ = mode; }
  void SetPseudoElementStyleRequest(const StyleRequest& request) {
    pseudo_style_request_ = request;
  }

  void SetMatchingUARules(bool matching_ua_rules) {
    matching_ua_rules_ = matching_ua_rules;
  }
  void SetMatchingRulesFromNoStyleSheet(
      bool matching_rules_from_no_style_sheet) {
    matching_rules_from_no_style_sheet_ = matching_rules_from_no_style_sheet;
  }
  // If true, :visited will never match. Has no effect otherwise.
  void SetSuppressVisited(bool suppress_visited) {
    suppress_visited_ = suppress_visited;
  }

  const MatchResult& MatchedResult() const;
  StyleRuleList* MatchedStyleRuleList();
  RuleIndexList* MatchedCSSRuleList();

  void CollectMatchingRules(const MatchRequest&, PartNames* part_names);
  void CollectMatchingShadowHostRules(const MatchRequest&);
  void CollectMatchingSlottedRules(const MatchRequest&);
  void CollectMatchingPartPseudoRules(const MatchRequest&, PartNames*);
  void SortAndTransferMatchedRules(CascadeOrigin origin,
                                   bool is_vtt_embedded_style,
                                   StyleRuleUsageTracker* tracker);
  void ClearMatchedRules();

  // Cheaper versions of CollectMatchingRules and CollectMatchingShadowHostRules
  // respectively, that only return true/false instead of actually collecting
  // the rules.
  bool CheckIfAnyRuleMatches(const MatchRequest&);
  bool CheckIfAnyShadowHostRuleMatches(const MatchRequest&);

  // True if an entire StyleScope can be rejected, i.e. all style rules
  // within the StyleScope are guaranteed to not match due to the given
  // StyleScope not being in scope [1].
  //
  // Return 'false' when we don't know if a StyleScope is in scope or not.
  //
  // [1] https://drafts.csswg.org/css-cascade-6/#in-scope
  bool CanRejectScope(const StyleScope&) const;

  void AddElementStyleProperties(const CSSPropertyValueSet*,
                                 CascadeOrigin,
                                 bool is_cacheable = true,
                                 bool is_inline_style = false);
  void AddTryStyleProperties();
  void AddTryTacticsStyleProperties();
  void BeginAddingAuthorRulesForTreeScope(const TreeScope& tree_scope) {
    current_matching_tree_scope_ = &tree_scope;
    result_.BeginAddingAuthorRulesForTreeScope(tree_scope);
  }

  // Return the pseudo id if the style request is for rules associated with a
  // pseudo element, or kPseudoNone if not.
  PseudoId GetPseudoId() const { return pseudo_style_request_.pseudo_id; }
  const AtomicString& GetPseudoArgument() const {
    return pseudo_style_request_.pseudo_argument;
  }

  void AddMatchedRulesToTracker(StyleRuleUsageTracker*) const;

  // Writes out the collected selector statistics and clears the values.
  // These values are gathered during rule matching and require higher-level
  // control of when they are output - the statistics are designed to be
  // aggregated per-rule for the entire style recalc pass.
  static void DumpAndClearRulesPerfMap();

  const HeapVector<MatchedRule, 32>& MatchedRulesForTest() const {
    return matched_rules_;
  }

 private:
  // If stop_at_first_match = true, CollectMatchingRules*() will stop
  // whenever any rule matches, return true, and not store the result
  // anywhere nor update the match counters. Otherwise, these functions
  // will return false (even if one or more rules matched).
  //
  // Note in the context of stop_at_first_match, a match against any
  // pseudo rule in the element counts as a match (e.g., “div::before”
  // will match the <div> element, not just its ::before pseudo-element).
  // This is convenient because this mode is used for invalidation on
  // changed rulesets only, where such a match causes us to have to
  // invalidate style on the element anyway.

  template <bool stop_at_first_match>
  bool CollectMatchingRulesInternal(const MatchRequest&, PartNames* part_names);

  template <bool stop_at_first_match, bool perf_trace_enabled>
  bool CollectMatchingRulesForListInternal(
      base::span<const RuleData>,
      const MatchRequest&,
      const RuleSet*,
      int,
      const SelectorChecker&,
      SelectorChecker::SelectorCheckingContext&);

  template <bool stop_at_first_match>
  bool CollectMatchingRulesForList(base::span<const RuleData>,
                                   const MatchRequest&,
                                   const RuleSet*,
                                   int,
                                   const SelectorChecker&,
                                   SelectorChecker::SelectorCheckingContext&);

  bool Match(SelectorChecker&,
             const SelectorChecker::SelectorCheckingContext&,
             MatchResult&);
  void DidMatchRule(const RuleData*,
                    uint16_t layer_order,
                    const ContainerQuery*,
                    unsigned proximity,
                    const SelectorChecker::MatchResult&,
                    int style_sheet_index);

  void AppendCSSOMWrapperForRule(const TreeScope* tree_scope_containing_rule,
                                 const MatchedRule& matched_rule,
                                 wtf_size_t position);

  void SortMatchedRules();

  RuleIndexList* EnsureRuleList();
  StyleRuleList* EnsureStyleRuleList();

 private:
  struct CompareRules;

  const ElementResolveContext& context_;
  StyleRecalcContext style_recalc_context_;
  const SelectorFilter& selector_filter_;

  StyleRequest pseudo_style_request_;
  SelectorChecker::Mode mode_;
  bool can_use_fast_reject_;
  bool matching_ua_rules_;
  bool matching_rules_from_no_style_sheet_ =
      false;  // Document rules and watched selectors.
  bool suppress_visited_;
  EInsideLink inside_link_;
  const TreeScope* current_matching_tree_scope_ = nullptr;

  HeapVector<MatchedRule, 32> matched_rules_;
  ContainerSelectorCache container_selector_cache_;

  // Output.
  RuleIndexList* css_rule_list_ = nullptr;
  StyleRuleList* style_rule_list_ = nullptr;
  MatchResult& result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ELEMENT_RULE_COLLECTOR_H_
