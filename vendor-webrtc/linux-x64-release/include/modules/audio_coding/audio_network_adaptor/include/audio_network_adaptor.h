/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_H_

#include <optional>

#include "api/audio_codecs/audio_encoder.h"
#include "modules/audio_coding/audio_network_adaptor/include/audio_network_adaptor_config.h"

namespace webrtc {

// An AudioNetworkAdaptor optimizes the audio experience by suggesting a
// suitable runtime configuration (bit rate, frame length, FEC, etc.) to the
// encoder based on network metrics.
class AudioNetworkAdaptor {
 public:
  virtual ~AudioNetworkAdaptor() = default;

  virtual void SetUplinkBandwidth(int uplink_bandwidth_bps) = 0;

  virtual void SetUplinkPacketLossFraction(
      float uplink_packet_loss_fraction) = 0;

  virtual void SetRtt(int rtt_ms) = 0;

  virtual void SetTargetAudioBitrate(int target_audio_bitrate_bps) = 0;

  virtual void SetOverhead(size_t overhead_bytes_per_packet) = 0;

  virtual AudioEncoderRuntimeConfig GetEncoderRuntimeConfig() = 0;

  virtual void StartDebugDump(FILE* file_handle) = 0;

  virtual void StopDebugDump() = 0;

  virtual ANAStats GetStats() const = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_H_
