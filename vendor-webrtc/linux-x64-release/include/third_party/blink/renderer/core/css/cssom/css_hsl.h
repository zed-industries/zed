// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HSL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HSL_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_color_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// Defines a javascript HSL color interface.
// https://drafts.css-houdini.org/css-typed-om-1/#csshsl
class CORE_EXPORT CSSHSL final : public CSSColorValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSHSL* Create(CSSNumericValue* hue,
                        const V8CSSNumberish* saturation,
                        const V8CSSNumberish* lightness,
                        const V8CSSNumberish* alpha,
                        ExceptionState& exception_state);

  // Internal constructor used by blink.
  explicit CSSHSL(const Color&);
  CSSHSL(CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*);

  // Getters and setters from the IDL
  CSSNumericValue* h() const { return h_.Get(); }
  V8CSSNumberish* s() const;
  V8CSSNumberish* l() const;
  V8CSSNumberish* alpha() const;
  void setH(CSSNumericValue* h, ExceptionState& exception_state);
  void setS(const V8CSSNumberish* s, ExceptionState& exception_state);
  void setL(const V8CSSNumberish* l, ExceptionState& exception_state);
  void setAlpha(const V8CSSNumberish* alpha, ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(h_);
    visitor->Trace(s_);
    visitor->Trace(l_);
    visitor->Trace(alpha_);
    CSSColorValue::Trace(visitor);
  }

  Color ToColor() const final;

 private:
  Member<CSSNumericValue> h_;
  Member<CSSNumericValue> s_;
  Member<CSSNumericValue> l_;
  Member<CSSNumericValue> alpha_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HSL_H_
