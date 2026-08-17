// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_RULE_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_function_parameter.h"
#include "third_party/blink/renderer/core/css/css_grouping_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class StyleRuleFunction;
class FunctionParameter;

// https://drafts.csswg.org/css-mixins-1/#cssfunctionrule
class CORE_EXPORT CSSFunctionRule final : public CSSGroupingRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFunctionRule(StyleRuleFunction*, CSSStyleSheet* parent);
  String name() const;
  HeapVector<Member<FunctionParameter>> getParameters() const;
  String returnType() const;
  String cssText() const override;

  StyleRuleFunction& FunctionRule() const {
    return To<StyleRuleFunction>(*group_rule_);
  }

 private:
  CSSRule::Type GetType() const override { return CSSRule::kFunctionRule; }
};

template <>
struct DowncastTraits<CSSFunctionRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kFunctionRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_RULE_H_
