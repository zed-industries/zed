// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class ConstraintSpace;
enum class BreakStatus;

class CORE_EXPORT FieldsetLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  explicit FieldsetLayoutAlgorithm(const LayoutAlgorithmParams& params);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&);

 private:
  BreakStatus LayoutChildren();
  void LayoutLegend(BlockNode& legend);
  BreakStatus LayoutFieldsetContent(BlockNode& fieldset_content,
                                    const BlockBreakToken* content_break_token,
                                    LogicalSize adjusted_padding_box_size,
                                    bool has_legend);

  const ConstraintSpace CreateConstraintSpaceForLegend(
      BlockNode legend,
      LogicalSize available_size,
      LogicalSize percentage_size);
  const ConstraintSpace CreateConstraintSpaceForFieldsetContent(
      BlockNode fieldset_content,
      LogicalSize padding_box_size,
      LayoutUnit block_offset);

  // Return the amount of block space available in the current fragmentainer
  // for the node being laid out by this algorithm.
  LayoutUnit FragmentainerSpaceAvailable() const;

  // Consume all remaining fragmentainer space. This happens when we decide to
  // break before a child.
  //
  // https://www.w3.org/TR/css-break-3/#box-splitting
  void ConsumeRemainingFragmentainerSpace();

  const WritingDirectionMode writing_direction_;

  LayoutUnit intrinsic_block_size_;
  const LayoutUnit consumed_block_size_;
  LogicalSize border_box_size_;

  // The legend may eat from the available content box block size. This
  // represents the minimum block size needed by the border box to encompass
  // the legend.
  LayoutUnit minimum_border_box_block_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FORMS_FIELDSET_LAYOUT_ALGORITHM_H_
