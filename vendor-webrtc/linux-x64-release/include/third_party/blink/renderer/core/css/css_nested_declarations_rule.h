// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NESTED_DECLARATIONS_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NESTED_DECLARATIONS_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSStyleDeclaration;
class StyleRuleNestedDeclarations;
class StyleRuleCSSStyleDeclaration;

// https://drafts.csswg.org/css-nesting-1/#the-cssnestrule
class CORE_EXPORT CSSNestedDeclarationsRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSNestedDeclarationsRule(StyleRuleNestedDeclarations*,
                            CSSStyleSheet* parent);

  // Note that a CSSNestedDeclarationsRule serializes without any prelude
  // (i.e. selector list), and also without any brackets surrounding the body.
  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  CSSStyleDeclaration* style() const;

  StyleRuleNestedDeclarations* NestedDeclarationsRule() const {
    return nested_declarations_rule_.Get();
  }

  CSSRule* InnerCSSStyleRule() const;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kNestedDeclarationsRule; }

  Member<StyleRuleNestedDeclarations> nested_declarations_rule_;
  mutable Member<StyleRuleCSSStyleDeclaration> properties_cssom_wrapper_;
  mutable Member<CSSRule> style_rule_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSNestedDeclarationsRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kNestedDeclarationsRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NESTED_DECLARATIONS_RULE_H_
