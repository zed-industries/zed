// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CLIP_PATH_CLIPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CLIP_PATH_CLIPPER_H_

#include <optional>

#include "third_party/blink/renderer/core/animation/element_animations.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/geometry/contoured_rect.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class DisplayItemClient;
class GraphicsContext;
class HitTestLocation;
class LayoutObject;

class CORE_EXPORT ClipPathClipper {
  STATIC_ONLY(ClipPathClipper);

 public:
  // Value used for HasCompositeClipPathAnimation to determine what, if any
  // update is required.
  enum class CompositedStateResolutionType {
    // This is used to resolve clip path status when the paint properties have
    // not been initialized. This is not typically used, but can be used in the
    // case that there are no other reasons to initialize the paint properties.
    // In this case, the status may not be fully resolved, and may instead
    // remain at kNeedsRepaint.
    kInitialResolve,

    // This is used to resolve the composited clip path status. When calling
    // with this option, the status will be set to a definitive value or
    // error.
    kFullResolve,

    // This is used to simply read the current value of the clip path status.
    // Like kFullResolve, it is guaranteed to return a definitive value or
    // fail, however this mode assumes the status has already been calculated,
    // ie, that ClipPathStatusResolved == true. If the status is kNeedsRepaint,
    // there will be an error.
    kReadCache
  };

  // Returns true if the given layout object a resolved clip path status
  static bool ClipPathStatusResolved(const LayoutObject& layout_object);

  // Gets the Animation object for an element with a compositable clip-path
  // animation. Returns nullptr if the animation is not compositable.
  static Animation* GetClipPathAnimation(const LayoutObject& layout_object);

  static ContouredRect RoundedReferenceBox(GeometryBox geometry_box,
                                           const LayoutObject& object);

  // Checks the composited paint status for a given Layout Object and checks
  // whether it contains a composited clip path animation. Assumes
  // ResolveClipPathStatus has been called, will fail otherwise.
  static bool HasCompositeClipPathAnimation(
      const LayoutObject& layout_object,
      CompositedStateResolutionType state);

  // Sets a potential composited clip path animation to be not composited.
  // Called during pre-paint, currently in the case of fragmented layouts.
  static void FallbackClipPathAnimationIfNecessary(
      const LayoutObject& layout_object,
      bool is_in_block_fragmentation);

  static void PaintClipPathAsMaskImage(GraphicsContext&,
                                       const LayoutObject&,
                                       const DisplayItemClient&);

  // Returns the reference box used by CSS clip-path.
  static gfx::RectF LocalReferenceBox(const LayoutObject&);

  // Returns the bounding box of the computed clip path, which could be
  // smaller or bigger than the reference box. Returns nullopt if the
  // clip path is invalid.
  static std::optional<gfx::RectF> LocalClipPathBoundingBox(
      const LayoutObject&);

  // The argument |clip_path_owner| is the layout object that owns the
  // ClipPathOperation we are currently processing. Usually it is the
  // same as the layout object getting clipped, but in the case of nested
  // clip-path, it could be one of the SVG clip path in the chain.
  // Returns the path if the clip-path can use path-based clip.
  static std::optional<Path> PathBasedClip(const LayoutObject& clip_path_owner,
                                           const gfx::Vector2dF& clip_offset);

  // Returns true if `location` intersects the `clip_path_owner`'s clip-path.
  // `reference_box`, which should be calculated from `reference_box_object`, is
  // used to resolve 'objectBoundingBox' units/percentages.
  static bool HitTest(const LayoutObject& clip_path_owner,
                      const gfx::RectF& reference_box,
                      const LayoutObject& reference_box_object,
                      const HitTestLocation& location);

  // Like the above, but derives the reference box from the LayoutObject using
  // `LocalReferenceBox()`.
  static bool HitTest(const LayoutObject&, const HitTestLocation& location);

 private:
  static std::optional<Path> PathBasedClipInternal(
      const LayoutObject& clip_path_owner,
      const gfx::RectF& reference_box,
      const LayoutObject& reference_box_object,
      const gfx::Vector2dF& clip_offset);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_CLIP_PATH_CLIPPER_H_
