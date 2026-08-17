// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_PADDED_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_PADDED_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_row_element.h"

namespace blink {

class ComputedStyle;
class ComputedStyleBuilder;
class Document;

class CORE_EXPORT MathMLPaddedElement final : public MathMLRowElement {
 public:
  explicit MathMLPaddedElement(Document&);

  void AddMathBaselineIfNeeded(ComputedStyleBuilder&,
                               const CSSToLengthConversionData&);
  void AddMathPaddedDepthIfNeeded(ComputedStyleBuilder&,
                                  const CSSToLengthConversionData&);
  void AddMathPaddedLSpaceIfNeeded(ComputedStyleBuilder&,
                                   const CSSToLengthConversionData&);
  void AddMathPaddedVOffsetIfNeeded(ComputedStyleBuilder&,
                                    const CSSToLengthConversionData&);

 private:
  void ParseAttribute(const AttributeModificationParams&) final;
  bool IsPresentationAttribute(const QualifiedName&) const final;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) final;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) final;

  bool IsGroupingElement() const final { return false; }
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_PADDED_ELEMENT_H_
