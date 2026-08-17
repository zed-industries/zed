// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHANDS_CUSTOM_PROPERTY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHANDS_CUSTOM_PROPERTY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/properties/longhands/variable.h"

#include "third_party/blink/renderer/core/css/property_registration.h"

namespace blink {

class PropertyRegistry;

// Represents a custom property (both registered and unregistered).
//
// Unlike all other CSSProperty instances, instances of this class are
// allocated dynamically on demand. (See CSSPropertyRef).
//
// TODO(andruud): Move functionality from Variable to here, and eventually
// remove Variable.
class CORE_EXPORT CustomProperty : public Variable {
  STACK_ALLOCATED();

 public:
  CustomProperty() = default;
  CustomProperty(AtomicString name, const Document&);
  CustomProperty(const AtomicString& name, const PropertyRegistry*);

  const AtomicString& GetPropertyNameAtomicString() const override;
  CSSPropertyName GetCSSPropertyName() const override;
  bool HasEqualCSSPropertyName(const CSSProperty& other) const override;

  void ApplyInitial(StyleResolverState&) const override;
  void ApplyInherit(StyleResolverState&) const override;
  void ApplyValue(StyleResolverState&,
                  const CSSValue&,
                  ValueMode) const override;

  // Never used.
  const CSSValue* ParseSingleValue(CSSParserTokenStream&,
                                   const CSSParserContext&,
                                   const CSSParserLocalContext&) const override;

  // The custom property is parsed according to the registered syntax (if
  // available).
  //
  // NOTE: This is distinct from ParseSingleValue() because it takes in
  // original_text, not a token stream.
  const CSSValue* Parse(StringView,
                        const CSSParserContext&,
                        const CSSParserLocalContext&) const;

  const CSSValue* CSSValueFromComputedStyleInternal(
      const ComputedStyle&,
      const LayoutObject*,
      bool allow_visited_style,
      CSSValuePhase value_phase) const override;

  bool IsRegistered() const { return registration_ != nullptr; }

  bool HasInitialValue() const;

  // https://drafts.csswg.org/css-variables/#guaranteed-invalid-value
  bool SupportsGuaranteedInvalid() const;

  // https://drafts.css-houdini.org/css-properties-values-api-1/#universal-syntax-definition
  bool HasUniversalSyntax() const;

 private:
  CustomProperty(const AtomicString& name,
                 const PropertyRegistration* registration);
  explicit CustomProperty(const PropertyRegistration* registration);

  const CSSValue* ParseUntyped(StringView,
                               const CSSParserContext&,
                               const CSSParserLocalContext&) const;

  AtomicString name_;
  const PropertyRegistration* registration_;
};

template <>
struct DowncastTraits<CustomProperty> {
  static bool AllowFrom(const CSSProperty& property) {
    DCHECK(!Variable::IsStaticInstance(property));
    return property.PropertyID() == CSSPropertyID::kVariable;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_LONGHANDS_CUSTOM_PROPERTY_H_
