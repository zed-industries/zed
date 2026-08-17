// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_BORDER_BOX_LAYOUT_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_BORDER_BOX_LAYOUT_ALGORITHM_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/box_fragment_builder.h"
#include "third_party/blink/renderer/core/layout/layout_algorithm.h"

namespace blink {

class BlockBreakToken;
class BlockNode;
class ConstraintSpace;

struct PageAreaLayoutParams {
  STACK_ALLOCATED();

 public:
  const BlockBreakToken* break_token = nullptr;
  const PhysicalBoxFragment* template_fragmentainer = nullptr;
};

// Algorithm that generates a fragment for the border box of a page. Creates a
// page area child (which is a fragmentainer), where the actual fragmented
// document contents will be placed.
//
// See https://drafts.csswg.org/css-page-3/#page-model
//
// The page border box is the innermost part of what the spec refers to as "page
// box". The outermost part is the page container, which is the parent of a page
// border box.
class CORE_EXPORT PageBorderBoxLayoutAlgorithm
    : public LayoutAlgorithm<BlockNode, BoxFragmentBuilder, BlockBreakToken> {
 public:
  PageBorderBoxLayoutAlgorithm(const LayoutAlgorithmParams& params,
                               const BlockNode& content_node,
                               const PageAreaLayoutParams&);

  const LayoutResult* Layout();

  MinMaxSizesResult ComputeMinMaxSizes(const MinMaxSizesFloatInput&) {
    NOTREACHED();
  }

  // Return the outgoing break token from the fragmentainer (page area).
  const BlockBreakToken* FragmentainerBreakToken() const {
    return fragmentainer_break_token_;
  }

 private:
  ConstraintSpace CreateConstraintSpaceForPageArea() const;

  // The document content node to lay out inside the fragmentainer. Typically a
  // LayoutView.
  const BlockNode& content_node_;

  const PageAreaLayoutParams& page_area_params_;

  const BlockBreakToken* fragmentainer_break_token_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_PAGE_BORDER_BOX_LAYOUT_ALGORITHM_H_
