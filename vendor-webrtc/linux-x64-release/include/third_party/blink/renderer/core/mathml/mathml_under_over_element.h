// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_UNDER_OVER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_UNDER_OVER_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_scripts_element.h"

namespace blink {

class Document;

class CORE_EXPORT MathMLUnderOverElement final : public MathMLScriptsElement {
 public:
  MathMLUnderOverElement(const QualifiedName& tagName, Document& document);
  std::optional<bool> Accent() const;
  std::optional<bool> AccentUnder() const;

 private:
  void ParseAttribute(const AttributeModificationParams&) final;
};

template <>
struct DowncastTraits<MathMLUnderOverElement> {
  static bool AllowFrom(const Node& node) {
    auto* mathml_element = DynamicTo<MathMLElement>(node);
    return mathml_element && AllowFrom(*mathml_element);
  }
  static bool AllowFrom(const MathMLElement& mathml_element) {
    return mathml_element.HasTagName(mathml_names::kMunderTag) ||
           mathml_element.HasTagName(mathml_names::kMoverTag) ||
           mathml_element.HasTagName(mathml_names::kMunderoverTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_UNDER_OVER_ELEMENT_H_
