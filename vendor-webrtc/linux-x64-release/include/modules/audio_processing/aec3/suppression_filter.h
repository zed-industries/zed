/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_SUPPRESSION_FILTER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_SUPPRESSION_FILTER_H_

#include <array>
#include <vector>

#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/aec3_fft.h"
#include "modules/audio_processing/aec3/block.h"
#include "modules/audio_processing/aec3/fft_data.h"

namespace webrtc {

class SuppressionFilter {
 public:
  SuppressionFilter(Aec3Optimization optimization,
                    int sample_rate_hz,
                    size_t num_capture_channels_);
  ~SuppressionFilter();

  SuppressionFilter(const SuppressionFilter&) = delete;
  SuppressionFilter& operator=(const SuppressionFilter&) = delete;

  void ApplyGain(ArrayView<const FftData> comfort_noise,
                 ArrayView<const FftData> comfort_noise_high_bands,
                 const std::array<float, kFftLengthBy2Plus1>& suppression_gain,
                 float high_bands_gain,
                 ArrayView<const FftData> E_lowest_band,
                 Block* e);

 private:
  const Aec3Optimization optimization_;
  const int sample_rate_hz_;
  const size_t num_capture_channels_;
  const Aec3Fft fft_;
  std::vector<std::vector<std::array<float, kFftLengthBy2>>> e_output_old_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_SUPPRESSION_FILTER_H_
