// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_REPLACED_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_REPLACED_PAINTER_H_

#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}  // namespace gfx

namespace blink {

struct PaintInfo;
struct PhysicalRect;
class DisplayItemClient;
class ScopedPaintState;
class LayoutReplaced;

class ReplacedPainter {
  STACK_ALLOCATED();

 public:
  ReplacedPainter(const LayoutReplaced& layout_replaced)
      : layout_replaced_(layout_replaced) {}

  void Paint(const PaintInfo&);

  bool ShouldPaint(const ScopedPaintState&) const;

 private:
  bool ShouldPaintBoxDecorationBackground(const PaintInfo&);
  void MeasureOverflowMetrics() const;

  void PaintBoxDecorationBackground(const PaintInfo&,
                                    const PhysicalOffset& paint_offset);

  // |visual_rect| is for the drawing display item, covering overflowing box
  // shadows and border image outsets. |paint_rect| is the border box rect in
  // paint coordinates.
  void PaintBoxDecorationBackgroundWithRect(
      const PaintInfo& paint_info,
      const gfx::Rect& visual_rect,
      const PhysicalRect& paint_rect,
      const DisplayItemClient& background_client);

  void PaintBackground(const PaintInfo&,
                       const PhysicalRect&,
                       const Color& background_color,
                       BackgroundBleedAvoidance = kBackgroundBleedNone);

  void PaintMask(const PaintInfo&, const PhysicalOffset& paint_offset);
  void PaintMaskImages(const PaintInfo&, const PhysicalRect&);

  const LayoutReplaced& layout_replaced_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_REPLACED_PAINTER_H_
