// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_DIRECTION_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_DIRECTION_MODE_H_

#include "third_party/blink/renderer/platform/geometry/logical_direction.h"
#include "third_party/blink/renderer/platform/geometry/physical_direction.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class packs |WritingMode| and |TextDirection|, two enums that are often
// used and passed around together, into the size of the minimum memory align.
class PLATFORM_EXPORT WritingDirectionMode {
  DISALLOW_NEW();

 public:
  WritingDirectionMode(WritingMode writing_mode, TextDirection direction)
      : writing_mode_(writing_mode), direction_(direction) {}

  //
  // Inline direction functions.
  //
  TextDirection Direction() const { return direction_; }
  void SetDirection(TextDirection direction) { direction_ = direction; }

  bool IsLtr() const { return blink::IsLtr(direction_); }
  bool IsRtl() const { return blink::IsRtl(direction_); }

  //
  // Block direction functions.
  //
  WritingMode GetWritingMode() const { return writing_mode_; }
  void SetWritingMode(WritingMode writing_mode) {
    writing_mode_ = writing_mode;
  }

  bool IsHorizontal() const { return IsHorizontalWritingMode(writing_mode_); }

  // Block progression increases in the opposite direction to normal; modes
  // vertical-rl.
  bool IsFlippedBlocks() const {
    return IsFlippedBlocksWritingMode(writing_mode_);
  }

  bool IsFlippedInlines() const {
    return IsRtl() ^ (writing_mode_ == WritingMode::kSidewaysLr);
  }

  // Bottom of the line occurs earlier in the block; modes vertical-lr.
  bool IsFlippedLines() const {
    return IsFlippedLinesWritingMode(writing_mode_);
  }

  // Returns whether x/y is flipped.
  bool IsFlippedX() const;
  bool IsFlippedY() const;

  //
  // Functions for both inline and block directions.
  //
  bool IsHorizontalLtr() const { return IsHorizontal() && IsLtr(); }

  // Returns a physical direction corresponding to a logical direction.
  PhysicalDirection InlineStart() const;
  PhysicalDirection InlineEnd() const;
  PhysicalDirection BlockStart() const;
  PhysicalDirection BlockEnd() const;
  PhysicalDirection LineOver() const;
  PhysicalDirection LineUnder() const;

  // Returns a logical direction corresponding to a physical direction.
  LogicalDirection Top() const;
  LogicalDirection Right() const;
  LogicalDirection Left() const;
  LogicalDirection Bottom() const;

  bool operator==(const WritingDirectionMode& other) const {
    return writing_mode_ == other.writing_mode_ &&
           direction_ == other.direction_;
  }
  bool operator!=(const WritingDirectionMode& other) const {
    return !operator==(other);
  }

 private:
  WritingMode writing_mode_;
  TextDirection direction_;
};

inline bool WritingDirectionMode::IsFlippedX() const {
  if (IsHorizontal())
    return IsRtl();
  return IsFlippedBlocks();
}

inline bool WritingDirectionMode::IsFlippedY() const {
  if (IsHorizontal()) {
    DCHECK(!IsFlippedBlocks());
    return false;
  }
  return IsFlippedInlines();
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         const WritingDirectionMode&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_DIRECTION_MODE_H_
