// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_H_

#include <algorithm>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

// A struct that holds a pair of two sizes, a "min" size and a "max" size.
// Useful for holding a {min,max}-content size pair or a
// {min,max}-{width,height}.
struct CORE_EXPORT MinMaxSizes {
  LayoutUnit min_size;
  LayoutUnit max_size;

  bool IsEmpty() const { return !min_size && max_size == LayoutUnit::Max(); }

  // Make sure that our min/max sizes are at least as large as |other|.
  void Encompass(const MinMaxSizes& other) {
    min_size = std::max(min_size, other.min_size);
    max_size = std::max(max_size, other.max_size);
  }

  // Make sure that our min/max sizes are at least as large as |value|.
  void Encompass(LayoutUnit value) {
    min_size = std::max(min_size, value);
    max_size = std::max(max_size, value);
  }

  // Make sure that our min/max sizes aren't larger than |value|.
  void Constrain(LayoutUnit value) {
    min_size = std::min(min_size, value);
    max_size = std::min(max_size, value);
  }

  // Interprets the sizes as a min-content/max-content pair and computes the
  // "shrink-to-fit" size based on them for the given available size.
  LayoutUnit ShrinkToFit(LayoutUnit available_size) const {
    DCHECK_GE(max_size, min_size);
    return std::min(max_size, std::max(min_size, available_size));
  }

  // Interprets the sizes as a {min-max}-size pair and clamps the given input
  // size to that.
  LayoutUnit ClampSizeToMinAndMax(LayoutUnit size) const {
    return std::max(min_size, std::min(size, max_size));
  }

  bool operator==(const MinMaxSizes& other) const {
    return min_size == other.min_size && max_size == other.max_size;
  }
  bool operator!=(const MinMaxSizes& other) const { return !operator==(other); }

  void operator=(LayoutUnit value) { min_size = max_size = value; }
  MinMaxSizes& operator+=(MinMaxSizes extra) {
    min_size += extra.min_size;
    max_size += extra.max_size;
    return *this;
  }
  MinMaxSizes& operator+=(const LayoutUnit length) {
    min_size += length;
    max_size += length;
    return *this;
  }
  MinMaxSizes& operator-=(const LayoutUnit length) {
    min_size -= length;
    max_size -= length;
    return *this;
  }
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const MinMaxSizes&);

// The output of the min/max inline size calculation algorithm. Contains the
// min/max sizes, and if this calculation will change if the block constraints
// change.
struct MinMaxSizesResult {
  MinMaxSizesResult() = default;
  MinMaxSizesResult(MinMaxSizes sizes, bool depends_on_block_constraints)
      : sizes(sizes),
        depends_on_block_constraints(depends_on_block_constraints) {}

  // This constructor is only used within `BlockNode::ComputeMinMaxSizes` when
  // the aspect-ratio has been applied.
  //
  // The `applied_aspect_ratio` flag is not propagated up the tree, unlike
  // `depends_on_block_constraints`.
  MinMaxSizesResult(MinMaxSizes sizes,
                    bool depends_on_block_constraints,
                    bool applied_aspect_ratio)
      : sizes(sizes),
        depends_on_block_constraints(depends_on_block_constraints),
        applied_aspect_ratio(applied_aspect_ratio) {}

  MinMaxSizes sizes;
  bool depends_on_block_constraints = false;
  bool applied_aspect_ratio = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_H_
