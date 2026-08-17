/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_LP_RESIDUAL_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_LP_RESIDUAL_H_

#include <stddef.h>

#include "api/array_view.h"

namespace webrtc {
namespace rnn_vad {

// Linear predictive coding (LPC) inverse filter length.
constexpr int kNumLpcCoefficients = 5;

// Given a frame `x`, computes a post-processed version of LPC coefficients
// tailored for pitch estimation.
void ComputeAndPostProcessLpcCoefficients(
    ArrayView<const float> x,
    ArrayView<float, kNumLpcCoefficients> lpc_coeffs);

// Computes the LP residual for the input frame `x` and the LPC coefficients
// `lpc_coeffs`. `y` and `x` can point to the same array for in-place
// computation.
void ComputeLpResidual(ArrayView<const float, kNumLpcCoefficients> lpc_coeffs,
                       ArrayView<const float> x,
                       ArrayView<float> y);

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_LP_RESIDUAL_H_
