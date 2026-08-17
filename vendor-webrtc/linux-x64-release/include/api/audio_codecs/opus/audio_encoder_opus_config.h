/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_OPUS_AUDIO_ENCODER_OPUS_CONFIG_H_
#define API_AUDIO_CODECS_OPUS_AUDIO_ENCODER_OPUS_CONFIG_H_

#include <stddef.h>

#include <optional>
#include <vector>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

struct RTC_EXPORT AudioEncoderOpusConfig {
  static constexpr int kDefaultFrameSizeMs = 20;

  // Opus API allows a min bitrate of 500bps, but Opus documentation suggests
  // bitrate should be in the range of 6000 to 510000, inclusive.
  static constexpr int kMinBitrateBps = 6000;
  static constexpr int kMaxBitrateBps = 510000;

  AudioEncoderOpusConfig();
  AudioEncoderOpusConfig(const AudioEncoderOpusConfig&);
  ~AudioEncoderOpusConfig();
  AudioEncoderOpusConfig& operator=(const AudioEncoderOpusConfig&);

  bool IsOk() const;  // Checks if the values are currently OK.

  int frame_size_ms;
  int sample_rate_hz;
  size_t num_channels;
  enum class ApplicationMode { kVoip, kAudio };
  ApplicationMode application;

  // NOTE: This member must always be set.
  // TODO(kwiberg): Turn it into just an int.
  std::optional<int> bitrate_bps;

  bool fec_enabled;
  bool cbr_enabled;
  int max_playback_rate_hz;

  // `complexity` is used when the bitrate goes above
  // `complexity_threshold_bps` + `complexity_threshold_window_bps`;
  // `low_rate_complexity` is used when the bitrate falls below
  // `complexity_threshold_bps` - `complexity_threshold_window_bps`. In the
  // interval in the middle, we keep using the most recent of the two
  // complexity settings.
  int complexity;
  int low_rate_complexity;
  int complexity_threshold_bps;
  int complexity_threshold_window_bps;

  bool dtx_enabled;
  std::vector<int> supported_frame_lengths_ms;
  int uplink_bandwidth_update_interval_ms;

  // NOTE: This member isn't necessary, and will soon go away. See
  // https://bugs.chromium.org/p/webrtc/issues/detail?id=7847
  int payload_type;
};

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_OPUS_AUDIO_ENCODER_OPUS_CONFIG_H_
