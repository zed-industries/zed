/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_NS_HISTOGRAMS_H_
#define MODULES_AUDIO_PROCESSING_NS_HISTOGRAMS_H_

#include <array>

#include "api/array_view.h"
#include "modules/audio_processing/ns/ns_common.h"
#include "modules/audio_processing/ns/signal_model.h"

namespace webrtc {

constexpr int kHistogramSize = 1000;

// Class for handling the updating of histograms.
class Histograms {
 public:
  Histograms();
  Histograms(const Histograms&) = delete;
  Histograms& operator=(const Histograms&) = delete;

  // Clears the histograms.
  void Clear();

  // Extracts thresholds for feature parameters and updates the corresponding
  // histogram.
  void Update(const SignalModel& features_);

  // Methods for accessing the histograms.
  ArrayView<const int, kHistogramSize> get_lrt() const { return lrt_; }
  ArrayView<const int, kHistogramSize> get_spectral_flatness() const {
    return spectral_flatness_;
  }
  ArrayView<const int, kHistogramSize> get_spectral_diff() const {
    return spectral_diff_;
  }

 private:
  std::array<int, kHistogramSize> lrt_;
  std::array<int, kHistogramSize> spectral_flatness_;
  std::array<int, kHistogramSize> spectral_diff_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_NS_HISTOGRAMS_H_
