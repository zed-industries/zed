// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HWB_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HWB_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_color_value.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// Defines a javascript HSL color interface.
// https://drafts.css-houdini.org/css-typed-om-1/#csshwb
class CORE_EXPORT CSSHWB final : public CSSColorValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor defined in the IDL.
  static CSSHWB* Create(CSSNumericValue* hue,
                        const V8CSSNumberish* white,
                        const V8CSSNumberish* black,
                        const V8CSSNumberish* alpha,
                        ExceptionState& exception_state);

  // Internal constructor used by blink.
  explicit CSSHWB(const Color&);
  CSSHWB(CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*,
         CSSNumericValue*);

  // Getters and setters from the IDL
  CSSNumericValue* h() const { return h_.Get(); }
  V8CSSNumberish* w() const;
  V8CSSNumberish* b() const;
  V8CSSNumberish* alpha() const;
  void setH(CSSNumericValue* h, ExceptionState& exception_state);
  void setW(const V8CSSNumberish* w, ExceptionState& exception_state);
  void setB(const V8CSSNumberish* b, ExceptionState& exception_state);
  void setAlpha(const V8CSSNumberish* alpha, ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(h_);
    visitor->Trace(w_);
    visitor->Trace(b_);
    visitor->Trace(alpha_);
    CSSColorValue::Trace(visitor);
  }

  Color ToColor() const final;

 private:
  Member<CSSNumericValue> h_;
  Member<CSSNumericValue> w_;
  Member<CSSNumericValue> b_;
  Member<CSSNumericValue> alpha_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_HWB_H_
