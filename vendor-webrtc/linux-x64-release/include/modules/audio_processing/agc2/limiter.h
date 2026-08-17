/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_LIMITER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_LIMITER_H_

#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "modules/audio_processing/agc2/fixed_digital_level_estimator.h"
#include "modules/audio_processing/agc2/interpolated_gain_curve.h"
#include "modules/audio_processing/include/audio_frame_view.h"

namespace webrtc {
class ApmDataDumper;

class Limiter {
 public:
  // See `SetSamplesPerChannel()` for valid values for `samples_per_channel`.
  Limiter(ApmDataDumper* apm_data_dumper,
          size_t samples_per_channel,
          absl::string_view histogram_name_prefix);

  Limiter(const Limiter& limiter) = delete;
  Limiter& operator=(const Limiter& limiter) = delete;
  ~Limiter();

  // Applies limiter and hard-clipping to `signal`.
  void Process(DeinterleavedView<float> signal);

  InterpolatedGainCurve::Stats GetGainCurveStats() const;

  // Supported values must be
  // * Supported by FixedDigitalLevelEstimator
  // * Below or equal to kMaximalNumberOfSamplesPerChannel so that samples
  //   fit in the per_sample_scaling_factors_ array.
  void SetSamplesPerChannel(size_t samples_per_channel);

  // Resets the internal state.
  void Reset();

  float LastAudioLevel() const;

 private:
  const InterpolatedGainCurve interp_gain_curve_;
  FixedDigitalLevelEstimator level_estimator_;
  ApmDataDumper* const apm_data_dumper_ = nullptr;

  // Work array containing the sub-frame scaling factors to be interpolated.
  std::array<float, kSubFramesInFrame + 1> scaling_factors_ = {};
  std::array<float, kMaximalNumberOfSamplesPerChannel>
      per_sample_scaling_factors_ = {};
  float last_scaling_factor_ = 1.f;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_LIMITER_H_
