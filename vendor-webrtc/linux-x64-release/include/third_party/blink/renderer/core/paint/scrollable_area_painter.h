// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCROLLABLE_AREA_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCROLLABLE_AREA_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class CullRect;
class FragmentData;
class GraphicsContext;
class Scrollbar;
struct PaintInfo;
class PaintLayerScrollableArea;

class ScrollableAreaPainter {
  STACK_ALLOCATED();

 public:
  explicit ScrollableAreaPainter(
      const PaintLayerScrollableArea& paint_layer_scrollable_area)
      : scrollable_area_(paint_layer_scrollable_area) {}
  ScrollableAreaPainter(const ScrollableAreaPainter&) = delete;
  ScrollableAreaPainter& operator=(const ScrollableAreaPainter&) = delete;

  // Returns true if the overflow controls are painted.
  bool PaintOverflowControls(const PaintInfo&,
                             const PhysicalOffset& paint_offset,
                             const FragmentData*);
  void PaintResizer(GraphicsContext&,
                    const PhysicalOffset& paint_offset,
                    const CullRect&);

  // Records a scroll hit test data to force main thread handling of events
  // in the expanded resizer touch area.
  void RecordResizerScrollHitTestData(GraphicsContext&,
                                      const PhysicalOffset& paint_offset);

 private:
  void PaintScrollbar(GraphicsContext&,
                      Scrollbar&,
                      const PhysicalOffset& paint_offset,
                      const CullRect&);
  void PaintScrollCorner(GraphicsContext&,
                         const PhysicalOffset& paint_offset,
                         const CullRect&);

  void DrawPlatformResizerImage(GraphicsContext&,
                                const gfx::Rect& resizer_corner_rect);

  void PaintNativeScrollbar(GraphicsContext& context,
                            Scrollbar& scrollbar,
                            gfx::Rect visual_rect);

  const PaintLayerScrollableArea& scrollable_area_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SCROLLABLE_AREA_PAINTER_H_
