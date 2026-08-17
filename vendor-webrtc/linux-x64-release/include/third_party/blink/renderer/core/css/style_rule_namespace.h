// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_NAMESPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_NAMESPACE_H_

#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// This class is never actually stored anywhere currently, but only used for
// the parser to pass to a stylesheet
class StyleRuleNamespace final : public StyleRuleBase {
 public:
  StyleRuleNamespace(AtomicString prefix, AtomicString uri)
      : StyleRuleBase(kNamespace), prefix_(prefix), uri_(uri) {}

  StyleRuleNamespace* Copy() const {
    return MakeGarbageCollected<StyleRuleNamespace>(prefix_, uri_);
  }

  AtomicString Prefix() const { return prefix_; }
  AtomicString Uri() const { return uri_; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    StyleRuleBase::TraceAfterDispatch(visitor);
  }

 private:
  AtomicString prefix_;
  AtomicString uri_;
};

template <>
struct DowncastTraits<StyleRuleNamespace> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsNamespaceRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_NAMESPACE_H_
