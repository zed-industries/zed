// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_JUSTIFICATION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_JUSTIFICATION_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/inline/logical_line_item.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class LineInfo;

enum class JustificationTarget {
  kNormal,
  kRubyBase,
  kRubyText,
  kSvgText,
};

// Justify the line. This changes the size of items by adding spacing.
// Returns std::nullopt if justification failed and should fall back to
// start-aligned.
std::optional<LayoutUnit> ApplyJustification(LayoutUnit space,
                                             JustificationTarget target,
                                             LineInfo* line_info);

// Compute `inset` value without applying justification.
// `line_info.IsRubyBase()` must be true.
std::optional<LayoutUnit> ComputeRubyBaseInset(LayoutUnit space,
                                               const LineInfo& line_info);

// Add spaces to a part of a line. This works only for ruby-base and ruby-text
// for now. Returns false if we couldn't expand the line.
bool ApplyLeftAndRightExpansion(LayoutUnit leading_expansion,
                                LayoutUnit trailing_expansion,
                                base::span<LogicalLineItem> items);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_JUSTIFICATION_UTILS_H_
