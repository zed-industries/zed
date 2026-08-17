// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_UTILS_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/inline/inline_cursor.h"

namespace blink {

// Returns the NG line box fragment containing the caret position of the given
// position. Returns false if the position is not in Layout NG, or does not
// have any caret position.
InlineCursor NGContainingLineBoxOf(const PositionWithAffinity&);

// Returns true if the caret positions of the two positions are in the same NG
// line box. Returns false in all other cases.
bool InSameNGLineBox(const PositionWithAffinity&, const PositionWithAffinity&);

// Given the expected line-height property of `line_height`, the metrics of the
// font for a inline box `current_height`, compute the half-leading height that
// should be added to the ascent and descent directions separately.
// https://drafts.csswg.org/css2/#leading.
FontHeight CalculateLeadingSpace(const LayoutUnit& line_height,
                                 const FontHeight& current_height);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_LINE_UTILS_H_
