// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_CONTENT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_CONTENT_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutEmbeddedContent;
struct PaintInfo;

class EmbeddedContentPainter {
  STACK_ALLOCATED();

 public:
  EmbeddedContentPainter(const LayoutEmbeddedContent& layout_embedded_content)
      : layout_embedded_content_(layout_embedded_content) {}

  void PaintReplaced(const PaintInfo&, const PhysicalOffset& paint_offset);

 private:
  const LayoutEmbeddedContent& layout_embedded_content_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_CONTENT_PAINTER_H_
