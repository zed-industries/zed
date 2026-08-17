/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_RENDER_DELAY_CONTROLLER_METRICS_H_
#define MODULES_AUDIO_PROCESSING_AEC3_RENDER_DELAY_CONTROLLER_METRICS_H_

#include <stddef.h>

#include <optional>

#include "modules/audio_processing/aec3/clockdrift_detector.h"

namespace webrtc {

// Handles the reporting of metrics for the render delay controller.
class RenderDelayControllerMetrics {
 public:
  RenderDelayControllerMetrics();

  RenderDelayControllerMetrics(const RenderDelayControllerMetrics&) = delete;
  RenderDelayControllerMetrics& operator=(const RenderDelayControllerMetrics&) =
      delete;

  // Updates the metric with new data.
  void Update(std::optional<size_t> delay_samples,
              std::optional<size_t> buffer_delay_blocks,
              ClockdriftDetector::Level clockdrift);

 private:
  // Resets the metrics.
  void ResetMetrics();

  size_t delay_blocks_ = 0;
  int reliable_delay_estimate_counter_ = 0;
  int delay_change_counter_ = 0;
  int call_counter_ = 0;
  int initial_call_counter_ = 0;
  bool initial_update = true;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_RENDER_DELAY_CONTROLLER_METRICS_H_
