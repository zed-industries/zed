// Copyright 2024 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LINE_CLAMP_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LINE_CLAMP_DATA_H_

#include <optional>

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace blink {

struct LineClampData {
  DISALLOW_NEW();

  LineClampData() {}

  enum State {
    kDisabled,
    kClampByLines,
    kMeasureLinesUntilBfcOffset,
  };

  bool IsLineClampContext() const { return state != kDisabled; }

  std::optional<int> LinesUntilClamp(bool show_measured_lines = false) const {
    if (state == kClampByLines ||
        (show_measured_lines && state == kMeasureLinesUntilBfcOffset)) {
      return lines_until_clamp;
    }
    return std::optional<int>();
  }

  bool IsAtClampPoint() const {
    return state == kClampByLines && lines_until_clamp == 1;
  }

  bool IsPastClampPoint() const {
    return state == kClampByLines && lines_until_clamp <= 0;
  }

  bool ShouldHideForPaint() const {
    return RuntimeEnabledFeatures::CSSLineClampEnabled() && IsPastClampPoint();
  }

  bool operator==(const LineClampData& other) const {
    if (state != other.state) {
      return false;
    }
    switch (state) {
      case kClampByLines:
        return lines_until_clamp == other.lines_until_clamp;
      case kMeasureLinesUntilBfcOffset:
        return lines_until_clamp == other.lines_until_clamp &&
               clamp_bfc_offset == other.clamp_bfc_offset;
      default:
        return true;
    }
  }

  // The BFC offset where the current block container should clamp.
  // (Might not be the same BFC offset as other block containers in the same
  // BFC, depending on the bottom bmp).
  // Only valid if state == kClampByBfcOffset
  LayoutUnit clamp_bfc_offset;

  // If state == kClampByLines, the number of lines until the clamp point.
  // A value of 1 indicates the current line should be clamped. May go negative.
  // With state == kMeasureLinesUntilBfcOffset, the number of lines found in the
  // BFC so far.
  int lines_until_clamp = 0;

  State state = kDisabled;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LINE_CLAMP_DATA_H_
