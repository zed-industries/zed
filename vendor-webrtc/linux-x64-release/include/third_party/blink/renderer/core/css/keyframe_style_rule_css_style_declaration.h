// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KEYFRAME_STYLE_RULE_CSS_STYLE_DECLARATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KEYFRAME_STYLE_RULE_CSS_STYLE_DECLARATION_H_

#include "third_party/blink/renderer/core/css/style_rule_css_style_declaration.h"

namespace blink {

class CSSKeyframeRule;

class KeyframeStyleRuleCSSStyleDeclaration final
    : public StyleRuleCSSStyleDeclaration {
 public:
  KeyframeStyleRuleCSSStyleDeclaration(MutableCSSPropertyValueSet&,
                                       CSSKeyframeRule*);

 private:
  void DidMutate(MutationType) override;
  bool IsKeyframeStyle() const final { return true; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KEYFRAME_STYLE_RULE_CSS_STYLE_DECLARATION_H_
