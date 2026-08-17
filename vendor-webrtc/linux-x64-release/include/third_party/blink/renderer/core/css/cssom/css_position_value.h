// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_POSITION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_POSITION_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CSSNumericValue;
class ExceptionState;

class CORE_EXPORT CSSPositionValue final : public CSSStyleValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSPositionValue* Create(CSSNumericValue* x,
                                  CSSNumericValue* y,
                                  ExceptionState&);

  // Blink-internal constructor
  static CSSPositionValue* Create(CSSNumericValue* x, CSSNumericValue* y);

  static CSSPositionValue* FromCSSValue(const CSSValue&);

  CSSPositionValue(CSSNumericValue* x, CSSNumericValue* y) : x_(x), y_(y) {}
  CSSPositionValue(const CSSPositionValue&) = delete;
  CSSPositionValue& operator=(const CSSPositionValue&) = delete;

  // Getters and setters defined in the IDL.
  CSSNumericValue* x() { return x_.Get(); }
  CSSNumericValue* y() { return y_.Get(); }
  void setX(CSSNumericValue* x, ExceptionState&);
  void setY(CSSNumericValue* x, ExceptionState&);

  // Internal methods - from CSSStyleValue.
  StyleValueType GetType() const final { return kPositionType; }

  const CSSValue* ToCSSValue() const final;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(x_);
    visitor->Trace(y_);
    CSSStyleValue::Trace(visitor);
  }

 private:
  static bool IsValidCoordinate(CSSNumericValue* coord);

 protected:
  Member<CSSNumericValue> x_;
  Member<CSSNumericValue> y_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_POSITION_VALUE_H_
