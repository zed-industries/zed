// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "components/viz/common/frame_timing_details.h"
#include "third_party/blink/public/web/web_performance_metrics_for_reporting.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/paint/paint_event.h"
#include "third_party/blink/renderer/core/paint/timing/first_meaningful_paint_detector.h"
#include "third_party/blink/renderer/core/timing/animation_frame_timing_info.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace base {
class TickClock;
}

namespace blink {

struct DOMPaintTimingInfo;
class LocalFrame;

using PaintTimingCallback =
    base::OnceCallback<void(const base::TimeTicks&, const DOMPaintTimingInfo&)>;

using OptionalPaintTimingCallback = std::optional<PaintTimingCallback>;

// PaintTiming is responsible for tracking paint-related timings for a given
// document.
class CORE_EXPORT PaintTiming final : public GarbageCollected<PaintTiming>,
                                      public Supplement<Document> {
  friend class FirstMeaningfulPaintDetector;
  using ReportTimeCallback =
      base::OnceCallback<void(const viz::FrameTimingDetails&)>;
  using RequestAnimationFrameTimesAfterBackForwardCacheRestore = std::array<
      base::TimeTicks,
      WebPerformanceMetricsForReporting::
          kRequestAnimationFramesToRecordAfterBackForwardCacheRestore>;

 public:
  static const char kSupplementName[];

  struct PaintTimingInfo {
    // https://w3c.github.io/paint-timing/#paint-timing-info-rendering-update-end-time
    base::TimeTicks rendering_update_end_time;

    // https://w3c.github.io/paint-timing/#paint-timing-info-implementation-defined-presentation-time
    base::TimeTicks presentation_time;
  };

  explicit PaintTiming(Document&);
  PaintTiming(const PaintTiming&) = delete;
  PaintTiming& operator=(const PaintTiming&) = delete;
  ~PaintTiming() = default;

  static PaintTiming& From(Document&);
  static const PaintTiming* From(const Document&);

  // Mark*() methods record the time for the given paint event and queue a
  // presentation promise to record the |first_*_presentation_| timestamp. These
  // methods do nothing (early return) if a time has already been recorded for
  // the given paint event.
  void MarkFirstPaint();

  // MarkFirstImagePaint, and MarkFirstContentfulPaint
  // will also record first paint if first paint hasn't been recorded yet.
  void MarkFirstContentfulPaint();

  // MarkFirstImagePaint will also record first contentful paint if first
  // contentful paint hasn't been recorded yet.
  void MarkFirstImagePaint();

  // MarkFirstEligibleToPaint records the first time that the frame is not
  // throttled and so is eligible to paint. A null value indicates throttling.
  void MarkFirstEligibleToPaint();

  // MarkIneligibleToPaint resets the paint eligibility timestamp to null.
  // A null value indicates throttling. This call is ignored if a first
  // contentful paint has already been recorded.
  void MarkIneligibleToPaint();

  void SetFirstMeaningfulPaintCandidate(base::TimeTicks timestamp);
  void SetFirstMeaningfulPaint(
      base::TimeTicks presentation_time,
      FirstMeaningfulPaintDetector::HadUserInput had_input);
  void NotifyPaint(bool is_first_paint, bool text_painted, bool image_painted);
  void NotifyPaintFinished() { MarkPaintTimingInternal(); }

  // The getters below return monotonically-increasing seconds, or zero if the
  // given paint event has not yet occurred. See the comments for
  // monotonicallyIncreasingTime in wtf/Time.h for additional details.

  // Returns the first time that anything was painted for the
  // current document after a hard navigation. This is not considering soft
  // navigations.
  base::TimeTicks FirstPaintForMetrics() const {
    return first_paint_presentation_for_ukm_;
  }

  // Times when the first paint happens after the page is restored from the
  // back-forward cache. If the element value is zero time tick, the first paint
  // event did not happen for that navigation.
  WTF::Vector<base::TimeTicks> FirstPaintsAfterBackForwardCacheRestore() const {
    return first_paints_after_back_forward_cache_restore_presentation_;
  }

  WTF::Vector<RequestAnimationFrameTimesAfterBackForwardCacheRestore>
  RequestAnimationFramesAfterBackForwardCacheRestore() const {
    return request_animation_frames_after_back_forward_cache_restore_;
  }

  // Returns the first time that 'contentful' content was painted in the current
  // document after a hard navigation (and ignoring soft navigations). For
  // instance, the first time that text or image content was painted after the
  // user landed on the page.
  base::TimeTicks FirstContentfulPaintIgnoringSoftNavigations() const {
    return first_contentful_paint_presentation_ignoring_soft_navigations_;
  }

  base::TimeTicks FirstContentfulPaintRenderedButNotPresentedAsMonotonicTime()
      const {
    return paint_details_.first_contentful_paint_;
  }

  void ResetFirstPaintAndFCP() {
    soft_navigation_pending_paint_details_ = PaintDetails();
    first_paints_reset_ = true;
    soft_navigation_detected_ = false;
    soft_navigation_fp_reported_ = false;
    soft_navigation_fcp_reported_ = false;
  }

  // FirstImagePaint returns the first time that image content was painted.
  base::TimeTicks FirstImagePaint() const {
    return paint_details_.first_image_paint_presentation_;
  }

  // FirstEligibleToPaint returns the first time that the frame is not
  // throttled and is eligible to paint. A null value indicates throttling.
  base::TimeTicks FirstEligibleToPaint() const {
    return first_eligible_to_paint_;
  }

  // FirstMeaningfulPaint returns the first time that page's primary content
  // was painted.
  base::TimeTicks FirstMeaningfulPaint() const {
    return first_meaningful_paint_presentation_;
  }

  // FirstMeaningfulPaintCandidate indicates the first time we considered a
  // paint to qualify as the potentially first meaningful paint. Unlike
  // firstMeaningfulPaint, this signal is available in real time, but it may be
  // an optimistic (i.e., too early) estimate.
  base::TimeTicks FirstMeaningfulPaintCandidate() const {
    return first_meaningful_paint_candidate_;
  }

  base::TimeTicks FirstContentfulPaintPresentation() const {
    return paint_details_.first_contentful_paint_presentation_;
  }

  FirstMeaningfulPaintDetector& GetFirstMeaningfulPaintDetector() {
    return *fmp_detector_;
  }

  void RegisterNotifyPresentationTime(ReportTimeCallback);
  void ReportPresentationTime(PaintEvent,
                              base::TimeTicks rendering_update_end_time,
                              const viz::FrameTimingDetails&);
  void RecordFirstContentfulPaintTimingMetrics(const viz::FrameTimingDetails&);
  void ReportFirstPaintAfterBackForwardCacheRestorePresentationTime(
      wtf_size_t index,
      const viz::FrameTimingDetails&);

  // The caller owns the |clock| which must outlive the PaintTiming.
  void SetTickClockForTesting(const base::TickClock* clock);

  void OnRestoredFromBackForwardCache();

  void SoftNavigationDetected();

  void MarkPaintTiming();

  void Trace(Visitor*) const override;

 private:
  friend class RecodingTimeAfterBackForwardCacheRestoreFrameCallback;

  LocalFrame* GetFrame() const;
  void NotifyPaintTimingChanged();

  void MarkPaintTimingInternal();

  // Set*() set the timing for the given paint event to the given timestamp if
  // the value is currently zero, and queue a presentation promise to record the
  // |first_*_presentation_| timestamp. These methods can be invoked from other
  // Mark*() or Set*() methods to make sure that first paint is marked as part
  // of marking first contentful paint, or that first contentful paint is marked
  // as part of marking first text/image paint, for example.
  void SetFirstPaint(base::TimeTicks stamp);

  // setFirstContentfulPaint will also set first paint time if first paint
  // time has not yet been recorded.
  void SetFirstContentfulPaint(base::TimeTicks stamp);

  // Set*Presentation() are called when the presentation promise is fulfilled
  // and the presentation timestamp is available. These methods will record
  // trace events, update Web Perf API (FP and FCP only), and notify that paint
  // timing has changed, which triggers UMAs and UKMS. |stamp| is the
  // presentation timestamp used for tracing, UMA, UKM, and Web Perf API.
  void SetFirstPaintPresentation(const PaintTimingInfo&);
  void SetFirstContentfulPaintPresentation(const PaintTimingInfo&);
  void SetFirstImagePaintPresentation(base::TimeTicks stamp);

  // When quickly navigating back and forward between the pages in the cache
  // paint events might race with navigations. Pass explicit bfcache restore
  // index to avoid confusing the data from different navigations.
  void SetFirstPaintAfterBackForwardCacheRestorePresentation(
      base::TimeTicks stamp,
      wtf_size_t index);
  void SetRequestAnimationFrameAfterBackForwardCacheRestore(wtf_size_t index,
                                                            size_t count);

  void Mark(PaintEvent);
  void RegisterNotifyFirstPaintAfterBackForwardCacheRestorePresentationTime(
      wtf_size_t index);

  base::TimeTicks FirstPaintRendered() const {
    return paint_details_.first_paint_;
  }

  WTF::Vector<base::TimeTicks>
      first_paints_after_back_forward_cache_restore_presentation_;
  WTF::Vector<RequestAnimationFrameTimesAfterBackForwardCacheRestore>
      request_animation_frames_after_back_forward_cache_restore_;
  struct PaintDetails {
    // TODO(crbug/738235): Non first_*_presentation_ variables are only being
    // tracked to compute deltas for reporting histograms and should be removed
    // once we confirm the deltas and discrepancies look reasonable.
    base::TimeTicks first_paint_;
    base::TimeTicks first_paint_presentation_;
    base::TimeTicks first_image_paint_;
    base::TimeTicks first_image_paint_presentation_;
    base::TimeTicks first_contentful_paint_;
    base::TimeTicks first_contentful_paint_presentation_;
  };

  PaintDetails& GetRelevantPaintDetails() {
    return first_paints_reset_ ? soft_navigation_pending_paint_details_
                               : paint_details_;
  }

  DOMPaintTimingInfo ToDOMPaintTimingInfo(const PaintTimingInfo&) const;

  PaintDetails paint_details_;
  PaintDetails soft_navigation_pending_paint_details_;
  std::optional<PaintTimingInfo>
      soft_navigation_pending_first_paint_timing_info_;
  std::optional<PaintTimingInfo>
      soft_navigation_pending_first_contentful_paint_timing_info_;
  // First paint timestamp that doesn't update after soft navigations, and only
  // used for UKM reporting.
  base::TimeTicks first_paint_presentation_for_ukm_;
  // FCP timestamp that does not update after soft navigations.
  base::TimeTicks
      first_contentful_paint_presentation_ignoring_soft_navigations_;
  base::TimeTicks first_meaningful_paint_presentation_;
  base::TimeTicks first_meaningful_paint_candidate_;
  base::TimeTicks first_eligible_to_paint_;
  base::TimeTicks last_rendering_update_end_time_;
  bool first_paints_reset_ = false;
  bool soft_navigation_detected_ = false;
  bool soft_navigation_fp_reported_ = false;
  bool soft_navigation_fcp_reported_ = false;

  base::TimeTicks lcp_mouse_over_dispatch_time_;

  Member<FirstMeaningfulPaintDetector> fmp_detector_;
  // The callback ID for requestAnimationFrame to record its time after the page
  // is restored from the back-forward cache.
  int raf_after_bfcache_restore_measurement_callback_id_ = 0;

  const base::TickClock* clock_;

  FRIEND_TEST_ALL_PREFIXES(FirstMeaningfulPaintDetectorTest,
                           TwoLayoutsSignificantFirst);
  HashSet<PaintEvent> pending_paint_events_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_H_
