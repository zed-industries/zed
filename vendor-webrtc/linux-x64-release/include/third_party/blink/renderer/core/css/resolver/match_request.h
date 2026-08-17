/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_REQUEST_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_style_sheet.h"
#include "third_party/blink/renderer/core/css/rule_set.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/element.h"

namespace blink {

class ContainerNode;
class ElementRuleCollector;

// Encapsulates the context for matching against a group of style sheets
// by ElementRuleCollector. Carries the RuleSets and some precomputed
// information. This is fairly expensive to compute, so except for one-offs,
// it is expected that you precompute RuleSetGroups and reuse them across many
// elements (MatchRequest, below, is cheap to produce).
//
// We allow up to 32 style sheets in a group. More than one allows us to
// amortize checks on the element between style sheets (e.g. fetching its
// parents, or lowercasing attributes), and since we have a fixed upper limit,
// we can store information about them in bitmaps to be efficiently
// iterated over. (Having multiple bitmaps would of course be possible,
// but probably not worth it in iteration complexity.) Note that it is
// possible to check entire bitmaps for zero to avoid expensive precomputation
// altogether if no sheets need it; this is effectively free, since we need
// a zero check as part of the for loop anyway.
//
// All style sheets have an index, which are assumed to be consecutive.
class CORE_EXPORT RuleSetGroup {
  DISALLOW_NEW();

 public:
  using RuleSetBitmap = uint32_t;
  static constexpr unsigned kRulesetsRoom = CHAR_BIT * sizeof(RuleSetBitmap);

  explicit RuleSetGroup(unsigned rule_set_group_index)
      : style_sheet_first_index_(rule_set_group_index * kRulesetsRoom) {}

  // Note that a RuleSetGroup only has a finite capacity, so if IsFull()
  // returns true, you need to start a new one.
  // AddRuleSetToRuleSetGroupList() below helps with this, and is the
  // interface most callers should use instead of creating RuleSetGroups
  // directly.
  void AddRuleSet(RuleSet* rule_set);

  bool IsEmpty() const { return num_rule_sets_ == 0; }
  bool IsFull() const { return num_rule_sets_ == kRulesetsRoom; }

#if DCHECK_IS_ON()
  void AssertEqualTo(const RuleSetGroup& other) const {
    DCHECK_EQ(num_rule_sets_, other.num_rule_sets_);
    for (unsigned i = 0; i < num_rule_sets_; ++i) {
      DCHECK_EQ(rule_sets_[i], other.rule_sets_[i])
          << "Mismatched rule set " << i;
    }
    DCHECK_EQ(style_sheet_first_index_, other.style_sheet_first_index_);
    DCHECK_EQ(single_scope_, other.single_scope_);
    DCHECK_EQ(has_any_attr_rules_, other.has_any_attr_rules_);
    DCHECK_EQ(has_universal_rules_, other.has_universal_rules_);
    DCHECK_EQ(need_style_synchronized_, other.need_style_synchronized_);
    DCHECK_EQ(has_link_pseudo_class_rules_, other.has_link_pseudo_class_rules_);
    DCHECK_EQ(has_focus_pseudo_class_rules_,
              other.has_focus_pseudo_class_rules_);
    DCHECK_EQ(has_focus_visible_pseudo_class_rules_,
              other.has_focus_visible_pseudo_class_rules_);
  }
#endif

  void Trace(Visitor* visitor) const {
    for (auto& rule_set : rule_sets_) {
      visitor->Trace(rule_set);
    }
  }

 private:
  std::array<subtle::UncompressedMember<const RuleSet>, kRulesetsRoom>
      rule_sets_;
  unsigned num_rule_sets_ = 0;
  unsigned style_sheet_first_index_ = 0;

  // Which RuleSets are limited to a single @scope, and their converse.
  RuleSetBitmap single_scope_ = 0;
  RuleSetBitmap not_single_scope_ = 0;

