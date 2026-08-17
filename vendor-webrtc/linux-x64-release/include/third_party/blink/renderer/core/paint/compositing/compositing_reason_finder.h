// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_COMPOSITING_COMPOSITING_REASON_FINDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_COMPOSITING_COMPOSITING_REASON_FINDER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/compositing_reasons.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutObject;
class ComputedStyle;

class CORE_EXPORT CompositingReasonFinder {
  STATIC_ONLY(CompositingReasonFinder);

 public:
  static CompositingReasons DirectReasonsForPaintProperties(
      const LayoutObject&,
      const LayoutObject* container_for_fixed_position = nullptr);

  static bool ShouldForcePreferCompositingToLCDText(
      const LayoutObject&,
      CompositingReasons reasons_except_scrolling);

  static CompositingReasons CompositingReasonsForAnimation(const LayoutObject&);
  // Some LayoutObject types do not support transforms (see:
  // |LayoutObject::HasTransformRelatedProperty|) so this can return reasons
  // that the LayoutObject does not end up using.
  static CompositingReasons PotentialCompositingReasonsFor3DTransform(
      const ComputedStyle&);
  static bool RequiresCompositingForRootScroller(const LayoutObject&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_COMPOSITING_COMPOSITING_REASON_FINDER_H_
