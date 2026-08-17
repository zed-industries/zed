// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_SNAPSHOT_TIMELINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_SNAPSHOT_TIMELINE_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_scroll_axis.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/animation/animation_timeline.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/scroll/scroll_snapshot_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

// See ScrollTimeline.
class CORE_EXPORT ScrollSnapshotTimeline : public AnimationTimeline,
                                           public ScrollSnapshotClient {
 public:
  using ScrollOffsets = cc::ScrollTimeline::ScrollOffsets;
  using ScrollAxis = V8ScrollAxis::Enum;
  using ViewOffsets = TimelineRange::ViewOffsets;

  explicit ScrollSnapshotTimeline(Document*);

  bool IsScrollSnapshotTimeline() const override { return true; }

  // ScrollTimeline is not resolved if source is null, does not currently
  // have a CSS layout box, or if its layout box is not a scroll container.
  bool IsResolved() const override;

  // ScrollTimeline is not active if not resolved or if the current time is
  // unresolved (e.g. before the timeline ticks).
  // https://github.com/WICG/scroll-animations/issues/31
  bool IsActive() const override;

  std::optional<base::TimeDelta> InitialStartTimeForAnimations() override;

  TimelineRange GetTimelineRange() const override;

  AnimationTimeDelta ZeroTime() override { return AnimationTimeDelta(); }

  void ServiceAnimations(TimingUpdateReason) override;
  void ScheduleNextService() override;

  V8CSSNumberish* currentTime() override;
  V8CSSNumberish* duration() override;
  V8CSSNumberish* ConvertTimeToProgress(AnimationTimeDelta time) const;

  // Returns the Node that should actually have the ScrollableArea (if one
  // exists). This can differ from |source| when defaulting to the
  // Document's scrollingElement, and it may be null if the document was
  // removed before the ScrollTimeline was created.
  //
  // NOTE: Do not use the layout box for the resolved source directly since the
  // scroll container may be in a layout box that is a descendant of the
  // resolved source's layout box.
  Node* ResolvedSource() const {
    return timeline_state_snapshotted_.resolved_source.Get();
  }

  // Returns the layout box for the resolved source's scrollable area. In most
  // cases, the layout box for the resolved source is a scroll container.  A
  // fieldset has a legend and scrollable content.  The scrollable content is
  // in an anonymous block.
  LayoutBox* ScrollContainer() const {
    return ComputeScrollContainer(ResolvedSource());
  }

  // Return the latest resolved scroll/view offsets. This will be empty when
  // timeline is inactive.
  std::optional<ScrollOffsets> GetResolvedScrollOffsets() const;
  std::optional<ViewOffsets> GetResolvedViewOffsets() const;

  std::optional<ScrollOffsets> GetResolvedScrollLimits() const;

  float GetResolvedZoom() const { return timeline_state_snapshotted_.zoom; }

  // Mark every effect target of every Animation attached to this timeline
  // for style recalc.
  void InvalidateEffectTargetStyle() const;

  void Trace(Visitor*) const override;

  // Duration is the maximum value a timeline may generate for current time.
  // Used to convert time values to proportional values.
  std::optional<AnimationTimeDelta> GetDuration() const final {
    // A fallback value is used for the duration since getAnimations must pick
    // up a scroll driven animation even it is inactive, and uses the presence
    // of a duration to flag as an SDA. Furthermore timing conversion methods
    // must work even if the timeline has not ticked. A proper duration is
    // calculated on the first tick.
    return timeline_state_snapshotted_.duration.has_value()
               ? timeline_state_snapshotted_.duration
               : std::make_optional(ANIMATION_TIME_DELTA_FROM_SECONDS(100));
  }

  void ResolveTimelineOffsets() const;

  cc::AnimationTimeline* EnsureCompositorTimeline() override;
  void UpdateCompositorTimeline() override;

  virtual ScrollAxis GetAxis() const = 0;

 protected:
  // For access to TimelineState.
  friend class AnimationTrigger;

  PhaseAndTime CurrentPhaseAndTime() override;

  AnimationTimeDelta CalculateIntrinsicIterationDuration(
      const TimelineRange&,
      const std::optional<TimelineOffset>& range_start,
      const std::optional<TimelineOffset>& range_end,
      const Timing&) override;

  static LayoutBox* ComputeScrollContainer(Node* resolved_source);

  struct TimelineState {
    DISALLOW_NEW();

   public:
    // TODO(crbug.com/1338167): Remove phase as it can be inferred from
    // current_time.
    TimelinePhase phase = TimelinePhase::kInactive;
    std::optional<base::TimeDelta> current_time;
    // Offsets corresponding to the entire scroll range of the scroll
    // container backing the timeline in the axis of the timeline.
    std::optional<ScrollOffsets> scroll_limits;
    // Offset corresponding to either:
    //  1. the entire scroll range of the scroll container backing the timeline
    //     (if it's a ScrollTimeline).
    //  2. the boundaries of the view progress of the subject within the scroll
    //     container backing the timeline. (if it's a ViewTimeline).
    std::optional<ScrollOffsets> scroll_offsets;
    // The view offsets will be null unless using a view timeline.
    std::optional<ViewOffsets> view_offsets;
    // Zoom factor applied to the scroll offsets.
    float zoom = 1.0f;
    // Duration to use for time scaling. The duration is set so that LayoutUnit
    // precision (1/64th of a pixel) aligns with the time precision requirement
    // of 1 microsecond.
    std::optional<AnimationTimeDelta> duration;

    // The scroller driving the timeline. The layout box is stored in addition
    // to the resolved source since for fieldset, the scroller container is
    // a child of the resolved source's layout box due to the fieldset
    // containing a non-scrollable legend and scrollable content.
    Member<Node> resolved_source;

    bool HasConsistentLayout(const TimelineState& other) const {
      return scroll_offsets == other.scroll_offsets && zoom == other.zoom &&
             view_offsets == other.view_offsets;
    }

    bool operator==(const TimelineState& other) const {
      return phase == other.phase && current_time == other.current_time &&
             scroll_offsets == other.scroll_offsets && zoom == other.zoom &&
             view_offsets == other.view_offsets &&
             resolved_source == other.resolved_source;
    }

    void Trace(blink::Visitor* visitor) const {
      visitor->Trace(resolved_source);
    }
  };

  // ScrollSnapshotClient:
  // https://wicg.github.io/scroll-animations/#avoiding-cycles
  // Snapshots scroll timeline current time and phase.
  // Called once per animation frame.
  void UpdateSnapshot() override;
  bool ValidateSnapshot() override;
  bool ShouldScheduleNextService() override;

 public:
  // Public for DeferredTimeline::ComputeTimelineState.
  virtual TimelineState ComputeTimelineState() const = 0;

  void CalculateScrollLimits(PaintLayerScrollableArea* scrollable_area,
                             ScrollOrientation physical_orientation,
                             TimelineState* state) const;

 private:
  // Snapshotted value produced by the last SnapshotState call.
  TimelineState timeline_state_snapshotted_;
};

template <>
struct DowncastTraits<ScrollSnapshotTimeline> {
  static bool AllowFrom(const AnimationTimeline& value) {
    return value.IsScrollSnapshotTimeline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_SCROLL_SNAPSHOT_TIMELINE_H_
