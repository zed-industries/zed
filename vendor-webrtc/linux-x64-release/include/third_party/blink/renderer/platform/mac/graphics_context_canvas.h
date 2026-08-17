// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MAC_GRAPHICS_CONTEXT_CANVAS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MAC_GRAPHICS_CONTEXT_CANVAS_H_

#include <ApplicationServices/ApplicationServices.h>

#include "base/apple/scoped_cftyperef.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkBitmap.h"

struct SkIRect;

namespace cc {
class PaintCanvas;
}

namespace blink {

// Creates a bridge for painting into a PaintCanvas with a CGContext.
class PLATFORM_EXPORT GraphicsContextCanvas {
 public:
  // Internally creates a bitmap the same size |paint_rect|, scaled by
  // |bitmap_scale_factor|.  Painting into the CgContext will go into the
  // bitmap.  Upon destruction, that bitmap will be painted into the
  // canvas as the rectangle |paint_rect|.  Users are expected to
  // clip |paint_rect| to reasonable sizes to not create giant bitmaps.
  // The |paint_rect| is in canvas device space.  The CgContext is set
  // up to be in exactly the same space as the canvas is at construction
  // time.
  GraphicsContextCanvas(cc::PaintCanvas*,
                        const SkIRect& paint_rect,
                        SkScalar bitmap_scale_factor = 1);
  ~GraphicsContextCanvas();

  CGContextRef CgContext();

 private:
  void ReleaseIfNeeded();

  raw_ptr<cc::PaintCanvas> canvas_;

  base::apple::ScopedCFTypeRef<CGContextRef> cg_context_;
  SkBitmap offscreen_;
  SkScalar bitmap_scale_factor_;

  SkIRect paint_rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MAC_GRAPHICS_CONTEXT_CANVAS_H_
