/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_ECHO_CONTROL_H_
#define API_AUDIO_ECHO_CONTROL_H_

#include <memory>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"

namespace webrtc {

class AudioBuffer;

// Interface for an acoustic echo cancellation (AEC) submodule.
class EchoControl {
 public:
  // Analysis (not changing) of the render signal.
  virtual void AnalyzeRender(AudioBuffer* render) = 0;

  // Analysis (not changing) of the capture signal.
  virtual void AnalyzeCapture(AudioBuffer* capture) = 0;

  // Processes the capture signal in order to remove the echo.
  virtual void ProcessCapture(AudioBuffer* capture, bool level_change) = 0;

  // As above, but also returns the linear filter output.
  virtual void ProcessCapture(AudioBuffer* capture,
                              AudioBuffer* linear_output,
                              bool level_change) = 0;

  struct Metrics {
    double echo_return_loss;
    double echo_return_loss_enhancement;
    int delay_ms;
  };

  // Collect current metrics from the echo controller.
  virtual Metrics GetMetrics() const = 0;

  // Provides an optional external estimate of the audio buffer delay.
  virtual void SetAudioBufferDelay(int delay_ms) = 0;

  // Specifies whether the capture output will be used. The purpose of this is
  // to allow the echo controller to deactivate some of the processing when the
  // resulting output is anyway not used, for instance when the endpoint is
  // muted.
  // TODO(b/177830919): Make pure virtual.
  virtual void SetCaptureOutputUsage(bool /* capture_output_used */) {}

  // Returns wheter the signal is altered.
  virtual bool ActiveProcessing() const = 0;

  virtual ~EchoControl() {}
};

// Interface for a factory that creates EchoControllers.
class EchoControlFactory {
 public:
  virtual ~EchoControlFactory() = default;

  virtual absl_nonnull std::unique_ptr<EchoControl> Create(
      const Environment& env,
      int sample_rate_hz,
      int num_render_channels,
      int num_capture_channels) = 0;
};
}  // namespace webrtc

#endif  // API_AUDIO_ECHO_CONTROL_H_
