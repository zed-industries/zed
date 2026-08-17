// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NAMESPACE_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NAMESPACE_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleNamespace;

class CSSNamespaceRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSNamespaceRule(StyleRuleNamespace*, CSSStyleSheet*);
  ~CSSNamespaceRule() override;

  String cssText() const override;
  void Reattach(StyleRuleBase*) override {}

  AtomicString namespaceURI() const;
  AtomicString prefix() const;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kNamespaceRule; }

  Member<StyleRuleNamespace> namespace_rule_;
};

template <>
struct DowncastTraits<CSSNamespaceRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kNamespaceRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_NAMESPACE_RULE_H_
