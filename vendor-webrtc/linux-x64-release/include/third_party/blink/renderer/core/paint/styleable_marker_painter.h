// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_STYLEABLE_MARKER_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_STYLEABLE_MARKER_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class GraphicsContext;
class StyleableMarker;
struct LineRelativeRect;

// Painter for StyleableMarkers.
// This paints text decorations (underlines) for composition (input method) and
// suggestion markers.
class StyleableMarkerPainter {
  STATIC_ONLY(StyleableMarkerPainter);

 public:
  static void PaintUnderline(const StyleableMarker& marker,
                             GraphicsContext& context,
                             const PhysicalOffset& box_origin,
                             const ComputedStyle& style,
                             const LineRelativeRect& marker_rect,
                             LayoutUnit logical_height,
                             bool in_dark_mode);
  static bool ShouldPaintUnderline(const StyleableMarker& marker);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_STYLEABLE_MARKER_PAINTER_H_
