// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_X_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_X_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"

namespace blink {

class DOMMatrix;
class ExceptionState;

// Represents a skewX value in a CSSTransformValue used for properties like
// "transform".
// See CSSSkewX.idl for more information about this class.
class CORE_EXPORT CSSSkewX final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSSkewX* Create(CSSNumericValue*, ExceptionState&);
  static CSSSkewX* Create(CSSNumericValue* ax) {
    return MakeGarbageCollected<CSSSkewX>(ax);
  }

  // Internal ways of creating CSSSkewX.
  static CSSSkewX* FromCSSValue(const CSSFunctionValue&);

  CSSSkewX(CSSNumericValue* ax);
  CSSSkewX(const CSSSkewX&) = delete;
  CSSSkewX& operator=(const CSSSkewX&) = delete;

  // Getters and setters for the ax attributes defined in the IDL.
  CSSNumericValue* ax() { return ax_.Get(); }
  void setAx(CSSNumericValue*, ExceptionState&);

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // From CSSTransformComponent
  // Setting is2D for CSSSkewX does nothing.
  // https://drafts.css-houdini.org/css-typed-om/#dom-cssskew-is2d
  void setIs2D(bool is2D) final {}

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const override { return kSkewXType; }
  const CSSFunctionValue* ToCSSValue() const override;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(ax_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<CSSNumericValue> ax_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_SKEW_X_H_
