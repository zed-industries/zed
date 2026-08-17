// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_VIEW_TRANSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_VIEW_TRANSITION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_rule.h"

namespace blink {

class CORE_EXPORT StyleRuleViewTransition : public StyleRuleBase {
 public:
  explicit StyleRuleViewTransition(CSSPropertyValueSet&);
  StyleRuleViewTransition(const StyleRuleViewTransition&);
  ~StyleRuleViewTransition();

  const CSSValue* GetNavigation() const;

  const Vector<String>& GetTypes() const { return types_; }

  StyleRuleViewTransition* Copy() const {
    return MakeGarbageCollected<StyleRuleViewTransition>(*this);
  }

  void SetCascadeLayer(const CascadeLayer* layer) { layer_ = layer; }
  const CascadeLayer* GetCascadeLayer() const { return layer_.Get(); }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CascadeLayer> layer_;
  Member<const CSSValue> navigation_;
  Vector<String> types_;
};

template <>
struct DowncastTraits<StyleRuleViewTransition> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsViewTransitionRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_VIEW_TRANSITION_H_
