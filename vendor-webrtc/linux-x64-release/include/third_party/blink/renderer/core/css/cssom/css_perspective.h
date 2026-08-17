// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PERSPECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PERSPECTIVE_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_union_csskeywordvalue_cssnumericvalue_string.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"

namespace blink {

class DOMMatrix;
class ExceptionState;

// Represents a perspective value in a CSSTransformValue used for properties
// like "transform".
// See CSSPerspective.idl for more information about this class.
class CORE_EXPORT CSSPerspective final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSPerspective* Create(V8CSSPerspectiveValue*, ExceptionState&);

  // Blink-internal ways of creating CSSPerspectives.
  static CSSPerspective* FromCSSValue(const CSSFunctionValue&);

  explicit CSSPerspective(V8CSSPerspectiveValue* length);
  CSSPerspective(const CSSPerspective&) = delete;
  CSSPerspective& operator=(const CSSPerspective&) = delete;

  // Getters and setters for attributes defined in the IDL.
  V8CSSPerspectiveValue* length() { return length_.Get(); }
  void setLength(V8CSSPerspectiveValue*, ExceptionState&);

  // From CSSTransformComponent
  // Setting is2D for CSSPerspective does nothing.
  // https://drafts.css-houdini.org/css-typed-om/#dom-cssskew-is2d
  void setIs2D(bool is2D) final {}

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const final { return kPerspectiveType; }
  const CSSFunctionValue* ToCSSValue() const final;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(length_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<V8CSSPerspectiveValue> length_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PERSPECTIVE_H_
