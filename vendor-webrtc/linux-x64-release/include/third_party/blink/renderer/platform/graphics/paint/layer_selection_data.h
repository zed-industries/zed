// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_LAYER_SELECTION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_LAYER_SELECTION_DATA_H_

#include <optional>

#include "third_party/blink/renderer/platform/graphics/paint/painted_selection_bound.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Represents paint-related metadata about the start and end of a selection.
// The information is stored on a paint chunk - note that the frame's selection
// can start in one paint chunk and end in another, so it's possible for one
// or both of the bounds to be set. The start and end can also be set
// independently by different painters within the same paint chunk.
// In the common case of no selection (or if the selection completely surrounds
// a paint chunk), neither would be set.
struct PLATFORM_EXPORT LayerSelectionData
    : public GarbageCollected<LayerSelectionData> {
  std::optional<PaintedSelectionBound> start;
  std::optional<PaintedSelectionBound> end;
  bool any_selection_was_painted = false;

  void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_LAYER_SELECTION_DATA_H_
