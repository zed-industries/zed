// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_H_

#include "third_party/blink/renderer/core/layout/layout_block.h"

namespace blink {

class LayoutMathMLBlock : public LayoutBlock {
 public:
  explicit LayoutMathMLBlock(Element*);

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutMathMLBlock";
  }

 private:
  bool IsMathML() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsMathMLRoot() const final;

  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const final;
  bool CanHaveChildren() const final;
  void StyleDidChange(StyleDifference, const ComputedStyle*) final;

  bool IsMonolithic() const final {
    NOT_DESTROYED();
    return true;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_H_
