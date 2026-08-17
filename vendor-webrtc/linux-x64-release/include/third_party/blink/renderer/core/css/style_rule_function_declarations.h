// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FUNCTION_DECLARATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FUNCTION_DECLARATIONS_H_

#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSPropertyValueSet;

class CORE_EXPORT StyleRuleFunctionDeclarations : public StyleRuleBase {
 public:
  explicit StyleRuleFunctionDeclarations(CSSPropertyValueSet& properties)
      : StyleRuleBase(kFunctionDeclarations), properties_(properties) {}

  StyleRuleFunctionDeclarations(const StyleRuleFunctionDeclarations& o)
      : StyleRuleBase(o), properties_(o.properties_->ImmutableCopyIfNeeded()) {}

  const CSSPropertyValueSet& Properties() const { return *properties_; }

  MutableCSSPropertyValueSet& MutableProperties() {
    if (!properties_->IsMutable()) {
      properties_ = properties_->MutableCopy();
    }
    return *To<MutableCSSPropertyValueSet>(properties_.Get());
  }

  StyleRuleFunctionDeclarations* Copy() const {
    return MakeGarbageCollected<StyleRuleFunctionDeclarations>(*this);
  }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(properties_);
    StyleRuleBase::TraceAfterDispatch(visitor);
  }

 private:
  Member<CSSPropertyValueSet> properties_;
};

template <>
struct DowncastTraits<StyleRuleFunctionDeclarations> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsFunctionDeclarationsRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_FUNCTION_DECLARATIONS_H_
