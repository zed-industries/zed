// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HTML_CANVAS_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HTML_CANVAS_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutHTMLCanvas;
struct PaintInfo;

class HTMLCanvasPainter {
  STACK_ALLOCATED();

 public:
  HTMLCanvasPainter(const LayoutHTMLCanvas& layout_html_canvas)
      : layout_html_canvas_(layout_html_canvas) {}
  void PaintReplaced(const PaintInfo&, const PhysicalOffset& paint_offset);

 private:
  const LayoutHTMLCanvas& layout_html_canvas_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_HTML_CANVAS_PAINTER_H_
