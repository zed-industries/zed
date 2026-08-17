// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_RASTER_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_RASTER_INVALIDATOR_H_

#include "third_party/blink/renderer/platform/graphics/paint/display_item_list.h"
#include "third_party/blink/renderer/platform/graphics/paint/raster_invalidator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Generates raster invalidation for changed display items in a chunk.
class DisplayItemRasterInvalidator {
  STACK_ALLOCATED();

 public:
  DisplayItemRasterInvalidator(
      RasterInvalidator& invalidator,
      const DisplayItemRange& old_display_items,
      const DisplayItemRange& new_display_items,
      const ChunkToLayerMapper& mapper)
      : invalidator_(invalidator),
        old_display_items_(old_display_items),
        new_display_items_(new_display_items),
        mapper_(mapper) {}

  void Generate();

 private:
  static const auto kClientIsOld = RasterInvalidator::kClientIsOld;
  static const auto kClientIsNew = RasterInvalidator::kClientIsNew;

  ALWAYS_INLINE void AddRasterInvalidation(DisplayItemClientId,
                                           const gfx::Rect&,
                                           PaintInvalidationReason,
                                           RasterInvalidator::ClientIsOldOrNew);
  ALWAYS_INLINE DisplayItemIterator
  MatchNewDisplayItemInOldChunk(const DisplayItem& new_item,
                                DisplayItemIterator& next_old_item_to_match);
  ALWAYS_INLINE void GenerateRasterInvalidation(
      DisplayItemClientId,
      const gfx::Rect& old_visual_rect,
      const gfx::Rect& new_visual_rect,
      PaintInvalidationReason);
  ALWAYS_INLINE void GenerateIncrementalRasterInvalidation(
      DisplayItemClientId,
      const gfx::Rect& old_visual_rect,
      const gfx::Rect& new_visual_rect);
  ALWAYS_INLINE void GenerateFullRasterInvalidation(
      DisplayItemClientId,
      const gfx::Rect& old_visual_rect,
      const gfx::Rect& new_visual_rect,
      PaintInvalidationReason reason);

  RasterInvalidator& invalidator_;
  const DisplayItemRange& old_display_items_;
  const DisplayItemRange& new_display_items_;
  const ChunkToLayerMapper& mapper_;
  // Maps clients to indices of display items in old_display_items_.
  HashMap<DisplayItemClientId, Vector<DisplayItemIterator>>
      old_display_items_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_RASTER_INVALIDATOR_H_
