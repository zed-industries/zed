/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_REVERB_FREQUENCY_RESPONSE_H_
#define MODULES_AUDIO_PROCESSING_AEC3_REVERB_FREQUENCY_RESPONSE_H_

#include <array>
#include <optional>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"

namespace webrtc {

// Class for updating the frequency response for the reverb.
class ReverbFrequencyResponse {
 public:
  explicit ReverbFrequencyResponse(
      bool use_conservative_tail_frequency_response);
  ~ReverbFrequencyResponse();

  // Updates the frequency response estimate of the reverb.
  void Update(const std::vector<std::array<float, kFftLengthBy2Plus1>>&
                  frequency_response,
              int filter_delay_blocks,
              const std::optional<float>& linear_filter_quality,
              bool stationary_block);

  // Returns the estimated frequency response for the reverb.
  ArrayView<const float> FrequencyResponse() const { return tail_response_; }

 private:
  void Update(const std::vector<std::array<float, kFftLengthBy2Plus1>>&
                  frequency_response,
              int filter_delay_blocks,
              float linear_filter_quality);

  const bool use_conservative_tail_frequency_response_;
  float average_decay_ = 0.f;
  std::array<float, kFftLengthBy2Plus1> tail_response_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_REVERB_FREQUENCY_RESPONSE_H_
