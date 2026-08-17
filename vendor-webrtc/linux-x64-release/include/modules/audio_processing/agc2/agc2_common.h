/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_AGC2_COMMON_H_
#define MODULES_AUDIO_PROCESSING_AGC2_AGC2_COMMON_H_

namespace webrtc {

constexpr float kMinFloatS16Value = -32768.0f;
constexpr float kMaxFloatS16Value = 32767.0f;
constexpr float kMaxAbsFloatS16Value = 32768.0f;

// Minimum audio level in dBFS scale for S16 samples.
constexpr float kMinLevelDbfs = -90.31f;

constexpr int kFrameDurationMs = 10;
constexpr int kSubFramesInFrame = 20;
constexpr int kMaximalNumberOfSamplesPerChannel = 480;

// Adaptive digital gain applier settings.

// At what limiter levels should we start decreasing the adaptive digital gain.
constexpr float kLimiterThresholdForAgcGainDbfs = -1.0f;

// Number of milliseconds to wait to periodically reset the VAD.
constexpr int kVadResetPeriodMs = 1500;

// Speech probability threshold to detect speech activity.
constexpr float kVadConfidenceThreshold = 0.95f;

// Minimum number of adjacent speech frames having a sufficiently high speech
// probability to reliably detect speech activity.
constexpr int kAdjacentSpeechFramesThreshold = 12;

// Number of milliseconds of speech frames to observe to make the estimator
// confident.
constexpr float kLevelEstimatorTimeToConfidenceMs = 400;
constexpr float kLevelEstimatorLeakFactor =
    1.0f - 1.0f / kLevelEstimatorTimeToConfidenceMs;

// Saturation Protector settings.
constexpr float kSaturationProtectorInitialHeadroomDb = 20.0f;
constexpr int kSaturationProtectorBufferSize = 4;

// Number of interpolation points for each region of the limiter.
// These values have been tuned to limit the interpolated gain curve error given
// the limiter parameters and allowing a maximum error of +/- 32768^-1.
constexpr int kInterpolatedGainCurveKneePoints = 22;
constexpr int kInterpolatedGainCurveBeyondKneePoints = 10;
constexpr int kInterpolatedGainCurveTotalPoints =
    kInterpolatedGainCurveKneePoints + kInterpolatedGainCurveBeyondKneePoints;

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_AGC2_COMMON_H_
