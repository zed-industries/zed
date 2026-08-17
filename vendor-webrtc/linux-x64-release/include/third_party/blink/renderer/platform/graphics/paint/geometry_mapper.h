// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_H_

#include <optional>

#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/graphics/paint/float_clip_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "third_party/blink/renderer/platform/graphics/visual_rect_flags.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

// GeometryMapper is a helper class for fast computations of transformed and
// visual rects in different PropertyTreeStates. The design document has a
// number of details on use cases, algorithmic definitions, and running times.
//
// NOTE: A GeometryMapper object is only valid for property trees that do not
// change. If any mutation occurs, a new GeometryMapper object must be allocated
// corresponding to the new state.
//
// Design document: http://bit.ly/28P4FDA
class PLATFORM_EXPORT GeometryMapper {
  STATIC_ONLY(GeometryMapper);

 public:
  // Returns the matrix that is suitable to map geometries on the source plane
  // to some backing in the destination plane.
  // Formal definition:
  //   output = flatten(destination_to_screen)^-1 * flatten(source_to_screen)
  // There are some cases that flatten(destination_to_screen) being
  // singular yet we can still define a reasonable projection, for example:
  // 1. Both nodes inherited a common singular flat ancestor:
  // 2. Both nodes are co-planar to a common singular ancestor:
  // Not every cases outlined above are supported!
  // Read implementation comments for specific restrictions.
  static gfx::Transform SourceToDestinationProjection(
      const TransformPaintPropertyNodeOrAlias& source,
      const TransformPaintPropertyNodeOrAlias& destination) {
    return SourceToDestinationProjection(source.Unalias(),
                                         destination.Unalias());
  }
  static gfx::Transform SourceToDestinationProjection(
      const TransformPaintPropertyNode& source,
      const TransformPaintPropertyNode& destination);

  // Same as SourceToDestinationProjection() except that it maps the rect
  // rather than returning the matrix.
  // |mapping_rect| is both input and output. Its type can be gfx::RectF,
  // gfx::Rect, gfx::Rect or gfx::RectF.
  template <typename Rect>
  static void SourceToDestinationRect(
      const TransformPaintPropertyNodeOrAlias& source,
      const TransformPaintPropertyNodeOrAlias& destination,
      Rect& mapping_rect) {
    SourceToDestinationRect(source.Unalias(), destination.Unalias(),
                            mapping_rect);
  }
  template <typename Rect>
  static void SourceToDestinationRect(
      const TransformPaintPropertyNode& source,
      const TransformPaintPropertyNode& destination,
      Rect& mapping_rect) {
    mapping_rect = SourceToDestinationProjection(source, destination)
                       .MapRect(mapping_rect);
  }

  static float SourceToDestinationApproximateMinimumScale(
      const TransformPaintPropertyNode& source,
      const TransformPaintPropertyNode& destination);

  // Returns the clip rect between |local_state| and |ancestor_state|. The clip
  // rect is the total clip rect that should be applied when painting contents
  // of |local_state| in |ancestor_state| space. Because this clip rect applies
  // on contents of |local_state|, it's not affected by any effect nodes between
  // |local_state| and |ancestor_state|.
  //
  // The LayoutClipRect of any clip nodes is used, *not* the PaintClipRect.
  //
  // Note that the clip of |ancestor_state| is *not* applied.
  //
  // The output FloatClipRect may contain false positives for rounded-ness
  // if a rounded clip is clipped out, and overly conservative results
  // in the presences of transforms.

  static FloatClipRect LocalToAncestorClipRect(
      const PropertyTreeStateOrAlias& local_state,
      const PropertyTreeStateOrAlias& ancestor_state,
      OverlayScrollbarClipBehavior behavior = kIgnoreOverlayScrollbarSize) {
    return LocalToAncestorClipRect(local_state.Unalias(),
                                   ancestor_state.Unalias(), behavior);
  }
  static FloatClipRect LocalToAncestorClipRect(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize);

