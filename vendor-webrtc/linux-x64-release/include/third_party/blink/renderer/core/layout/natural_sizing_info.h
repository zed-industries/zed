// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NATURAL_SIZING_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NATURAL_SIZING_INFO_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

struct NaturalSizingInfo {
  DISALLOW_NEW();

  static NaturalSizingInfo None() {
    return {gfx::SizeF(), gfx::SizeF(), false, false};
  }
  static NaturalSizingInfo MakeFixed(const gfx::SizeF& natural_size) {
    return {natural_size, natural_size, true, true};
  }

  bool IsNone() const {
    return !has_width && !has_height && aspect_ratio.IsEmpty();
  }

  gfx::SizeF size;
  gfx::SizeF aspect_ratio;
  bool has_width = true;
  bool has_height = true;
};

struct PhysicalNaturalSizingInfo {
  DISALLOW_NEW();

  static PhysicalNaturalSizingInfo None() { return {{}, {}, false, false}; }
  static PhysicalNaturalSizingInfo MakeFixed(const PhysicalSize& natural_size) {
    return {natural_size, natural_size, true, true};
  }
  static PhysicalNaturalSizingInfo FromSizingInfo(const NaturalSizingInfo&);

  bool operator==(const PhysicalNaturalSizingInfo&) const = default;

  PhysicalSize size;
  PhysicalSize aspect_ratio;
  bool has_width = true;
  bool has_height = true;
};

inline float ResolveWidthForRatio(float height,
                                  const gfx::SizeF& natural_ratio) {
  return height * natural_ratio.width() / natural_ratio.height();
}
inline LayoutUnit ResolveWidthForRatio(LayoutUnit height,
                                       const PhysicalSize& natural_ratio) {
  return height.MulDiv(natural_ratio.width, natural_ratio.height);
}

inline float ResolveHeightForRatio(float width,
                                   const gfx::SizeF& natural_ratio) {
  return width * natural_ratio.height() / natural_ratio.width();
}
inline LayoutUnit ResolveHeightForRatio(LayoutUnit width,
                                        const PhysicalSize& natural_ratio) {
  return width.MulDiv(natural_ratio.height, natural_ratio.width);
}

// Implements the algorithm at
// https://www.w3.org/TR/css3-images/#default-sizing with a specified size with
// no constraints and a contain constraint.
CORE_EXPORT gfx::SizeF ConcreteObjectSize(
    const NaturalSizingInfo& sizing_info,
    const gfx::SizeF& default_object_size);
CORE_EXPORT PhysicalSize
ConcreteObjectSize(const PhysicalNaturalSizingInfo& sizing_info,
                   const PhysicalSize& default_object_size);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_NATURAL_SIZING_INFO_H_
