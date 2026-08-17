// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_MODEL_OBJECT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_MODEL_OBJECT_PAINTER_H_

#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/core/paint/box_painter_base.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FillLayer;
class LayoutBoxModelObject;
struct PaintInfo;
struct PhysicalRect;

// BoxModelObjectPainter is a class that can paint either a LayoutBox or a
// LayoutInline and allows code sharing between block and inline block painting.
class BoxModelObjectPainter : public BoxPainterBase {
  STACK_ALLOCATED();

 public:
  explicit BoxModelObjectPainter(const LayoutBoxModelObject&);

 protected:
  BoxPainterBase::FillLayerInfo GetFillLayerInfo(
      const Color&,
      const FillLayer&,
      BackgroundBleedAvoidance,
      bool is_painting_background_in_contents_space,
      PaintFlags paint_flags) const override;

  void PaintTextClipMask(const PaintInfo&,
                         const gfx::Rect& mask_rect,
                         const PhysicalOffset& paint_offset,
                         bool object_has_multiple_boxes) override {}
  PhysicalRect AdjustRectForScrolledContent(GraphicsContext&,
                                            const PhysicalBoxStrut& borders,
                                            const PhysicalRect&) const override;

 private:
  const LayoutBoxModelObject& box_model_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_MODEL_OBJECT_PAINTER_H_
