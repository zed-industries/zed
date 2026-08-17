// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_PAINT_CHUNK_PROPERTIES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_PAINT_CHUNK_PROPERTIES_H_

#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_controller.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScopedPaintChunkProperties {
  STACK_ALLOCATED();

 public:
  // Use new PropertyTreeState for the scope.
  ScopedPaintChunkProperties(PaintController& paint_controller,
                             const PropertyTreeStateOrAlias& properties,
                             const DisplayItemClient& client,
                             DisplayItem::Type type)
      : paint_controller_(paint_controller),
        previous_properties_(paint_controller.CurrentPaintChunkProperties()) {
    PaintChunk::Id id(client.Id(), type);
    paint_controller_.UpdateCurrentPaintChunkProperties(id, client, properties);
  }

  // Use new transform state, and keep the current other properties.
  ScopedPaintChunkProperties(PaintController& paint_controller,
                             const TransformPaintPropertyNodeOrAlias& transform,
                             const DisplayItemClient& client,
                             DisplayItem::Type type)
      : ScopedPaintChunkProperties(
            paint_controller,
            GetPaintChunkProperties(transform, paint_controller),
            client,
            type) {}

  // Use new clip state, and keep the current other properties.
  ScopedPaintChunkProperties(PaintController& paint_controller,
                             const ClipPaintPropertyNodeOrAlias& clip,
                             const DisplayItemClient& client,
                             DisplayItem::Type type)
      : ScopedPaintChunkProperties(
            paint_controller,
            GetPaintChunkProperties(clip, paint_controller),
            client,
            type) {}

  // Use new effect state, and keep the current other properties.
  ScopedPaintChunkProperties(PaintController& paint_controller,
                             const EffectPaintPropertyNodeOrAlias& effect,
                             const DisplayItemClient& client,
                             DisplayItem::Type type)
      : ScopedPaintChunkProperties(
            paint_controller,
            GetPaintChunkProperties(effect, paint_controller),
            client,
            type) {}

  ScopedPaintChunkProperties(const ScopedPaintChunkProperties&) = delete;
  ScopedPaintChunkProperties& operator=(const ScopedPaintChunkProperties&) =
      delete;

  ~ScopedPaintChunkProperties() {
    // We should not return to the previous id, because that may cause a new
    // chunk to use the same id as that of the previous chunk before this
    // ScopedPaintChunkProperties. The painter should create another scope of
    // paint properties with new id, or the new chunk will use the id of the
    // first display item as its id.
    paint_controller_.UpdateCurrentPaintChunkProperties(previous_properties_);
  }

 private:
  static PropertyTreeStateOrAlias GetPaintChunkProperties(
      const TransformPaintPropertyNodeOrAlias& transform,
      PaintController& paint_controller) {
    PropertyTreeStateOrAlias properties(
        paint_controller.CurrentPaintChunkProperties());
    properties.SetTransform(transform);
    return properties;
  }

  static PropertyTreeStateOrAlias GetPaintChunkProperties(
      const ClipPaintPropertyNodeOrAlias& clip,
      PaintController& paint_controller) {
    PropertyTreeStateOrAlias properties(
        paint_controller.CurrentPaintChunkProperties());
    properties.SetClip(clip);
    return properties;
  }

  static PropertyTreeStateOrAlias GetPaintChunkProperties(
      const EffectPaintPropertyNodeOrAlias& effect,
      PaintController& paint_controller) {
    PropertyTreeStateOrAlias properties(
        paint_controller.CurrentPaintChunkProperties());
    properties.SetEffect(effect);
    return properties;
  }

  PaintController& paint_controller_;
  PropertyTreeStateOrAlias previous_properties_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_SCOPED_PAINT_CHUNK_PROPERTIES_H_
