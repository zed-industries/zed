// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_GEOMETRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_GEOMETRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/transform.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class Element;
class LayoutBox;
class LayoutObject;
class Node;

// Computes the intersection between an ancestor (root) node and a
// descendant (target) element, with overflow and CSS clipping applied.
// Optionally also checks whether the target is occluded or has visual
// effects applied.
//
// If the root argument to the constructor is null, computes the intersection
// of the target with the top-level frame viewport (AKA the "implicit root").
class CORE_EXPORT IntersectionGeometry {
 public:
  // See comment of IntersectionObserver::kMinimumThreshold.
  static constexpr float kMinimumThreshold = std::numeric_limits<float>::min();

  static constexpr gfx::Vector2dF kInfiniteScrollDelta{
      std::numeric_limits<float>::max(), std::numeric_limits<float>::max()};

  enum Flags {
    // These flags should passed to the constructor
    kShouldReportRootBounds = 1 << 0,
    kShouldComputeVisibility = 1 << 1,
    kShouldTrackFractionOfRoot = 1 << 2,
    kForFrameViewportIntersection = 1 << 3,
    kShouldConvertToCSSPixels = 1 << 4,
    // Applies to boxes. If true, OverflowClipRect() is used if necessary
    // instead of BorderBoundingBox().
    kUseOverflowClipEdge = 1 << 5,
    kRespectFilters = 1 << 6,
    kScrollAndVisibilityOnly = 1 << 7,

    // These flags will be computed
    kShouldUseCachedRects = 1 << 8,
    kRootIsImplicit = 1 << 9,
    kDidComputeGeometry = 1 << 10,
    kIsVisible = 1 << 11,

    // These flags to expose extra information of occluded state
    kShouldExposeOccluderNodeId = 1 << 12,
  };

  struct RootGeometry {
    STACK_ALLOCATED();

   public:
    RootGeometry(const LayoutObject* root, const Vector<Length>& margin);
    bool operator==(const RootGeometry&) const;

    float zoom = 1.0f;
    // The root object's content rect in the root object's own coordinate system
    gfx::RectF pre_margin_local_root_rect;
    gfx::RectF local_root_rect;
    gfx::Transform root_to_view_transform;

    void UpdateMargin(const Vector<Length>& margin);
  };

  struct CachedRects {
    // Target's bounding rect in the target's coordinate space
    gfx::RectF local_target_rect;
    // Target rect mapped up to the root's space, with intermediate clips
    // applied, but without applying the root's clip or scroll offset.
    gfx::RectF unscrolled_unclipped_intersection_rect;
    // This is calculated basically based on the distance between the root rect
    // and the target rect, when it's applicable. On each scroll, we subtract
    // the absolute scroll delta from it, and only need to update intersection
    // geometry if it becomes <= 0 along either axis.
    gfx::Vector2dF min_scroll_delta_to_update;
    // True iff unscrolled_unclipped_intersection_rect actually intersects the
    // root, as defined by edge-inclusive intersection rules.
    bool does_intersect = false;
    // True iff the target rect before any margins were applied was empty
    bool pre_margin_target_rect_is_empty = false;
    // Invalidation flag
    bool valid = false;
  };

  static const LayoutObject* GetTargetLayoutObject(
      const Element& target_element);
  static const LayoutObject* GetExplicitRootLayoutObject(const Node& root_node);
  static bool CanUseGeometryMapper(const LayoutObject&);

  // If `root_geometry` is nullopt, it will be emplaced with `root` and
  // `root_margin`. The caller can call this constructor again with the same
  // `root_geometry` as long as `root` and `root_margin` are the same as the
  // first call.
  IntersectionGeometry(const Node* root,
                       const Element& target,
                       const Vector<Length>& root_margin,
                       const Vector<float>& thresholds,
                       const Vector<Length>& target_margin,
                       const Vector<Length>& scroll_margin,
                       unsigned flags,
                       std::optional<RootGeometry>& root_geometry,
                       CachedRects* cached_rects = nullptr);

  IntersectionGeometry(const IntersectionGeometry&) = default;

  bool ShouldReportRootBounds() const {
    return flags_ & kShouldReportRootBounds;
  }
  bool ShouldComputeVisibility() const {
    return flags_ & kShouldComputeVisibility;
  }
  bool ShouldTrackFractionOfRoot() const {
    return flags_ & kShouldTrackFractionOfRoot;
  }

  gfx::RectF TargetRect() const { return target_rect_; }
  gfx::RectF IntersectionRect() const { return intersection_rect_; }

  // The intersection rect without applying viewport clipping.
  gfx::RectF UnclippedIntersectionRect() const {
    return unclipped_intersection_rect_;
  }

