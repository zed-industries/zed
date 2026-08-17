/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_AUDIO_SAMPLES_SCALER_H_
#define MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_AUDIO_SAMPLES_SCALER_H_

#include <stddef.h>

#include "modules/audio_processing/audio_buffer.h"

namespace webrtc {

// Handles and applies a gain to the samples in an audio buffer.
// The gain is applied for each sample and any changes in the gain take effect
// gradually (in a linear manner) over one frame.
class AudioSamplesScaler {
 public:
  // C-tor. The supplied `initial_gain` is used immediately at the first call to
  // Process(), i.e., in contrast to the gain supplied by SetGain(...) there is
  // no gradual change to the `initial_gain`.
  explicit AudioSamplesScaler(float initial_gain);
  AudioSamplesScaler(const AudioSamplesScaler&) = delete;
  AudioSamplesScaler& operator=(const AudioSamplesScaler&) = delete;

  // Applies the specified gain to the audio in `audio_buffer`.
  void Process(AudioBuffer& audio_buffer);

  // Sets the gain to apply to each sample.
  void SetGain(float gain) { target_gain_ = gain; }

 private:
  float previous_gain_ = 1.f;
  float target_gain_ = 1.f;
  int samples_per_channel_ = -1;
  float one_by_samples_per_channel_ = -1.f;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_AUDIO_SAMPLES_SCALER_H_
