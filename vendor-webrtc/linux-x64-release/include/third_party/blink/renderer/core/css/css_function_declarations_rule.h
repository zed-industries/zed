// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DECLARATIONS_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DECLARATIONS_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSFunctionDescriptors;
class StyleRuleFunctionDeclarations;

// https://drafts.csswg.org/css-mixins-1/#cssfunctiondeclarations
class CORE_EXPORT CSSFunctionDeclarationsRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFunctionDeclarationsRule(StyleRuleFunctionDeclarations*,
                              CSSStyleSheet* parent);

  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  CSSFunctionDescriptors* style() const;

  StyleRuleFunctionDeclarations* FunctionDeclarationsRule() const {
    return function_declarations_rule_.Get();
  }

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override {
    return CSSRule::kFunctionDeclarationsRule;
  }

  Member<StyleRuleFunctionDeclarations> function_declarations_rule_;
  mutable Member<CSSFunctionDescriptors> properties_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSFunctionDeclarationsRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kFunctionDeclarationsRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_DECLARATIONS_RULE_H_
