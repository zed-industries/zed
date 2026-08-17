// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_DATA_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_scroll_axis.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/dom/scroll_marker_pseudo_element.h"
#include "third_party/blink/renderer/core/scroll/scroll_snapshot_client.h"
#include "third_party/blink/renderer/core/scroll/scrollable_area.h"

namespace blink {

const float kScrollMarkerSelectionReserveRatio = 1.f / 8.f;

// Helper class which houses the logic of selecting a scrollmarker based on a
// given ScrollOffset.
class ScrollMarkerChooser {
  STACK_ALLOCATED();

 public:
  using ScrollAxis = V8ScrollAxis::Enum;

  ScrollMarkerChooser(const ScrollOffset& scroll_offset,
                      const ScrollAxis& axis,
                      ScrollableArea* scrollable_area,
                      const HeapVector<Member<Element>>& candidates,
                      LayoutBox* scroller_box)
      : axis_(axis),
        max_position_(axis_ == ScrollAxis::kY
                          ? scrollable_area->MaximumScrollOffset().y()
                          : scrollable_area->MaximumScrollOffset().x()),
        min_position_(axis_ == ScrollAxis::kY
                          ? scrollable_area->MinimumScrollOffset().y()
                          : scrollable_area->MinimumScrollOffset().x()),
        reserved_length_((axis_ == ScrollAxis::kY
                              ? scrollable_area->VisibleHeight()
                              : scrollable_area->VisibleWidth()) *
                         kScrollMarkerSelectionReserveRatio),
        intended_position_(axis == ScrollAxis::kY ? scroll_offset.y()
                                                  : scroll_offset.x()),
        scrollable_area_(scrollable_area),
        candidates_(candidates),
        scroller_box_(scroller_box) {}

  HeapVector<Member<Element>> Choose();

 private:
  // An auxiliary struct to help with selecting scroll markers.
  // It is only created when selecting scroll markers.
  struct ScrollTargetOffsetData {
    ScrollTargetOffsetData(float scroll_offset,
                           float layout_offset,
                           float layout_size)
        : aligned_scroll_offset(scroll_offset),
          layout_offset(layout_offset),
          layout_size(layout_size) {}

    // The scroll offset at which the scroll marker, for which this object was
    // created, is considered aligned in the particular axis for which this
    // object was generated.
    float aligned_scroll_offset;
    // The position, in coordinates of the associated scroll container's content
    // area, occupied by the scroll marker generating this object in the
    // particular axis for which this object was generated.
    float layout_offset;
    // The size of the scroll marker generating this object in the particular
    // axis for which this object was generated.
    float layout_size;
  };

  // Compute a ScrollTargetOffsetData for a given element, |scroll_marker|
  // within |scrollable_area|'s content area along the |axis| specified.
  std::optional<ScrollMarkerChooser::ScrollTargetOffsetData>
  GetScrollTargetOffsetData(Element* scroll_marker);

  // Select a scroll marker from the given |candidates| if the
  // |intended_scroll_offset_| is within the region "reserved" so that
  // unreachable scroll markers can be selected.
  HeapVector<Member<Element>> ChooseReserved(
      const HeapVector<Member<Element>>& candidates);

  // Select a scroll marker from the given |candidates| in the |axis|
  // by selecting the scroll marker with the largest target position which is at
  // or before |intended_scroll_offset_| in the relevant |axis|.
  HeapVector<Member<Element>> ChooseGeneric(
      const HeapVector<Member<Element>>& candidates);

  // Select a scroll marker from the given |candidates| using their positions
  // within |scrollable_area_|'s content area (rather than their aligned scroll
  // positions). This should only be used to break ties between items at the
  // same aligned scroll positions.
  HeapVector<Member<Element>> ChooseVisual(
      const HeapVector<Member<Element>>& candidates);

