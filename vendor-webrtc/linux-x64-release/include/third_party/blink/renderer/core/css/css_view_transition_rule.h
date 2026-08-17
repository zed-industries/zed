// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VIEW_TRANSITION_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VIEW_TRANSITION_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleViewTransition;

class CSSViewTransitionRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSViewTransitionRule(StyleRuleViewTransition*, CSSStyleSheet*);
  ~CSSViewTransitionRule() override = default;

  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  void Trace(Visitor*) const override;

  String navigation() const;
  Vector<String> types() const;

 private:
  CSSRule::Type GetType() const override { return kViewTransitionRule; }

  Member<StyleRuleViewTransition> view_transition_rule_;
};

template <>
struct DowncastTraits<CSSViewTransitionRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kViewTransitionRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VIEW_TRANSITION_RULE_H_
