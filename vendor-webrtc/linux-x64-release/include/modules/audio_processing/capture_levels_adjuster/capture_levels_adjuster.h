/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_CAPTURE_LEVELS_ADJUSTER_H_
#define MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_CAPTURE_LEVELS_ADJUSTER_H_

#include <stddef.h>

#include "modules/audio_processing/audio_buffer.h"
#include "modules/audio_processing/capture_levels_adjuster/audio_samples_scaler.h"

namespace webrtc {

// Adjusts the level of the capture signal before and after all capture-side
// processing is done using a combination of explicitly specified gains
// and an emulated analog gain functionality where a specified analog level
// results in an additional gain. The pre-adjustment is achieved by combining
// the gain value `pre_gain` and the level `emulated_analog_mic_gain_level` to
// form a combined gain of `pre_gain`*`emulated_analog_mic_gain_level`/255 which
// is multiplied to each sample. The intention of the
// `emulated_analog_mic_gain_level` is to be controlled by the analog AGC
// functionality and to produce an emulated analog mic gain equal to
// `emulated_analog_mic_gain_level`/255. The post level adjustment is achieved
// by multiplying each sample with the value of `post_gain`. Any changes in the
// gains take are done smoothly over one frame and the scaled samples are
// clamped to fit into the allowed S16 sample range.
class CaptureLevelsAdjuster {
 public:
  // C-tor. The values for the level and the gains must fulfill
  // 0 <= emulated_analog_mic_gain_level <= 255.
  // 0.f <= pre_gain.
  // 0.f <= post_gain.
  CaptureLevelsAdjuster(bool emulated_analog_mic_gain_enabled,
                        int emulated_analog_mic_gain_level,
                        float pre_gain,
                        float post_gain);
  CaptureLevelsAdjuster(const CaptureLevelsAdjuster&) = delete;
  CaptureLevelsAdjuster& operator=(const CaptureLevelsAdjuster&) = delete;

  // Adjusts the level of the signal. This should be called before any of the
  // other processing is performed.
  void ApplyPreLevelAdjustment(AudioBuffer& audio_buffer);

  // Adjusts the level of the signal. This should be called after all of the
  // other processing have been performed.
  void ApplyPostLevelAdjustment(AudioBuffer& audio_buffer);

  // Sets the gain to apply to each sample before any of the other processing is
  // performed.
  void SetPreGain(float pre_gain);

  // Returns the total pre-adjustment gain applied, comprising both the pre_gain
  // as well as the gain from the emulated analog mic, to each sample before any
  // of the other processing is performed.
  float GetPreAdjustmentGain() const { return pre_adjustment_gain_; }

  // Sets the gain to apply to each sample after all of the other processing
  // have been performed.
  void SetPostGain(float post_gain);

  // Sets the analog gain level to use for the emulated analog gain.
  // `level` must be in the range [0...255].
  void SetAnalogMicGainLevel(int level);

  // Returns the current analog gain level used for the emulated analog gain.
  int GetAnalogMicGainLevel() const { return emulated_analog_mic_gain_level_; }

 private:
  // Updates the value of `pre_adjustment_gain_` based on the supplied values
  // for `pre_gain` and `emulated_analog_mic_gain_level_`.
  void UpdatePreAdjustmentGain();

  const bool emulated_analog_mic_gain_enabled_;
  int emulated_analog_mic_gain_level_;
  float pre_gain_;
  float pre_adjustment_gain_;
  AudioSamplesScaler pre_scaler_;
  AudioSamplesScaler post_scaler_;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_CAPTURE_LEVELS_ADJUSTER_CAPTURE_LEVELS_ADJUSTER_H_
