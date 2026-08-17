// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_ROOT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_ROOT_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class AffineTransform;
class LayoutSVGRoot;
struct PaintInfo;

class SVGRootPainter {
  STACK_ALLOCATED();

 public:
  SVGRootPainter(const LayoutSVGRoot& layout_svg_root)
      : layout_svg_root_(layout_svg_root) {}

  void PaintReplaced(const PaintInfo&, const PhysicalOffset& paint_offset);

  // The embedded SVG document uses an unsnapped viewport box for layout, while
  // SVG root's border box ultimately gets snapped during paint. This
  // translate/scale transform is applied to compensate the difference, in
  // addition to applying the local to border box transform.
  AffineTransform TransformToPixelSnappedBorderBox(
      const PhysicalOffset& paint_offset) const;

 private:
  gfx::Rect PixelSnappedSize(const PhysicalOffset& paint_offset) const;

  const LayoutSVGRoot& layout_svg_root_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_ROOT_PAINTER_H_
