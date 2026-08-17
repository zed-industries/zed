/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_CROSS_CORRELATION_H_
#define MODULES_AUDIO_CODING_NETEQ_CROSS_CORRELATION_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

// The function calculates the cross-correlation between two sequences
// `sequence_1` and `sequence_2`. `sequence_1` is taken as reference, with
// `sequence_1_length` as its length. `sequence_2` slides for the calculation of
// cross-correlation. The result will be saved in `cross_correlation`.
// `cross_correlation_length` correlation points are calculated.
// The corresponding lag starts from 0, and increases with a step of
// `cross_correlation_step`. The result is without normalization. To avoid
// overflow, the result will be right shifted. The amount of shifts will be
// returned.
//
// Input:
//     - sequence_1     : First sequence (reference).
//     - sequence_2     : Second sequence (sliding during calculation).
//     - sequence_1_length : Length of `sequence_1`.
//     - cross_correlation_length : Number of cross-correlations to calculate.
//     - cross_correlation_step : Step in the lag for the cross-correlation.
//
// Output:
//      - cross_correlation : The cross-correlation in Q(-right_shifts)
//
// Return:
//      Number of right shifts in cross_correlation.

int CrossCorrelationWithAutoShift(const int16_t* sequence_1,
                                  const int16_t* sequence_2,
                                  size_t sequence_1_length,
                                  size_t cross_correlation_length,
                                  int cross_correlation_step,
                                  int32_t* cross_correlation);

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_CROSS_CORRELATION_H_
