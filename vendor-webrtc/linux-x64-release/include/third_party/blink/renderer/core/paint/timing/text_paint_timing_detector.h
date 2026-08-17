// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_PAINT_TIMING_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_PAINT_TIMING_DETECTOR_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/paint/timing/lcp_objects.h"
#include "third_party/blink/renderer/core/paint/timing/text_element_timing.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/rect_conversions.h"

namespace blink {
class LayoutBoxModelObject;
class LocalFrameView;
class PaintTimingCallbackManager;
class PropertyTreeStateOrAlias;
class TextElementTiming;
class TracedValue;
struct DOMPaintTimingInfo;

class TextRecord final : public GarbageCollected<TextRecord> {
 public:
  TextRecord(Node& node,
             uint64_t new_recorded_size,
             const gfx::RectF& element_timing_rect,
             const gfx::Rect& frame_visual_rect,
             const gfx::RectF& root_visual_rect,
             uint32_t frame_index,
             bool is_needed_for_timing)
      : node_(&node),
        recorded_size(new_recorded_size),
        frame_index_(frame_index),
        element_timing_rect_(element_timing_rect),
        is_needed_for_timing_(is_needed_for_timing) {
    if (PaintTimingVisualizer::IsTracingEnabled()) {
      lcp_rect_info_ = std::make_unique<LCPRectInfo>(
          frame_visual_rect, gfx::ToRoundedRect(root_visual_rect));
    }
  }
  TextRecord(const TextRecord&) = delete;
  TextRecord& operator=(const TextRecord&) = delete;

  void Trace(Visitor*) const;

