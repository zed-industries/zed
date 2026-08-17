// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PAINT_CHUNKS_TO_CC_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PAINT_CHUNKS_TO_CC_LAYER_H_

#include "cc/input/layer_selection_bound.h"
#include "cc/paint/display_item_list.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"

namespace cc {
class DisplayItemList;
class Layer;
}  // namespace cc

namespace blink {

class PaintChunkSubset;
class PropertyTreeState;
class RasterInvalidationTracking;

struct RasterUnderInvalidationCheckingParams {
  STACK_ALLOCATED();

 public:
  RasterUnderInvalidationCheckingParams(RasterInvalidationTracking& tracking,
                                        const gfx::Rect& interest_rect,
                                        const String& debug_name)
      : tracking(tracking),
        interest_rect(interest_rect),
        debug_name(debug_name) {}

  RasterInvalidationTracking& tracking;
  gfx::Rect interest_rect;
  String debug_name;
};

class PLATFORM_EXPORT PaintChunksToCcLayer {
  STATIC_ONLY(PaintChunksToCcLayer);

 public:
  // Converts a list of Blink paint chunks and display items into cc display
  // items, inserting appropriate begin/end items with respect to property
  // tree state. The converted items are appended into a unfinalized cc display
  // item list.
  // |layer_state| is the target property tree state of the output. This method
  // generates begin/end items for the relative state differences between the
  // layer state and the chunk state.
  // |layer_offset| is an extra translation on top of layer_state.Transform(),
  // in other word, point (x, y) in the output list maps to
  // layer_state.Transform() * (layer_offset + (x, y)) on the screen. It is
  // equivalent to say that |layer_offset| is the layer origin in the space
  // of layer_state.Transform().
  static void ConvertInto(const PaintChunkSubset&,
                          const PropertyTreeState& layer_state,
                          const gfx::Vector2dF& layer_offset,
                          RasterUnderInvalidationCheckingParams*,
                          cc::DisplayItemList&);

  // Similar to ConvertInto(), but returns a PaintRecord.
  static PaintRecord Convert(const PaintChunkSubset&,
                             const PropertyTreeState& layer_state,
                             const gfx::Rect* cull_rect = nullptr);

  static void UpdateLayerProperties(cc::Layer& layer,
                                    const PropertyTreeState& layer_state,
                                    const PaintChunkSubset&,
                                    cc::LayerSelection& layer_selection,
                                    bool selection_only);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PAINT_CHUNKS_TO_CC_LAYER_H_
