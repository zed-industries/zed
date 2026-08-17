// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MODEL_OBJECT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MODEL_OBJECT_PAINTER_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_model_object.h"
#include "third_party/blink/renderer/platform/graphics/paint/drawing_recorder.h"
#include "ui/gfx/geometry/rect_conversions.h"

namespace blink {

struct PaintInfo;

class SVGModelObjectPainter {
  STACK_ALLOCATED();

 public:
  // See ObjectPainter::RecordHitTestData().
  static void RecordHitTestData(const LayoutObject& svg_object,
                                const PaintInfo&);

  // Records region capture bounds for the current paint chunk. This should
  // be called when painting the background even if there is no other painted
  // content.
  static void RecordRegionCaptureData(const LayoutObject& svg_object,
                                      const PaintInfo&);

  explicit SVGModelObjectPainter(
      const LayoutSVGModelObject& layout_svg_model_object)
      : layout_svg_model_object_(layout_svg_model_object) {}

  // Should we use an infinite cull rect when painting an object with the
  // specified style.
  static bool CanUseCullRect(const ComputedStyle&);

  void PaintOutline(const PaintInfo&);

 private:
  const LayoutSVGModelObject& layout_svg_model_object_;
};

// A wrapper of DrawingRecorder for SVG children, providing the default visual
// rect (see DisplayItem::VisualRect() for definition) for the SVG contents
// not including outlines. Using a template so that
// VisualRectInLocalSVGCoordinates() can be called directly instead of through
// vtable.
class SVGDrawingRecorder : public DrawingRecorder {
 public:
  template <typename LayoutObjectType>
  SVGDrawingRecorder(GraphicsContext& context,
                     const LayoutObjectType& object,
                     DisplayItem::Type type)
      : DrawingRecorder(
            context,
            object,
            type,
            gfx::ToEnclosingRect(object.VisualRectInLocalSVGCoordinates())) {
    DCHECK(object.IsSVGChild());
    // We should not use this for SVG containers which paint effects only,
    // while VisualRectInLocalSVGCoordinates() contains visual rects from
    // children which are not painted by the container. We calculate the correct
    // visual rect when painting effects.
    DCHECK(!object.IsSVGContainer());
  }

  template <typename LayoutObjectType>
  SVGDrawingRecorder(GraphicsContext& context,
                     const LayoutObjectType& object,
                     PaintPhase phase)
      : SVGDrawingRecorder(context,
                           object,
                           DisplayItem::PaintPhaseToDrawingType(phase)) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MODEL_OBJECT_PAINTER_H_