  WeakMember<Node> node_;
  uint64_t recorded_size = 0;
  uint32_t frame_index_ = 0;
  gfx::RectF element_timing_rect_;
  std::unique_ptr<LCPRectInfo> lcp_rect_info_;
  // The time of the first paint after fully loaded.
  base::TimeTicks paint_time = base::TimeTicks();
  DOMPaintTimingInfo paint_timing_info;
  bool is_needed_for_timing_ = false;
};

class CORE_EXPORT LargestTextPaintManager final
    : public GarbageCollected<LargestTextPaintManager> {
 public:
  LargestTextPaintManager(LocalFrameView*, PaintTimingDetector*);
  LargestTextPaintManager(const LargestTextPaintManager&) = delete;
  LargestTextPaintManager& operator=(const LargestTextPaintManager&) = delete;

  inline TextRecord* LargestText() {
    DCHECK(!largest_text_ || !largest_text_->paint_time.is_null());
    return largest_text_.Get();
  }
  void MaybeUpdateLargestText(TextRecord* record);
  void MaybeUpdateLargestIgnoredText(const LayoutObject&,
                                     const uint64_t&,
                                     const gfx::Rect& frame_visual_rect,
                                     const gfx::RectF& root_visual_rect);

  // Return the text LCP candidate and whether the candidate has changed.
  std::pair<TextRecord*, bool> UpdateMetricsCandidate();

  void ReportCandidateToTrace(const TextRecord&);
  void PopulateTraceValue(TracedValue&, const TextRecord& first_text_paint);

  Member<TextRecord> PopLargestIgnoredText() {
    return std::move(largest_ignored_text_);
  }

  void Clear() {
    count_candidates_ = 0;
    largest_text_.Clear();
    largest_ignored_text_.Clear();
  }
  bool IsUnrelatedSoftNavigationPaint(const Node& node) {
    CHECK(paint_timing_detector_);
    return paint_timing_detector_->IsUnrelatedSoftNavigationPaint(node);
  }

  void Trace(Visitor*) const;

 private:
  friend class LargestContentfulPaintCalculatorTest;
  friend class TextPaintTimingDetectorTest;

  // The current largest text.
  Member<TextRecord> largest_text_;

  unsigned count_candidates_ = 0;

  // Text paints are ignored when they (or an ancestor) have opacity 0. This can
  // be a problem later on if the opacity changes to nonzero but this change is
  // composited. We solve this for the special case of documentElement by
  // storing a record for the largest ignored text without nested opacity. We
  // consider this an LCP candidate when the documentElement's opacity changes
  // from zero to nonzero.
  Member<TextRecord> largest_ignored_text_;

  Member<const LocalFrameView> frame_view_;
  Member<PaintTimingDetector> paint_timing_detector_;
};

// TextPaintTimingDetector contains Largest Text Paint and support for Text
// Element Timing.
//
// Largest Text Paint timing measures when the largest text element gets painted
// within the viewport. Specifically, it:
// 1. Tracks all texts' first paints. If the text may be a largest text or is
// required by Element Timing, it records the visual size and paint time.
// 2. It keeps track of information regarding the largest text paint seen so
// far. Because the new version of LCP includes removed content, this record may
// only increase in size over time. See also this doc, which is now somewhat
// outdated: http://bit.ly/fcp_plus_plus.
class CORE_EXPORT TextPaintTimingDetector final
    : public GarbageCollected<TextPaintTimingDetector> {
  friend class TextPaintTimingDetectorTest;

 public:
  explicit TextPaintTimingDetector(LocalFrameView*, PaintTimingDetector*);
  TextPaintTimingDetector(const TextPaintTimingDetector&) = delete;
  TextPaintTimingDetector& operator=(const TextPaintTimingDetector&) = delete;

  bool ShouldWalkObject(const LayoutBoxModelObject&) const;
  void RecordAggregatedText(const LayoutBoxModelObject& aggregator,
                            const gfx::Rect& aggregated_visual_rect,
                            const PropertyTreeStateOrAlias&);
  std::optional<base::OnceCallback<void(const base::TimeTicks&,
                                        const DOMPaintTimingInfo&)>>
  TakePaintTimingCallback();
  void LayoutObjectWillBeDestroyed(const LayoutObject&);
  void StopRecordingLargestTextPaint();
  void RestartRecordingLargestTextPaint();
  void ResetCallbackManager(PaintTimingCallbackManager* manager) {
    callback_manager_ = manager;
  }

  inline bool IsRecordingLargestTextPaint() const {
    return recording_largest_text_paint_;
  }
  inline std::pair<TextRecord*, bool> UpdateMetricsCandidate() {
    return ltp_manager_->UpdateMetricsCandidate();
  }
  void ReportLargestIgnoredText();
  void Trace(Visitor*) const;

 private:
  friend class LargestContentfulPaintCalculatorTest;

  void AssignPaintTimeToQueuedRecords(uint32_t frame_index,
                                      const base::TimeTicks&,
                                      const DOMPaintTimingInfo&);
  void MaybeRecordTextRecord(
      const LayoutObject& object,
      const uint64_t& visual_size,
      const PropertyTreeStateOrAlias& property_tree_state,
      const gfx::Rect& frame_visual_rect,
      const gfx::RectF& root_visual_rect);
  inline void QueueToMeasurePaintTime(const LayoutObject& object,
                                      TextRecord* record) {
    texts_queued_for_paint_time_.insert(&object, record);
    added_entry_in_latest_frame_ = true;
  }

  // LayoutObjects for which text has been aggregated.
  HeapHashSet<Member<const LayoutObject>> recorded_set_;
  HeapHashSet<Member<const LayoutObject>> rewalkable_set_;

  // Text records queued for paint time. Indexed by LayoutObject to make removal
  // easy.
  HeapHashMap<Member<const LayoutObject>, Member<TextRecord>>
      texts_queued_for_paint_time_;

  Member<PaintTimingCallbackManager> callback_manager_;
  Member<const LocalFrameView> frame_view_;
  // Set lazily because we may not have the correct Window when first
  // initializing this class.
  Member<TextElementTiming> text_element_timing_;

  Member<LargestTextPaintManager> ltp_manager_;
  bool recording_largest_text_paint_ = true;

  // Used to decide which frame a record belongs to, monotonically increasing.
  uint32_t frame_index_ = 1;
  bool added_entry_in_latest_frame_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_TEXT_PAINT_TIMING_DETECTOR_H_
