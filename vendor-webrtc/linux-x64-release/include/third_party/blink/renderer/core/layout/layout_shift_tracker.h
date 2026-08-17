// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_TRACKER_H_

#include <vector>

#include "base/check_op.h"
#include "base/time/time.h"
#include "cc/base/region.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/logical_size.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/layout_shift_region.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/core/timing/layout_shift.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class LayoutBox;
class LayoutObject;
class LayoutText;
class LocalFrameView;
class PropertyTreeStateOrAlias;
class TracedValue;
class WebInputEvent;

// Tracks "layout shifts" from layout objects changing their visual location
// between animation frames. See https://github.com/WICG/layout-instability.
class CORE_EXPORT LayoutShiftTracker final
    : public GarbageCollected<LayoutShiftTracker> {
 public:
  explicit LayoutShiftTracker(LocalFrameView*);
  ~LayoutShiftTracker() = default;

  bool NeedsToTrack(const LayoutObject&) const;

  // |old_rect| and |new_rect| are border box rects, united with scrollable
  // overflow rects if the box has scrollable overflow and doesn't clip
  // overflow, in the local transform space (property_tree_state.Transform()).
  // |old_paint_offset| and |new_paint_offset| are the offsets of the border box
  // rect in the local transform space, which are the same as |old_rect.offset|
  // and |new_rect.offset| respectively if the rects are border box rects.
  // As we don't save the old property tree state, the caller should adjust
  // |old_rect| and |old_paint_offset| so that we can calculate the correct old
  // visual representation and old starting point in the initial containing
  // block and the viewport with the new property tree state in most cases.
  // The adjustment should include the deltas of 2d translations and scrolls,
  // and LayoutShiftTracker can determine stability by including (by default)
  // or excluding |translation_delta| and/or |scroll_delta|.
  //
  // See renderer/core/layout/layout-shift-tracker-old-paint-offset.md for
  // more details about |old_paint_offset|.
  void NotifyBoxPrePaint(const LayoutBox& box,
                         const PropertyTreeStateOrAlias& property_tree_state,
                         const PhysicalRect& old_rect,
                         const PhysicalRect& new_rect,
                         const PhysicalOffset& old_paint_offset,
                         const gfx::Vector2dF& translation_delta,
                         const gfx::Vector2dF& scroll_delta,
                         const gfx::Vector2dF& scroll_anchor_adjustment,
                         const PhysicalOffset& new_paint_offset);

  void NotifyTextPrePaint(const LayoutText& text,
                          const PropertyTreeStateOrAlias& property_tree_state,
                          const LogicalOffset& old_starting_point,
                          const LogicalOffset& new_starting_point,
                          const PhysicalOffset& old_paint_offset,
                          const gfx::Vector2dF& translation_delta,
                          const gfx::Vector2dF& scroll_delta,
                          const gfx::Vector2dF& scroll_anchor_adjustment,
                          const PhysicalOffset& new_paint_offset,
                          const LayoutUnit logical_height);

  void NotifyPrePaintFinished();
  void NotifyInput(const WebInputEvent&);
  void NotifyScroll(mojom::blink::ScrollType, ScrollOffset delta);
  void NotifyViewportSizeChanged();
  void NotifyFindInPageInput();
  void NotifyChangeEvent();
  void NotifyZoomLevelChanged();
  void NotifyBrowserInitiatedSameDocumentNavigation();
  bool IsActive() const { return is_active_; }
  double Score() const { return score_; }
  double WeightedScore() const { return weighted_score_; }
  float OverallMaxDistance() const { return overall_max_distance_; }
  bool ObservedInputOrScroll() const { return observed_input_or_scroll_; }
  void Dispose();
  base::TimeTicks MostRecentInputTimestamp() {
    return most_recent_input_timestamp_;
  }
  void ResetTimerForTesting();
  void Trace(Visitor* visitor) const;

  // Saves and restores geometry on layout boxes when a layout tree is rebuilt
  // by Node::ReattachLayoutTree.
  class ReattachHookScope {
    STACK_ALLOCATED();

   public:
    explicit ReattachHookScope(const Node&);
    ~ReattachHookScope();

    ReattachHookScope(const ReattachHookScope&) = delete;
    ReattachHookScope& operator=(const ReattachHookScope&) = delete;

    static void NotifyDetach(const Node&);
    static void NotifyAttach(const Node&);

   private:
    ReattachHookScope* outer_;
    static ReattachHookScope* top_;
    struct Geometry {
      PhysicalOffset paint_offset;
      PhysicalSize size;
      PhysicalRect visual_overflow_rect;
      bool has_paint_offset_translation;
    };
    HeapHashMap<Member<const Node>, Geometry> geometries_before_detach_;
  };

  class CORE_EXPORT ContainingBlockScope {
    USING_FAST_MALLOC(ContainingBlockScope);

   public:
    // |old_size| and |new_size| are the border box sizes.
    // |old_rect| and |new_rect| have the same definition as in
    // NotifyBoxPrePaint().
    ContainingBlockScope(const PhysicalSize& old_size,
                         const PhysicalSize& new_size,
                         const PhysicalRect& old_rect,
                         const PhysicalRect& new_rect)
        : outer_(top_),
          old_size_(old_size),
          new_size_(new_size),
          old_rect_(old_rect),
          new_rect_(new_rect) {
      top_ = this;
    }
    ~ContainingBlockScope() {
      DCHECK_EQ(top_, this);
      top_ = outer_;
    }

    ContainingBlockScope(const ContainingBlockScope&) = delete;
    ContainingBlockScope& operator=(const ContainingBlockScope&) = delete;

   private:
    friend class LayoutShiftTracker;
    ContainingBlockScope* outer_;
    static ContainingBlockScope* top_;
    PhysicalSize old_size_;
    PhysicalSize new_size_;
    PhysicalRect old_rect_;
    PhysicalRect new_rect_;
  };

 private:
  void ObjectShifted(const LayoutObject&,
                     const PropertyTreeStateOrAlias&,
                     const PhysicalRect& old_rect,
                     const PhysicalRect& new_rect,
                     const gfx::PointF& old_starting_point,
                     const gfx::Vector2dF& translation_delta,
                     const gfx::Vector2dF& scroll_offset_delta,
                     const gfx::Vector2dF& scroll_anchor_adjustment,
                     const gfx::PointF& new_starting_point);

  void ReportShift(double score_delta, double weighted_score_delta);
  void TimerFired(TimerBase*) {}
  std::unique_ptr<TracedValue> PerFrameTraceData(double score_delta,
                                                 double weighted_score_delta,
                                                 bool input_detected) const;
  void AttributionsToTracedValue(TracedValue&) const;
  double SubframeWeightingFactor() const;

  // Sends layout shift rects to the heads-up display (HUD) layer, if
  // visualization is enabled (by --show-layout-shift-regions or devtools
  // "Layout Shift Regions" option).
  void SendLayoutShiftRectsToHud(const Vector<gfx::Rect>& rects);

  void UpdateInputTimestamps(base::TimeTicks timestamp);
  bool HasRecentInput();

  LayoutShift::AttributionList CreateAttributionList() const;
  void SubmitPerformanceEntry(double score_delta, bool input_detected) const;
  void NotifyPrePaintFinishedInternal();
  double LastInputTimestamp() const;
  void UpdateTimerAndInputTimestamp();

  Member<LocalFrameView> frame_view_;
  bool is_active_;

  // The document cumulative layout shift (DCLS) score for this LocalFrame,
  // unweighted, with move distance applied.
  double score_;

  // The cumulative layout shift score for this LocalFrame, with each increase
  // weighted by the extent to which the LocalFrame visibly occupied the main
  // frame at the time the shift occurred, e.g. x0.5 if the subframe occupied
  // half of the main frame's reported size; see SubframeWeightingFactor().
  double weighted_score_;

  // Stores information related to buffering layout shifts after pointerdown.
  // We accumulate score deltas in this object until we know whether the
  // pointerdown should be treated as a tap (triggering layout shift exclusion)
  // or a scroll (not triggering layout shift exclusion).  Once the correct
  // treatment is known, the pending layout shifts are reported appropriately
  // and the PointerdownPendingData object is reset.
  struct PointerdownPendingData {
    PointerdownPendingData() = default;
    int num_pointerdowns = 0;
    int num_pressed_mouse_buttons = 0;
    double score_delta = 0;
    double weighted_score_delta = 0;
  };

  PointerdownPendingData pointerdown_pending_data_;

  // The per-animation-frame impact region.
  LayoutShiftRegion region_;

  // Tracks the short period after an input event during which we ignore shifts
  // for the purpose of cumulative scoring, and report them to the web perf API
  // with hadRecentInput == true.
  HeapTaskRunnerTimer<LayoutShiftTracker> timer_;

  // The maximum distance any layout object has moved in the current animation
  // frame.
  float frame_max_distance_;

  // The maximum distance any layout object has moved, across all animation
  // frames.
  float overall_max_distance_;

  // Whether either a user input or document scroll have been observed during
  // the session. (This is only tracked so UkmPageLoadMetricsObserver to report
  // LayoutInstability.CumulativeShiftScore.MainFrame.BeforeInputOrScroll. It's
  // not related to input exclusion or the LayoutShift::had_recent_input_ bit.)
  bool observed_input_or_scroll_;

  // Most recent timestamp of a user input event that has been observed.
  // User input includes window resizing but not scrolling.
  base::TimeTicks most_recent_input_timestamp_;
  bool most_recent_input_timestamp_initialized_;

  // Timestamp used to run an imaginary timer for tracking period since last
  // input processed. It resets 500ms after the most recent input is processed.
  base::TimeTicks most_recent_input_processing_timestamp_;

  struct Attribution {
    DOMNodeId node_id = kInvalidDOMNodeId;
    gfx::Rect old_visual_rect;
    gfx::Rect new_visual_rect;

    explicit operator bool() const;
    bool Encloses(const Attribution&) const;
    bool MoreImpactfulThan(const Attribution&) const;
    uint64_t Area() const;
  };

  void MaybeRecordAttribution(const Attribution&);

  // Nodes that have contributed to the impact region for the current frame.
  std::array<Attribution, LayoutShift::kMaxAttributions> attributions_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_SHIFT_TRACKER_H_
