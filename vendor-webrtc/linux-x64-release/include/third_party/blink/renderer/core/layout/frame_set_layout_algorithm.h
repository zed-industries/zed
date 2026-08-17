// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class HTMLDimension;

class CORE_EXPORT FrameSetLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit FrameSetLayoutAlgorithm(const LayoutAlgorithmParams& params);

  const LayoutResult* Layout();
  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

 private:
  Vector<LayoutUnit> LayoutAxis(wtf_size_t count,
                                const Vector<HTMLDimension>& grid,
                                const Vector<int>& deltas,
                                LayoutUnit available_length);
  void LayoutChildren(const FrameSetLayoutData& layout_data);
  void LayoutChild(const LayoutInputNode&,
                   LogicalSize available_size,
                   PhysicalOffset position,
                   PhysicalSize child_size);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_ALGORITHM_H_
