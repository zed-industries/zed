/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_METRICS_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_METRICS_H_

#include <array>

#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/aec_state.h"

namespace webrtc {

// Handles the reporting of metrics for the echo remover.
class EchoRemoverMetrics {
 public:
  struct DbMetric {
    DbMetric();
    DbMetric(float sum_value, float floor_value, float ceil_value);
    void Update(float value);
    void UpdateInstant(float value);
    float sum_value;
    float floor_value;
    float ceil_value;
  };

  EchoRemoverMetrics();

  EchoRemoverMetrics(const EchoRemoverMetrics&) = delete;
  EchoRemoverMetrics& operator=(const EchoRemoverMetrics&) = delete;

  // Updates the metric with new data.
  void Update(
      const AecState& aec_state,
      const std::array<float, kFftLengthBy2Plus1>& comfort_noise_spectrum,
      const std::array<float, kFftLengthBy2Plus1>& suppressor_gain);

  // Returns true if the metrics have just been reported, otherwise false.
  bool MetricsReported() { return metrics_reported_; }

 private:
  // Resets the metrics.
  void ResetMetrics();

  int block_counter_ = 0;
  DbMetric erl_time_domain_;
  DbMetric erle_time_domain_;
  bool saturated_capture_ = false;
  bool metrics_reported_ = false;
};

namespace aec3 {

// Updates a banded metric of type DbMetric with the values in the supplied
// array.
void UpdateDbMetric(const std::array<float, kFftLengthBy2Plus1>& value,
                    std::array<EchoRemoverMetrics::DbMetric, 2>* statistic);

// Transforms a DbMetric from the linear domain into the logarithmic domain.
int TransformDbMetricForReporting(bool negate,
                                  float min_value,
                                  float max_value,
                                  float offset,
                                  float scaling,
                                  float value);

}  // namespace aec3

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_METRICS_H_
