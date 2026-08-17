/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_H_

#include <memory>
#include <optional>
#include <vector>

#include "api/audio/echo_canceller3_config.h"
#include "api/audio/echo_control.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/aec3/block.h"
#include "modules/audio_processing/aec3/delay_estimate.h"
#include "modules/audio_processing/aec3/echo_path_variability.h"
#include "modules/audio_processing/aec3/render_buffer.h"

namespace webrtc {

// Class for removing the echo from the capture signal.
class EchoRemover {
 public:
  static std::unique_ptr<EchoRemover> Create(const Environment& env,
                                             const EchoCanceller3Config& config,
                                             int sample_rate_hz,
                                             size_t num_render_channels,
                                             size_t num_capture_channels);
  virtual ~EchoRemover() = default;

  // Get current metrics.
  virtual void GetMetrics(EchoControl::Metrics* metrics) const = 0;

  // Removes the echo from a block of samples from the capture signal. The
  // supplied render signal is assumed to be pre-aligned with the capture
  // signal.
  virtual void ProcessCapture(
      EchoPathVariability echo_path_variability,
      bool capture_signal_saturation,
      const std::optional<DelayEstimate>& external_delay,
      RenderBuffer* render_buffer,
      Block* linear_output,
      Block* capture) = 0;

  // Updates the status on whether echo leakage is detected in the output of the
  // echo remover.
  virtual void UpdateEchoLeakageStatus(bool leakage_detected) = 0;

  // Specifies whether the capture output will be used. The purpose of this is
  // to allow the echo remover to deactivate some of the processing when the
  // resulting output is anyway not used, for instance when the endpoint is
  // muted.
  virtual void SetCaptureOutputUsage(bool capture_output_used) = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_ECHO_REMOVER_H_
