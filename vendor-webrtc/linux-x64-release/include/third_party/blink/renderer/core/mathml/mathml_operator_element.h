// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_OPERATOR_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_OPERATOR_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_token_element.h"

namespace blink {

class ComputedStyleBuilder;
class CSSToLengthConversionData;
class Document;

enum class MathMLOperatorDictionaryCategory : uint8_t;

// Math units are 1/18em.
constexpr double kMathUnitFraction = 1.0 / 18.0;

class CORE_EXPORT MathMLOperatorElement final : public MathMLTokenElement {
 public:
  explicit MathMLOperatorElement(Document&);

  enum OperatorPropertyFlag {
    kStretchy = 0x1,
    kSymmetric = 0x2,
    kLargeOp = 0x4,
    kMovableLimits = 0x8,
  };
  // Query whether given flag is set in the operator dictionary.
  bool HasBooleanProperty(OperatorPropertyFlag);

  void AddMathLSpaceIfNeeded(ComputedStyleBuilder&,
                             const CSSToLengthConversionData&);
  void AddMathRSpaceIfNeeded(ComputedStyleBuilder&,
                             const CSSToLengthConversionData&);
  void AddMathMinSizeIfNeeded(ComputedStyleBuilder&,
                              const CSSToLengthConversionData&);
  void AddMathMaxSizeIfNeeded(ComputedStyleBuilder&,
                              const CSSToLengthConversionData&);
  bool IsVertical();

  double DefaultLeadingSpace();
  double DefaultTrailingSpace();

  void CheckFormAfterSiblingChange();

 private:
  // Whether the operator stretches along the block or inline axis.
  // https://w3c.github.io/mathml-core/#dfn-stretch-axis
  std::optional<bool> is_vertical_;
  // Operator properties calculated from dictionary and attributes.
  // It contains dirty flags to allow efficient dictionary updating.
  // https://w3c.github.io/mathml-core/#dictionary-based-attributes
  struct Properties {
    MathMLOperatorDictionaryCategory dictionary_category;
    unsigned flags : 4;
    unsigned dirty_flags : 4;
  };
  Properties properties_;
  void ComputeDictionaryCategory();
  void ComputeOperatorProperty(OperatorPropertyFlag);
  void SetOperatorFormDirty();
  void ParseAttribute(const AttributeModificationParams&) final;
  void SetOperatorPropertyDirtyFlagIfNeeded(const AttributeModificationParams&,
                                            const OperatorPropertyFlag&,
                                            bool& needs_layout);
  void ChildrenChanged(const ChildrenChange&) final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_OPERATOR_ELEMENT_H_
