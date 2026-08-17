/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_AV1_H_
#define MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_AV1_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "api/array_view.h"
#include "api/scoped_refptr.h"
#include "api/video/encoded_image.h"
#include "modules/rtp_rtcp/source/video_rtp_depacketizer.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

class VideoRtpDepacketizerAv1 : public VideoRtpDepacketizer {
 public:
  VideoRtpDepacketizerAv1() = default;
  VideoRtpDepacketizerAv1(const VideoRtpDepacketizerAv1&) = delete;
  VideoRtpDepacketizerAv1& operator=(const VideoRtpDepacketizerAv1&) = delete;
  ~VideoRtpDepacketizerAv1() override = default;

  scoped_refptr<EncodedImageBuffer> AssembleFrame(
      ArrayView<const ArrayView<const uint8_t>> rtp_payloads) override;

  std::optional<ParsedRtpPayload> Parse(CopyOnWriteBuffer rtp_payload) override;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_VIDEO_RTP_DEPACKETIZER_AV1_H_
