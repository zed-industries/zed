// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BOX_FRAGMENT_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BOX_FRAGMENT_BUILDER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/fragment_builder.h"
#include "third_party/blink/renderer/core/layout/inline/inline_break_token.h"
#include "third_party/blink/renderer/core/layout/inline/inline_node.h"
#include "third_party/blink/renderer/core/layout/inline/physical_line_box_fragment.h"
#include "third_party/blink/renderer/platform/fonts/font_height.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class LayoutResult;
class LogicalLineContainer;
class LogicalLineItems;

class CORE_EXPORT LineBoxFragmentBuilder final : public FragmentBuilder {
  STACK_ALLOCATED();

 public:
  LineBoxFragmentBuilder(InlineNode node,
                         const ComputedStyle* style,
                         const ConstraintSpace& space,
                         WritingDirectionMode writing_direction,
                         const InlineBreakToken* break_token)
      : FragmentBuilder(
            node,
            style,
            space,
            // Always use LTR because line items are in visual order.
            {writing_direction.GetWritingMode(), TextDirection::kLtr},
            break_token),
        line_box_type_(PhysicalLineBoxFragment::kNormalLineBox),
        base_direction_(TextDirection::kLtr) {}
  LineBoxFragmentBuilder(const LineBoxFragmentBuilder&) = delete;
  LineBoxFragmentBuilder& operator=(const LineBoxFragmentBuilder&) = delete;

  void Reset();

  const InlineBreakToken* PreviousBreakToken() const {
    return To<InlineBreakToken>(previous_break_token_);
  }

  LayoutUnit LineHeight() const {
    return metrics_.LineHeight().ClampNegativeToZero();
  }

  void SetInlineSize(LayoutUnit inline_size) {
    size_.inline_size = inline_size;
  }

  void SetHangInlineSize(LayoutUnit hang_inline_size) {
    hang_inline_size_ = hang_inline_size;
  }

  // Mark this line box is an "empty" line box. See LineBoxType.
  void SetIsEmptyLineBox();

  std::optional<LayoutUnit> LineBoxBfcBlockOffset() const {
    return line_box_bfc_block_offset_;
  }
  void SetLineBoxBfcBlockOffset(LayoutUnit offset) {
    DCHECK(bfc_block_offset_);
    line_box_bfc_block_offset_ = offset;
  }

  const std::optional<LayoutUnit>& TrimBlockEndBy() const {
    return trim_block_end_by_;
  }
  void SetTrimBlockEndBy(LayoutUnit trim_block_end_by) {
    trim_block_end_by_ = trim_block_end_by;
  }

  void SetAnnotationBlockOffsetAdjustment(LayoutUnit adjustment) {
    annotation_block_offset_adjustment_ = adjustment;
  }

  const FontHeight& Metrics() const { return metrics_; }
  void SetMetrics(const FontHeight& metrics) { metrics_ = metrics; }

  void SetBaseDirection(TextDirection direction) {
    base_direction_ = direction;
  }

  // Set the break token for the fragment to build.
  // Is nullptr if we didn't break.
  void SetBreakToken(const InlineBreakToken* break_token) {
    break_token_ = break_token;
  }

  // Propagate data in |ChildList| without adding them to this builder. When
  // adding children as fragment items, they appear in the container, but there
  // are some data that should be propagated through line box fragments.
  void PropagateChildrenData(LogicalLineContainer& container);

  void SetClearanceAfterLine(LayoutUnit clearance) {
    clearance_after_line_ = clearance;
  }

  // Creates the fragment. Can only be called once.
  const LayoutResult* ToLineBoxFragment();

 private:
  void PropagateChildrenDataFromLineItems(LogicalLineItems& children);

  std::optional<LayoutUnit> line_box_bfc_block_offset_;
  std::optional<LayoutUnit> clearance_after_line_;
  std::optional<LayoutUnit> trim_block_end_by_;
  LayoutUnit annotation_block_offset_adjustment_;
  FontHeight metrics_ = FontHeight::Empty();
  LayoutUnit hang_inline_size_;
  PhysicalLineBoxFragment::LineBoxType line_box_type_;
  TextDirection base_direction_;

  friend class LayoutResult;
  friend class PhysicalLineBoxFragment;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_BOX_FRAGMENT_BUILDER_H_
