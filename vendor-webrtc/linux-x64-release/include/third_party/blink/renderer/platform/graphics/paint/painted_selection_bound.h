// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINTED_SELECTION_BOUND_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINTED_SELECTION_BOUND_H_

#include "third_party/blink/renderer/platform/platform_export.h"

#include "ui/gfx/geometry/point.h"
#include "ui/gfx/selection_bound.h"

namespace blink {

// Blink's notion of cc::LayerSelectionBound. Note that the points are
// gfx::Points to match the painted selection rect, which is always pixel
// aligned. There is also no layer_id as that is determined at composition time.
struct PLATFORM_EXPORT PaintedSelectionBound {
  gfx::SelectionBound::Type type;
  gfx::Point edge_start;
  gfx::Point edge_end;
  // Whether this bound is hidden (clipped out/occluded) within the painted
  // content of the layer (as opposed to being outside of the layer's bounds).
  // TODO(crbug.com/376381581): This is not used in
  // SelectionVisibilityAfterPaint. Remove this field when removing the flag.
  bool hidden;

  DISALLOW_NEW();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_PAINTED_SELECTION_BOUND_H_
