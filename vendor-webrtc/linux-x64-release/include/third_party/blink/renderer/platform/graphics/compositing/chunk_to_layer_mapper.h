// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_CHUNK_TO_LAYER_MAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_CHUNK_TO_LAYER_MAPPER_H_

#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/graphics/paint/float_clip_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/geometry_mapper.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"

namespace blink {

struct PaintChunk;

// Maps geometry from PaintChunks to the containing composited layer.
// It provides higher performance than GeometryMapper by reusing computed
// transforms and clips for unchanged states within or across paint chunks.
class PLATFORM_EXPORT ChunkToLayerMapper {
  STACK_ALLOCATED();

 public:
  ChunkToLayerMapper(const PropertyTreeState& layer_state,
                     const gfx::Vector2dF& layer_offset);

  const PropertyTreeState& LayerState() const { return layer_state_; }
  gfx::Vector2dF LayerOffset() const { return layer_offset_; }

  // This class can map from multiple chunks. Before mapping from a chunk, this
  // method must be called to prepare for the chunk.
  void SwitchToChunk(const PaintChunk&);
  void SwitchToChunkWithState(const PaintChunk&, const PropertyTreeState&);

  const PropertyTreeState& ChunkState() const { return chunk_state_; }

  // Maps a visual rectangle in the current chunk space into the layer space.
  gfx::Rect MapVisualRect(const gfx::Rect&) const;

  // Maps a visual rectangle from the give state into the layer space.
  gfx::Rect MapVisualRectFromState(const gfx::Rect&,
                                   const PropertyTreeState&) const;

  // Returns the combined transform from the current chunk to the layer.
  const gfx::Transform& Transform() const { return transform_; }

  // Returns the combined clip from the current chunk to the layer if it can
  // be calculated (there is no filter that moves pixels), or infinite loose
  // clip rect otherwise.
  const FloatClipRect& ClipRect() const { return clip_rect_; }

 private:
  friend class ChunkToLayerMapperTest;

  gfx::Rect MapUsingGeometryMapper(const gfx::Rect&) const;
  void InflateForRasterEffectOutset(gfx::RectF&) const;

  const PropertyTreeState layer_state_;
  const gfx::Vector2dF layer_offset_;

  // The following fields are chunk-specific which are updated in
  // SwitchToChunk().
  PropertyTreeState chunk_state_;
  gfx::Transform transform_;
  FloatClipRect clip_rect_;
  RasterEffectOutset raster_effect_outset_ = RasterEffectOutset::kNone;
  // True if there is any pixel-moving filter between chunk state and layer
  // state, and we will call GeometryMapper for each mapping.
  bool has_filter_that_moves_pixels_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_CHUNK_TO_LAYER_MAPPER_H_
