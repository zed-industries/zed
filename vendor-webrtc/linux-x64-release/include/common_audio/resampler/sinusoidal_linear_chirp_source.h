/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Modified from the Chromium original here:
// src/media/base/sinc_resampler_unittest.cc

#ifndef COMMON_AUDIO_RESAMPLER_SINUSOIDAL_LINEAR_CHIRP_SOURCE_H_
#define COMMON_AUDIO_RESAMPLER_SINUSOIDAL_LINEAR_CHIRP_SOURCE_H_

#include "common_audio/resampler/sinc_resampler.h"

namespace webrtc {

// Fake audio source for testing the resampler.  Generates a sinusoidal linear
// chirp (http://en.wikipedia.org/wiki/Chirp) which can be tuned to stress the
// resampler for the specific sample rate conversion being used.
class SinusoidalLinearChirpSource : public SincResamplerCallback {
 public:
  // `delay_samples` can be used to insert a fractional sample delay into the
  // source.  It will produce zeros until non-negative time is reached.
  SinusoidalLinearChirpSource(int sample_rate,
                              size_t samples,
                              double max_frequency,
                              double delay_samples);

  ~SinusoidalLinearChirpSource() override {}

  SinusoidalLinearChirpSource(const SinusoidalLinearChirpSource&) = delete;
  SinusoidalLinearChirpSource& operator=(const SinusoidalLinearChirpSource&) =
      delete;

  void Run(size_t frames, float* destination) override;

  double Frequency(size_t position);

 private:
  static constexpr int kMinFrequency = 5;

  int sample_rate_;
  size_t total_samples_;
  double max_frequency_;
  double k_;
  size_t current_index_;
  double delay_samples_;
};

}  // namespace webrtc

#endif  // COMMON_AUDIO_RESAMPLER_SINUSOIDAL_LINEAR_CHIRP_SOURCE_H_
