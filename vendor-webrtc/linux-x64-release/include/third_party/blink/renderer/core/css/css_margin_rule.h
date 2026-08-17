// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARGIN_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARGIN_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSStyleDeclaration;
class CSSStyleSheet;
class StyleRulePageMargin;
class StyleRuleCSSStyleDeclaration;

class CSSMarginRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSMarginRule(StyleRulePageMargin*, CSSStyleSheet*);

  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  CSSStyleDeclaration* style() const;

  String name() const;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kMarginRule; }

  Member<StyleRulePageMargin> margin_rule_;
  mutable Member<StyleRuleCSSStyleDeclaration> properties_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSMarginRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kMarginRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_MARGIN_RULE_H_
