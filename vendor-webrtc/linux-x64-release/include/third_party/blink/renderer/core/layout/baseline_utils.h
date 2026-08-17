// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASELINE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASELINE_UTILS_H_

#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"

namespace blink {

enum class BaselineGroup { kMajor, kMinor };

// Determines the writing-mode to read a baseline from a fragment.
inline WritingMode DetermineBaselineWritingMode(
    const WritingDirectionMode container_writing_direction,
    const WritingMode child_writing_mode,
    bool is_parallel_context) {
  // From: https://drafts.csswg.org/css-align-3/#generate-baselines
  //
  // For a parallel alignment context:
  //   "If the box establishing the alignment context has a block flow
  //    direction that is orthogonal to the axis of the alignment context, use
  //    its writing mode."
  //
  // Non-parallel:
  //   "If the child's writing-mode isn't parallel to the alignment context use
  //    either "horizontal-tb" or "vertical-lr" whichever is orthogonal."
  const auto orthogonal_writing_mode =
      (is_parallel_context)
          ? container_writing_direction.GetWritingMode()
          : ((child_writing_mode == WritingMode::kHorizontalTb)
                 ? (container_writing_direction.IsLtr()
                        ? WritingMode::kVerticalLr
                        : WritingMode::kVerticalRl)
                 : WritingMode::kHorizontalTb);
  const bool is_parallel = IsParallelWritingMode(
      container_writing_direction.GetWritingMode(), child_writing_mode);

  if (is_parallel_context)
    return is_parallel ? child_writing_mode : orthogonal_writing_mode;
  else
    return is_parallel ? orthogonal_writing_mode : child_writing_mode;
}

// There are potentially two different baseline groups for a column/row.
// See: https://www.w3.org/TR/css-align-3/#baseline-sharing-group
//
// We label these "major"/"minor" to separate them. The "major" group should be
// aligned to the appropriate "start" axis.
inline BaselineGroup DetermineBaselineGroup(
    const WritingDirectionMode container_writing_direction,
    const WritingMode baseline_writing_mode,
    bool is_parallel_context,
    bool is_last_baseline,
    bool is_flipped = false) {
  const auto container_writing_mode =
      container_writing_direction.GetWritingMode();

  auto start_group = BaselineGroup::kMajor;
  auto end_group = BaselineGroup::kMinor;
  if (is_last_baseline)
    std::swap(start_group, end_group);
  if (is_flipped)
    std::swap(start_group, end_group);

  if (is_parallel_context) {
    DCHECK(
        IsParallelWritingMode(container_writing_mode, baseline_writing_mode));
    return (baseline_writing_mode == container_writing_mode) ? start_group
                                                             : end_group;
  }

  // For each writing-mode the "major" group is aligned with the container's
  // direction. This is to ensure the inline-start offset (for the grid-item)
  // matches the baseline offset we calculate.
  bool is_ltr = container_writing_direction.IsLtr();
  switch (baseline_writing_mode) {
    case WritingMode::kHorizontalTb:
    case WritingMode::kVerticalLr:
    case WritingMode::kSidewaysLr:
      return is_ltr ? start_group : end_group;
    case WritingMode::kVerticalRl:
    case WritingMode::kSidewaysRl:
      return is_ltr ? end_group : start_group;
  }

  NOTREACHED();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_BASELINE_UTILS_H_
