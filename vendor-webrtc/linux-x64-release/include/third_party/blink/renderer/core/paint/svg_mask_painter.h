// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MASK_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MASK_PAINTER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkBlendMode.h"

namespace gfx {
class RectF;
}  // namespace gfx

namespace blink {

class DisplayItemClient;
class GraphicsContext;
class ImageResourceObserver;
class LayoutObject;
class StyleMaskSourceImage;

class SVGMaskPainter {
  STATIC_ONLY(SVGMaskPainter);

 public:
  static void Paint(GraphicsContext& context,
                    const LayoutObject& layout_object,
                    const DisplayItemClient& display_item_client);
  static void PaintSVGMaskLayer(GraphicsContext&,
                                const StyleMaskSourceImage&,
                                const ImageResourceObserver&,
                                const gfx::RectF& reference_box,
                                const float zoom,
                                const SkBlendMode composite_op,
                                const bool apply_mask_type);
  static gfx::RectF ResourceBoundsForSVGChild(
      const LayoutObject& layout_object);
  static bool MaskIsValid(const StyleMaskSourceImage&,
                          const ImageResourceObserver&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_MASK_PAINTER_H_
