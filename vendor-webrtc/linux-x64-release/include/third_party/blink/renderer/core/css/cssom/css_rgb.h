// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RGB_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RGB_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_color_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// Defines a javascript RGB color interface.
// https://drafts.css-houdini.org/css-typed-om-1/#dom-cssrgb-r
class CORE_EXPORT CSSRGB final : public CSSColorValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSRGB* Create(const V8CSSNumberish* r,
                        const V8CSSNumberish* g,
                        const V8CSSNumberish* b,
                        const V8CSSNumberish* alpha,
                        ExceptionState& exception_state);

  // Internal constructor used by blink.
  explicit CSSRGB(const Color&);
  CSSRGB(CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*);

  // Getters and setters from the IDL
  V8CSSNumberish* r() const;
  V8CSSNumberish* g() const;
  V8CSSNumberish* b() const;
  V8CSSNumberish* alpha() const;
  void setR(const V8CSSNumberish* r, ExceptionState& exception_state);
  void setG(const V8CSSNumberish* g, ExceptionState& exception_state);
  void setB(const V8CSSNumberish* b, ExceptionState& exception_state);
  void setAlpha(const V8CSSNumberish* alpha, ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(r_);
    visitor->Trace(g_);
    visitor->Trace(b_);
    visitor->Trace(alpha_);
    CSSColorValue::Trace(visitor);
  }

  Color ToColor() const final;

 private:
  Member<CSSNumericValue> r_;
  Member<CSSNumericValue> g_;
  Member<CSSNumericValue> b_;
  Member<CSSNumericValue> alpha_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RGB_H_
