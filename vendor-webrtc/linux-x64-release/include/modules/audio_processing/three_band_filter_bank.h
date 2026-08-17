/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_THREE_BAND_FILTER_BANK_H_
#define MODULES_AUDIO_PROCESSING_THREE_BAND_FILTER_BANK_H_

#include <array>
#include <cstring>
#include <memory>
#include <vector>

#include "api/array_view.h"

namespace webrtc {

constexpr int kSparsity = 4;
constexpr int kStrideLog2 = 2;
constexpr int kStride = 1 << kStrideLog2;
constexpr int kNumZeroFilters = 2;
constexpr int kFilterSize = 4;
constexpr int kMemorySize = kFilterSize * kStride - 1;
static_assert(kMemorySize == 15,
              "The memory size must be sufficient to provide memory for the "
              "shifted filters");

// An implementation of a 3-band FIR filter-bank with DCT modulation, similar to
// the proposed in "Multirate Signal Processing for Communication Systems" by
// Fredric J Harris.
// The low-pass filter prototype has these characteristics:
// * Pass-band ripple = 0.3dB
// * Pass-band frequency = 0.147 (7kHz at 48kHz)
// * Stop-band attenuation = 40dB
// * Stop-band frequency = 0.192 (9.2kHz at 48kHz)
// * Delay = 24 samples (500us at 48kHz)
// * Linear phase
// This filter bank does not satisfy perfect reconstruction. The SNR after
// analysis and synthesis (with no processing in between) is approximately 9.5dB
// depending on the input signal after compensating for the delay.
class ThreeBandFilterBank final {
 public:
  static const int kNumBands = 3;
  static const int kFullBandSize = 480;
  static const int kSplitBandSize =
      ThreeBandFilterBank::kFullBandSize / ThreeBandFilterBank::kNumBands;
  static const int kNumNonZeroFilters =
      kSparsity * ThreeBandFilterBank::kNumBands - kNumZeroFilters;

  ThreeBandFilterBank();
  ~ThreeBandFilterBank();

  // Splits `in` of size kFullBandSize into 3 downsampled frequency bands in
  // `out`, each of size 160.
  void Analysis(ArrayView<const float, kFullBandSize> in,
                ArrayView<const ArrayView<float>, kNumBands> out);

  // Merges the 3 downsampled frequency bands in `in`, each of size 160, into
  // `out`, which is of size kFullBandSize.
  void Synthesis(ArrayView<const ArrayView<float>, kNumBands> in,
                 ArrayView<float, kFullBandSize> out);

 private:
  std::array<std::array<float, kMemorySize>, kNumNonZeroFilters>
      state_analysis_;
  std::array<std::array<float, kMemorySize>, kNumNonZeroFilters>
      state_synthesis_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_THREE_BAND_FILTER_BANK_H_
