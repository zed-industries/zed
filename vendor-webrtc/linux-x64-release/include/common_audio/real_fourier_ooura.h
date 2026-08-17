/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_AUDIO_REAL_FOURIER_OOURA_H_
#define COMMON_AUDIO_REAL_FOURIER_OOURA_H_

#include <stddef.h>

#include <complex>
#include <memory>

#include "common_audio/real_fourier.h"

namespace webrtc {

class RealFourierOoura : public RealFourier {
 public:
  explicit RealFourierOoura(int fft_order);
  ~RealFourierOoura() override;

  void Forward(const float* src, std::complex<float>* dest) const override;
  void Inverse(const std::complex<float>* src, float* dest) const override;

  int order() const override;

 private:
  const int order_;
  const size_t length_;
  const size_t complex_length_;
  // These are work arrays for Ooura. The names are based on the comments in
  // common_audio/third_party/ooura/fft_size_256/fft4g.cc.
  const std::unique_ptr<size_t[]> work_ip_;
  const std::unique_ptr<float[]> work_w_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_REAL_FOURIER_OOURA_H_
