// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_ELEMENT_TIMING_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_ELEMENT_TIMING_UTILS_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
class RectF;
}  // namespace gfx

namespace blink {

class LocalFrame;
class PropertyTreeStateOrAlias;

// Class containing methods shared between ImageElementTiming and
// TextElementTiming.
class ElementTimingUtils {
  STATIC_ONLY(ElementTimingUtils);

 public:
  // Computes the part a rect in a local transform space that is visible in the
  // specified frame, and returns a result in DIPs.
  static gfx::RectF ComputeIntersectionRect(LocalFrame*,
                                            const gfx::Rect&,
                                            const PropertyTreeStateOrAlias&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_ELEMENT_TIMING_UTILS_H_
