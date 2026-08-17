// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
enum class SecureContextMode;

class CSSStyleValue;
using CSSStyleValueVector = HeapVector<Member<CSSStyleValue>>;
using GCedCSSStyleValueVector = GCedHeapVector<Member<CSSStyleValue>>;

// The base class for all CSS values returned by the Typed OM.
// See CSSStyleValue.idl for additional documentation about this class.
class CORE_EXPORT CSSStyleValue : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This enum ordering is significant for CSSStyleValue::IsNumericValue.
  enum StyleValueType {
    kUnknownType,
    kUnparsedType,
    kKeywordType,
    // Start of CSSNumericValue subclasses
    kUnitType,
    kSumType,
    kProductType,
    kNegateType,
    kInvertType,
    kMinType,
    kMaxType,
    kClampType,
    // End of CSSNumericValue subclasses
    kTransformType,
    kPositionType,
    kURLImageType,
    kColorType,
    kUnsupportedColorType,
  };

  static CSSStyleValue* parse(const ExecutionContext*,
                              const String& property_name,
                              const String& value,
                              ExceptionState&);
  static CSSStyleValueVector parseAll(const ExecutionContext*,
                                      const String& property_name,
                                      const String& value,
                                      ExceptionState&);

  CSSStyleValue(const CSSStyleValue&) = delete;
  CSSStyleValue& operator=(const CSSStyleValue&) = delete;
  ~CSSStyleValue() override = default;

  virtual StyleValueType GetType() const = 0;
  bool IsNumericValue() const {
    return GetType() >= kUnitType && GetType() <= kClampType;
  }

  virtual const CSSValue* ToCSSValue() const = 0;
  // FIXME: We should make this a method on CSSProperty instead.
  virtual const CSSValue* ToCSSValueWithProperty(CSSPropertyID) const {
    return ToCSSValue();
  }
  // https://drafts.css-houdini.org/css-typed-om/#stylevalue-serialization
  virtual String toString() const;

  // TODO(801935): Actually use this for serialization in subclasses.
  // Currently only used by CSSUnsupportedStyleValue because it's
  // immutable, so we have to worry about the serialization changing.
  const String& CSSText() const { return css_text_; }
  void SetCSSText(const String& css_text) { css_text_ = css_text; }

 protected:
  CSSStyleValue() = default;

 private:
  String css_text_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_STYLE_VALUE_H_