  gfx::RectF RootRect() const { return root_rect_; }

  double IntersectionRatio() const { return intersection_ratio_; }
  wtf_size_t ThresholdIndex() const { return threshold_index_; }

  bool DidComputeGeometry() const { return flags_ & kDidComputeGeometry; }
  bool IsIntersecting() const { return threshold_index_ > 0; }
  bool IsVisible() const { return flags_ & kIsVisible; }
  DOMNodeId occluder_node_id() const { return occluder_node_id_; }

  bool CanUseCachedRectsForTesting() const { return ShouldUseCachedRects(); }

 private:
  bool RootIsImplicit() const { return flags_ & kRootIsImplicit; }
  bool ShouldUseCachedRects() const { return flags_ & kShouldUseCachedRects; }
  bool ShouldRespectFilters() const { return flags_ & kRespectFilters; }
  bool IsForFrameViewportIntersection() const {
    return flags_ & kForFrameViewportIntersection;
  }

  struct RootAndTarget {
    STACK_ALLOCATED();

   public:
    RootAndTarget(const Node* root_node,
                  const Element& target_element,
                  bool has_target_margin,
                  bool has_scroll_margin);
    const LayoutObject* target;
    const LayoutObject* root;
    enum Relationship {
      kInvalid,
      // The target is in a sub-frame of the implicit root.
      kTargetInSubFrame,
      // There are intermediate clippers (scroll containers or not) between the
      // root and the target. The target is likely to be scrollable in root.
      kHasIntermediateClippers,
      // The target can't be scrolled in the root by any scroller, without any
      // intermediate clippers.
      kNotScrollable,
      // The target can be scrolled in the root by the root only, without any
      // intermediate clippers.
      kScrollableByRootOnly,
    };
    Relationship relationship = kInvalid;
    // Whether `root` scrolls `target` directly or indirectly. This is false if
    // - `root` is not a scroll container,
    // - `root` doesn't have any scrollable overflow, or
    // - `root` is the LayoutView and `target` is contained by a fixed-position
    //   element that is fixed to the viewport.
    bool root_scrolls_target = false;
    // This is used only when relationship is kHasIntermediateClippers or
    // kScrollableByRootOnly.
    bool has_filter = false;
    // This is collected only if has_scroll_margin is true.
    HeapVector<Member<const LayoutBox>, 2> intermediate_scrollers;

   private:
    const LayoutObject* GetRootLayoutObject(const Node* root_node) const;
    void ComputeRelationship(bool root_is_implicit,
                             bool has_target_margin,
                             bool has_scroll_margin);
  };

  void UpdateShouldUseCachedRects(const RootAndTarget& root_and_target,
                                  CachedRects* cached_rects);

  void ComputeGeometry(const RootGeometry& root_geometry,
                       const RootAndTarget& root_and_target,
                       const Vector<float>& thresholds,
                       const Vector<Length>& target_margin,
                       const Vector<Length>& scroll_margin,
                       CachedRects* cached_rects);

  // Map intersection_rect from the coordinate system of the target to the
  // coordinate system of the root, applying intervening clips.
  bool ClipToRoot(const RootAndTarget& root_and_target,
                  const gfx::RectF& root_rect,
                  gfx::RectF& unclipped_intersection_rect,
                  gfx::RectF& intersection_rect,
                  const Vector<Length>& scroll_margin,
                  CachedRects* cached_rects);
  bool ApplyClip(const LayoutObject* target,
                 const LayoutBox* local_ancestor,
                 const LayoutObject* root,
                 const gfx::RectF& root_rect,
                 gfx::RectF& unclipped_intersection_rect,
                 gfx::RectF& intersection_rect,
                 const Vector<Length>& scroll_margin,
                 bool ignore_local_clip_path,
                 bool root_scrolls_target,
                 CachedRects* cached_rects);

  unsigned FirstThresholdGreaterThan(float ratio,
                                     const Vector<float>& thresholds) const;

  gfx::Vector2dF ComputeMinScrollDeltaToUpdate(
      const RootAndTarget& root_and_target,
      const gfx::Transform& target_to_view_transform,
      const gfx::Transform& root_to_view_transform,
      const Vector<float>& thresholds,
      const Vector<Length>& scroll_margin) const;

  gfx::RectF target_rect_;
  gfx::RectF intersection_rect_;
  gfx::RectF unclipped_intersection_rect_;
  gfx::RectF root_rect_;
  unsigned flags_;
  double intersection_ratio_ = 0;
  wtf_size_t threshold_index_ = 0;
  DOMNodeId occluder_node_id_ = kInvalidDOMNodeId;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INTERSECTION_OBSERVER_INTERSECTION_GEOMETRY_H_
