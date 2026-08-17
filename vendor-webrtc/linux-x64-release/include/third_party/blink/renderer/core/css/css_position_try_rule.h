// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/style_rule.h"

namespace blink {

class StyleRuleCSSStyleDeclaration;

class StyleRulePositionTry final : public StyleRuleBase {
 public:
  StyleRulePositionTry(const AtomicString& name, CSSPropertyValueSet*);
  StyleRulePositionTry(const StyleRulePositionTry&) = default;
  ~StyleRulePositionTry();

  StyleRulePositionTry* Copy() const {
    return MakeGarbageCollected<StyleRulePositionTry>(*this);
  }

  const AtomicString& Name() const { return name_; }
  const CSSPropertyValueSet& Properties() const { return *properties_; }
  MutableCSSPropertyValueSet& MutableProperties();

  void SetCascadeLayer(const CascadeLayer* layer) { layer_ = layer; }
  const CascadeLayer* GetCascadeLayer() const { return layer_.Get(); }

  void TraceAfterDispatch(Visitor*) const;

 private:
  AtomicString name_;
  Member<const CascadeLayer> layer_;
  Member<CSSPropertyValueSet> properties_;
};

template <>
struct DowncastTraits<StyleRulePositionTry> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsPositionTryRule();
  }
};

class CSSPositionTryRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSPositionTryRule(StyleRulePositionTry*, CSSStyleSheet* parent);
  ~CSSPositionTryRule() final;

  CSSStyleDeclaration* style() const;
  Type GetType() const final { return kPositionTryRule; }

  String name() const { return position_try_rule_->Name(); }
  String cssText() const final;
  void Reattach(StyleRuleBase*) final;

  StyleRulePositionTry* PositionTry() const { return position_try_rule_.Get(); }

  void Trace(Visitor*) const final;

 private:
  Member<StyleRulePositionTry> position_try_rule_;
  mutable Member<StyleRuleCSSStyleDeclaration> properties_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSPositionTryRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kPositionTryRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_RULE_H_
