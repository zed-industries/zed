/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_CONFIG_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_CONFIG_H_

#include <stddef.h>

#include <optional>

namespace webrtc {

struct AudioEncoderRuntimeConfig {
  AudioEncoderRuntimeConfig();
  AudioEncoderRuntimeConfig(const AudioEncoderRuntimeConfig& other);
  ~AudioEncoderRuntimeConfig();
  AudioEncoderRuntimeConfig& operator=(const AudioEncoderRuntimeConfig& other);
  bool operator==(const AudioEncoderRuntimeConfig& other) const;
  std::optional<int> bitrate_bps;
  std::optional<int> frame_length_ms;
  // Note: This is what we tell the encoder. It doesn't have to reflect
  // the actual NetworkMetrics; it's subject to our decision.
  std::optional<float> uplink_packet_loss_fraction;
  std::optional<bool> enable_fec;
  std::optional<bool> enable_dtx;

  // Some encoders can encode fewer channels than the actual input to make
  // better use of the bandwidth. `num_channels` sets the number of channels
  // to encode.
  std::optional<size_t> num_channels;

  // This is true if the last frame length change was an increase, and otherwise
  // false.
  // The value of this boolean is used to apply a different offset to the
  // per-packet overhead that is reported by the BWE. The exact offset value
  // is most important right after a frame length change, because the frame
  // length change affects the overhead. In the steady state, the exact value is
  // not important because the BWE will compensate.
  bool last_fl_change_increase = false;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_INCLUDE_AUDIO_NETWORK_ADAPTOR_CONFIG_H_