  // Which RuleSets have any attribute rules at all.
  RuleSetBitmap has_any_attr_rules_ = 0;

  // Which RuleSets have any universal rules.
  RuleSetBitmap has_universal_rules_ = 0;

  // Which RuleSets have any :link pseudo-class rules.
  RuleSetBitmap has_link_pseudo_class_rules_ = 0;

  // Which RuleSets have any :focus pseudo-class rules.
  RuleSetBitmap has_focus_pseudo_class_rules_ = 0;

  // Which RuleSets have any :focus-visible pseudo-class rules.
  RuleSetBitmap has_focus_visible_pseudo_class_rules_ = 0;

  // Whether there are any attribute-bucketed rules that depend on the
  // [style] attribute.
  bool need_style_synchronized_ = false;

  friend class MatchRequest;
};

// Encapsulates the context for matching against a group of style sheets
// by ElementRuleCollector. Points to the scope (a ContainerNode)
// and the rule sets (in RuleSetGroup).
class CORE_EXPORT MatchRequest {
  STACK_ALLOCATED();

 public:
  // The template declaration of ElementRuleCollector is a somewhat hackish way
  // of avoiding a circular dependency between match_request.h and
  // element_rule_collector.h without resorting to an -inl.h file.
  template <class ElementRuleCollector>
  explicit MatchRequest(const RuleSetGroup& rule_set_group,
                        const ContainerNode* scope,
                        const ElementRuleCollector& collector,
                        Element* vtt_originating_element = nullptr)
      : rule_set_group_(rule_set_group),
        scope_(scope),
        vtt_originating_element_(vtt_originating_element),
        enabled_(rule_set_group.not_single_scope_) {
    for (const auto [rule_set, index] :
         RuleSetIteratorProxy(&rule_set_group, rule_set_group.single_scope_)) {
      if (!collector.CanRejectScope(*rule_set->SingleScope())) {
        enabled_ |= RuleSetGroup::RuleSetBitmap{1}
                    << (index - rule_set_group.style_sheet_first_index_);
      }
    }
  }

  // This simpler version never disables single-scope rulesets.
  MatchRequest(const RuleSetGroup& rule_set_group,
               const ContainerNode* scope,
               Element* vtt_originating_element = nullptr)
      : rule_set_group_(rule_set_group),
        scope_(scope),
        vtt_originating_element_(vtt_originating_element),
        enabled_(rule_set_group.single_scope_ |
                 rule_set_group.not_single_scope_) {}

  const ContainerNode* Scope() const { return scope_; }
  Element* VTTOriginatingElement() const { return vtt_originating_element_; }

  // Used for returning from RuleSetIterator; not actually stored.
  struct RuleSetWithIndex {
    STACK_ALLOCATED();

   public:
    const RuleSet* rule_set;
    unsigned style_sheet_index;
  };

  // An iterator over all the rule sets in a given bitmap, intended for use in
  // range-based for loops (use AllRuleSets()). The index is automatically
  // generated based on style_sheet_first_index_. RuleSets that are not part of
  // the bitmap are skipped over at no cost.
  class RuleSetIterator {
    STACK_ALLOCATED();

   public:
    RuleSetIterator(const RuleSetGroup* rule_set_group,
                    RuleSetGroup::RuleSetBitmap bitmap)
        : rule_set_group_(*rule_set_group), bitmap_(bitmap) {}

    RuleSetWithIndex operator*() const {
      DCHECK_NE(0u, bitmap_);
      unsigned index = std::countr_zero(bitmap_);
      return {rule_set_group_.rule_sets_[index],
              index + rule_set_group_.style_sheet_first_index_};
    }

    RuleSetIterator& operator++() {
      bitmap_ &= bitmap_ - 1;
      return *this;
    }

    bool operator==(const RuleSetIterator& other) const {
      DCHECK_EQ(&rule_set_group_, &other.rule_set_group_);
      return bitmap_ == other.bitmap_;
    }

