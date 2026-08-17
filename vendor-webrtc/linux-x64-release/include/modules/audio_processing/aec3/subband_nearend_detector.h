/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_NEAREND_DETECTOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_NEAREND_DETECTOR_H_

#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/moving_average.h"
#include "modules/audio_processing/aec3/nearend_detector.h"

namespace webrtc {
// Class for selecting whether the suppressor is in the nearend or echo state.
class SubbandNearendDetector : public NearendDetector {
 public:
  SubbandNearendDetector(
      const EchoCanceller3Config::Suppressor::SubbandNearendDetection& config,
      size_t num_capture_channels);

  // Returns whether the current state is the nearend state.
  bool IsNearendState() const override { return nearend_state_; }

  // Updates the state selection based on latest spectral estimates.
  void Update(
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> nearend_spectrum,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>>
          residual_echo_spectrum,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>>
          comfort_noise_spectrum,
      bool initial_state) override;

 private:
  const EchoCanceller3Config::Suppressor::SubbandNearendDetection config_;
  const size_t num_capture_channels_;
  std::vector<aec3::MovingAverage> nearend_smoothers_;
  const float one_over_subband_length1_;
  const float one_over_subband_length2_;
  bool nearend_state_ = false;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_SUBBAND_NEAREND_DETECTOR_H_
