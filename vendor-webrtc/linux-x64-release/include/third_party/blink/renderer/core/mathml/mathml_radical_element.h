// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_RADICAL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_RADICAL_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/mathml/mathml_row_element.h"

namespace blink {

class Document;

class CORE_EXPORT MathMLRadicalElement : public MathMLRowElement {
 public:
  MathMLRadicalElement(const QualifiedName&, Document&);

  bool HasIndex() const;

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  bool IsGroupingElement() const final { return false; }
};

template <>
struct DowncastTraits<MathMLRadicalElement> {
  static bool AllowFrom(const Node& node) {
    auto* mathml_element = DynamicTo<MathMLElement>(node);
    return mathml_element && AllowFrom(*mathml_element);
  }
  static bool AllowFrom(const MathMLElement& mathml_element) {
    return mathml_element.HasTagName(mathml_names::kMsqrtTag) ||
           mathml_element.HasTagName(mathml_names::kMrootTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_RADICAL_ELEMENT_H_
