// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_ADJUST_MASK_LAYER_GEOMETRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_ADJUST_MASK_LAYER_GEOMETRY_H_

namespace gfx {
class Size;
class Vector2dF;
}  // namespace gfx

namespace blink {

class TransformPaintPropertyNode;

// Expands the bounds of a mask layer to ensure it covers the clipped masked
// layers in case of the mask layer and the masked layers have different
// raster scales in the compositor.
void AdjustMaskLayerGeometry(const TransformPaintPropertyNode&,
                             gfx::Vector2dF& layer_offset,
                             gfx::Size& layer_bounds);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_ADJUST_MASK_LAYER_GEOMETRY_H_
