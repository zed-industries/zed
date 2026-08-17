// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_IMAGE_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_IMAGE_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class GraphicsContext;
class LayoutImage;
struct PaintInfo;
struct PhysicalRect;

class ImagePainter {
  STACK_ALLOCATED();

 public:
  ImagePainter(const LayoutImage& layout_image) : layout_image_(layout_image) {}

  void Paint(const PaintInfo&);
  void PaintReplaced(const PaintInfo&, const PhysicalOffset& paint_offset);

  // Paint the image into |destRect|, after clipping by |contentRect|. Both
  // |destRect| and |contentRect| should be in local coordinates plus the paint
  // offset.
  void PaintIntoRect(GraphicsContext&,
                     const PhysicalRect& dest_rect,
                     const PhysicalRect& content_rect);

 private:
  void PaintAreaElementFocusRing(const PaintInfo&);

  const LayoutImage& layout_image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_IMAGE_PAINTER_H_
