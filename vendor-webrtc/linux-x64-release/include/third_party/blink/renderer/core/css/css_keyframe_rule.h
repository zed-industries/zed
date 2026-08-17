/*
 * Copyright (C) 2007, 2008, 2012 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/style_rule_keyframe.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSKeyframesRule;
class CSSStyleDeclaration;
class ExceptionState;
class KeyframeStyleRuleCSSStyleDeclaration;

class CSSKeyframeRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSKeyframeRule(StyleRuleKeyframe*, CSSKeyframesRule* parent);
  ~CSSKeyframeRule() override;

  String cssText() const override { return keyframe_->CssText(); }
  void Reattach(StyleRuleBase*) override;

  String keyText() const { return keyframe_->KeyText(); }
  void setKeyText(const ExecutionContext*, const String&, ExceptionState&);

  CSSStyleDeclaration* style() const;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kKeyframeRule; }

  Member<StyleRuleKeyframe> keyframe_;
  mutable Member<KeyframeStyleRuleCSSStyleDeclaration>
      properties_cssom_wrapper_;

  friend class CSSKeyframesRule;
};

template <>
struct DowncastTraits<CSSKeyframeRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kKeyframeRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_KEYFRAME_RULE_H_
