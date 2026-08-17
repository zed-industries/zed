// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_REPORTING_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_REPORTING_H_

#include <optional>
#include <vector>

#include "base/time/time.h"
#include "third_party/blink/public/common/performance/largest_contentful_paint_type.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/web/web_navigation_type.h"

namespace blink {

class WindowPerformance;

struct ResourceLoadTimingsForReporting {
  std::optional<base::TimeDelta> discovery_time = std::nullopt;
  std::optional<base::TimeDelta> load_start = std::nullopt;
  std::optional<base::TimeDelta> load_end = std::nullopt;
};

struct LargestContentfulPaintDetailsForReporting {
  double image_paint_time = 0;
  uint64_t image_paint_size = 0;
  ResourceLoadTimingsForReporting resource_load_timings = {};
  blink::LargestContentfulPaintType type =
      blink::LargestContentfulPaintType::kNone;
  double image_bpp = 0.0;
  double text_paint_time = 0;
  uint64_t text_paint_size = 0;
  base::TimeTicks paint_time = base::TimeTicks();
  std::optional<WebURLRequest::Priority> image_request_priority = std::nullopt;
  // The unclamped paint time of the largest content (image/text).
  std::optional<base::TimeTicks> merged_unclamped_paint_time = std::nullopt;
};

// This class is used for reporting purposes (e.g. ukm) of non-web-exposed
// metrics.
class BLINK_EXPORT WebPerformanceMetricsForReporting {
 public:
  // The count to record the times on requestAnimationFrame after the page is
  // restored from the back-forward cache.
  static constexpr int
      kRequestAnimationFramesToRecordAfterBackForwardCacheRestore = 3;

  struct BackForwardCacheRestoreTiming {
    double navigation_start = 0;
    double first_paint = 0;
    std::array<double,
               kRequestAnimationFramesToRecordAfterBackForwardCacheRestore>
        request_animation_frames = {};
    std::optional<base::TimeDelta> first_input_delay;
  };

  using BackForwardCacheRestoreTimings =
      std::vector<BackForwardCacheRestoreTiming>;

  ~WebPerformanceMetricsForReporting() { Reset(); }

  WebPerformanceMetricsForReporting() = default;

  WebPerformanceMetricsForReporting(
      const WebPerformanceMetricsForReporting& p) {
    Assign(p);
  }

  WebPerformanceMetricsForReporting& operator=(
      const WebPerformanceMetricsForReporting& p) {
    Assign(p);
    return *this;
  }

  void Reset();
  void Assign(const WebPerformanceMetricsForReporting&);

  // This only returns one of {Other|Reload|BackForward}.
  // Form submits and link clicks all fall under other.
  WebNavigationType GetNavigationType() const;

  // These functions return time in seconds (not milliseconds) since the epoch.
  //
  // TODO (crbug.com/355962211): Update the methods which return double for
  // timing information to return `base::TimeTicks`.
  double InputForNavigationStart() const;
  double NavigationStart() const;
  base::TimeTicks NavigationStartAsMonotonicTime() const;
  BackForwardCacheRestoreTimings BackForwardCacheRestore() const;
  double DomainLookupStart() const;
  double DomainLookupEnd() const;
  double ConnectStart() const;
  double ConnectEnd() const;
  double ResponseStart() const;
  double DomContentLoadedEventStart() const;
  double DomContentLoadedEventEnd() const;
  double LoadEventStart() const;
  double LoadEventEnd() const;
  double FirstPaint() const;
  double FirstImagePaint() const;
  double FirstContentfulPaint() const;
  base::TimeTicks FirstContentfulPaintAsMonotonicTime() const;
  base::TimeTicks FirstContentfulPaintRenderedButNotPresentedAsMonotonicTime()
      const;
  double FirstMeaningfulPaint() const;
  LargestContentfulPaintDetailsForReporting LargestContentfulDetailsForMetrics()
      const;
  LargestContentfulPaintDetailsForReporting
  SoftNavigationLargestContentfulDetailsForMetrics() const;
  double FirstEligibleToPaint() const;
  double FirstInputOrScrollNotifiedTimestamp() const;
  std::optional<base::TimeDelta> FirstInputDelay() const;
  std::optional<base::TimeDelta> FirstInputTimestamp() const;
  std::optional<base::TimeTicks> FirstInputTimestampAsMonotonicTime() const;
  std::optional<base::TimeDelta> LongestInputDelay() const;
  std::optional<base::TimeDelta> LongestInputTimestamp() const;
  std::optional<base::TimeDelta> FirstInputProcessingTime() const;
  std::optional<base::TimeDelta> FirstScrollDelay() const;
  std::optional<base::TimeDelta> FirstScrollTimestamp() const;
  double ParseStart() const;
  double ParseStop() const;
  double ParseBlockedOnScriptLoadDuration() const;
  double ParseBlockedOnScriptLoadFromDocumentWriteDuration() const;
  double ParseBlockedOnScriptExecutionDuration() const;
  double ParseBlockedOnScriptExecutionFromDocumentWriteDuration() const;
  std::optional<base::TimeDelta> PrerenderActivationStart() const;
  std::optional<base::TimeDelta> UserTimingMarkFullyLoaded() const;
  std::optional<base::TimeDelta> UserTimingMarkFullyVisible() const;
  std::optional<base::TimeDelta> UserTimingMarkInteractive() const;
  std::optional<std::tuple<std::string, base::TimeDelta>> CustomUserTimingMark()
      const;

#if INSIDE_BLINK
  explicit WebPerformanceMetricsForReporting(WindowPerformance*);
  WebPerformanceMetricsForReporting& operator=(WindowPerformance*);
#endif

 private:
  WebPrivatePtrForGC<WindowPerformance> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PERFORMANCE_METRICS_FOR_REPORTING_H_
