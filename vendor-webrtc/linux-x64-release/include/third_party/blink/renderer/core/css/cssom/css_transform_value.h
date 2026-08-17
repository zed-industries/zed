// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_transform_component.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

class DOMMatrix;

class CORE_EXPORT CSSTransformValue final : public CSSStyleValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CSSTransformValue* Create(
      const HeapVector<Member<CSSTransformComponent>>& transform_components,
      ExceptionState&);

  // Blink-internal constructor
  static CSSTransformValue* Create(
      const HeapVector<Member<CSSTransformComponent>>& transform_components);

  static CSSTransformValue* FromCSSValue(const CSSValue&);

  CSSTransformValue(
      const HeapVector<Member<CSSTransformComponent>>& transform_components)
      : CSSStyleValue(), transform_components_(transform_components) {}
  CSSTransformValue(const CSSTransformValue&) = delete;
  CSSTransformValue& operator=(const CSSTransformValue&) = delete;

  bool is2D() const;

  DOMMatrix* toMatrix(ExceptionState&) const;

  const CSSValue* ToCSSValue() const override;

  StyleValueType GetType() const override { return kTransformType; }

  CSSTransformComponent* AnonymousIndexedGetter(wtf_size_t index) {
    return transform_components_.at(index).Get();
  }
  IndexedPropertySetterResult AnonymousIndexedSetter(
      unsigned,
      const Member<CSSTransformComponent>,
      ExceptionState&);

  wtf_size_t length() const { return transform_components_.size(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(transform_components_);
    CSSStyleValue::Trace(visitor);
  }

 private:
  HeapVector<Member<CSSTransformComponent>> transform_components_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_TRANSFORM_VALUE_H_
