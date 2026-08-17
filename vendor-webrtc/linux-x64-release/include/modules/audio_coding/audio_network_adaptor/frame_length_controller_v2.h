/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_FRAME_LENGTH_CONTROLLER_V2_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_FRAME_LENGTH_CONTROLLER_V2_H_

#include <optional>
#include <vector>

#include "modules/audio_coding/audio_network_adaptor/controller.h"
#include "modules/audio_coding/audio_network_adaptor/include/audio_network_adaptor.h"

namespace webrtc {

class FrameLengthControllerV2 final : public Controller {
 public:
  FrameLengthControllerV2(ArrayView<const int> encoder_frame_lengths_ms,
                          int min_payload_bitrate_bps,
                          bool use_slow_adaptation);

  void UpdateNetworkMetrics(const NetworkMetrics& network_metrics) override;

  void MakeDecision(AudioEncoderRuntimeConfig* config) override;

 private:
  std::vector<int> encoder_frame_lengths_ms_;
  const int min_payload_bitrate_bps_;
  const bool use_slow_adaptation_;

  std::optional<int> uplink_bandwidth_bps_;
  std::optional<int> target_bitrate_bps_;
  std::optional<int> overhead_bytes_per_packet_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_FRAME_LENGTH_CONTROLLER_V2_H_
