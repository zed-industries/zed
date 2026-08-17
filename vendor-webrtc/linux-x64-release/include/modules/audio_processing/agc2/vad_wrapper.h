/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_VAD_WRAPPER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_VAD_WRAPPER_H_

#include <memory>
#include <vector>

#include "api/audio/audio_view.h"
#include "common_audio/resampler/include/push_resampler.h"
#include "modules/audio_processing/agc2/cpu_features.h"

namespace webrtc {

// Wraps a single-channel Voice Activity Detector (VAD) which is used to analyze
// the first channel of the input audio frames. Takes care of resampling the
// input frames to match the sample rate of the wrapped VAD and periodically
// resets the VAD.
class VoiceActivityDetectorWrapper {
 public:
  // Single channel VAD interface.
  class MonoVad {
   public:
    virtual ~MonoVad() = default;
    // Returns the sample rate (Hz) required for the input frames analyzed by
    // `ComputeProbability`.
    virtual int SampleRateHz() const = 0;
    // Resets the internal state.
    virtual void Reset() = 0;
    // Analyzes an audio frame and returns the speech probability.
    virtual float Analyze(MonoView<const float> frame) = 0;
  };

  // Ctor. Uses `cpu_features` to instantiate the default VAD.
  VoiceActivityDetectorWrapper(const AvailableCpuFeatures& cpu_features,
                               int sample_rate_hz);

  // Ctor. `vad_reset_period_ms` indicates the period in milliseconds to call
  // `MonoVad::Reset()`; it must be equal to or greater than the duration of two
  // frames. Uses `cpu_features` to instantiate the default VAD.
  VoiceActivityDetectorWrapper(int vad_reset_period_ms,
                               const AvailableCpuFeatures& cpu_features,
                               int sample_rate_hz);
  // Ctor. Uses a custom `vad`.
  VoiceActivityDetectorWrapper(int vad_reset_period_ms,
                               std::unique_ptr<MonoVad> vad,
                               int sample_rate_hz);

  VoiceActivityDetectorWrapper(const VoiceActivityDetectorWrapper&) = delete;
  VoiceActivityDetectorWrapper& operator=(const VoiceActivityDetectorWrapper&) =
      delete;
  ~VoiceActivityDetectorWrapper();

  // Analyzes the first channel of `frame` and returns the speech probability.
  // `frame` must be a 10 ms frame with the sample rate specified in the last
  // `Initialize()` call.
  float Analyze(DeinterleavedView<const float> frame);

 private:
  const int vad_reset_period_frames_;
  const int frame_size_;
  int time_to_vad_reset_;
  std::unique_ptr<MonoVad> vad_;
  std::vector<float> resampled_buffer_;
  PushResampler<float> resampler_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_VAD_WRAPPER_H_
