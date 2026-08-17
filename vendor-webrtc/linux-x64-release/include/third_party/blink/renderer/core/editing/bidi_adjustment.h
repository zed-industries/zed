// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_BIDI_ADJUSTMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_BIDI_ADJUSTMENT_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct InlineCaretPosition;
enum class TextDirection : uint8_t;

class BidiAdjustment final {
  STATIC_ONLY(BidiAdjustment);

 public:
  // Function to be called at the end of caret position resolution, adjusting
  // the result in bidi text runs.
  static InlineCaretPosition AdjustForInlineCaretPositionResolution(
      const InlineCaretPosition&);

  // Function to be called at the end of hit tests, adjusting the result in bidi
  // text runs.
  static InlineCaretPosition AdjustForHitTest(const InlineCaretPosition&);

  // Function to be called at the end of creating a range selection by mouse
  // dragging, ensuring that the created range selection matches the dragging
  // even with bidi adjustment.
  // TODO(editing-dev): Eliminate |VisiblePosition| from this function.
  static SelectionInFlatTree AdjustForRangeSelection(
      const PositionInFlatTreeWithAffinity&,
      const PositionInFlatTreeWithAffinity&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_BIDI_ADJUSTMENT_H_
