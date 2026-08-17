// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_FLAGS_H_

namespace blink {

// Using an anonymous enum under a namespace instead of an enum class to allow
// bitwise operations with unsigned PaintFlags.
namespace PaintFlag {
enum : unsigned {
  kNoFlag = 0,

  // Used when painting selection as part of a drag-image. This flag disables
  // a lot of the painting code and specifically triggers a
  // PaintPhase::kSelectionDragImage.
  kSelectionDragImageOnly = 1 << 0,

  // Used when painting a drag-image, printing, etc. when we won't use the
  // information for creating compositing layers.
  kOmitCompositingInfo = 1 << 1,

  // Used when printing or painting a preview to in order to add URL
  // metadata for links.
  kAddUrlMetadata = 1 << 2,

  // Used to paint a mask-based clip-path.
  kPaintingClipPathAsMask = 1 << 3,

  // Used to paint SVG resource subtree for masks, filter images, etc.
  kPaintingResourceSubtree = 1 << 4,

  // Used to paint SVG resource subtree for masks.
  kPaintingSVGMask = 1 << 5,

  // Used to paint CanvasRenderingContext2D.placeElement()
  kPlacedElement = 1 << 6,
};
}  // namespace PaintFlag

// Combination of bits under PaintFlag.
using PaintFlags = unsigned;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_FLAGS_H_
