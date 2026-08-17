/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_COMFORT_NOISE_GENERATOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_COMFORT_NOISE_GENERATOR_H_

#include <stdint.h>

#include <array>
#include <memory>

#include "modules/audio_processing/aec3/aec3_common.h"
#include "modules/audio_processing/aec3/aec_state.h"
#include "modules/audio_processing/aec3/fft_data.h"
#include "rtc_base/system/arch.h"

namespace webrtc {
namespace aec3 {
#if defined(WEBRTC_ARCH_X86_FAMILY)

void EstimateComfortNoise_SSE2(const std::array<float, kFftLengthBy2Plus1>& N2,
                               uint32_t* seed,
                               FftData* lower_band_noise,
                               FftData* upper_band_noise);
#endif
void EstimateComfortNoise(const std::array<float, kFftLengthBy2Plus1>& N2,
                          uint32_t* seed,
                          FftData* lower_band_noise,
                          FftData* upper_band_noise);

}  // namespace aec3

// Generates the comfort noise.
class ComfortNoiseGenerator {
 public:
  ComfortNoiseGenerator(const EchoCanceller3Config& config,
                        Aec3Optimization optimization,
                        size_t num_capture_channels);
  ComfortNoiseGenerator() = delete;
  ~ComfortNoiseGenerator();
  ComfortNoiseGenerator(const ComfortNoiseGenerator&) = delete;

  // Computes the comfort noise.
  void Compute(
      bool saturated_capture,
      ArrayView<const std::array<float, kFftLengthBy2Plus1>> capture_spectrum,
      ArrayView<FftData> lower_band_noise,
      ArrayView<FftData> upper_band_noise);

  // Returns the estimate of the background noise spectrum.
  ArrayView<const std::array<float, kFftLengthBy2Plus1>> NoiseSpectrum() const {
    return N2_;
  }

 private:
  const Aec3Optimization optimization_;
  uint32_t seed_;
  const size_t num_capture_channels_;
  const float noise_floor_;
  std::unique_ptr<std::vector<std::array<float, kFftLengthBy2Plus1>>>
      N2_initial_;
  std::vector<std::array<float, kFftLengthBy2Plus1>> Y2_smoothed_;
  std::vector<std::array<float, kFftLengthBy2Plus1>> N2_;
  int N2_counter_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_COMFORT_NOISE_GENERATOR_H_
