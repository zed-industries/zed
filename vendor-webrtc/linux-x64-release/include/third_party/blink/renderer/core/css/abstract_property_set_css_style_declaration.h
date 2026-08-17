/*
 * Copyright (C) 2012 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ABSTRACT_PROPERTY_SET_CSS_STYLE_DECLARATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ABSTRACT_PROPERTY_SET_CSS_STYLE_DECLARATION_H_

#include "third_party/blink/renderer/core/css/css_style_declaration.h"

namespace blink {

class CSSRule;
class CSSValue;
class Element;
class ExceptionState;
class ExecutionContext;
class MutableCSSPropertyValueSet;
class StyleSheetContents;

class CORE_EXPORT AbstractPropertySetCSSStyleDeclaration
    : public CSSStyleDeclaration {
 public:
  virtual Element* ParentElement() const { return nullptr; }
  StyleSheetContents* ContextStyleSheet() const;
  explicit AbstractPropertySetCSSStyleDeclaration(ExecutionContext* context)
      : CSSStyleDeclaration(context) {}

  // Some subclasses only allow a subset of the properties, for example
  // CSSPositionTryDescriptors only allows inset and sizing properties.
  virtual bool IsPropertyValid(CSSPropertyID) const = 0;

  void Trace(Visitor*) const override;

  String GetPropertyValueInternal(CSSPropertyID) final;
  void SetPropertyInternal(CSSPropertyID,
                           const String& custom_property_name,
                           StringView value,
                           bool important,
                           SecureContextMode,
                           ExceptionState&) final;

 private:
  bool IsAbstractPropertySet() const final { return true; }
  CSSRule* parentRule() const override { return nullptr; }
  unsigned length() const final;
  String item(unsigned index) const final;
  String getPropertyValue(const String& property_name) final;
  String getPropertyPriority(const String& property_name) final;
  String GetPropertyShorthand(const String& property_name) final;
  bool IsPropertyImplicit(const String& property_name) final;
  void setProperty(const ExecutionContext*,
                   const String& property_name,
                   const String& value,
                   const String& priority,
                   ExceptionState&) final;
  String removeProperty(const String& property_name, ExceptionState&) final;
  void QuietlyRemoveProperty(const String& property_name) final;
  String CssFloat() const;
  void SetCSSFloat(const String&, ExceptionState&);
  String cssText() const final;
  void setCSSText(const ExecutionContext*,
                  const String&,
                  ExceptionState&) final;
  const CSSValue* GetPropertyCSSValueInternal(CSSPropertyID) final;
  const CSSValue* GetPropertyCSSValueInternal(
      const AtomicString& custom_property_name) final;
  String GetPropertyValueWithHint(const String& property_name,
                                  unsigned index) final;
  String GetPropertyPriorityWithHint(const String& property_name,
                                     unsigned index) final;

  bool CssPropertyMatches(CSSPropertyID, const CSSValue&) const final;

 protected:
  enum MutationType {
    kNoChanges,
    // Only properties that were independent changed, so that if there are
    // no other changes and this is on the inline style, it may be
    // possible to reuse an already-computed style and just apply
    // the new changes on top of it.
    kIndependentPropertyChanged,
    kPropertyChanged
  };
  virtual void WillMutate() {}
  virtual void DidMutate(MutationType) {}
  virtual MutableCSSPropertyValueSet& PropertySet() const = 0;
  virtual bool IsKeyframeStyle() const { return false; }
  bool FastPathSetProperty(CSSPropertyID unresolved_property,
                           double value) override;
};

template <>
struct DowncastTraits<AbstractPropertySetCSSStyleDeclaration> {
  static bool AllowFrom(const CSSStyleDeclaration& declaration) {
    return declaration.IsAbstractPropertySet();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ABSTRACT_PROPERTY_SET_CSS_STYLE_DECLARATION_H_
