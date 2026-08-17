/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_COMMON_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_COMMON_H_

#include <stddef.h>

namespace webrtc {
namespace rnn_vad {

constexpr double kPi = 3.14159265358979323846;

constexpr int kSampleRate24kHz = 24000;
constexpr int kFrameSize10ms24kHz = kSampleRate24kHz / 100;
constexpr int kFrameSize20ms24kHz = kFrameSize10ms24kHz * 2;

// Pitch buffer.
constexpr int kMinPitch24kHz = kSampleRate24kHz / 800;   // 0.00125 s.
constexpr int kMaxPitch24kHz = kSampleRate24kHz / 62.5;  // 0.016 s.
constexpr int kBufSize24kHz = kMaxPitch24kHz + kFrameSize20ms24kHz;
static_assert((kBufSize24kHz & 1) == 0, "The buffer size must be even.");

// 24 kHz analysis.
// Define a higher minimum pitch period for the initial search. This is used to
// avoid searching for very short periods, for which a refinement step is
// responsible.
constexpr int kInitialMinPitch24kHz = 3 * kMinPitch24kHz;
static_assert(kMinPitch24kHz < kInitialMinPitch24kHz, "");
static_assert(kInitialMinPitch24kHz < kMaxPitch24kHz, "");
static_assert(kMaxPitch24kHz > kInitialMinPitch24kHz, "");
// Number of (inverted) lags during the initial pitch search phase at 24 kHz.
constexpr int kInitialNumLags24kHz = kMaxPitch24kHz - kInitialMinPitch24kHz;
// Number of (inverted) lags during the pitch search refinement phase at 24 kHz.
constexpr int kRefineNumLags24kHz = kMaxPitch24kHz + 1;
static_assert(
    kRefineNumLags24kHz > kInitialNumLags24kHz,
    "The refinement step must search the pitch in an extended pitch range.");

// 12 kHz analysis.
constexpr int kSampleRate12kHz = 12000;
constexpr int kFrameSize10ms12kHz = kSampleRate12kHz / 100;
constexpr int kFrameSize20ms12kHz = kFrameSize10ms12kHz * 2;
constexpr int kBufSize12kHz = kBufSize24kHz / 2;
constexpr int kInitialMinPitch12kHz = kInitialMinPitch24kHz / 2;
constexpr int kMaxPitch12kHz = kMaxPitch24kHz / 2;
static_assert(kMaxPitch12kHz > kInitialMinPitch12kHz, "");
// The inverted lags for the pitch interval [`kInitialMinPitch12kHz`,
// `kMaxPitch12kHz`] are in the range [0, `kNumLags12kHz`].
constexpr int kNumLags12kHz = kMaxPitch12kHz - kInitialMinPitch12kHz;

// 48 kHz constants.
constexpr int kMinPitch48kHz = kMinPitch24kHz * 2;
constexpr int kMaxPitch48kHz = kMaxPitch24kHz * 2;

// Spectral features.
constexpr int kNumBands = 22;
constexpr int kNumLowerBands = 6;
static_assert((0 < kNumLowerBands) && (kNumLowerBands < kNumBands), "");
constexpr int kCepstralCoeffsHistorySize = 8;
static_assert(kCepstralCoeffsHistorySize > 2,
              "The history size must at least be 3 to compute first and second "
              "derivatives.");

constexpr int kFeatureVectorSize = 42;

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_COMMON_H_
