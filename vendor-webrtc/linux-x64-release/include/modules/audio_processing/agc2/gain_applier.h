/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_GAIN_APPLIER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_GAIN_APPLIER_H_

#include <stddef.h>

#include "api/audio/audio_view.h"
#include "modules/audio_processing/include/audio_frame_view.h"

namespace webrtc {
class GainApplier {
 public:
  GainApplier(bool hard_clip_samples, float initial_gain_factor);

  void ApplyGain(DeinterleavedView<float> signal);
  void SetGainFactor(float gain_factor);
  float GetGainFactor() const { return current_gain_factor_; }

  [[deprecated("Use DeinterleavedView<> version")]] void ApplyGain(
      AudioFrameView<float> signal) {
    ApplyGain(signal.view());
  }

 private:
  void Initialize(int samples_per_channel);

  // Whether to clip samples after gain is applied. If 'true', result
  // will fit in FloatS16 range.
  const bool hard_clip_samples_;
  float last_gain_factor_;

  // If this value is not equal to 'last_gain_factor', gain will be
  // ramped from 'last_gain_factor_' to this value during the next
  // 'ApplyGain'.
  float current_gain_factor_;
  int samples_per_channel_ = -1;
  float inverse_samples_per_channel_ = -1.f;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_GAIN_APPLIER_H_
