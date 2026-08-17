// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_STATEMENT_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_STATEMENT_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleLayerStatement;

class CSSLayerStatementRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSLayerStatementRule(StyleRuleLayerStatement*, CSSStyleSheet*);
  ~CSSLayerStatementRule() override;

  Vector<String> nameList() const;

  void Reattach(StyleRuleBase*) override;
  String cssText() const override;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kLayerStatementRule; }

  Member<StyleRuleLayerStatement> layer_statement_rule_;
};

template <>
struct DowncastTraits<CSSLayerStatementRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kLayerStatementRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_STATEMENT_RULE_H_