  // The axis this chooser is picking a target in.
  const ScrollAxis axis_;
  // The max scroll offset of the associated scroll container along |axis_|.
  const float max_position_;
  // The min scroll offset of the associated scroll container along |axis_|.
  const float min_position_;
  // The size of the scroll port, along |axis_|, which should be treated as
  // "reserved" so that all scroll markers are selectable.
  const float reserved_length_;
  // The scroll offset, along |axis_|, at which this chooser is picking a
  // marker.
  const float intended_position_;
  // The ScrollableArea of the scroll container associated with the
  // scroll markers this chooser is picking from.
  const ScrollableArea* scrollable_area_;
  // The list of scroll markers from which this chooser should pick.
  const HeapVector<Member<Element>>& candidates_;
  // The LayoutBox of the scroll container associated with the scroll markers
  // this chooser is picking from.
  const LayoutBox* scroller_box_;
};

class PaintLayerScrollableArea;

class ScrollMarkerGroupData : public GarbageCollected<ScrollMarkerGroupData>,
                              public ScrollSnapshotClient,
                              public ElementRareDataField {
 public:
  explicit ScrollMarkerGroupData(LocalFrame* frame)
      : ScrollSnapshotClient(frame) {}
  void AddToFocusGroup(Element& scroll_marker);
  void RemoveFromFocusGroup(Element& scroll_marker);
  void ClearFocusGroup();
  const HeapVector<Member<Element>>& ScrollMarkers() { return focus_group_; }

  // Set selected scroll marker. Returns true if the selected marker changed.
  CORE_EXPORT bool SetSelected(Element* scroll_marker,
                               bool apply_snap_alignment = true);
  Element* Selected() const;
  void UpdateSelectedScrollMarker();

  Element* FindNextScrollMarker(const Element* current);
  Element* FindPreviousScrollMarker(const Element* current);

  void SetNeedsScrollersMapUpdate() { needs_scrollers_map_update_ = true; }
  void UpdateScrollableAreaSubscriptions(
      HeapHashSet<Member<PaintLayerScrollableArea>>& scrollable_areas);
  bool NeedsScrollersMapUpdate() const { return needs_scrollers_map_update_; }

  void Trace(Visitor* v) const final;

  // ScrollSnapshotClient:
  void UpdateSnapshot() override;
  bool ValidateSnapshot() override;
  bool ShouldScheduleNextService() override;

  // When a "targeted" scroll occurs, we should consider the selected scroll
  // marker pinned until a non-targeted scroll occurs.
  void PinSelectedMarker(Element* scroll_marker) {
    pending_selected_marker_ = scroll_marker;
    selected_marker_is_pinned_ = true;
  }
  void UnPinSelectedMarker() { selected_marker_is_pinned_ = false; }
  bool SelectedMarkerIsPinned() const { return selected_marker_is_pinned_; }

 private:
  Element* ChooseMarker(const ScrollOffset& scroll_offset,
                        ScrollableArea* scrollable_area,
                        LayoutBox* scroller_box,
                        const HeapVector<Member<Element>>& candidates);
  Element* ChooseMarkerRecursively();

  bool UpdateSnapshotInternal();

  // TODO(332396355): Add spec link, once it's created.
  HeapVector<Member<Element>> focus_group_;

  // True, if some <a> scroll markers have been added or removed. It signals
  // to Document that ScrollMarkerGroupData -> "scrollers with <a> scroll
  // marker targets" map should be updated.
  bool needs_scrollers_map_update_ = false;
  // Whether to resist changing the selected scroll marker. We resist updating
  // the last selected scroll marker if it was selected due to a targeted
  // scroll. It should remain the selected scroll marker until we clear this bit
  // due to a non-targeted scroll.
  bool selected_marker_is_pinned_;
  // The scroll marker selected based on the last scroll update observed.
  // At the next snapshot, it will become the |selected_marker_|, if it isn't
  // already, and be cleared.
  Member<Element> pending_selected_marker_;
  // The selected scroll marker that was captured at the time of the last
  // snapshot.
  Member<Element> selected_marker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SCROLL_MARKER_GROUP_DATA_H_
