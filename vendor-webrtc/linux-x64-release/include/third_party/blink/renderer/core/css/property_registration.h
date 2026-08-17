// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRATION_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/animation/interpolation_types_map.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_syntax_definition.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class PropertyDefinition;
class PropertyRegistry;
class StyleRuleProperty;

class CORE_EXPORT PropertyRegistration final
    : public GarbageCollected<PropertyRegistration> {
 public:
  // Creates a PropertyRegistration for a valid @property rule, or returns
  // nullptr if the rule is invalid.
  static PropertyRegistration* MaybeCreateForDeclaredProperty(
      Document&,
      const AtomicString& name,
      StyleRuleProperty&);

  static void registerProperty(ExecutionContext*,
                               const PropertyDefinition*,
                               ExceptionState&);

  static void RemoveDeclaredProperties(Document&);

  static const PropertyRegistration* From(const ExecutionContext*,
                                          const AtomicString& property_name);

  static std::optional<CSSSyntaxDefinition> ConvertSyntax(
      const CSSValue* value);
  static std::optional<bool> ConvertInherits(const CSSValue* value);
  static std::optional<const CSSValue*> ConvertInitial(
      const CSSValue* value,
      const CSSSyntaxDefinition& syntax,
      const CSSParserContext& parser_context);

  PropertyRegistration(const AtomicString& name,
                       const CSSSyntaxDefinition&,
                       bool inherits,
                       const CSSValue* initial,
                       StyleRuleProperty* property_rule = nullptr);
  ~PropertyRegistration();

  const CSSSyntaxDefinition& Syntax() const { return syntax_; }
  bool Inherits() const { return inherits_; }
  const CSSValue* Initial() const { return initial_.Get(); }
  StyleRuleProperty* PropertyRule() const { return property_rule_.Get(); }
  const InterpolationTypes& GetInterpolationTypes() const {
    return interpolation_types_;
  }
  // See `ViewportUnitFlag`.
  unsigned GetViewportUnitFlags() const;

  void Trace(Visitor* visitor) const;

 private:
  friend class ::blink::PropertyRegistry;

  const CSSSyntaxDefinition syntax_;
  const bool inherits_;
  const Member<const CSSValue> initial_;
  Member<StyleRuleProperty> property_rule_;
  const InterpolationTypes interpolation_types_;
  mutable bool referenced_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRATION_H_
