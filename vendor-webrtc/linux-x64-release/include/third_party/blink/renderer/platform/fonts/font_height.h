// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_HEIGHT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_HEIGHT_H_

#include "third_party/blink/renderer/platform/fonts/font_baseline.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

// Represents line-progression metrics for line boxes and inline boxes.
// Computed for inline boxes, then the metrics of inline boxes are united to
// compute metrics for line boxes.
// https://drafts.csswg.org/css2/visudet.html#line-height
struct PLATFORM_EXPORT FontHeight {
  FontHeight() = default;
  FontHeight(LayoutUnit ascent, LayoutUnit descent)
      : ascent(ascent), descent(descent) {}

  // "Empty" is for when zero is a valid non-empty value.
  static FontHeight Empty() {
    return FontHeight(LayoutUnit::Min(), LayoutUnit::Min());
  }
  bool IsEmpty() const {
    return ascent == LayoutUnit::Min() && descent == LayoutUnit::Min();
  }

  LayoutUnit LineHeight() const { return ascent + descent; }

  bool operator==(const FontHeight& other) const {
    return ascent == other.ascent && descent == other.descent;
  }
  bool operator!=(const FontHeight& other) const { return !operator==(other); }

  // True if `other`'s ascent and descent are both equal to or less than `this`.
  bool Contains(const FontHeight& other) const {
    return other.ascent <= ascent && other.descent <= descent;
  }

  // Add the leading space. `leading_space.ascent` will be added into ascent and
  // `leading_space.descent` will be added to descent.
  // https://drafts.csswg.org/css2/visudet.html#leading
  void AddLeading(const FontHeight& leading_space);

  // Move the metrics by the specified amount, in line progression direction.
  void Move(LayoutUnit);

  // Unite a metrics for an inline box to a metrics for a line box.
  void Unite(const FontHeight&);

  void operator+=(const FontHeight&);

  // Ascent and descent of glyphs, or synthesized for replaced elements.
  // Then united to compute 'text-top' and 'text-bottom' of line boxes.
  LayoutUnit ascent;
  LayoutUnit descent;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const FontHeight&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_HEIGHT_H_
