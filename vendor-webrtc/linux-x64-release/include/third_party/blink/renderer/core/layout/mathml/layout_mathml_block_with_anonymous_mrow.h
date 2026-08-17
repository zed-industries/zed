// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_WITH_ANONYMOUS_MROW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_WITH_ANONYMOUS_MROW_H_

#include "third_party/blink/renderer/core/layout/mathml/layout_mathml_block.h"

namespace blink {

class LayoutMathMLBlockWithAnonymousMrow : public LayoutMathMLBlock {
 public:
  explicit LayoutMathMLBlockWithAnonymousMrow(Element*);

  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_LAYOUT_MATHML_BLOCK_WITH_ANONYMOUS_MROW_H_
