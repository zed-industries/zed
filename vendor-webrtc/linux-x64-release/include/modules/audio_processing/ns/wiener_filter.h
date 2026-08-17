/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_WIENER_FILTER_H_
#define MODULES_AUDIO_PROCESSING_NS_WIENER_FILTER_H_

#include <array>

#include "api/array_view.h"
#include "modules/audio_processing/ns/ns_common.h"
#include "modules/audio_processing/ns/suppression_params.h"

namespace webrtc {

// Estimates a Wiener-filter based frequency domain noise reduction filter.
class WienerFilter {
 public:
  explicit WienerFilter(const SuppressionParams& suppression_params);
  WienerFilter(const WienerFilter&) = delete;
  WienerFilter& operator=(const WienerFilter&) = delete;

  // Updates the filter estimate.
  void Update(
      int32_t num_analyzed_frames,
      ArrayView<const float, kFftSizeBy2Plus1> noise_spectrum,
      ArrayView<const float, kFftSizeBy2Plus1> prev_noise_spectrum,
      ArrayView<const float, kFftSizeBy2Plus1> parametric_noise_spectrum,
      ArrayView<const float, kFftSizeBy2Plus1> signal_spectrum);

  // Compute an overall gain scaling factor.
  float ComputeOverallScalingFactor(int32_t num_analyzed_frames,
                                    float prior_speech_probability,
                                    float energy_before_filtering,
                                    float energy_after_filtering) const;

  // Returns the filter.
  ArrayView<const float, kFftSizeBy2Plus1> get_filter() const {
    return filter_;
  }

 private:
  const SuppressionParams& suppression_params_;
  std::array<float, kFftSizeBy2Plus1> spectrum_prev_process_;
  std::array<float, kFftSizeBy2Plus1> initial_spectral_estimate_;
  std::array<float, kFftSizeBy2Plus1> filter_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_WIENER_FILTER_H_
