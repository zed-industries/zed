// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"

namespace blink {

class DOMMatrix;
class ExceptionState;

// Represents a skew value in a CSSTransformValue used for properties like
// "transform".
// See CSSSkew.idl for more information about this class.
class CORE_EXPORT CSSSkew final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSSkew* Create(CSSNumericValue*, CSSNumericValue*, ExceptionState&);
  static CSSSkew* Create(CSSNumericValue* ax, CSSNumericValue* ay) {
    return MakeGarbageCollected<CSSSkew>(ax, ay);
  }

  // Internal ways of creating CSSSkew.
  static CSSSkew* FromCSSValue(const CSSFunctionValue&);

  CSSSkew(CSSNumericValue* ax, CSSNumericValue* ay);
  CSSSkew(const CSSSkew&) = delete;
  CSSSkew& operator=(const CSSSkew&) = delete;

  // Getters and setters for the ax and ay attributes defined in the IDL.
  CSSNumericValue* ax() { return ax_.Get(); }
  CSSNumericValue* ay() { return ay_.Get(); }
  void setAx(CSSNumericValue*, ExceptionState&);
  void setAy(CSSNumericValue*, ExceptionState&);

  // From CSSTransformComponent
  // Setting is2D for CSSSkew does nothing.
  // https://drafts.css-houdini.org/css-typed-om/#dom-cssskew-is2d
  void setIs2D(bool is2D) final {}

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const override { return kSkewType; }
  const CSSFunctionValue* ToCSSValue() const override;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(ax_);
    visitor->Trace(ay_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<CSSNumericValue> ax_;
  Member<CSSNumericValue> ay_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_H_
