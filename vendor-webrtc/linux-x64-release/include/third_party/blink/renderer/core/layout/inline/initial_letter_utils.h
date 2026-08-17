// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INITIAL_LETTER_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INITIAL_LETTER_UTILS_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class LineInfo;
class LogicalLineItems;
struct BfcOffset;
struct BoxStrut;
struct ExclusionArea;
struct FontHeight;

// Adjust text position of texts in inline text box and returns adjusted
// `FontHeight` to fit initial letter box in block direction.
// Note: `LineBreaker::NextLine()` adjust inline size.
FontHeight AdjustInitialLetterInTextPosition(const FontHeight& line_box_metrics,
                                             LogicalLineItems* line_box);

// Calculate inline size of initial letter text.
LayoutUnit CalculateInitialLetterBoxInlineSize(const LineInfo& line_info);

// Places initial letter box and returns `ExclusionArea` contains initial letter
// box. `initial_letter_box_origin` holds left/right edge.
const ExclusionArea* PostPlaceInitialLetterBox(
    const FontHeight& line_box_metrics,
    const BoxStrut& initial_letter_box_margins,
    LogicalLineItems* line_box,
    const BfcOffset& line_origin,
    LineInfo* line_info);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_INITIAL_LETTER_UTILS_H_
