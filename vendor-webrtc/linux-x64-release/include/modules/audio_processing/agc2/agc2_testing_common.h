/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_AGC2_TESTING_COMMON_H_
#define MODULES_AUDIO_PROCESSING_AGC2_AGC2_TESTING_COMMON_H_

#include <limits>
#include <vector>

#include "rtc_base/random.h"

namespace webrtc {
namespace test {

constexpr float kMinS16 =
    static_cast<float>(std::numeric_limits<int16_t>::min());
constexpr float kMaxS16 =
    static_cast<float>(std::numeric_limits<int16_t>::max());

// Level Estimator test parameters.
constexpr float kDecayMs = 20.0f;

// Limiter parameters.
constexpr float kLimiterMaxInputLevelDbFs = 1.f;
constexpr float kLimiterKneeSmoothnessDb = 1.f;
constexpr float kLimiterCompressionRatio = 5.f;

// Returns evenly spaced `num_points` numbers over a specified interval [l, r].
std::vector<double> LinSpace(double l, double r, int num_points);

// Generates white noise.
class WhiteNoiseGenerator {
 public:
  WhiteNoiseGenerator(int min_amplitude, int max_amplitude);
  float operator()();

 private:
  Random rand_gen_;
  const int min_amplitude_;
  const int max_amplitude_;
};

// Generates a sine function.
class SineGenerator {
 public:
  SineGenerator(float amplitude, float frequency_hz, int sample_rate_hz);
  float operator()();

 private:
  const float amplitude_;
  const float frequency_hz_;
  const int sample_rate_hz_;
  float x_radians_;
};

// Generates periodic pulses.
class PulseGenerator {
 public:
  PulseGenerator(float pulse_amplitude,
                 float no_pulse_amplitude,
                 float frequency_hz,
                 int sample_rate_hz);
  float operator()();

 private:
  const float pulse_amplitude_;
  const float no_pulse_amplitude_;
  const int samples_period_;
  int sample_counter_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_AGC2_TESTING_COMMON_H_
