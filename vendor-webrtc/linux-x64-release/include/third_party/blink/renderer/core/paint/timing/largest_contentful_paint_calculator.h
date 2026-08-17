// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LARGEST_CONTENTFUL_PAINT_CALCULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LARGEST_CONTENTFUL_PAINT_CALCULATOR_H_

#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/paint/timing/lcp_objects.h"
#include "third_party/blink/renderer/core/timing/window_performance.h"

namespace blink {

// LargestContentfulPaintCalculator is responsible for tracking the largest
// image paint and the largest text paint and notifying WindowPerformance
// whenever a new LatestLargestContentfulPaint entry should be dispatched.
class CORE_EXPORT LargestContentfulPaintCalculator final
    : public GarbageCollected<LargestContentfulPaintCalculator> {
 public:
  explicit LargestContentfulPaintCalculator(WindowPerformance*);
  LargestContentfulPaintCalculator(const LargestContentfulPaintCalculator&) =
      delete;
  LargestContentfulPaintCalculator& operator=(
      const LargestContentfulPaintCalculator&) = delete;

  void UpdateWebExposedLargestContentfulPaintIfNeeded(
      const TextRecord* largest_text,
      const ImageRecord* largest_image,
      bool is_triggered_by_soft_navigation);

  bool HasLargestImagePaintChangedForMetrics(
      base::TimeTicks largest_image_paint_time,
      uint64_t largest_image_paint_size) const;

  bool HasLargestTextPaintChangedForMetrics(
      base::TimeTicks largest_text_paint_time,
      uint64_t largest_text_paint_size) const;

  bool NotifyMetricsIfLargestImagePaintChanged(
      base::TimeTicks image_paint_time,
      uint64_t image_paint_size,
      ImageRecord* image_record,
      double image_bpp,
      std::optional<WebURLRequest::Priority> priority);

  bool NotifyMetricsIfLargestTextPaintChanged(base::TimeTicks text_paint_time,
                                              uint64_t text_paint_size);

  void UpdateLatestLcpDetails();

  const LargestContentfulPaintDetails& LatestLcpDetails() const {
    return latest_lcp_details_;
  }

  void ResetMetricsLcp() {
    latest_lcp_details_ = LargestContentfulPaintDetails();
  }

  void Trace(Visitor* visitor) const;

 private:
  friend class LargestContentfulPaintCalculatorTest;

  void UpdateWebExposedLargestContentfulImage(
      const ImageRecord* largest_image,
      bool is_triggered_by_soft_navigation);
  void UpdateWebExposedLargestContentfulText(
      const TextRecord& largest_text,
      bool is_triggered_by_soft_navigation);

  std::unique_ptr<TracedValue> TextCandidateTraceData(
      const TextRecord& largest_text,
      bool is_triggered_by_soft_navigation);
  std::unique_ptr<TracedValue> ImageCandidateTraceData(
      const ImageRecord* largest_image,
      bool is_triggered_by_soft_navigation,
      Element* image_element);

  Member<WindowPerformance> window_performance_;

  uint64_t largest_reported_size_ = 0u;
  double largest_image_bpp_ = 0.0;
  unsigned count_candidates_ = 0;

  // The |latest_lcp_details_| struct is just for internal accounting purposes
  // and is not reported anywhere (neither to metrics, nor to the web exposed
  // API).
  LargestContentfulPaintDetails latest_lcp_details_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_LARGEST_CONTENTFUL_PAINT_CALCULATOR_H_
