/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_MIXER_SINE_WAVE_GENERATOR_H_
#define MODULES_AUDIO_MIXER_SINE_WAVE_GENERATOR_H_

#include <stdint.h>

#include "api/audio/audio_frame.h"
#include "rtc_base/checks.h"

namespace webrtc {

class SineWaveGenerator {
 public:
  SineWaveGenerator(float wave_frequency_hz, int16_t amplitude)
      : wave_frequency_hz_(wave_frequency_hz), amplitude_(amplitude) {
    RTC_DCHECK_GT(wave_frequency_hz, 0);
  }

  // Produces appropriate output based on frame->num_channels_,
  // frame->sample_rate_hz_.
  void GenerateNextFrame(AudioFrame* frame);

 private:
  float phase_ = 0.f;
  const float wave_frequency_hz_;
  const int16_t amplitude_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_MIXER_SINE_WAVE_GENERATOR_H_
