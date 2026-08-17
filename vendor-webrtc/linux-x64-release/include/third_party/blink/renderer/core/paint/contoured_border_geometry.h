// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CONTOURED_BORDER_GEOMETRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CONTOURED_BORDER_GEOMETRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/box_sides.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class ContouredRect;
struct PhysicalBoxStrut;
struct PhysicalRect;

class CORE_EXPORT ContouredBorderGeometry {
  STATIC_ONLY(ContouredBorderGeometry);

 public:
  static ContouredRect ContouredBorder(const ComputedStyle&,
                                       const PhysicalRect& border_rect);

  static ContouredRect PixelSnappedContouredBorder(
      const ComputedStyle&,
      const PhysicalRect& border_rect,
      PhysicalBoxSides edges_to_include = PhysicalBoxSides());

  static ContouredRect ContouredInnerBorder(const ComputedStyle&,
                                            const PhysicalRect& border_rect);

  static ContouredRect PixelSnappedContouredInnerBorder(
      const ComputedStyle&,
      const PhysicalRect& border_rect,
      PhysicalBoxSides edges_to_include = PhysicalBoxSides());

  // Values in |outsets| must be either all >= 0 to expand from |border_rect|,
  // or all <= 0 to shrink from |border_rect|.
  static ContouredRect PixelSnappedContouredBorderWithOutsets(
      const ComputedStyle&,
      const PhysicalRect& border_rect,
      const PhysicalBoxStrut& outsets_from_border,
      PhysicalBoxSides edges_to_include = PhysicalBoxSides());
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CONTOURED_BORDER_GEOMETRY_H_
