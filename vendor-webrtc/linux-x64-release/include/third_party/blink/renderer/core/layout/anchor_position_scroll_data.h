// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_SCROLL_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_SCROLL_DATA_H_

#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/scroll/scroll_snapshot_client.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/vector2d.h"

namespace blink {

class AnchorPositionVisibilityObserver;
class Element;
class LayoutObject;

// Created for each anchor-positioned element.
//
// https://drafts.csswg.org/css-anchor-position-1/#scroll
//
// To adjust the location of the anchor-positioned element based on scroll,
// sticky and anchor-positioning offsets between this element and the anchor,
// this class stores a snapshot of all the scroll adjustment containers [1] of
// the anchor up to the containing block (exclusively) of the anchor-positioned
// element, along the containing block hierarchy. Note that "containing block"
// is in the spec meaning, which corresponds to LayoutObject::Container()
// instead of ContainingBlock().
//
// [1] An element is a scroll adjustment container if it is a scroll container,
// has sticky position, or is anchor-positioned.
//
// https://drafts.csswg.org/css-anchor-position-1/#fallback
//
// Also stores a snapshot of the scroll offset of the scroll container of the
// anchor-positioned element, which affects position fallback.
//
// The snapshot passed as input to the position fallback and position visibility
// algorithm.
//
// The snapshot is updated once per frame update on top of animation frame to
// avoid layout cycling. If there is any change, we trigger an update to
// layout and/or paint.
class AnchorPositionScrollData
    : public GarbageCollected<AnchorPositionScrollData>,
      public ScrollSnapshotClient,
      public ElementRareDataField {
 public:
  explicit AnchorPositionScrollData(Element* anchored_element);
  virtual ~AnchorPositionScrollData();

  Element* AnchoredElement() const { return anchored_element_.Get(); }

  bool NeedsScrollAdjustment() const {
    return !default_anchor_adjustment_data_.adjustment_container_ids.empty();
  }
  bool NeedsScrollAdjustmentInX() const {
    return default_anchor_adjustment_data_.needs_scroll_adjustment_in_x;
  }
  bool NeedsScrollAdjustmentInY() const {
    return default_anchor_adjustment_data_.needs_scroll_adjustment_in_y;
  }

  // Returns the total offset of the anchored element from the layout location
  // due to scroll and other adjustments from the containers between the given
  // `anchor_object` and the anchored element and the scroll container of the
  // anchored element itself. There are two cases:
  // 1. If `anchor_object` is nullptr or the anchor object used to create the
  //    snapshot, the result will be from the last snapshotted result.
  // 2. Otherwise the result will be calculated on the fly, which may use stale
  //    layout data if this is called during layout.
  // ValidateSnapshot() (called after the first layout during a lifecycle
  // update) will reschedule layout, or ShouldScheduleNextService() (called at
  // the end of a lifecycle update) will schedule another lifecycle update,
  // if the final layout data may cause layout changes.
  PhysicalOffset TotalOffset(const LayoutObject* anchor_object = nullptr) const;

  PhysicalOffset AccumulatedAdjustment() const {
    return default_anchor_adjustment_data_.accumulated_adjustment;
  }
  gfx::Vector2d AccumulatedAdjustmentScrollOrigin() const {
    return default_anchor_adjustment_data_.accumulated_adjustment_scroll_origin;
  }
  const Vector<CompositorElementId>& AdjustmentContainerIds() const {
    return default_anchor_adjustment_data_.adjustment_container_ids;
  }

  // Returns true if the snapshotted scroll offset is affected by viewport's
  // scroll offset.
  bool IsAffectedByViewportScrolling() const {
    return default_anchor_adjustment_data_.containers_include_viewport;
  }

  // Utility function that returns AccumulatedAdjustment() rounded as a
  // PhysicalOffset.
  // TODO(crbug.com/1309178): It's conceptually wrong to use
  // Physical/LogicalOffset, which only represents the location of a box within
  // a container, to represent a scroll offset. Stop using this function.
  PhysicalOffset TranslationAsPhysicalOffset() const {
    return -AccumulatedAdjustment();
  }

  // Returns whether `anchored_element_` is still an anchor-positioned element
  // using `this` as its AnchroScrollData.
  bool IsActive() const;

  // ScrollSnapshotClient:
  void UpdateSnapshot() override;
  bool ValidateSnapshot() override;
  bool ShouldScheduleNextService() override;
  bool IsAnchorPositionScrollData() const override { return true; }

  AnchorPositionVisibilityObserver& EnsureAnchorPositionVisibilityObserver();
  AnchorPositionVisibilityObserver* GetAnchorPositionVisibilityObserver()
      const {
    return position_visibility_observer_;
  }

  // Whether the default anchor or any ancestor of the default anchor (until
  // the container of the `anchored_element_`, not included) is also anchor
  // positioned.
  bool DefaultAnchorHasChainedAnchor() const {
    return default_anchor_adjustment_data_.has_chained_anchor;
  }

  void Trace(Visitor*) const override;

 private:
  enum class SnapshotDiff { kNone, kScrollersOrFallbackPosition, kOffsetOnly };

  struct AdjustmentData {
    DISALLOW_NEW();

    // The anchor element used when calculating this data.
    Member<const Element> anchor_element;

    // Compositor element ids of the ancestor scroll adjustment containers
    // (see the class documentation) of some element (anchor), up to the
    // containing block of `anchored_element_` (exclusively), along the
    // containing block hierarchy.
    Vector<CompositorElementId> adjustment_container_ids;

    // Sum of the adjustment offsets of the above containers. This includes
    // snapshots of
    // - scroll offsets of scroll containers,
    // - opposite of sticky offsets of stick-positioned containers,
    // - `accumulated_adjustment` of anchor-positioned containers.
    PhysicalOffset accumulated_adjustment;

    // Sum of the scroll origins of scroll containers in the above containers.
    // Used by the compositor to deal with writing modes.
    gfx::Vector2d accumulated_adjustment_scroll_origin;

    // The scroll offset of the containing block of `anchored_element_` if it's
    // a scroll container. The offset doesn't contribute to the adjustment, but
    // may affect the results of position fallback and position visibility.
    PhysicalOffset anchored_element_container_scroll_offset;

    // Whether viewport is in `container_ids`.
    bool containers_include_viewport = false;

    // These fields are for the default anchor only. The values are from the
    // fields with the same names in OutOfFlowLayoutPart::OffsetInfo. See
    // documentation there for their meanings.
    bool needs_scroll_adjustment_in_x = false;
    bool needs_scroll_adjustment_in_y = false;

    bool has_chained_anchor = false;

    void Trace(Visitor* visitor) const { visitor->Trace(anchor_element); }

    PhysicalOffset TotalOffset() const {
      return accumulated_adjustment + anchored_element_container_scroll_offset;
    }
  };

  AdjustmentData ComputeAdjustmentContainersData(
      const LayoutObject& anchor) const;
  AdjustmentData ComputeDefaultAnchorAdjustmentData() const;
  // Takes an up-to-date snapshot, and compares it with the existing one.
  // If `update` is true, also rewrites the existing snapshot.
  SnapshotDiff TakeAndCompareSnapshot(bool update);
  bool IsFallbackPositionValid(const AdjustmentData& new_adjustment_data) const;

  void InvalidateLayoutAndPaint();
  void InvalidatePaint();

  // The anchor-positioned element.
  Member<Element> anchored_element_;

  AdjustmentData default_anchor_adjustment_data_;

  Member<AnchorPositionVisibilityObserver> position_visibility_observer_;
};

template <>
struct DowncastTraits<AnchorPositionScrollData> {
  static bool AllowFrom(const ScrollSnapshotClient& client) {
    return client.IsAnchorPositionScrollData();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ANCHOR_POSITION_SCROLL_DATA_H_
