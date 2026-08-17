/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_CREATE_VIDEO_RTP_DEPACKETIZER_H_
#define MODULES_RTP_RTCP_SOURCE_CREATE_VIDEO_RTP_DEPACKETIZER_H_

#include <memory>

#include "api/video/video_codec_type.h"
#include "modules/rtp_rtcp/source/video_rtp_depacketizer.h"

namespace webrtc {

std::unique_ptr<VideoRtpDepacketizer> CreateVideoRtpDepacketizer(
    VideoCodecType codec);

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_CREATE_VIDEO_RTP_DEPACKETIZER_H_
