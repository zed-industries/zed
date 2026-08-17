// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_UTILS_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/block_node.h"

namespace blink {

class ConstraintSpace;
class LayoutResult;
struct MarginStrut;

// LayoutCacheStatus indicates what type of cache hit/miss occurred. For
// various types of misses we may be able to perform less work than a full
// layout.
//
// See |SimplifiedLayoutAlgorithm| for details about the
// |kNeedsSimplifiedLayout| cache miss type.
enum class LayoutCacheStatus {
  kHit,                    // Cache hit, no additional work required.
  kNeedsLayout,            // Cache miss, full layout required.
  kNeedsSimplifiedLayout,  // Cache miss, simplified layout required.
  kCanReuseLines           // Cache miss, may be possible to reuse lines.
};

// Calculates the |LayoutCacheStatus| based on sizing information. Returns:
//  - |LayoutCacheStatus::kHit| if the size will be the same as
//    |cached_layout_result|, and therefore might be able to skip layout.
//  - |LayoutCacheStatus::kNeedsSimplifiedLayout| if a simplified layout may
//    be possible (just based on the sizing information at this point).
//  - |LayoutCacheStatus::kNeedsLayout| if a full layout is required.
//
// May pre-compute the |fragment_geometry| while calculating this status.
LayoutCacheStatus CalculateSizeBasedLayoutCacheStatus(
    const BlockNode& node,
    const BlockBreakToken* break_token,
    const LayoutResult& cached_layout_result,
    const ConstraintSpace& new_space,
    std::optional<FragmentGeometry>* fragment_geometry);

// Returns true if for a given |new_space|, the |cached_layout_result| won't be
// affected by clearance, or floats, and therefore might be able to skip
// layout.
// Additionally (if this function returns true) it will calculate the new
// |bfc_block_offset|, |block_offset_delta|, and |end_margin_strut| for the
// layout result.
//
// |bfc_block_offset| may still be |std::nullopt| if not previously set.
//
// If this function returns false, |bfc_block_offset|, |block_offset_delta|,
// and |end_margin_strut| are in an undefined state and should not be used.
bool MaySkipLayoutWithinBlockFormattingContext(
    const LayoutResult& cached_layout_result,
    const ConstraintSpace& new_space,
    std::optional<LayoutUnit>* bfc_block_offset,
    LayoutUnit* block_offset_delta,
    MarginStrut* end_margin_strut);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_UTILS_H_
