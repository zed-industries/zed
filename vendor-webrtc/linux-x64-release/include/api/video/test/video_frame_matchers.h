/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_TEST_VIDEO_FRAME_MATCHERS_H_
#define API_VIDEO_TEST_VIDEO_FRAME_MATCHERS_H_

#include "test/gmock.h"

namespace webrtc::test::video_frame_matchers {

MATCHER_P(Rotation, rotation, "") {
  return ::testing::Matches(::testing::Eq(rotation))(arg.rotation());
}

MATCHER_P(NtpTimestamp, ntp_ts, "") {
  return arg.ntp_time_ms() == ntp_ts.ms();
}

MATCHER_P(PacketInfos, m, "") {
  return ::testing::Matches(m)(arg.packet_infos());
}

}  // namespace webrtc::test::video_frame_matchers

#endif  // API_VIDEO_TEST_VIDEO_FRAME_MATCHERS_H_
