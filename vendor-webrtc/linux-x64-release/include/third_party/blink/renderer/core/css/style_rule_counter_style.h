// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_COUNTER_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_COUNTER_STYLE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/at_rule_descriptors.h"
#include "third_party/blink/renderer/core/css/style_rule.h"

namespace blink {

class CascadeLayer;

class CORE_EXPORT StyleRuleCounterStyle : public StyleRuleBase {
 public:
  StyleRuleCounterStyle(const AtomicString&, CSSPropertyValueSet*);
  StyleRuleCounterStyle(const StyleRuleCounterStyle&);
  ~StyleRuleCounterStyle();

  int GetVersion() const { return version_; }

  // Different 'system' values have different requirements on 'symbols' and
  // 'additive-symbols'. Returns true if the requirement is met.
  // https://drafts.csswg.org/css-counter-styles-3/#counter-style-symbols
  bool HasValidSymbols() const;

  AtomicString GetName() const { return name_; }
  const CSSValue* GetSystem() const { return system_.Get(); }
  const CSSValue* GetNegative() const { return negative_.Get(); }
  const CSSValue* GetPrefix() const { return prefix_.Get(); }
  const CSSValue* GetSuffix() const { return suffix_.Get(); }
  const CSSValue* GetRange() const { return range_.Get(); }
  const CSSValue* GetPad() const { return pad_.Get(); }
  const CSSValue* GetFallback() const { return fallback_.Get(); }
  const CSSValue* GetSymbols() const { return symbols_.Get(); }
  const CSSValue* GetAdditiveSymbols() const { return additive_symbols_.Get(); }
  const CSSValue* GetSpeakAs() const { return speak_as_.Get(); }

  // Returns false if the new value is invalid or equivalent to the old value.
  bool NewValueInvalidOrEqual(AtRuleDescriptorID, const CSSValue*);
  void SetDescriptorValue(AtRuleDescriptorID, const CSSValue*);

  void SetName(const AtomicString& name) {
    name_ = name;
    ++version_;
  }

  bool HasFailedOrCanceledSubresources() const {
    // TODO(crbug.com/1176323): Handle image symbols when we implement it.
    return false;
  }

  StyleRuleCounterStyle* Copy() const {
    return MakeGarbageCollected<StyleRuleCounterStyle>(*this);
  }

  void SetCascadeLayer(const CascadeLayer* layer) { layer_ = layer; }
  const CascadeLayer* GetCascadeLayer() const { return layer_.Get(); }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CSSValue>& GetDescriptorReference(AtRuleDescriptorID);

  AtomicString name_;
  Member<const CSSValue> system_;
  Member<const CSSValue> negative_;
  Member<const CSSValue> prefix_;
  Member<const CSSValue> suffix_;
  Member<const CSSValue> range_;
  Member<const CSSValue> pad_;
  Member<const CSSValue> fallback_;
  Member<const CSSValue> symbols_;
  Member<const CSSValue> additive_symbols_;
  Member<const CSSValue> speak_as_;

  Member<const CascadeLayer> layer_;

  // Tracks mutations due to setter functions.
  int version_ = 0;
};

template <>
struct DowncastTraits<StyleRuleCounterStyle> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsCounterStyleRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_COUNTER_STYLE_H_
