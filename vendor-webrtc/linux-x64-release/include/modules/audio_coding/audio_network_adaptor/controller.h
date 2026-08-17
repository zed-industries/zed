/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_CONTROLLER_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_CONTROLLER_H_

#include <optional>

#include "modules/audio_coding/audio_network_adaptor/include/audio_network_adaptor.h"

namespace webrtc {

class Controller {
 public:
  struct NetworkMetrics {
    NetworkMetrics();
    ~NetworkMetrics();
    std::optional<int> uplink_bandwidth_bps;
    std::optional<float> uplink_packet_loss_fraction;
    std::optional<int> target_audio_bitrate_bps;
    std::optional<int> rtt_ms;
    std::optional<size_t> overhead_bytes_per_packet;
  };

  virtual ~Controller() = default;

  // Informs network metrics update to this controller. Any non-empty field
  // indicates an update on the corresponding network metric.
  virtual void UpdateNetworkMetrics(const NetworkMetrics& network_metrics) = 0;

  virtual void MakeDecision(AudioEncoderRuntimeConfig* config) = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_CONTROLLER_H_
