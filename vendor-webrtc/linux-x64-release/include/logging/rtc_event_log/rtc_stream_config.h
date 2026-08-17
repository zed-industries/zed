/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_RTC_STREAM_CONFIG_H_
#define LOGGING_RTC_EVENT_LOG_RTC_STREAM_CONFIG_H_

#include <stdint.h>

#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"

namespace webrtc {
namespace rtclog {

struct StreamConfig {
  StreamConfig();
  StreamConfig(const StreamConfig& other);
  ~StreamConfig();

  bool operator==(const StreamConfig& other) const;
  bool operator!=(const StreamConfig& other) const;

  uint32_t local_ssrc = 0;
  uint32_t remote_ssrc = 0;
  uint32_t rtx_ssrc = 0;
  std::string rsid;

  bool remb = false;
  std::vector<RtpExtension> rtp_extensions;

  RtcpMode rtcp_mode = RtcpMode::kReducedSize;

  struct Codec {
    Codec(absl::string_view payload_name,
          int payload_type,
          int rtx_payload_type);

    bool operator==(const Codec& other) const;

    std::string payload_name;
    int payload_type;
    int rtx_payload_type;
  };

  std::vector<Codec> codecs;
};

}  // namespace rtclog
}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_RTC_STREAM_CONFIG_H_
