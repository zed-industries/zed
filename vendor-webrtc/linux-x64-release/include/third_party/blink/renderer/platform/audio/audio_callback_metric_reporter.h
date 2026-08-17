// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_CALLBACK_METRIC_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_CALLBACK_METRIC_REPORTER_H_

#include "base/time/time.h"

namespace blink {

// A data storage for callback/render related metric. WebAudio DevTool consumes
// this data eventually.
struct AudioCallbackMetric {
  // The total number of callback so far.
  int64_t number_of_callbacks = 0;

  // A value represents the current capacity of WebAudio renderer.
  // = (time spent on actual graph rendering) / (platform callback interval)`
  double render_capacity = 0.0;

  // Expected time (in sec) between callbacks, derived from the platform
  // callback buffer size and the context sample rate.
  double expected_callback_interval = 0.0;

  // A running mean of callback interval. This value should be close to
  // |expected_callback_interval| on a system with accurate callback timer.
  double mean_callback_interval = 0.0;

  // A running variance of callback interval. This value should be
  // close to zero.
  double variance_callback_interval = 0.0;
};

class AudioCallbackMetricReporter final {
 public:
  AudioCallbackMetricReporter() = default;

  // Must be called after WebAudioDevice is created, because we need the
  // platform parameters like callback buffer size and hardware sample rate.
  void Initialize(int callback_buffer_size, float sample_rate);

  // Gets called when the callback function starts. Calculates the metric for
  // the previous callback right after obtaining the timestamp of current
  // callback.
  void BeginTrace();

  // Gets called just before the callback function ends. Stores data for the
  // current callback.
  void EndTrace();

  const AudioCallbackMetric& GetMetric() const { return metric_; }

 private:
  void UpdateMetric();

  // The start time of current callback function.
  base::TimeTicks callback_start_time_;

  // The end time of previous callback function. The callback interval can be
  // measured by |callback_start_time_| - |previous_callback_start_time_|.
  base::TimeTicks previous_callback_start_time_;

  // The end time of previous render quantum. The previous render duration can
  // be calculated by
  // |previous_callback_start_time_| - |previous_render_end_time|.
  base::TimeTicks previous_render_end_time_;

  double callback_interval_ = 0.0;
  double render_duration_ = 0.0;

  // Time constant for smoothing the mean and variance metrics. Roughly, five
  // times this value will be the memory of the metrics.
  double time_constant_ = 1.0;

  // Filter coefficient for weighted mean and variance. Derived from
  // |time_constant_|.
  double alpha_ = 0.0;

  AudioCallbackMetric metric_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_AUDIO_CALLBACK_METRIC_REPORTER_H_
