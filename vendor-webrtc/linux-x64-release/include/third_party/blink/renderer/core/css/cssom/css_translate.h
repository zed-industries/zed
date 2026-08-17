// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSLATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSLATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"
#include "third_party/blink/renderer/core/css/cssom/css_unit_value.h"

namespace blink {

class CSSNumericValue;
class DOMMatrix;
class ExceptionState;

// Represents a translation value in a CSSTransformValue used for properties
// like "transform".
// See CSSTranslate.idl for more information about this class.
class CORE_EXPORT CSSTranslate final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructors defined in the IDL.
  static CSSTranslate* Create(CSSNumericValue* x,
                              CSSNumericValue* y,
                              CSSNumericValue* z,
                              ExceptionState&);
  static CSSTranslate* Create(CSSNumericValue* x,
                              CSSNumericValue* y,
                              ExceptionState&);

  // Blink-internal ways of creating CSSTranslates.
  static CSSTranslate* Create(CSSNumericValue* x, CSSNumericValue* y);
  static CSSTranslate* Create(CSSNumericValue* x,
                              CSSNumericValue* y,
                              CSSNumericValue* z);
  static CSSTranslate* FromCSSValue(const CSSFunctionValue&);

  CSSTranslate(CSSNumericValue* x,
               CSSNumericValue* y,
               CSSNumericValue* z,
               bool is2D);
  CSSTranslate(const CSSTranslate&) = delete;
  CSSTranslate& operator=(const CSSTranslate&) = delete;

  // Getters and setters for attributes defined in the IDL.
  CSSNumericValue* x() { return x_.Get(); }
  CSSNumericValue* y() { return y_.Get(); }
  CSSNumericValue* z() { return z_.Get(); }
  void setX(CSSNumericValue* x, ExceptionState&);
  void setY(CSSNumericValue* y, ExceptionState&);
  void setZ(CSSNumericValue* z, ExceptionState&);

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const final { return kTranslationType; }
  const CSSFunctionValue* ToCSSValue() const final;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(x_);
    visitor->Trace(y_);
    visitor->Trace(z_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<CSSNumericValue> x_;
  Member<CSSNumericValue> y_;
  Member<CSSNumericValue> z_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSLATE_H_
