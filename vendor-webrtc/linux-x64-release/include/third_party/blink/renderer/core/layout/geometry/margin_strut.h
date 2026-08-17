// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_MARGIN_STRUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_MARGIN_STRUT_H_

#include <algorithm>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

// This struct is used for the margin collapsing calculation.
struct CORE_EXPORT MarginStrut {
  LayoutUnit positive_margin;
  LayoutUnit negative_margin;

  // Store quirky margins separately, quirky containers need to ignore
  // quirky end margins.  Quirky margins are always default margins,
  // which are always positive.
  LayoutUnit quirky_positive_margin;

  // If this flag is set, we only Append non-quirky margins to this strut.
  // See comment inside BlockLayoutAlgorithm for when this occurs.
  bool is_quirky_container_start = false;

  // If set, we will discard all adjoining margins.
  bool discard_margins = false;

  // Appends negative or positive value to the current margin strut.
  void Append(const LayoutUnit& value, bool is_quirky);

  // Sum up negative and positive margins of this strut.
  LayoutUnit Sum() const {
    if (discard_margins)
      return LayoutUnit();
    return std::max(quirky_positive_margin, positive_margin) + negative_margin;
  }

  // Sum up non-quirky margins of this strut, used by quirky
  // containers to sum up the last margin.
  LayoutUnit QuirkyContainerSum() const {
    if (discard_margins)
      return LayoutUnit();
    return positive_margin + negative_margin;
  }

  // Whether there have been no margins appended to this margin strut.
  bool IsEmpty() const;

  bool operator==(const MarginStrut& other) const;
  bool operator!=(const MarginStrut& other) const { return !(*this == other); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_GEOMETRY_MARGIN_STRUT_H_
