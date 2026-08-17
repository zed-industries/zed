// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SCALE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SCALE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"
#include "third_party/blink/renderer/core/css/cssom/css_unit_value.h"
#include "third_party/blink/renderer/core/geometry/dom_matrix.h"

namespace blink {

class CSSNumericValue;
class DOMMatrix;

// Represents a scale value in a CSSTransformValue used for properties like
// "transform".
// See CSSScale.idl for more information about this class.
class CORE_EXPORT CSSScale final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructors defined in the IDL.
  static CSSScale* Create(const V8CSSNumberish* x,
                          const V8CSSNumberish* y,
                          ExceptionState& exception_state);
  static CSSScale* Create(const V8CSSNumberish* x,
                          const V8CSSNumberish* y,
                          const V8CSSNumberish* z,
                          ExceptionState& exception_state);

  // Blink-internal ways of creating CSSScales.
  static CSSScale* Create(CSSNumericValue* x, CSSNumericValue* y) {
    return MakeGarbageCollected<CSSScale>(x, y, CSSUnitValue::Create(1),
                                          true /* is2D */);
  }
  static CSSScale* Create(CSSNumericValue* x,
                          CSSNumericValue* y,
                          CSSNumericValue* z) {
    return MakeGarbageCollected<CSSScale>(x, y, z, false /* is2D */);
  }
  static CSSScale* FromCSSValue(const CSSFunctionValue&);

  CSSScale(CSSNumericValue* x,
           CSSNumericValue* y,
           CSSNumericValue* z,
           bool is2D);
  CSSScale(const CSSScale&) = delete;
  CSSScale& operator=(const CSSScale&) = delete;

  // Getters and setters for attributes defined in the IDL.
  V8CSSNumberish* x();
  V8CSSNumberish* y();
  V8CSSNumberish* z();
  void setX(const V8CSSNumberish* x, ExceptionState& exception_state);
  void setY(const V8CSSNumberish* y, ExceptionState& exception_state);
  void setZ(const V8CSSNumberish* z, ExceptionState& exception_state);

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const final { return kScaleType; }
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

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SCALE_H_
