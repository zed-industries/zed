/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * (C) 2002-2003 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2002, 2006, 2008, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Samuel Weinig (sam@webkit.org)
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GROUPING_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GROUPING_RULE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ExceptionState;
class CSSRuleList;

StyleRule* FindClosestParentStyleRuleOrNull(CSSRule* parent);

struct NestingContext {
  STACK_ALLOCATED();

 public:
  // Whether we are nested inside a regular style rule (kNesting),
  // or an @scope rule (kScope).
  CSSNestingType nesting_type;
  // What the '&' selector references.
  StyleRule* parent_rule_for_nesting;
};

// Finds the parent rule for nesting (i.e. what the '&' selector should
// refer to), starting at `parent_rule` (inclusive), and traversing up the
// ancestor chain.
NestingContext CalculateNestingContext(const CSSRule* parent_rule);

// Utility function also used by CSSStyleRule, which can have child rules
// just like CSSGroupingRule can (we share insertRule() / deleteRule()
// implementation). Returns nullptr if an exception was raised.
StyleRuleBase* ParseRuleForInsert(const ExecutionContext* execution_context,
                                  const String& rule_string,
                                  unsigned index,
                                  size_t num_child_rules,
                                  const CSSRule& parent_rule,
                                  ExceptionState& exception_state);

// See CSSStyleRule/CSSGroupingRule::QuietlyInsertRule.
void ParseAndQuietlyInsertRule(
    const ExecutionContext*,
    const String& rule_string,
    unsigned index,
    CSSRule& parent_rule,
    HeapVector<Member<StyleRuleBase>>& child_rules,
    HeapVector<Member<CSSRule>>& child_rule_cssom_wrappers);

// See CSSStyleRule/CSSGroupingRule::QuietlyDeleteRule.
void QuietlyDeleteRule(unsigned index,
                       HeapVector<Member<StyleRuleBase>>& child_rules,
                       HeapVector<Member<CSSRule>>& child_rule_cssom_wrappers);

class CORE_EXPORT CSSGroupingRule : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~CSSGroupingRule() override;

  void Reattach(StyleRuleBase*) override;

  CSSRuleList* cssRules() const override;

  unsigned insertRule(const ExecutionContext*,
                      const String& rule,
                      unsigned index,
                      ExceptionState&);
  void deleteRule(unsigned index, ExceptionState&);

  // Like insertRule/deleteRule, but does not cause any invalidation.
  // Used by Inspector to temporarily insert non-existent rules for
  // the purposes of rule matching (see InspectorGhostRules).
  void QuietlyInsertRule(const ExecutionContext*,
                         const String& rule,
                         unsigned index);
  void QuietlyDeleteRule(unsigned index);

  // For CSSRuleList
  unsigned length() const;
  CSSRule* Item(unsigned index, bool trigger_use_counters = true) const;

  // Get an item, but signal that it's been requested internally from the
  // engine, and not directly from a script.
  CSSRule* ItemInternal(unsigned index) const {
    return Item(index, /*trigger_use_counters=*/false);
  }

  void Trace(Visitor*) const override;

 protected:
  CSSGroupingRule(StyleRuleGroup* group_rule, CSSStyleSheet* parent);

  void AppendCSSTextForItems(StringBuilder&) const;

  Member<StyleRuleGroup> group_rule_;
  mutable HeapVector<Member<CSSRule>> child_rule_cssom_wrappers_;
  mutable Member<CSSRuleList> rule_list_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSGroupingRule> {
  static bool AllowFrom(const CSSRule& rule) {
    switch (rule.GetType()) {
      // CSSConditionRule (inherits CSSGroupingRule):
      case CSSRule::kMediaRule:
      case CSSRule::kSupportsRule:
      case CSSRule::kContainerRule:
      // CSSGroupingRule:
      case CSSRule::kFunctionRule:
      case CSSRule::kLayerBlockRule:
      case CSSRule::kPageRule:
      case CSSRule::kScopeRule:
      case CSSRule::kStartingStyleRule:
        return true;
      // go/keep-sorted start
      case CSSRule::kCharsetRule:
      case CSSRule::kCounterStyleRule:
      case CSSRule::kFontFaceRule:
      case CSSRule::kFontFeatureRule:
      case CSSRule::kFontFeatureValuesRule:
      case CSSRule::kFontPaletteValuesRule:
      case CSSRule::kFunctionDeclarationsRule:
      case CSSRule::kImportRule:
      case CSSRule::kKeyframeRule:
      case CSSRule::kKeyframesRule:
      case CSSRule::kLayerStatementRule:
      case CSSRule::kMarginRule:
      case CSSRule::kNamespaceRule:
      case CSSRule::kNestedDeclarationsRule:
      case CSSRule::kPositionTryRule:
      case CSSRule::kPropertyRule:
      case CSSRule::kStyleRule:
      case CSSRule::kViewTransitionRule:
        // go/keep-sorted end
        return false;
    }
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GROUPING_RULE_H_
