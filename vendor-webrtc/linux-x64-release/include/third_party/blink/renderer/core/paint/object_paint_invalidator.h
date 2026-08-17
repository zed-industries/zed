// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_INVALIDATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class LayoutObject;
struct PaintInvalidatorContext;

class CORE_EXPORT ObjectPaintInvalidator {
  STACK_ALLOCATED();

 public:
  ObjectPaintInvalidator(const LayoutObject& object) : object_(object) {}

  // This calls LayoutObject::PaintingLayer() which walks up the tree.
  // If possible, use the faster
  // PaintInvalidatorContext.painting_layer.SetNeedsRepaint() instead.
  void SlowSetPaintingLayerNeedsRepaint();

  void SlowSetPaintingLayerNeedsRepaintAndInvalidateDisplayItemClient(
      const DisplayItemClient& client,
      PaintInvalidationReason reason) {
    SlowSetPaintingLayerNeedsRepaint();
    InvalidateDisplayItemClient(client, reason);
  }

  // The caller should ensure the painting layer has been SetNeedsRepaint
  // before calling this function.
  void InvalidateDisplayItemClient(const DisplayItemClient&,
                                   PaintInvalidationReason);

 protected:
#if DCHECK_IS_ON()
  void CheckPaintLayerNeedsRepaint();
#endif

  const LayoutObject& object_;
};

class ObjectPaintInvalidatorWithContext : public ObjectPaintInvalidator {
 public:
  ObjectPaintInvalidatorWithContext(const LayoutObject& object,
                                    const PaintInvalidatorContext& context)
      : ObjectPaintInvalidator(object), context_(context) {}

  void InvalidatePaint() {
    InvalidatePaintWithComputedReason(ComputePaintInvalidationReason());
  }

  PaintInvalidationReason ComputePaintInvalidationReason();
  void InvalidatePaintWithComputedReason(PaintInvalidationReason);

 private:
  const PaintInvalidatorContext& context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINT_INVALIDATOR_H_
