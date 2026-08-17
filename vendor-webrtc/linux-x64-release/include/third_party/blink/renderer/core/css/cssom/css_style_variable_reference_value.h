// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VARIABLE_REFERENCE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VARIABLE_REFERENCE_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_unparsed_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;

// CSSStyleVariableReferenceValue represents a CSS var() value for CSS Typed OM.
// The corresponding idl file is css_variable_reference_value.idl.
class CORE_EXPORT CSSStyleVariableReferenceValue final
    : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CSSStyleVariableReferenceValue* Create(const String& variable,
                                                ExceptionState&);

  static CSSStyleVariableReferenceValue* Create(const String& variable,
                                                CSSUnparsedValue* fallback,
                                                ExceptionState&);

  static CSSStyleVariableReferenceValue* Create(
      const String& variable,
      CSSUnparsedValue* fallback = nullptr);

  CSSStyleVariableReferenceValue(const String& variable,
                                 CSSUnparsedValue* fallback)
      : variable_(variable), fallback_(fallback) {}
  CSSStyleVariableReferenceValue(const CSSStyleVariableReferenceValue&) =
      delete;
  CSSStyleVariableReferenceValue& operator=(
      const CSSStyleVariableReferenceValue&) = delete;

  const String& variable() const { return variable_; }
  void setVariable(const String&, ExceptionState&);

  CSSUnparsedValue* fallback() { return fallback_.Get(); }
  const CSSUnparsedValue* fallback() const { return fallback_.Get(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(fallback_);
    ScriptWrappable::Trace(visitor);
  }

 protected:
  String variable_;
  Member<CSSUnparsedValue> fallback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VARIABLE_REFERENCE_VALUE_H_
