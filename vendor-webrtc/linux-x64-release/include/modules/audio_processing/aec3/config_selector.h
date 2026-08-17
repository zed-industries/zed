/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_CONFIG_SELECTOR_H_
#define MODULES_AUDIO_PROCESSING_AEC3_CONFIG_SELECTOR_H_

#include <optional>

#include "api/audio/echo_canceller3_config.h"

namespace webrtc {

// Selects the config to use.
class ConfigSelector {
 public:
  ConfigSelector(const EchoCanceller3Config& config,
                 const std::optional<EchoCanceller3Config>& multichannel_config,
                 int num_render_input_channels);

  // Updates the config selection based on the detection of multichannel
  // content.
  void Update(bool multichannel_content);

  const EchoCanceller3Config& active_config() const { return *active_config_; }

 private:
  const EchoCanceller3Config config_;
  const std::optional<EchoCanceller3Config> multichannel_config_;
  const EchoCanceller3Config* active_config_ = nullptr;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_CONFIG_SELECTOR_H_
