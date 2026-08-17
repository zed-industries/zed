/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_VP8_H_
#define MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_VP8_H_

#include <cstdint>
#include <optional>

#include "api/array_view.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/rtp_rtcp/source/video_rtp_depacketizer.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

class VideoRtpDepacketizerVp8 : public VideoRtpDepacketizer {
 public:
  VideoRtpDepacketizerVp8() = default;
  VideoRtpDepacketizerVp8(const VideoRtpDepacketizerVp8&) = delete;
  VideoRtpDepacketizerVp8& operator=(const VideoRtpDepacketizerVp8&) = delete;
  ~VideoRtpDepacketizerVp8() override = default;

  // Parses vp8 rtp payload descriptor.
  // Returns zero on error or vp8 payload header offset on success.
  static int ParseRtpPayload(ArrayView<const uint8_t> rtp_payload,
                             RTPVideoHeader* video_header);

  std::optional<ParsedRtpPayload> Parse(CopyOnWriteBuffer rtp_payload) override;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_VP8_H_
