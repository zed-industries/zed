/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_PITCH_SEARCH_INTERNAL_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_PITCH_SEARCH_INTERNAL_H_

#include <stddef.h>

#include <array>
#include <utility>

#include "api/array_view.h"
#include "modules/audio_processing/agc2/cpu_features.h"
#include "modules/audio_processing/agc2/rnn_vad/common.h"

namespace webrtc {
namespace rnn_vad {

// Performs 2x decimation without any anti-aliasing filter.
void Decimate2x(ArrayView<const float, kBufSize24kHz> src,
                ArrayView<float, kBufSize12kHz> dst);

// Key concepts and keywords used below in this file.
//
// The pitch estimation relies on a pitch buffer, which is an array-like data
// structured designed as follows:
//
// |....A....|.....B.....|
//
// The part on the left, named `A` contains the oldest samples, whereas `B`
// contains the most recent ones. The size of `A` corresponds to the maximum
// pitch period, that of `B` to the analysis frame size (e.g., 16 ms and 20 ms
// respectively).
//
// Pitch estimation is essentially based on the analysis of two 20 ms frames
// extracted from the pitch buffer. One frame, called `x`, is kept fixed and
// corresponds to `B` - i.e., the most recent 20 ms. The other frame, called
// `y`, is extracted from different parts of the buffer instead.
//
// The offset between `x` and `y` corresponds to a specific pitch period.
// For instance, if `y` is positioned at the beginning of the pitch buffer, then
// the cross-correlation between `x` and `y` can be used as an indication of the
// strength for the maximum pitch.
//
// Such an offset can be encoded in two ways:
// - As a lag, which is the index in the pitch buffer for the first item in `y`
// - As an inverted lag, which is the number of samples from the beginning of
//   `x` and the end of `y`
//
// |---->| lag
// |....A....|.....B.....|
//       |<--| inverted lag
//       |.....y.....| `y` 20 ms frame
//
// The inverted lag has the advantage of being directly proportional to the
// corresponding pitch period.

// Computes the sum of squared samples for every sliding frame `y` in the pitch
// buffer. The indexes of `y_energy` are inverted lags.
void ComputeSlidingFrameSquareEnergies24kHz(
    ArrayView<const float, kBufSize24kHz> pitch_buffer,
    ArrayView<float, kRefineNumLags24kHz> y_energy,
    AvailableCpuFeatures cpu_features);

// Top-2 pitch period candidates. Unit: number of samples - i.e., inverted lags.
struct CandidatePitchPeriods {
  int best;
  int second_best;
};

// Computes the candidate pitch periods at 12 kHz given a view on the 12 kHz
// pitch buffer and the auto-correlation values (having inverted lags as
// indexes).
CandidatePitchPeriods ComputePitchPeriod12kHz(
    ArrayView<const float, kBufSize12kHz> pitch_buffer,
    ArrayView<const float, kNumLags12kHz> auto_correlation,
    AvailableCpuFeatures cpu_features);

// Computes the pitch period at 48 kHz given a view on the 24 kHz pitch buffer,
// the energies for the sliding frames `y` at 24 kHz and the pitch period
// candidates at 24 kHz (encoded as inverted lag).
int ComputePitchPeriod48kHz(
    ArrayView<const float, kBufSize24kHz> pitch_buffer,
    ArrayView<const float, kRefineNumLags24kHz> y_energy,
    CandidatePitchPeriods pitch_candidates_24kHz,
    AvailableCpuFeatures cpu_features);

struct PitchInfo {
  int period;
  float strength;
};

// Computes the pitch period at 48 kHz searching in an extended pitch range
// given a view on the 24 kHz pitch buffer, the energies for the sliding frames
// `y` at 24 kHz, the initial 48 kHz estimation (computed by
// `ComputePitchPeriod48kHz()`) and the last estimated pitch.
PitchInfo ComputeExtendedPitchPeriod48kHz(
    ArrayView<const float, kBufSize24kHz> pitch_buffer,
    ArrayView<const float, kRefineNumLags24kHz> y_energy,
    int initial_pitch_period_48kHz,
    PitchInfo last_pitch_48kHz,
    AvailableCpuFeatures cpu_features);

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_PITCH_SEARCH_INTERNAL_H_
