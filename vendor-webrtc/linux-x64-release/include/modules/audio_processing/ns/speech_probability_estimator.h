/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_SPEECH_PROBABILITY_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_NS_SPEECH_PROBABILITY_ESTIMATOR_H_

#include <array>

#include "api/array_view.h"
#include "modules/audio_processing/ns/ns_common.h"
#include "modules/audio_processing/ns/signal_model_estimator.h"

namespace webrtc {

// Class for estimating the probability of speech.
class SpeechProbabilityEstimator {
 public:
  SpeechProbabilityEstimator();
  SpeechProbabilityEstimator(const SpeechProbabilityEstimator&) = delete;
  SpeechProbabilityEstimator& operator=(const SpeechProbabilityEstimator&) =
      delete;

  // Compute speech probability.
  void Update(
      int32_t num_analyzed_frames,
      ArrayView<const float, kFftSizeBy2Plus1> prior_snr,
      ArrayView<const float, kFftSizeBy2Plus1> post_snr,
      ArrayView<const float, kFftSizeBy2Plus1> conservative_noise_spectrum,
      ArrayView<const float, kFftSizeBy2Plus1> signal_spectrum,
      float signal_spectral_sum,
      float signal_energy);

  float get_prior_probability() const { return prior_speech_prob_; }
  ArrayView<const float> get_probability() { return speech_probability_; }

 private:
  SignalModelEstimator signal_model_estimator_;
  float prior_speech_prob_ = .5f;
  std::array<float, kFftSizeBy2Plus1> speech_probability_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_SPEECH_PROBABILITY_ESTIMATOR_H_
