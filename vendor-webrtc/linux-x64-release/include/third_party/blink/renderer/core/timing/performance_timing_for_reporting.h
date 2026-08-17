// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_FOR_REPORTING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_FOR_REPORTING_H_

#include "base/time/time.h"
#include "third_party/blink/public/common/performance/largest_contentful_paint_type.h"
#include "third_party/blink/public/web/web_performance_metrics_for_reporting.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/paint/timing/paint_timing_detector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"

namespace blink {

class DocumentLoadTiming;
class DocumentLoader;
class DocumentParserTiming;
class DocumentTiming;
class InteractiveDetector;
class PaintTiming;
struct LargestContentfulPaintDetails;

// This class is only used for non-web-exposed reporting purposes (e.g. UKM).
class CORE_EXPORT PerformanceTimingForReporting final
    : public GarbageCollected<PerformanceTimingForReporting>,
      public ExecutionContextClient {
 public:
  struct BackForwardCacheRestoreTiming {
    uint64_t navigation_start;
    uint64_t first_paint;
    std::array<uint64_t,
               WebPerformanceMetricsForReporting::
                   kRequestAnimationFramesToRecordAfterBackForwardCacheRestore>
        request_animation_frames;
    std::optional<base::TimeDelta> first_input_delay;
  };

  using BackForwardCacheRestoreTimings =
      WTF::Vector<BackForwardCacheRestoreTiming>;

  explicit PerformanceTimingForReporting(ExecutionContext*);

  // These getters are for non-spec timings and used for metrics reporting
  // purposes. They are not to be exposed to JavaScript.

  uint64_t inputStart() const;

  // The time immediately after the user agent finishes prompting to unload the
  // previous document, or if there is no previous document, the same value as
  // fetchStart.  Intended to be used for correlation with other events internal
  // to blink.
  base::TimeTicks NavigationStartAsMonotonicTime() const;

  // The timings after the page is restored from back-forward cache.
  BackForwardCacheRestoreTimings BackForwardCacheRestore() const;

  // The time the first paint operation was performed.
  uint64_t FirstPaintForMetrics() const;

  // The time the first paint operation for image was performed.
  uint64_t FirstImagePaint() const;

  // The first 'contentful' paint as full-resolution monotonic time. This is
  // the point at which blink painted the content for FCP; actual FCP is
  // recorded as the time the generated content makes it to the screen (also
  // known as presentation time). Intended to be used for correlation with other
  // events internal to blink.
  base::TimeTicks FirstContentfulPaintRenderedButNotPresentedAsMonotonicTime()
      const;

  // The time of the first 'contentful' paint. A contentful paint is a paint
  // that includes content of some kind (for example, text or image content).
  uint64_t FirstContentfulPaintIgnoringSoftNavigations() const;

  // The first 'contentful' paint as full-resolution monotonic time. Intended to
  // be used for correlation with other events internal to blink.
  base::TimeTicks FirstContentfulPaintAsMonotonicTimeForMetrics() const;

  // The time of the first 'meaningful' paint, A meaningful paint is a paint
  // where the page's primary content is visible.
  uint64_t FirstMeaningfulPaint() const;

  // The time of the candidate of first 'meaningful' paint, A meaningful paint
  // candidate indicates the first time we considered a paint to qualify as the
  // potential first meaningful paint. But, be careful that it may be an
  // optimistic (i.e., too early) estimate.
  // TODO(crbug.com/848639): This function is exposed as an experiment, and if
  // not useful, this function can be removed.
  uint64_t FirstMeaningfulPaintCandidate() const;

  LargestContentfulPaintDetailsForReporting
  LargestContentfulPaintDetailsForMetrics() const;

  LargestContentfulPaintDetailsForReporting
  SoftNavigationLargestContentfulPaintDetailsForMetrics() const;

  // The time at which the frame is first eligible for painting due to not
  // being throttled. A zero value indicates throttling.
  uint64_t FirstEligibleToPaint() const;

  // The time at which we are notified of the first input or scroll event which
  // causes the largest contentful paint algorithm to stop.
  uint64_t FirstInputOrScrollNotifiedTimestamp() const;

  // The duration between the hardware timestamp and being queued on the main
  // thread for the first click, tap, key press, cancellable touchstart, or
  // pointer down followed by a pointer up.
  std::optional<base::TimeDelta> FirstInputDelay() const;

  // The timestamp of the event whose delay is reported by FirstInputDelay().
  std::optional<base::TimeDelta> FirstInputTimestamp() const;

  // The timestamp of the event whose delay is reported by FirstInputDelay().
  // Intended to be used for correlation with other events internal to blink.
  std::optional<base::TimeTicks> FirstInputTimestampAsMonotonicTime() const;

  // The longest duration between the hardware timestamp and being queued on the
  // main thread for the click, tap, key press, cancellable touchstart, or
  // pointer down followed by a pointer up.
  std::optional<base::TimeDelta> LongestInputDelay() const;

  // The timestamp of the event whose delay is reported by LongestInputDelay().
  std::optional<base::TimeDelta> LongestInputTimestamp() const;

  // The duration of event handlers processing the first input event.
  std::optional<base::TimeDelta> FirstInputProcessingTime() const;

  // The duration between the user's first scroll and display update.
  std::optional<base::TimeDelta> FirstScrollDelay() const;

  // The hardware timestamp of the first scroll.
  std::optional<base::TimeDelta> FirstScrollTimestamp() const;

  // TimeTicks for unload start and end.
  std::optional<base::TimeTicks> UnloadStart() const;
  std::optional<base::TimeTicks> UnloadEnd() const;

  // The timestamp of when the commit navigation finished in the frame loader.
  std::optional<base::TimeTicks> CommitNavigationEnd() const;

  // The timestamp of the user timing mark 'mark_fully_loaded', if
  // available.
  std::optional<base::TimeDelta> UserTimingMarkFullyLoaded() const;

  // The timestamp of the user timing mark 'mark_fully_visible', if
  // available.
  std::optional<base::TimeDelta> UserTimingMarkFullyVisible() const;

  // The timestamp of the user timing mark 'mark_interactive', if
  // available.
  std::optional<base::TimeDelta> UserTimingMarkInteractive() const;

  // The name and startTime of the user timing mark.
  std::optional<std::tuple<AtomicString, base::TimeDelta>>
  CustomUserTimingMark() const;

  uint64_t ParseStart() const;
  uint64_t ParseStop() const;
  uint64_t ParseBlockedOnScriptLoadDuration() const;
  uint64_t ParseBlockedOnScriptLoadFromDocumentWriteDuration() const;
  uint64_t ParseBlockedOnScriptExecutionDuration() const;
  uint64_t ParseBlockedOnScriptExecutionFromDocumentWriteDuration() const;

  // The start time of the prerender activation navigation.
  std::optional<base::TimeDelta> PrerenderActivationStart() const;

  void Trace(Visitor*) const override;

  uint64_t MonotonicTimeToIntegerMilliseconds(base::TimeTicks) const;

  std::unique_ptr<TracedValue> GetNavigationTracingData();

 private:
  const DocumentTiming* GetDocumentTiming() const;
  const DocumentParserTiming* GetDocumentParserTiming() const;
  const PaintTiming* GetPaintTiming() const;
  PaintTimingDetector* GetPaintTimingDetector() const;
  DocumentLoader* GetDocumentLoader() const;
  DocumentLoadTiming* GetDocumentLoadTiming() const;
  InteractiveDetector* GetInteractiveDetector() const;
  std::optional<base::TimeDelta> MonotonicTimeToPseudoWallTime(
      const std::optional<base::TimeTicks>&) const;
  LargestContentfulPaintDetailsForReporting
  PopulateLargestContentfulPaintDetailsForReporting(
      const LargestContentfulPaintDetails& timing) const;

  bool cross_origin_isolated_capability_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_PERFORMANCE_TIMING_FOR_REPORTING_H_
