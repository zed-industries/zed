// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_CONTAINER_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_CONTAINER_PAINTER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct PaintInfo;
class LayoutSVGContainer;

class SVGContainerPainter {
  STACK_ALLOCATED();

 public:
  SVGContainerPainter(const LayoutSVGContainer& layout_svg_container)
      : layout_svg_container_(layout_svg_container) {}

  void Paint(const PaintInfo&);

 private:
  bool CanUseCullRect() const;

  const LayoutSVGContainer& layout_svg_container_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_CONTAINER_PAINTER_H_
