/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_VAD_POLE_ZERO_FILTER_H_
#define MODULES_AUDIO_PROCESSING_VAD_POLE_ZERO_FILTER_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

class PoleZeroFilter {
 public:
  ~PoleZeroFilter() {}

  static PoleZeroFilter* Create(const float* numerator_coefficients,
                                size_t order_numerator,
                                const float* denominator_coefficients,
                                size_t order_denominator);

  int Filter(const int16_t* in, size_t num_input_samples, float* output);

 private:
  PoleZeroFilter(const float* numerator_coefficients,
                 size_t order_numerator,
                 const float* denominator_coefficients,
                 size_t order_denominator);

  static const int kMaxFilterOrder = 24;

  int16_t past_input_[kMaxFilterOrder * 2];
  float past_output_[kMaxFilterOrder * 2];

  float numerator_coefficients_[kMaxFilterOrder + 1];
  float denominator_coefficients_[kMaxFilterOrder + 1];

  size_t order_numerator_;
  size_t order_denominator_;
  size_t highest_order_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_VAD_POLE_ZERO_FILTER_H_
