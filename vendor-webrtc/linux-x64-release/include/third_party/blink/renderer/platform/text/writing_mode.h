/*
 * Copyright (c) 2012, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_MODE_H_

#include <cstdint>
#include <iosfwd>
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// These values are named to match the CSS keywords they correspond to: namely
// horizontal-tb, vertical-rl and vertical-lr.
// Since these names aren't very self-explanatory, where possible use the
// inline utility functions below.
enum class WritingMode : uint8_t {
  kHorizontalTb = 0,
  kVerticalRl = 1,
  kVerticalLr = 2,
  // sideways-rl and sideways-lr are only supported by LayoutNG.
  kSidewaysRl = 3,
  kSidewaysLr = 4,

  kMaxWritingMode = kSidewaysLr,
};

// Lines have horizontal orientation; modes horizontal-tb.
inline bool IsHorizontalWritingMode(WritingMode writing_mode) {
  return writing_mode == WritingMode::kHorizontalTb;
}

// Lines have vertical orientation; modes vertical-lr, vertical-rl.
inline bool IsVerticalWritingMode(WritingMode writing_mode) {
  return writing_mode == WritingMode::kVerticalLr ||
         writing_mode == WritingMode::kVerticalRl;
}

// Bottom of the line occurs earlier in the block; modes vertical-lr.
inline bool IsFlippedLinesWritingMode(WritingMode writing_mode) {
  return writing_mode == WritingMode::kVerticalLr;
}

// In flipped-lines writing mode, 'line-over' and 'block-start' don't match.
// When dealing with the logical coordinate system in the [line-relative
// directions], 'vertical-lr' has 'line-over' on right, which is equivalent to
// the 'vertical-rl' in the flow-relative directions.
// https://drafts.csswg.org/css-writing-modes-3/#line-directions
inline WritingMode ToLineWritingMode(WritingMode writing_mode) {
  return !IsFlippedLinesWritingMode(writing_mode) ? writing_mode
                                                  : WritingMode::kVerticalRl;
}

// Block progression increases in the opposite direction to normal; modes
// vertical-rl and sideways-rl.
inline bool IsFlippedBlocksWritingMode(WritingMode writing_mode) {
  return writing_mode == WritingMode::kVerticalRl ||
         writing_mode == WritingMode::kSidewaysRl;
}

// Whether the child and the containing block are parallel to each other.
// Example: vertical-rl and vertical-lr
inline bool IsParallelWritingMode(WritingMode a, WritingMode b) {
  return (a == WritingMode::kHorizontalTb) == (b == WritingMode::kHorizontalTb);
}

// Returns true if the specified writing-mode is a horizontal typographic
// mode; modes horizontal-tb, sideways-lr, and sideways-rl.
// https://drafts.csswg.org/css-writing-modes/#horizontal-typographic-mode
inline bool IsHorizontalTypographicMode(WritingMode writing_mode) {
  return writing_mode == WritingMode::kHorizontalTb ||
         writing_mode == WritingMode::kSidewaysLr ||
         writing_mode == WritingMode::kSidewaysRl;
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, WritingMode);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_WRITING_MODE_H_
