// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_LAYOUT_ALGORITHM_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/block_break_token.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class PhysicalFragment;
struct PhysicalFragmentLink;

// The "simplified" layout algorithm will run in the following circumstances:
//  - An OOF-positioned descendant of this node (this node is its containing
//    block) has its constraints changed.
//  - A child requires "simplified" layout, i.e. an indirect-descendant
//    OOF-positioned child has its constraints changed.
//  - The block-size of the fragment has changed, and we know that it won't
//    affect any inflow children (no %-block-size descendants).
//
// This algorithm effectively performs a (convoluted) "copy" of the previous
// layout result. It will:
//  1. Copy data from the previous |LayoutResult| into the
//     |BoxFragmentBuilder|, (e.g. flags, end margin strut, etc).
//  2. Iterate through all the children and:
//    a. If OOF-positioned determine the static-position and add it as an
//       OOF-positioned candidate.
//    b. Otherwise perform layout on the inflow child (which may trigger
//       "simplified" layout on its children).
//  3. Run the |OutOfFlowLayoutPart|.
class CORE_EXPORT SimplifiedLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  SimplifiedLayoutAlgorithm(const LayoutAlgorithmParams&,
                            const LayoutResult&,
                            bool keep_old_size = false);

  void AppendNewChildFragment(const PhysicalFragment&, LogicalOffset);

  // Attempt to perform simplified layout on all children and return a new
  // result. If nullptr is returned, it means that simplified layout isn't
  // possible.
  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&) {
    NOTREACHED();
  }

  NOINLINE const LayoutResult* LayoutWithItemsBuilder();

 private:
  void AddChildFragment(const PhysicalFragmentLink& old_fragment,
                        const PhysicalFragment& new_fragment,
                        const MarginStrut* margin_strut = nullptr,
                        bool is_self_collapsing = false);

  const LayoutResult& previous_result_;
  BoxStrut border_scrollbar_padding_;

  const WritingDirectionMode writing_direction_;

  PhysicalSize previous_physical_container_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SIMPLIFIED_LAYOUT_ALGORITHM_H_
