/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_MOCKS_MOCK_RTCP_RTT_STATS_H_
#define MODULES_RTP_RTCP_MOCKS_MOCK_RTCP_RTT_STATS_H_

#include <cstdint>

#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "test/gmock.h"

namespace webrtc {

class MockRtcpRttStats : public RtcpRttStats {
 public:
  MOCK_METHOD(void, OnRttUpdate, (int64_t rtt), (override));
  MOCK_METHOD(int64_t, LastProcessedRtt, (), (const, override));
};
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_MOCKS_MOCK_RTCP_RTT_STATS_H_
