/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_NOISE_LEVEL_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AGC2_NOISE_LEVEL_ESTIMATOR_H_

#include <memory>

#include "api/audio/audio_view.h"

namespace webrtc {
class ApmDataDumper;

// Noise level estimator interface.
class NoiseLevelEstimator {
 public:
  virtual ~NoiseLevelEstimator() = default;
  // Analyzes a 10 ms `frame`, updates the noise level estimation and returns
  // the value for the latter in dBFS.
  virtual float Analyze(DeinterleavedView<const float> frame) = 0;
};

// Creates a noise level estimator based on noise floor detection.
std::unique_ptr<NoiseLevelEstimator> CreateNoiseFloorEstimator(
    ApmDataDumper* data_dumper);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_NOISE_LEVEL_ESTIMATOR_H_
