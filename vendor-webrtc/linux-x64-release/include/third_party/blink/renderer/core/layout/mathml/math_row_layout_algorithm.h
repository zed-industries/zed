// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_ROW_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_ROW_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/block_node.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class CORE_EXPORT MathRowLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit MathRowLayoutAlgorithm(const LayoutAlgorithmParams& params);

  struct ChildWithOffsetAndMargins {
    DISALLOW_NEW();
    ChildWithOffsetAndMargins(const BlockNode& child,
                              const BoxStrut& margins,
                              LogicalOffset offset,
                              const LayoutResult* result)
        : child(child),
          margins(margins),
          offset(offset),
          result(std::move(result)) {}

    void Trace(Visitor* visitor) const {
      visitor->Trace(child);
      visitor->Trace(result);
    }

    BlockNode child;
    BoxStrut margins;
    LogicalOffset offset;
    Member<const LayoutResult> result;
  };
  typedef HeapVector<ChildWithOffsetAndMargins, 4> ChildrenVector;

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

 private:
  void LayoutRowItems(ChildrenVector*,
                      LayoutUnit* max_row_block_baseline,
                      LogicalSize* row_total_size);
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::MathRowLayoutAlgorithm::ChildWithOffsetAndMargins)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MATHML_MATH_ROW_LAYOUT_ALGORITHM_H_
