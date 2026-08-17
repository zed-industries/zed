// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DESTINATION_UMA_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DESTINATION_UMA_REPORTER_H_

#include <string>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_audio_latency_hint.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Calculate and report AudioDestination-related metric. Some metrics are
// updated at every audio callback, while others are accumulated over miultiple
// callbacks.
class PLATFORM_EXPORT AudioDestinationUmaReporter final {
 public:
  explicit AudioDestinationUmaReporter(const WebAudioLatencyHint&,
                                       int callback_buffer_size,
                                       float sample_rate);
  ~AudioDestinationUmaReporter();

  // These methods are not thread-safe and must be called within
  // `AudioDestination::RequestRender()` method for correct instrumentation.
  void UpdateFifoDelay(base::TimeDelta fifo_delay);
  void UpdateTotalPlayoutDelay(base::TimeDelta total_playout_delay);
  void IncreaseFifoUnderrunCount();
  void UpdateMetricNameForDualThreadMode();
  void Report();
  void AddRenderDuration(base::TimeDelta duration) {
    render_total_duration_ += duration;
  }
  void AddRequestRenderDuration(base::TimeDelta duration) {
    request_render_total_duration_ += duration;
  }
  void AddRequestRenderGapDuration(base::TimeDelta duration) {
    request_render_gap_total_duration_ += duration;
  }

  // The number of callbacks after which metrics are reported and reset.
  static constexpr int kMetricsReportCycle = 1000;
  static constexpr std::string_view kFifoDelayHistogramNameBase = "FIFODelay";
  static constexpr std::string_view kFifoUnderrunHistogramNameBase =
      "FIFOUnderrunCount";
  static constexpr std::string_view kTotalPlayoutDelayHistogramNameBase =
      "TotalPlayoutDelay";
  static constexpr std::string_view kRenderTimeRatioHistogramNameBase =
      "RenderTimeRatio";
  static constexpr std::string_view kRequestRenderTimeRatioHistogramNameBase =
      "RequestRenderTimeRatio";
  static constexpr std::string_view
      kRequestRenderGapTimeRatioHistogramNameBase = "RequestRenderGapTimeRatio";

 private:
  // Calculates the percentage of `delta` relative to the expected callback
  // interval, scaled by kMetricsReportCycle for reporting.
  // Returns the percentage (0-100).
  int PercentOfCallbackInterval(base::TimeDelta duration);

  // Indicates what period samples are aggregated over. kShort means entire
  // streams of less than 1000 callbacks, kIntervals means exactly 1000
  // callbacks.
  enum class SamplingPeriod { kShort, kIntervals };
  int callback_count_ = 0;
  int fifo_underrun_count_ = 0;
  const WebAudioLatencyHint latency_hint_;
  bool use_audio_worklet_ = false;

  // The audio delay (ms) computed the number of available frames of the
  // PushPUllFIFO in AudioDestination. Measured and reported at every audio
  // callback.
  base::TimeDelta fifo_delay_;

  // The audio delay (ms) covers the whole pipeline from the WebAudio graph to
  // the speaker. Measured and reported at every audio callback.
  base::TimeDelta total_playout_delay_;

  // Histogram names for metrics reported by `AudioDestinationUmaReporter`.
  // These names are constructed during initialization (or updated in
  // `UpdateMetricNameForDualThreadMode`) to avoid string manipulation during
  // the reporting phase.  Each metric has a base name and a variant with a
  // latency tag appended.
  std::string fifo_delay_histogram_name_;
  std::string fifo_delay_histogram_name_with_latency_tag_;
  std::string fifo_underrun_histogram_name_;
  std::string fifo_underrun_histogram_name_with_latency_tag_;
  std::string total_playout_delay_histogram_name_;
  std::string total_playout_delay_histogram_name_with_latency_tag_;
  std::string render_time_ratio_histogram_name_;
  std::string render_time_ratio_histogram_name_with_latency_tag_;
  std::string request_render_time_ratio_histogram_name_;
  std::string request_render_time_ratio_histogram_name_with_latency_tag_;
  std::string request_render_gap_time_ratio_histogram_name_;
  std::string request_render_gap_time_ratio_histogram_name_with_latency_tag_;

  // Indicates that the current audio stream is less than 1000 callbacks.
  bool is_stream_short_ = true;

  // Expected time between audio callbacks, calculated from the hardware buffer
  // size and the sampling rate.
  base::TimeDelta expected_callback_interval_;

  // Total duration spent in `AudioDestination::Render`, accumulated and
  // reported every `kMetricsReportCycle` callbacks.  Reset after reporting.
  base::TimeDelta render_total_duration_;
  // Total duration spent in `AudioDestination::RequestRender`, accumulated and
  // reported every `kMetricsReportCycle` callbacks. Reset after reporting.
  base::TimeDelta request_render_total_duration_;
  // Total duration elapsed between when `AudioDestination::RequestRender` is
  // requested and when it starts, accumulated and reported every
  // `kMetricsReportCycle` callbacks. Reset after reporting.
  base::TimeDelta request_render_gap_total_duration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_DESTINATION_UMA_REPORTER_H_

