/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_NOISE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_NS_NOISE_ESTIMATOR_H_

#include <array>

#include "api/array_view.h"
#include "modules/audio_processing/ns/ns_common.h"
#include "modules/audio_processing/ns/quantile_noise_estimator.h"
#include "modules/audio_processing/ns/suppression_params.h"

namespace webrtc {

// Class for estimating the spectral characteristics of the noise in an incoming
// signal.
class NoiseEstimator {
 public:
  explicit NoiseEstimator(const SuppressionParams& suppression_params);

  // Prepare the estimator for analysis of a new frame.
  void PrepareAnalysis();

  // Performs the first step of the estimator update.
  void PreUpdate(int32_t num_analyzed_frames,
                 ArrayView<const float, kFftSizeBy2Plus1> signal_spectrum,
                 float signal_spectral_sum);

  // Performs the second step of the estimator update.
  void PostUpdate(ArrayView<const float> speech_probability,
                  ArrayView<const float, kFftSizeBy2Plus1> signal_spectrum);

  // Returns the noise spectral estimate.
  ArrayView<const float, kFftSizeBy2Plus1> get_noise_spectrum() const {
    return noise_spectrum_;
  }

  // Returns the noise from the previous frame.
  ArrayView<const float, kFftSizeBy2Plus1> get_prev_noise_spectrum() const {
    return prev_noise_spectrum_;
  }

  // Returns a noise spectral estimate based on white and pink noise parameters.
  ArrayView<const float, kFftSizeBy2Plus1> get_parametric_noise_spectrum()
      const {
    return parametric_noise_spectrum_;
  }
  ArrayView<const float, kFftSizeBy2Plus1> get_conservative_noise_spectrum()
      const {
    return conservative_noise_spectrum_;
  }

 private:
  const SuppressionParams& suppression_params_;
  float white_noise_level_ = 0.f;
  float pink_noise_numerator_ = 0.f;
  float pink_noise_exp_ = 0.f;
  std::array<float, kFftSizeBy2Plus1> prev_noise_spectrum_;
  std::array<float, kFftSizeBy2Plus1> conservative_noise_spectrum_;
  std::array<float, kFftSizeBy2Plus1> parametric_noise_spectrum_;
  std::array<float, kFftSizeBy2Plus1> noise_spectrum_;
  QuantileNoiseEstimator quantile_noise_estimator_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_NOISE_ESTIMATOR_H_
