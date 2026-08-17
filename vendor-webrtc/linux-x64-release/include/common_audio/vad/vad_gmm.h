/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Gaussian probability calculations internally used in vad_core.c.

#ifndef COMMON_AUDIO_VAD_VAD_GMM_H_
#define COMMON_AUDIO_VAD_VAD_GMM_H_

#include <stdint.h>

// Calculates the probability for `input`, given that `input` comes from a
// normal distribution with mean and standard deviation (`mean`, `std`).
//
// Inputs:
//      - input         : input sample in Q4.
//      - mean          : mean input in the statistical model, Q7.
//      - std           : standard deviation, Q7.
//
// Output:
//
//      - delta         : input used when updating the model, Q11.
//                        `delta` = (`input` - `mean`) / `std`^2.
//
// Return:
//   (probability for `input`) =
//    1 / `std` * exp(-(`input` - `mean`)^2 / (2 * `std`^2));
int32_t WebRtcVad_GaussianProbability(int16_t input,
                                      int16_t mean,
                                      int16_t std,
                                      int16_t* delta);

#endif  // COMMON_AUDIO_VAD_VAD_GMM_H_
