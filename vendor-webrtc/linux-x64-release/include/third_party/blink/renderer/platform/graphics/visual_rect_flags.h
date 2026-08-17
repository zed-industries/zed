// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VISUAL_RECT_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VISUAL_RECT_FLAGS_H_

namespace blink {

enum VisualRectFlags : unsigned int {
  // The following flags are used in both
  // LayoutObject::MapToVisualRectInAncestorSpace() and
  // GeometryMapper::LocalToAncestorVisualRect().

  kDefaultVisualRectFlags = 0,
  // Use gfx::RectF::InclusiveIntersect instead of gfx::RectF::Intersect for
  // intersection.
  kEdgeInclusive = 1 << 0,
  // Don't expand visual rect for pixel-moving filters.
  kIgnoreFilters = 1 << 1,

  // The following flags are used in
  // LayoutObject::MapToVisualRectInAncestorSpace() only.

  // Use the GeometryMapper fast-path, if possible.
  kUseGeometryMapper = 1 << 2,
  // When mapping to absolute coordinates and the main frame is remote, don't
  // apply the main frame root scroller's overflow clip.
  kDontApplyMainFrameOverflowClip = 1 << 3,
  kIgnoreLocalClipPath = 1 << 4,

  // If the local root frame has a remote frame parent, apply the transformation
  // from the local root frame to the viewport, i.e., (0, 0) maps to the origin
  // of the window rendering the remote main Document.
  //
  // NOTE: This is guaranteed to provide a correct value only if the iframe is
  // onscreen. This is because we don't sync scroll updates from the main
  // frame's root scroller. See kSkipUnnecessaryRemoteFrameGeometryPropagation.
  kVisualRectApplyRemoteViewportTransform = 1 << 5,

  // Use the real clip-path bounding rect, ignoring any large clip path bounding
  // rect designed to facilitate painting of composited clip path animations.
  // Used for intersection observers.
  kUsePreciseClipPath = 1 << 6
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_VISUAL_RECT_FLAGS_H_
