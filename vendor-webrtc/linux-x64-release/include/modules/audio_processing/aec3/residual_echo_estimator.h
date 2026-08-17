/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_RESIDUAL_ECHO_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_RESIDUAL_ECHO_ESTIMATOR_H_

#include <array>
#include <memory>
#include <optional>

#include "api/audio/echo_canceller3_config.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/aec_state.h"
#include "modules/audio_processing/aec3/render_buffer.h"
#include "modules/audio_processing/aec3/reverb_model.h"
#include "modules/audio_processing/aec3/spectrum_buffer.h"
#include "rtc_base/checks.h"

namespace webrtc {

class ResidualEchoEstimator {
 public:
  ResidualEchoEstimator(const Environment& env,
                        const EchoCanceller3Config& config,
                        size_t num_render_channels);
  ~ResidualEchoEstimator();

  ResidualEchoEstimator(const ResidualEchoEstimator&) = delete;
  ResidualEchoEstimator& operator=(const ResidualEchoEstimator&) = delete;

  void Estimate(
      const AecState& aec_state,
      const RenderBuffer& render_buffer,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> S2_linear,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> Y2,
      bool dominant_nearend,
      ArrayView<std::array<float, kFftLengthBy2Plus1>> R2,
      ArrayView<std::array<float, kFftLengthBy2Plus1>> R2_unbounded);

 private:
  enum class ReverbType { kLinear, kNonLinear };

  // Resets the state.
  void Reset();

  // Updates estimate for the power of the stationary noise component in the
  // render signal.
  void UpdateRenderNoisePower(const RenderBuffer& render_buffer);

  // Updates the reverb estimation.
  void UpdateReverb(ReverbType reverb_type,
                    const AecState& aec_state,
                    const RenderBuffer& render_buffer,
                    bool dominant_nearend);

  // Adds the estimated unmodelled echo power to the residual echo power
  // estimate.
  void AddReverb(ArrayView<std::array<float, kFftLengthBy2Plus1>> R2) const;

  // Gets the echo path gain to apply.
  float GetEchoPathGain(const AecState& aec_state,
                        bool gain_for_early_reflections) const;

  const EchoCanceller3Config config_;
  const size_t num_render_channels_;
  const float early_reflections_transparent_mode_gain_;
  const float late_reflections_transparent_mode_gain_;
  const float early_reflections_general_gain_;
  const float late_reflections_general_gain_;
  const bool erle_onset_compensation_in_dominant_nearend_;
  std::array<float, kFftLengthBy2Plus1> X2_noise_floor_;
  std::array<int, kFftLengthBy2Plus1> X2_noise_floor_counter_;
  ReverbModel echo_reverb_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_RESIDUAL_ECHO_ESTIMATOR_H_
