// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_SIDES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_SIDES_H_

#include <utility>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Presence of something pertaining to box sides (e.g. borders, padding or
// insets), in the logical coordinate space. Note that all sides are set to true
// initially.
struct LogicalBoxSides {
 public:
  bool inline_start = true;
  bool inline_end = true;
  bool block_start = true;
  bool block_end = true;

  LogicalBoxSides() = default;
  LogicalBoxSides(bool inline_start,
                  bool inline_end,
                  bool block_start,
                  bool block_end)
      : inline_start(inline_start),
        inline_end(inline_end),
        block_start(block_start),
        block_end(block_end) {}

  bool operator==(const LogicalBoxSides& other) const {
    return block_start == other.block_start && inline_end == other.inline_end &&
           block_end == other.block_end && inline_start == other.inline_start;
  }
  bool IsEmpty() const {
    return !block_start && !inline_start && !block_end && !inline_end;
  }
};

// Presence of something pertaining to box sides (e.g. borders, padding or
// insets), in the line-logical coordinate space. Note that all sides are set to
// true initially.
struct LineLogicalBoxSides {
  STACK_ALLOCATED();

 public:
  bool block_start = true;
  bool line_right = true;
  bool block_end = true;
  bool line_left = true;

  LineLogicalBoxSides() = default;
  LineLogicalBoxSides(bool block_start,
                      bool line_right,
                      bool block_end,
                      bool line_left)
      : block_start(block_start),
        line_right(line_right),
        block_end(block_end),
        line_left(line_left) {}

  LineLogicalBoxSides(LogicalBoxSides sides, TextDirection dir)
      : block_start(sides.block_start),
        line_right(sides.inline_end),
        block_end(sides.block_end),
        line_left(sides.inline_start) {
    if (IsRtl(dir)) {
      std::swap(line_left, line_right);
    }
  }

  bool IsEmpty() const {
    return !block_start && !line_right && !block_end && !line_left;
  }
};

// Presence of something pertaining to box sides (e.g. borders, padding or
// inset), in the physical coordinate space. Note that all sides are set to true
// initially.
struct PhysicalBoxSides {
 public:
  bool top = true;
  bool right = true;
  bool bottom = true;
  bool left = true;

  PhysicalBoxSides() = default;
  PhysicalBoxSides(bool top, bool right, bool bottom, bool left)
      : top(top), right(right), bottom(bottom), left(left) {}
  PhysicalBoxSides(LineLogicalBoxSides logical, WritingMode writing_mode) {
    if (writing_mode == WritingMode::kHorizontalTb) {
      top = logical.block_start;
      right = logical.line_right;
      bottom = logical.block_end;
      left = logical.line_left;
    } else if (writing_mode == WritingMode::kSidewaysLr) {
      top = logical.line_right;
      bottom = logical.line_left;
      right = logical.block_end;
      left = logical.block_start;
    } else {
      top = logical.line_left;
      bottom = logical.line_right;
      if (writing_mode == WritingMode::kVerticalRl ||
          writing_mode == WritingMode::kSidewaysRl) {
        right = logical.block_start;
        left = logical.block_end;
      } else {
        DCHECK_EQ(writing_mode, WritingMode::kVerticalLr);
        right = logical.block_end;
        left = logical.block_start;
      }
    }
  }

  LogicalBoxSides ToLogical(WritingDirectionMode writing_direction) const {
    LogicalBoxSides logical;
    switch (writing_direction.GetWritingMode()) {
      case WritingMode::kHorizontalTb:
        logical = LogicalBoxSides(left, right, top, bottom);
        break;
      case WritingMode::kVerticalRl:
      case WritingMode::kSidewaysRl:
        logical = LogicalBoxSides(top, bottom, right, left);
        break;
      case WritingMode::kVerticalLr:
        logical = LogicalBoxSides(top, bottom, left, right);
        break;
      case WritingMode::kSidewaysLr:
        logical = LogicalBoxSides(bottom, top, left, right);
        break;
    }
    if (writing_direction.IsRtl()) {
      std::swap(logical.inline_start, logical.inline_end);
    }
    return logical;
  }

  bool operator==(const PhysicalBoxSides& other) const {
    return top == other.top && right == other.right && bottom == other.bottom &&
           left == other.left;
  }

  bool IsEmpty() const { return !top && !right && !bottom && !left; }
  bool HasAllSides() const { return top && right && bottom && left; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_BOX_SIDES_H_
