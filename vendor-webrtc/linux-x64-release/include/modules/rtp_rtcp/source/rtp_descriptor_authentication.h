/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_DESCRIPTOR_AUTHENTICATION_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_DESCRIPTOR_AUTHENTICATION_H_

#include <cstdint>
#include <vector>

#include "modules/rtp_rtcp/source/rtp_video_header.h"

namespace webrtc {

// Converts frame dependencies into array of bytes for authentication.
std::vector<uint8_t> RtpDescriptorAuthentication(
    const RTPVideoHeader& rtp_video_header);

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_DESCRIPTOR_AUTHENTICATION_H_
