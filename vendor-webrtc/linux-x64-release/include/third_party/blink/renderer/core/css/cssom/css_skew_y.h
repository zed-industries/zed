// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_Y_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_Y_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"

namespace blink {

class DOMMatrix;
class ExceptionState;

// Represents a skewY value in a CSSTransformValue used for properties like
// "transform".
// See CSSSkewY.idl for more information about this class.
class CORE_EXPORT CSSSkewY final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSSkewY* Create(CSSNumericValue*, ExceptionState&);
  static CSSSkewY* Create(CSSNumericValue* ay) {
    return MakeGarbageCollected<CSSSkewY>(ay);
  }

  // Internal ways of creating CSSSkewY.
  static CSSSkewY* FromCSSValue(const CSSFunctionValue&);

  CSSSkewY(CSSNumericValue* ay);
  CSSSkewY(const CSSSkewY&) = delete;
  CSSSkewY& operator=(const CSSSkewY&) = delete;

  // Getters and setters for the ay attributes defined in the IDL.
  CSSNumericValue* ay() { return ay_.Get(); }
  void setAy(CSSNumericValue*, ExceptionState&);

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // From CSSTransformComponent
  // Setting is2D for CSSSkewY does nothing.
  // https://drafts.css-houdini.org/css-typed-om/#dom-cssskew-is2d
  void setIs2D(bool is2D) final {}

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const override { return kSkewYType; }
  const CSSFunctionValue* ToCSSValue() const override;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(ay_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<CSSNumericValue> ay_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_Y_H_