    bool operator!=(const RuleSetIterator& other) const {
      DCHECK_EQ(&rule_set_group_, &other.rule_set_group_);
      return bitmap_ != other.bitmap_;
    }

   private:
    const RuleSetGroup& rule_set_group_;
    RuleSetGroup::RuleSetBitmap bitmap_;
  };

  // A proxy object to allow AllRuleSets() to be iterable in a range-based
  // for loop (ie., provide begin() and end() member functions).
  class RuleSetIteratorProxy {
    STACK_ALLOCATED();

   public:
    RuleSetIteratorProxy(const RuleSetGroup* rule_set_group,
                         RuleSetGroup::RuleSetBitmap bitmap)
        : rule_set_group_(*rule_set_group), bitmap_(bitmap) {}

    RuleSetIterator begin() const { return {&rule_set_group_, bitmap_}; }
    RuleSetIterator end() const { return {&rule_set_group_, 0}; }

   private:
    const RuleSetGroup& rule_set_group_;
    RuleSetGroup::RuleSetBitmap bitmap_;
  };

  RuleSetIteratorProxy AllRuleSets() const {
    return RuleSetIteratorProxy(&rule_set_group_, enabled_);
  }
  bool HasAnyRuleSetsWithAttrRules() const {
    return (rule_set_group_.has_any_attr_rules_ & enabled_) != 0;
  }
  RuleSetIteratorProxy RuleSetsWithAttrRules() const {
    return RuleSetIteratorProxy(&rule_set_group_,
                                rule_set_group_.has_any_attr_rules_ & enabled_);
  }
  RuleSetIteratorProxy RuleSetsWithUniversalRules() const {
    return RuleSetIteratorProxy(
        &rule_set_group_, rule_set_group_.has_universal_rules_ & enabled_);
  }
  RuleSetIteratorProxy RuleSetsWithLinkPseudoClassRules() const {
    return RuleSetIteratorProxy(
        &rule_set_group_,
        rule_set_group_.has_link_pseudo_class_rules_ & enabled_);
  }
  bool HasAnyRuleSetsWithFocusPseudoClassRules() const {
    return (rule_set_group_.has_focus_pseudo_class_rules_ & enabled_) != 0;
  }
  RuleSetIteratorProxy RuleSetsWithFocusPseudoClassRules() const {
    return RuleSetIteratorProxy(
        &rule_set_group_,
        rule_set_group_.has_focus_pseudo_class_rules_ & enabled_);
  }
  bool HasAnyRuleSetsWithFocusVisiblePseudoClassRules() const {
    return (rule_set_group_.has_focus_visible_pseudo_class_rules_ & enabled_) !=
           0;
  }
  RuleSetIteratorProxy RuleSetsWithFocusVisiblePseudoClassRules() const {
    return RuleSetIteratorProxy(
        &rule_set_group_,
        rule_set_group_.has_focus_visible_pseudo_class_rules_ & enabled_);
  }
  bool NeedStyleSynchronized() const {
    return rule_set_group_.need_style_synchronized_;
  }

#if DCHECK_IS_ON()
  bool IsEmpty() const { return rule_set_group_.IsEmpty(); }
#endif

 private:
  const RuleSetGroup& rule_set_group_;

  const ContainerNode* const scope_;
  // For WebVTT STYLE blocks, this is set to the featureless-like Element
  // described by the spec:
  // https://w3c.github.io/webvtt/#obtaining-css-boxes
  Element* const vtt_originating_element_;

  // Always set for every RuleSet added, except those currently disabled by the
  // single-scope optimization.
  RuleSetGroup::RuleSetBitmap enabled_;
};

void AddRuleSetToRuleSetGroupList(RuleSet* rule_set,
                                  HeapVector<RuleSetGroup>& rule_set_group);

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::RuleSetGroup>
    : VectorTraitsBase<blink::RuleSetGroup> {
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  static constexpr bool kCanMoveWithMemcpy = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCH_REQUEST_H_
