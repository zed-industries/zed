// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_BLOCK_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_BLOCK_RULE_H_

#include "third_party/blink/renderer/core/css/css_grouping_rule.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleLayerBlock;

class CSSLayerBlockRule final : public CSSGroupingRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSLayerBlockRule(StyleRuleLayerBlock*, CSSStyleSheet*);
  ~CSSLayerBlockRule() override;

  String name() const;

  void Reattach(StyleRuleBase*) override;
  String cssText() const override;

  void Trace(Visitor*) const override;

 private:
  // TODO(crbug.com/1240596): Add DevTools support.

  CSSRule::Type GetType() const override { return kLayerBlockRule; }
};

template <>
struct DowncastTraits<CSSLayerBlockRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kLayerBlockRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYER_BLOCK_RULE_H_
