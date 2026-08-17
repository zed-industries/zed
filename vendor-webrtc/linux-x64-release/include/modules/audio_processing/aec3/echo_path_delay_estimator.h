/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_DELAY_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_DELAY_ESTIMATOR_H_

#include <stddef.h>

#include <optional>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/alignment_mixer.h"
#include "modules/audio_processing/aec3/block.h"
#include "modules/audio_processing/aec3/clockdrift_detector.h"
#include "modules/audio_processing/aec3/decimator.h"
#include "modules/audio_processing/aec3/delay_estimate.h"
#include "modules/audio_processing/aec3/matched_filter.h"
#include "modules/audio_processing/aec3/matched_filter_lag_aggregator.h"

namespace webrtc {

class ApmDataDumper;
struct DownsampledRenderBuffer;
struct EchoCanceller3Config;

// Estimates the delay of the echo path.
class EchoPathDelayEstimator {
 public:
  EchoPathDelayEstimator(ApmDataDumper* data_dumper,
                         const EchoCanceller3Config& config,
                         size_t num_capture_channels);
  ~EchoPathDelayEstimator();

  EchoPathDelayEstimator(const EchoPathDelayEstimator&) = delete;
  EchoPathDelayEstimator& operator=(const EchoPathDelayEstimator&) = delete;

  // Resets the estimation. If the delay confidence is reset, the reset behavior
  // is as if the call is restarted.
  void Reset(bool reset_delay_confidence);

  // Produce a delay estimate if such is avaliable.
  std::optional<DelayEstimate> EstimateDelay(
      const DownsampledRenderBuffer& render_buffer,
      const Block& capture);

  // Log delay estimator properties.
  void LogDelayEstimationProperties(int sample_rate_hz, size_t shift) const {
    matched_filter_.LogFilterProperties(sample_rate_hz, shift,
                                        down_sampling_factor_);
  }

  // Returns the level of detected clockdrift.
  ClockdriftDetector::Level Clockdrift() const {
    return clockdrift_detector_.ClockdriftLevel();
  }

 private:
  ApmDataDumper* const data_dumper_;
  const size_t down_sampling_factor_;
  const size_t sub_block_size_;
  AlignmentMixer capture_mixer_;
  Decimator capture_decimator_;
  MatchedFilter matched_filter_;
  MatchedFilterLagAggregator matched_filter_lag_aggregator_;
  std::optional<DelayEstimate> old_aggregated_lag_;
  size_t consistent_estimate_counter_ = 0;
  ClockdriftDetector clockdrift_detector_;

  // Internal reset method with more granularity.
  void Reset(bool reset_lag_aggregator, bool reset_delay_confidence);
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ECHO_PATH_DELAY_ESTIMATOR_H_
