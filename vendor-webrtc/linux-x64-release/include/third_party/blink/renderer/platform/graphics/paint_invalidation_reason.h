// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_INVALIDATION_REASON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_INVALIDATION_REASON_H_

#include <stdint.h>

#include <iosfwd>

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Reasons of paint invalidation and raster invalidation. A paint invalidation
// reason (<= kLayoutMax) is set in renderer/core (mostly layout and paint)
// on a DisplayItemClient to indicate it will paint differently from the
// previous painted result. During raster invalidation, we use paint
// invalidation reasons as raster invalidation reasons for display items,
// and raster invalidation reasons (> kLayoutMax) for changes such as
// reordering of display item and paint chunks.
enum class PaintInvalidationReason : uint8_t {
  kNone,
  // This is used for mere size change of LayoutBox that can be invalidated for
  // the changed part instead of the whole box.
  kIncremental,
  // Hit test changes do not require raster invalidation.
  kHitTest,
  kNonFullMax = kHitTest,

  // Non-layout full paint invalidation reasons.

  kStyle,
  kOutline,
  kImage,
  kBackplate,
  kBackground,
  kSelection,
  kCaret,
  kNonLayoutMax = kCaret,

  // Full paint invalidation reasons related to layout changes.

  kLayout,
  kAppeared,
  kDisappeared,
  // Scroll bars, scroll corner, etc.
  kScrollControl,
  // The object is invalidated as a part of a subtree full invalidation (forced
  // by LayoutObject::SetSubtreeShouldDoFullPaintInvalidation()).
  kSubtree,
  kSVGResource,
  // TODO(wangxianzhu): This should probably be a non-layout reason.
  kDocumentMarker,
  kLayoutMax = kDocumentMarker,

  // The following are not used for paint invalidation, but for raster
  // invalidation only.

  // The initial PaintInvalidationReason of a DisplayItemClient.
  kJustCreated,
  kReordered,
  kChunkAppeared,
  kChunkDisappeared,
  kChunkUncacheable,
  kChunkReordered,
  kPaintProperty,
  // For tracking of direct raster invalidation of full composited layers. The
  // invalidation may be implicit, e.g. when a layer is created.
  kFullLayer,
  // This needs to be the last reason because DisplayItemClient::Invalidate()
  // requires this reason to override other reasons.
  kUncacheable,
  kMax = kUncacheable,
};

PLATFORM_EXPORT const char* PaintInvalidationReasonToString(
    PaintInvalidationReason);

inline constexpr bool IsFullPaintInvalidationReason(
    PaintInvalidationReason reason) {
  return reason > PaintInvalidationReason::kNonFullMax;
}

inline constexpr bool IsNonLayoutFullPaintInvalidationReason(
    PaintInvalidationReason reason) {
  return reason > PaintInvalidationReason::kNonFullMax &&
         reason <= PaintInvalidationReason::kNonLayoutMax;
}

inline constexpr bool IsLayoutFullPaintInvalidationReason(
    PaintInvalidationReason reason) {
  return reason > PaintInvalidationReason::kNonLayoutMax &&
         reason <= PaintInvalidationReason::kLayoutMax;
}

inline constexpr bool IsLayoutPaintInvalidationReason(
    PaintInvalidationReason reason) {
  return reason == PaintInvalidationReason::kIncremental ||
         IsLayoutFullPaintInvalidationReason(reason);
}

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&,
                                         PaintInvalidationReason);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_INVALIDATION_REASON_H_
