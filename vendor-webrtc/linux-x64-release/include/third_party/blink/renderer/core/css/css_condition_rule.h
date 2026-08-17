// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONDITION_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONDITION_RULE_H_

#include "third_party/blink/renderer/core/css/css_grouping_rule.h"

namespace blink {

class CORE_EXPORT CSSConditionRule : public CSSGroupingRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~CSSConditionRule() override;

  // Prefer ConditionTextInternal for internal use. (Avoids UseCounter).
  virtual String conditionText() const;
  virtual String ConditionTextInternal() const;

 protected:
  CSSConditionRule(StyleRuleCondition* condition_rule, CSSStyleSheet* parent);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONDITION_RULE_H_
