// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_PAINT_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_PAINT_INVALIDATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutBox;
struct PaintInvalidatorContext;
struct PhysicalRect;

class CORE_EXPORT BoxPaintInvalidator {
  STACK_ALLOCATED();

 public:
  BoxPaintInvalidator(const LayoutBox& box,
                      const PaintInvalidatorContext& context)
      : box_(box), context_(context) {}

  static void BoxWillBeDestroyed(const LayoutBox&);

  void InvalidatePaint();

 private:
  friend class BoxPaintInvalidatorTest;

  bool HasEffectiveBackground();
  bool BackgroundGeometryDependsOnScrollableOverflowRect();
  bool BackgroundPaintsInContentsSpace();
  bool BackgroundPaintsInBorderBoxSpace();
  bool ShouldFullyInvalidateBackgroundOnScrollableOverflowChange(
      const PhysicalRect& old_scrollable_overflow,
      const PhysicalRect& new_scrollable_overflow);

  enum class BackgroundInvalidationType { kNone = 0, kIncremental, kFull };
  BackgroundInvalidationType ComputeViewBackgroundInvalidation();
  BackgroundInvalidationType ComputeBackgroundInvalidation(
      bool& should_invalidate_all_layers);
  void InvalidateBackground();

  PaintInvalidationReason ComputePaintInvalidationReason();

  bool NeedsToSavePreviousContentBoxRect();
  bool NeedsToSavePreviousOverflowData();
  void SavePreviousBoxGeometriesIfNeeded();

  const LayoutBox& box_;
  const PaintInvalidatorContext& context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_PAINT_INVALIDATOR_H_
