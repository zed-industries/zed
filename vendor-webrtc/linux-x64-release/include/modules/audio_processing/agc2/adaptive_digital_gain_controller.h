/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_ADAPTIVE_DIGITAL_GAIN_CONTROLLER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_ADAPTIVE_DIGITAL_GAIN_CONTROLLER_H_

#include <vector>

#include "api/audio/audio_processing.h"
#include "api/audio/audio_view.h"
#include "modules/audio_processing/agc2/gain_applier.h"

namespace webrtc {

class ApmDataDumper;

// Selects the target digital gain, decides when and how quickly to adapt to the
// target and applies the current gain to 10 ms frames.
class AdaptiveDigitalGainController {
 public:
  // Information about a frame to process.
  struct FrameInfo {
    float speech_probability;    // Probability of speech in the [0, 1] range.
    float speech_level_dbfs;     // Estimated speech level (dBFS).
    bool speech_level_reliable;  // True with reliable speech level estimation.
    float noise_rms_dbfs;        // Estimated noise RMS level (dBFS).
    float headroom_db;           // Headroom (dB).
    // TODO(bugs.webrtc.org/7494): Remove `limiter_envelope_dbfs`.
    float limiter_envelope_dbfs;  // Envelope level from the limiter (dBFS).
  };

  AdaptiveDigitalGainController(
      ApmDataDumper* apm_data_dumper,
      const AudioProcessing::Config::GainController2::AdaptiveDigital& config,
      int adjacent_speech_frames_threshold);
  AdaptiveDigitalGainController(const AdaptiveDigitalGainController&) = delete;
  AdaptiveDigitalGainController& operator=(
      const AdaptiveDigitalGainController&) = delete;

  // Analyzes `info`, updates the digital gain and applies it to a 10 ms
  // `frame`. Supports any sample rate supported by APM.
  void Process(const FrameInfo& info, DeinterleavedView<float> frame);

 private:
  ApmDataDumper* const apm_data_dumper_;
  GainApplier gain_applier_;

  const AudioProcessing::Config::GainController2::AdaptiveDigital config_;
  const int adjacent_speech_frames_threshold_;
  const float max_gain_change_db_per_10ms_;

  int calls_since_last_gain_log_;
  int frames_to_gain_increase_allowed_;
  float last_gain_db_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_ADAPTIVE_DIGITAL_GAIN_CONTROLLER_H_
