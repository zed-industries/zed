/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_TRANSPARENT_MODE_H_
#define MODULES_AUDIO_PROCESSING_AEC3_TRANSPARENT_MODE_H_

#include <memory>

#include "api/audio/echo_canceller3_config.h"
#include "api/environment/environment.h"
#include "modules/audio_processing/aec3/aec3_common.h"

namespace webrtc {

// Class for detecting and toggling the transparent mode which causes the
// suppressor to apply less suppression.
class TransparentMode {
 public:
  static std::unique_ptr<TransparentMode> Create(
      const Environment& env,
      const EchoCanceller3Config& config);

  virtual ~TransparentMode() {}

  // Returns whether the transparent mode should be active.
  virtual bool Active() const = 0;

  // Resets the state of the detector.
  virtual void Reset() = 0;

  // Updates the detection decision based on new data.
  virtual void Update(int filter_delay_blocks,
                      bool any_filter_consistent,
                      bool any_filter_converged,
                      bool any_coarse_filter_converged,
                      bool all_filters_diverged,
                      bool active_render,
                      bool saturated_capture) = 0;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_PROCESSING_AEC3_TRANSPARENT_MODE_H_
