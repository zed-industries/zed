/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_REVERB_MODEL_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_REVERB_MODEL_ESTIMATOR_H_

#include <array>
#include <memory>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "api/audio/echo_canceller3_config.h"
#include "modules/audio_processing/aec3/aec3_common.h"  // kFftLengthBy2Plus1
#include "modules/audio_processing/aec3/reverb_decay_estimator.h"
#include "modules/audio_processing/aec3/reverb_frequency_response.h"

namespace webrtc {

class ApmDataDumper;

// Class for estimating the model parameters for the reverberant echo.
class ReverbModelEstimator {
 public:
  ReverbModelEstimator(const EchoCanceller3Config& config,
                       size_t num_capture_channels);
  ~ReverbModelEstimator();

  // Updates the estimates based on new data.
  void Update(
      ArrayView<const std::vector<float>> impulse_responses,
      ArrayView<const std::vector<std::array<float, kFftLengthBy2Plus1>>>
          frequency_responses,
      ArrayView<const std::optional<float>> linear_filter_qualities,
      ArrayView<const int> filter_delays_blocks,
      const std::vector<bool>& usable_linear_estimates,
      bool stationary_block);

  // Returns the exponential decay of the reverberant echo. The parameter `mild`
  // indicates which exponential decay to return, the default one or a milder
  // one.
  // TODO(peah): Correct to properly support multiple channels.
  float ReverbDecay(bool mild) const {
    return reverb_decay_estimators_[0]->Decay(mild);
  }

  // Return the frequency response of the reverberant echo.
  // TODO(peah): Correct to properly support multiple channels.
  ArrayView<const float> GetReverbFrequencyResponse() const {
    return reverb_frequency_responses_[0].FrequencyResponse();
  }

  // Dumps debug data.
  void Dump(ApmDataDumper* data_dumper) const {
    reverb_decay_estimators_[0]->Dump(data_dumper);
  }

 private:
  std::vector<std::unique_ptr<ReverbDecayEstimator>> reverb_decay_estimators_;
  std::vector<ReverbFrequencyResponse> reverb_frequency_responses_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_REVERB_MODEL_ESTIMATOR_H_
