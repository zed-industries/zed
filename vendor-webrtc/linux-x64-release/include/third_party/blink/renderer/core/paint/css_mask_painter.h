// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CSS_MASK_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CSS_MASK_PAINTER_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class LayoutObject;

class CORE_EXPORT CSSMaskPainter {
  STATIC_ONLY(CSSMaskPainter);

 public:
  // Returns the bounding box of the computed mask, which could be
  // smaller or bigger than the reference box. Returns nullopt if the
  // there is no mask or the mask is invalid.
  static std::optional<gfx::RectF> MaskBoundingBox(
      const LayoutObject&,
      const PhysicalOffset& paint_offset);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CSS_MASK_PAINTER_H_
