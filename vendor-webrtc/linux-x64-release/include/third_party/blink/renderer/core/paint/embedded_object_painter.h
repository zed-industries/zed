// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_OBJECT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_OBJECT_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutEmbeddedObject;
struct PaintInfo;

class EmbeddedObjectPainter {
  STACK_ALLOCATED();

 public:
  EmbeddedObjectPainter(const LayoutEmbeddedObject& layout_embedded_object)
      : layout_embedded_object_(layout_embedded_object) {}

  void PaintReplaced(const PaintInfo&, const PhysicalOffset& paint_offset);

 private:
  const LayoutEmbeddedObject& layout_embedded_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_EMBEDDED_OBJECT_PAINTER_H_
