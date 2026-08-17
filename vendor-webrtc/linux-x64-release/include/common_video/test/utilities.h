/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_TEST_UTILITIES_H_
#define COMMON_VIDEO_TEST_UTILITIES_H_

#include <initializer_list>

#include "api/rtp_packet_infos.h"
#include "api/video/color_space.h"

namespace webrtc {

HdrMetadata CreateTestHdrMetadata();
ColorSpace CreateTestColorSpace(bool with_hdr_metadata);
RtpPacketInfos CreatePacketInfos(size_t count);

}  // namespace webrtc
#endif  // COMMON_VIDEO_TEST_UTILITIES_H_
