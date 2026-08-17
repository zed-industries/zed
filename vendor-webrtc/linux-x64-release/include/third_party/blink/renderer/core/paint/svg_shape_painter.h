// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_SHAPE_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_SHAPE_PAINTER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class GraphicsContext;
class LayoutSVGResourceMarker;
class LayoutSVGShape;
struct MarkerPosition;
struct PaintInfo;

class SVGShapePainter {
  STACK_ALLOCATED();

 public:
  SVGShapePainter(const LayoutSVGShape& layout_svg_shape)
      : layout_svg_shape_(layout_svg_shape) {}

  void Paint(const PaintInfo&);

 private:
  void PaintShape(const PaintInfo&);
  void FillShape(GraphicsContext&, const cc::PaintFlags&, WindRule);
  void StrokeShape(GraphicsContext&, const cc::PaintFlags&);

  void PaintMarkers(const PaintInfo&);
  void PaintMarker(const PaintInfo&,
                   LayoutSVGResourceMarker&,
                   const MarkerPosition&,
                   float stroke_width);

  const LayoutSVGShape& layout_svg_shape_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_SHAPE_PAINTER_H_