  // Maps from a rect in |local_state| to its visual rect in |ancestor_state|.
  // If there is no effect node between |local_state| (included) and
  // |ancestor_state| (not included), the result is computed by multiplying the
  // rect by its combined transform between |local_state| and |ancestor_space|,
  // then flattening into 2D space, then intersecting by the clip for
  // |local_state|'s clips. If there are any pixel-moving effect nodes between
  // |local_state| and |ancestor_state|, for each segment of states separated
  // by the effect nodes, we'll execute the above process and map the result
  // rect with the effect.
  //
  // Note that the clip of |ancestor_state| is *not* applied.
  //
  // DCHECK fails if any of the paint property tree nodes in |local_state| are
  // not equal to or a descendant of that in |ancestor_state|.
  //
  // |mapping_rect| is both input and output.
  //
  // The output FloatClipRect may contain false positives for rounded-ness
  // if a rounded clip is clipped out, and overly conservative results
  // in the presences of transforms.
  //
  // Returns true if the mapped rect is non-empty. (Note: this has special
  // meaning in the presence of inclusive intersection.)
  //
  // Note: if inclusive intersection is specified, then the
  // GeometryMapperClipCache is bypassed (the GeometryMapperTransformCache is
  // still used, however).
  //
  // If kInclusiveIntersect is set, clipping operations will
  // use gfx::RectF::InclusiveIntersect, and the return value of
  // InclusiveIntersect will be propagated to the return value of this method.
  // Otherwise, clipping operations will use gfx::RectF::Intersect, and the
  // return value will be true only if the clipped rect has non-zero area.
  // See the documentation for gfx::RectF::InclusiveIntersect for more
  // information.
  static bool LocalToAncestorVisualRect(
      const PropertyTreeStateOrAlias& local_state,
      const PropertyTreeStateOrAlias& ancestor_state,
      FloatClipRect& mapping_rect,
      OverlayScrollbarClipBehavior clip = kIgnoreOverlayScrollbarSize,
      VisualRectFlags flags = kDefaultVisualRectFlags) {
    return LocalToAncestorVisualRect(local_state.Unalias(),
                                     ancestor_state.Unalias(), mapping_rect,
                                     clip, flags);
  }
  static bool LocalToAncestorVisualRect(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      FloatClipRect& mapping_rect,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize,
      VisualRectFlags flags = kDefaultVisualRectFlags);

  static bool MightOverlapForCompositing(const gfx::RectF& rect1,
                                         const PropertyTreeState& state1,
                                         const gfx::RectF& rect2,
                                         const PropertyTreeState& state2);

  // Returns a clip rect that limits the visibility of painted contents under
  // the given PropertyTreeState. For now only the following simple cases
  // are considered:
  // 1. The clip rect of `state`, if the clip's local transform space is the
  //    same as that of the state.
  // 2. The scrolling contents rect, if the transform is a scroll translation.
  //
  // The clip rect can be applied to the result of LocalToAncestorVisualRect()
  // to exclude areas that are never visible in the compositor without a
  // blink-side compositing update. MightOverlapForCompositing() uses this
  // function.
  //
  // TODO(wangxianzhu): Investigate if this can be integrated into
  // LocalToAncestorVisualRect().
  static std::optional<gfx::RectF> VisibilityLimit(
      const PropertyTreeState& state);

  static void ClearCache();

 private:
  struct ExtraProjectionResult {
    bool has_animation = false;
    bool has_sticky_or_anchor_position = false;
    STACK_ALLOCATED();
  };

  static gfx::Transform SourceToDestinationProjectionInternal(
      const TransformPaintPropertyNode& source,
      const TransformPaintPropertyNode& destination,
      ExtraProjectionResult&,
      bool& success);

  enum class ForCompositingOverlap { kNo, kYes };

  template <ForCompositingOverlap>
  static FloatClipRect LocalToAncestorClipRectInternal(
      const ClipPaintPropertyNode& descendant,
      const ClipPaintPropertyNode& ancestor_clip,
      const TransformPaintPropertyNode& ancestor_transform,
      OverlayScrollbarClipBehavior,
      VisualRectFlags flags = kDefaultVisualRectFlags);

  // The return value has the same meaning as that for
  // LocalToAncestorVisualRect.
  template <ForCompositingOverlap>
  static bool LocalToAncestorVisualRectInternal(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      FloatClipRect& mapping_rect,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize,
      VisualRectFlags flags = kDefaultVisualRectFlags);

  template <ForCompositingOverlap>
  static bool SlowLocalToAncestorVisualRectWithPixelMovingFilters(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      FloatClipRect& mapping_rect,
      OverlayScrollbarClipBehavior,
      VisualRectFlags flags);

  static bool MightOverlapForCompositingInternal(
      const PropertyTreeState& common_ancestor,
      const gfx::RectF& rect1,
      const PropertyTreeState& state1,
      const gfx::RectF& rect2,
      const PropertyTreeState& state2);

  static gfx::RectF VisualRectForCompositingOverlap(
      const gfx::RectF& local_rect,
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state);

  static void MapVisualRectAboveScrollForCompositingOverlap(
      const TransformPaintPropertyNode& scroll_translation,
      gfx::RectF& rect,
      PropertyTreeState& state);

  friend class GeometryMapperTest;
  static bool LocalToAncestorVisualRectInternalForTesting(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      FloatClipRect& mapping_rect);
  static bool LocalToAncestorVisualRectInternalForCompositingOverlapForTesting(
      const PropertyTreeState& local_state,
      const PropertyTreeState& ancestor_state,
      FloatClipRect& mapping_rect);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GEOMETRY_MAPPER_H_
