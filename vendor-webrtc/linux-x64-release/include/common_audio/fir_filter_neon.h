/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_FIR_FILTER_NEON_H_
#define COMMON_AUDIO_FIR_FILTER_NEON_H_

#include <memory>

#include "common_audio/fir_filter.h"
#include "rtc_base/memory/aligned_malloc.h"

namespace webrtc {

class FIRFilterNEON : public FIRFilter {
 public:
  FIRFilterNEON(const float* coefficients,
                size_t coefficients_length,
                size_t max_input_length);
  ~FIRFilterNEON() override;

  void Filter(const float* in, size_t length, float* out) override;

 private:
  size_t coefficients_length_;
  size_t state_length_;
  std::unique_ptr<float[], AlignedFreeDeleter> coefficients_;
  std::unique_ptr<float[], AlignedFreeDeleter> state_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_FIR_FILTER_NEON_H_
