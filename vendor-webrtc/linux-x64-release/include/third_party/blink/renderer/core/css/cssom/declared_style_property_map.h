// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_DECLARED_STYLE_PROPERTY_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_DECLARED_STYLE_PROPERTY_MAP_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/style_property_map.h"

namespace blink {

class CSSStyleRule;
class StyleRule;

// This class implements declared StylePropertMap in the Typed CSSOM
// API. The specification is here:
// https://drafts.css-houdini.org/css-typed-om/#declared-stylepropertymap-objects
//
// The declared StylePropertyMap retrieves styles specified by a CSS style rule
// and returns them as CSSStyleValues. The IDL for this class is in
// StylePropertyMap.idl. The declared StylePropertyMap for an element is
// accessed via CSSStyleRule.styleMap (see CSSStyleRule.idl)
class CORE_EXPORT DeclaredStylePropertyMap final : public StylePropertyMap {
 public:
  explicit DeclaredStylePropertyMap(CSSStyleRule* owner_rule);
  DeclaredStylePropertyMap(const DeclaredStylePropertyMap&) = delete;
  DeclaredStylePropertyMap& operator=(const DeclaredStylePropertyMap&) = delete;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(owner_rule_);
    StylePropertyMap::Trace(visitor);
  }

  unsigned int size() const final;

 protected:
  const CSSValue* GetProperty(CSSPropertyID) const override;
  const CSSValue* GetCustomProperty(const AtomicString&) const override;
  void ForEachProperty(IterationFunction visitor) override;
  void SetProperty(CSSPropertyID, const CSSValue&) override;
  bool SetShorthandProperty(CSSPropertyID,
                            const String&,
                            SecureContextMode) override;
  void SetCustomProperty(const AtomicString&, const CSSValue&) override;
  void RemoveProperty(CSSPropertyID) override;
  void RemoveCustomProperty(const AtomicString&) override;
  void RemoveAllProperties() final;

  String SerializationForShorthand(const CSSProperty&) const final;

 private:
  StyleRule* GetStyleRule() const;

  WeakMember<CSSStyleRule> owner_rule_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_DECLARED_STYLE_PROPERTY_MAP_H_
