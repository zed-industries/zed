// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"

namespace blink {

class V8CSSMathOperator;

// Represents mathematical operations, acting as nodes in a tree of
// CSSNumericValues. See CSSMathValue.idl for more information about this class.
class CORE_EXPORT CSSMathValue : public CSSNumericValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSMathValue(const CSSMathValue&) = delete;
  CSSMathValue& operator=(const CSSMathValue&) = delete;

  virtual V8CSSMathOperator getOperator() const = 0;

  // From CSSNumericValue.
  bool IsUnitValue() const final { return false; }

  // From CSSStyleValue.
  const CSSValue* ToCSSValue() const final;

 protected:
  CSSMathValue(const CSSNumericValueType& type) : CSSNumericValue(type) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_MATH_VALUE_H_
