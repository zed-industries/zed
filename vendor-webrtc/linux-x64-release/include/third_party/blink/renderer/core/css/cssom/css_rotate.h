// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_ROTATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_ROTATE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"

namespace blink {

class DOMMatrix;
class ExceptionState;
class CSSNumericValue;

// Represents a rotation value in a CSSTransformValue used for properties like
// "transform".
// See CSSRotate.idl for more information about this class.
class CORE_EXPORT CSSRotate final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructors defined in the IDL.
  static CSSRotate* Create(CSSNumericValue* angle, ExceptionState&);
  static CSSRotate* Create(const V8CSSNumberish* x,
                           const V8CSSNumberish* y,
                           const V8CSSNumberish* z,
                           CSSNumericValue* angle,
                           ExceptionState& exception_state);

  // Blink-internal ways of creating CSSRotates.
  static CSSRotate* Create(CSSNumericValue* angle);
  static CSSRotate* Create(CSSNumericValue* x,
                           CSSNumericValue* y,
                           CSSNumericValue* z,
                           CSSNumericValue* angle);
  static CSSRotate* FromCSSValue(const CSSFunctionValue&);

  CSSRotate(CSSNumericValue* x,
            CSSNumericValue* y,
            CSSNumericValue* z,
            CSSNumericValue* angle,
            bool is2D);
  CSSRotate(const CSSRotate&) = delete;
  CSSRotate& operator=(const CSSRotate&) = delete;

  // Getters and setters for attributes defined in the IDL.
  CSSNumericValue* angle() { return angle_.Get(); }
  void setAngle(CSSNumericValue* angle, ExceptionState&);
  V8CSSNumberish* x();
  V8CSSNumberish* y();
  V8CSSNumberish* z();
  void setX(const V8CSSNumberish* x, ExceptionState& exception_state);
  void setY(const V8CSSNumberish* y, ExceptionState& exception_state);
  void setZ(const V8CSSNumberish* z, ExceptionState& exception_state);

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const final { return kRotationType; }
  const CSSFunctionValue* ToCSSValue() const final;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(angle_);
    visitor->Trace(x_);
    visitor->Trace(y_);
    visitor->Trace(z_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<CSSNumericValue> angle_;
  Member<CSSNumericValue> x_;
  Member<CSSNumericValue> y_;
  Member<CSSNumericValue> z_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_ROTATE_H_
