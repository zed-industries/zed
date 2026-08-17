/*
 *  Copyright 2023 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_ARRIVAL_HISTORY_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_ARRIVAL_HISTORY_H_

#include "api/neteq/tick_timer.h"
#include "modules/audio_coding/neteq/packet_arrival_history.h"
#include "test/gmock.h"

namespace webrtc {

class MockPacketArrivalHistory : public PacketArrivalHistory {
 public:
  MockPacketArrivalHistory(const TickTimer* tick_timer)
      : PacketArrivalHistory(tick_timer, 0) {}

  MOCK_METHOD(int, GetDelayMs, (uint32_t rtp_timestamp), (const, override));
  MOCK_METHOD(int, GetMaxDelayMs, (), (const, override));
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_ARRIVAL_HISTORY_H_
