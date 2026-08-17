// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_FRACTION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_FRACTION_ELEMENT_H_

#include "third_party/blink/renderer/core/mathml/mathml_element.h"

namespace blink {

class ComputedStyleBuilder;
class CSSToLengthConversionData;

class MathMLFractionElement final : public MathMLElement {
 public:
  explicit MathMLFractionElement(Document&);

  void AddMathFractionBarThicknessIfNeeded(ComputedStyleBuilder&,
                                           const CSSToLengthConversionData&);

 private:
  void ParseAttribute(const AttributeModificationParams&) final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_FRACTION_ELEMENT_H_
