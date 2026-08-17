/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_BIQUAD_FILTER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_BIQUAD_FILTER_H_

#include "api/array_view.h"

namespace webrtc {

// Transposed direct form I implementation of a bi-quad filter.
//        b[0] + b[1] • z^(-1) + b[2] • z^(-2)
// H(z) = ------------------------------------
//          1 + a[1] • z^(-1) + a[2] • z^(-2)
class BiQuadFilter {
 public:
  // Normalized filter coefficients.
  // Computed as `[b, a] = scipy.signal.butter(N=2, Wn, btype)`.
  struct Config {
    float b[3];  // b[0], b[1], b[2].
    float a[2];  // a[1], a[2].
  };

  explicit BiQuadFilter(const Config& config);
  BiQuadFilter(const BiQuadFilter&) = delete;
  BiQuadFilter& operator=(const BiQuadFilter&) = delete;
  ~BiQuadFilter();

  // Sets the filter configuration and resets the internal state.
  void SetConfig(const Config& config);

  // Zeroes the filter state.
  void Reset();

  // Filters `x` and writes the output in `y`, which must have the same length
  // of `x`. In-place processing is supported.
  void Process(ArrayView<const float> x, ArrayView<float> y);

 private:
  Config config_;
  struct State {
    float b[2];
    float a[2];
  } state_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_BIQUAD_FILTER_H_
