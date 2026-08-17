/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_AUTO_CORRELATION_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_AUTO_CORRELATION_H_

#include <memory>

#include "api/array_view.h"
#include "modules/audio_processing/agc2/rnn_vad/common.h"
#include "modules/audio_processing/utility/pffft_wrapper.h"

namespace webrtc {
namespace rnn_vad {

// Class to compute the auto correlation on the pitch buffer for a target pitch
// interval.
class AutoCorrelationCalculator {
 public:
  AutoCorrelationCalculator();
  AutoCorrelationCalculator(const AutoCorrelationCalculator&) = delete;
  AutoCorrelationCalculator& operator=(const AutoCorrelationCalculator&) =
      delete;
  ~AutoCorrelationCalculator();

  // Computes the auto-correlation coefficients for a target pitch interval.
  // `auto_corr` indexes are inverted lags.
  void ComputeOnPitchBuffer(ArrayView<const float, kBufSize12kHz> pitch_buf,
                            ArrayView<float, kNumLags12kHz> auto_corr);

 private:
  Pffft fft_;
  std::unique_ptr<Pffft::FloatBuffer> tmp_;
  std::unique_ptr<Pffft::FloatBuffer> X_;
  std::unique_ptr<Pffft::FloatBuffer> H_;
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_AUTO_CORRELATION_H_
