// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TABLE_CELL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TABLE_CELL_ELEMENT_H_

#include "third_party/blink/renderer/core/mathml/mathml_element.h"

namespace blink {

class CORE_EXPORT MathMLTableCellElement final : public MathMLElement {
 public:
  explicit MathMLTableCellElement(Document&);

  unsigned colSpan() const;
  unsigned rowSpan() const;

 private:
  void ParseAttribute(const AttributeModificationParams&) final;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MATHML_MATHML_TABLE_CELL_ELEMENT_H_
