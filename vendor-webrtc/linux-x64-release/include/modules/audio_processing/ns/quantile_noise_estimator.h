/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_QUANTILE_NOISE_ESTIMATOR_H_
#define MODULES_AUDIO_PROCESSING_NS_QUANTILE_NOISE_ESTIMATOR_H_

#include <math.h>

#include <array>

#include "api/array_view.h"
#include "modules/audio_processing/ns/ns_common.h"

namespace webrtc {

constexpr int kSimult = 3;

// For quantile noise estimation.
class QuantileNoiseEstimator {
 public:
  QuantileNoiseEstimator();
  QuantileNoiseEstimator(const QuantileNoiseEstimator&) = delete;
  QuantileNoiseEstimator& operator=(const QuantileNoiseEstimator&) = delete;

  // Estimate noise.
  void Estimate(ArrayView<const float, kFftSizeBy2Plus1> signal_spectrum,
                ArrayView<float, kFftSizeBy2Plus1> noise_spectrum);

 private:
  std::array<float, kSimult * kFftSizeBy2Plus1> density_;
  std::array<float, kSimult * kFftSizeBy2Plus1> log_quantile_;
  std::array<float, kFftSizeBy2Plus1> quantile_;
  std::array<int, kSimult> counter_;
  int num_updates_ = 1;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_QUANTILE_NOISE_ESTIMATOR_H_
