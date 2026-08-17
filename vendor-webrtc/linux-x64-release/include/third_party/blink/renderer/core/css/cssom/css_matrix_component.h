// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATRIX_COMPONENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATRIX_COMPONENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"
#include "third_party/blink/renderer/core/geometry/dom_matrix.h"
#include "third_party/blink/renderer/core/geometry/dom_matrix_read_only.h"

namespace blink {

class CSSMatrixComponentOptions;

// Represents a matrix value in a CSSTransformValue used for properties like
// "transform".
// See CSSMatrixComponent.idl for more information about this class.
class CORE_EXPORT CSSMatrixComponent final : public CSSTransformComponent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructors defined in the IDL.
  static CSSMatrixComponent* Create(DOMMatrixReadOnly*,
                                    const CSSMatrixComponentOptions*);

  // Blink-internal ways of creating CSSMatrixComponents.
  static CSSMatrixComponent* FromCSSValue(const CSSFunctionValue&);

  CSSMatrixComponent(DOMMatrixReadOnly* matrix, bool is2D)
      : CSSTransformComponent(is2D), matrix_(DOMMatrix::Create(matrix)) {}
  CSSMatrixComponent(const CSSMatrixComponent&) = delete;
  CSSMatrixComponent& operator=(const CSSMatrixComponent&) = delete;

  // Getters and setters for attributes defined in the IDL.
  DOMMatrix* matrix() { return matrix_.Get(); }
  void setMatrix(DOMMatrix* matrix) { matrix_ = matrix; }

  DOMMatrix* toMatrix(ExceptionState&) const final;

  // Internal methods - from CSSTransformComponent.
  TransformComponentType GetType() const final { return kMatrixType; }
  const CSSFunctionValue* ToCSSValue() const final;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(matrix_);
    CSSTransformComponent::Trace(visitor);
  }

 private:
  Member<DOMMatrix> matrix_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATRIX_COMPONENT_H_
