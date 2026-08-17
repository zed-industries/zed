// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_COMPONENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_COMPONENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_function_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DOMMatrix;
class ExceptionState;

// CSSTransformComponent is the base class used for the representations of
// the individual CSS transforms. They are combined in a CSSTransformValue
// before they can be used as a value for properties like "transform".
// See CSSTransformComponent.idl for more information about this class.
class CORE_EXPORT CSSTransformComponent : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum TransformComponentType {
    kMatrixType,
    kPerspectiveType,
    kRotationType,
    kScaleType,
    kSkewType,
    kSkewXType,
    kSkewYType,
    kTranslationType,
  };

  CSSTransformComponent(const CSSTransformComponent&) = delete;
  CSSTransformComponent& operator=(const CSSTransformComponent&) = delete;
  ~CSSTransformComponent() override = default;

  // Blink-internal ways of creating CSSTransformComponents.
  static CSSTransformComponent* FromCSSValue(const CSSValue&);

  // Getters and setters for attributes defined in the IDL.
  bool is2D() const { return is2D_; }
  virtual void setIs2D(bool is2D) { is2D_ = is2D; }
  String toString() const;

  virtual DOMMatrix* toMatrix(ExceptionState&) const = 0;

  // Internal methods.
  virtual TransformComponentType GetType() const = 0;
  virtual const CSSFunctionValue* ToCSSValue() const = 0;

 protected:
  CSSTransformComponent(bool is2D) : is2D_(is2D) {}

 private:
  bool is2D_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_COMPONENT_H_
