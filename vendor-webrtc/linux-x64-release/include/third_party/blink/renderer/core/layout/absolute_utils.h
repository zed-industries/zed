// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ABSOLUTE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ABSOLUTE_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"

namespace blink {

class BlockNode;
class ConstraintSpace;
class LayoutResult;
struct LogicalStaticPosition;

struct CORE_EXPORT LogicalOofDimensions {
  LayoutUnit MarginBoxInlineStart() const {
    return inset.inline_start - margins.inline_start;
  }
  LayoutUnit MarginBoxBlockStart() const {
    return inset.block_start - margins.block_start;
  }
  LayoutUnit MarginBoxInlineEnd() const {
    return inset.inline_start + size.inline_size + margins.inline_end;
  }
  LayoutUnit MarginBoxBlockEnd() const {
    return inset.block_start + size.block_size + margins.block_end;
  }

  BoxStrut inset;
  LogicalSize size = {kIndefiniteSize, kIndefiniteSize};
  BoxStrut margins;
};

struct CORE_EXPORT LogicalOofInsets {
  std::optional<LayoutUnit> inline_start;
  std::optional<LayoutUnit> inline_end;
  std::optional<LayoutUnit> block_start;
  std::optional<LayoutUnit> block_end;
};

// The resolved alignment in the candidate's writing-direction.
struct LogicalAlignment {
  StyleSelfAlignmentData inline_alignment =
      StyleSelfAlignmentData(ItemPosition::kNormal,
                             OverflowAlignment::kDefault);
  StyleSelfAlignmentData block_alignment =
      StyleSelfAlignmentData(ItemPosition::kNormal,
                             OverflowAlignment::kDefault);
};

LogicalAlignment ComputeAlignment(
    const ComputedStyle& style,
    bool is_containing_block_scrollable,
    WritingDirectionMode container_writing_direction,
    WritingDirectionMode self_writing_direction);

// Represents the position that `anchor-center` alignment keyword resolves to.
// A nullopt means that anchor-center alignment doesn't apply to the axis.
struct LogicalAnchorCenterPosition {
  std::optional<LayoutUnit> inline_offset;
  std::optional<LayoutUnit> block_offset;
};

LogicalAnchorCenterPosition ComputeAnchorCenterPosition(
    const ComputedStyle& style,
    const LogicalAlignment& alignment,
    WritingDirectionMode writing_direction,
    LogicalSize available_size);

CORE_EXPORT LogicalOofInsets
ComputeOutOfFlowInsets(const ComputedStyle& style,
                       const LogicalSize& available_size,
                       const LogicalAlignment&,
                       WritingDirectionMode self_writing_direction);

struct CORE_EXPORT InsetModifiedContainingBlock {
  // The original containing block size that the insets refer to.
  LogicalSize available_size;

  // Resolved insets of the IMCB.
  LayoutUnit inline_start;
  LayoutUnit inline_end;
  LayoutUnit block_start;
  LayoutUnit block_end;

  // If the axis has any auto inset.
  bool has_auto_inline_inset = false;
  bool has_auto_block_inset = false;

  // Indicates how the insets were calculated. Besides, when we need to clamp
  // the IMCB size, the stronger inset (i.e., the inset we are biased towards)
  // stays at the same place, and the weaker inset is moved; If both insets are
  // equally strong, both are moved by the same amount.
  enum class InsetBias { kStart, kEnd, kEqual };
  InsetBias inline_inset_bias = InsetBias::kStart;
  InsetBias block_inset_bias = InsetBias::kStart;

  // If safe alignment is specified (e.g. "align-self: safe end") and the
  // object overflows its containing block it'll become start aligned instead.
  // This field indicates the "start" edge of the containing block.
  std::optional<InsetBias> inline_safe_inset_bias;
  std::optional<InsetBias> block_safe_inset_bias;

  // If non-normal alignment is specified (e.g. "align-self: center") we'll
  // adjust the position so that it doesn't overflow the containing block.
  // This field indicates the "start" edge of the containing block.
  std::optional<InsetBias> inline_default_inset_bias;
  std::optional<InsetBias> block_default_inset_bias;

  LayoutUnit InlineEndOffset() const {
    return available_size.inline_size - inline_end;
  }
  LayoutUnit BlockEndOffset() const {
    return available_size.block_size - block_end;
  }
  LayoutUnit InlineSize() const {
    return available_size.inline_size - inline_start - inline_end;
  }
  LayoutUnit BlockSize() const {
    return available_size.block_size - block_start - block_end;
  }
  LogicalSize Size() const { return LogicalSize(InlineSize(), BlockSize()); }
};

// Computes the inset-modified containing block for resolving size, margins and
// final position of the out-of-flow node.
// https://www.w3.org/TR/css-position-3/#inset-modified-containing-block
CORE_EXPORT InsetModifiedContainingBlock ComputeInsetModifiedContainingBlock(
    const BlockNode& node,
    const LogicalSize& available_size,
    const LogicalAlignment&,
    const LogicalOofInsets&,
    const LogicalStaticPosition&,
    WritingDirectionMode container_writing_direction,
    WritingDirectionMode self_writing_direction);

// Similar to `ComputeInsetModifiedContainingBlock`, but returns the
// scroll-adjusted IMCB at the initial scroll position, which is for the
// position fallback algorithm only.
// https://www.w3.org/TR/css-anchor-position-1/#fallback-apply
CORE_EXPORT InsetModifiedContainingBlock
ComputeIMCBForPositionFallback(const LogicalSize& available_size,
                               const LogicalAlignment&,
                               const LogicalOofInsets&,
                               const LogicalStaticPosition&,
                               const ComputedStyle&,
                               WritingDirectionMode container_writing_direction,
                               WritingDirectionMode self_writing_direction);

// The following routines implement the absolute size resolution algorithm.
// https://www.w3.org/TR/css-position-3/#abs-non-replaced-width
//
// The size is computed as |LogicalOofDimensions|.
// It needs to be computed in 2 stages:
// 1. The inline-dimensions with |ComputeOofInlineDimensions|.
// 2. The block-dimensions with |ComputeOofBlockDimensions|.
//
// NOTE: |ComputeOofInlineDimensions| may call |ComputeOofBlockDimensions| if
// its required to correctly determine the min/max content sizes.

// |replaced_size| should be set if and only if element is replaced element.
// Will return true if |BlockNode::ComputeMinMaxSizes| was called.
CORE_EXPORT bool ComputeOofInlineDimensions(
    const BlockNode&,
    const ComputedStyle& style,
    const ConstraintSpace&,
    const InsetModifiedContainingBlock&,
    const LogicalAnchorCenterPosition&,
    const LogicalAlignment&,
    const BoxStrut& border_padding,
    const std::optional<LogicalSize>& replaced_size,
    const BoxStrut& container_insets,
    WritingDirectionMode container_writing_direction,
    LogicalOofDimensions* dimensions);

// If layout was performed to determine the position, this will be returned
// otherwise it will return nullptr.
CORE_EXPORT const LayoutResult* ComputeOofBlockDimensions(
    const BlockNode&,
    const ComputedStyle& style,
    const ConstraintSpace&,
    const InsetModifiedContainingBlock&,
    const LogicalAnchorCenterPosition&,
    const LogicalAlignment&,
    const BoxStrut& border_padding,
    const std::optional<LogicalSize>& replaced_size,
    const BoxStrut& container_insets,
    WritingDirectionMode container_writing_direction,
    LogicalOofDimensions* dimensions);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ABSOLUTE_UTILS_H_
