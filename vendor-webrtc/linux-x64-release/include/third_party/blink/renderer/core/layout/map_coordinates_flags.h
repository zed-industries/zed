// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MAP_COORDINATES_FLAGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MAP_COORDINATES_FLAGS_H_

namespace blink {

enum MapCoordinatesMode {
  // Only needed in some special cases to intentionally ignore transforms.
  kIgnoreTransforms = 1 << 2,

  kTraverseDocumentBoundaries = 1 << 3,

  // Ignore offset adjustments caused by position:sticky calculations when
  // walking the chain.
  kIgnoreStickyOffset = 1 << 4,

  // Ignore scroll offset from container, i.e. scrolling has no effect on mapped
  // position.
  kIgnoreScrollOffset = 1 << 5,

  // Ignore scroll origin *and* scroll offset (which can be non-zero for some
  // scrollers, like RTL and flippd-blocks writing modes).
  kIgnoreScrollOriginAndOffset = 1 << 6,

  // If the local root frame has a remote frame parent, apply the transformation
  // from the local root frame to the remote main frame. The coordinates are
  // relative to the remote main frame document, i.e., (0, 0) maps to where the
  // remote main frame's content starts.
  kApplyRemoteMainFrameTransform = 1 << 7,

  // If the local root frame has a remote frame parent, apply the transformation
  // from the local root frame to the viewport, i.e., (0, 0) maps to the origin
  // of the window rendering the remote main Document.
  //
  // NOTE: This is guaranteed to provide a correct value only if the iframe is
  // onscreen. This is because we don't sync scroll updates from the main
  // frame's root scroller. See kSkipUnnecessaryRemoteFrameGeometryPropagation.
  kApplyRemoteViewportTransform = 1 << 8,
};
typedef unsigned MapCoordinatesFlags;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MAP_COORDINATES_FLAGS_H_
