/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_BUFFER_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_BUFFER_H_

#include "modules/audio_coding/neteq/packet_buffer.h"
#include "test/gmock.h"

namespace webrtc {

class MockPacketBuffer : public PacketBuffer {
 public:
  MockPacketBuffer(size_t max_number_of_packets,
                   const TickTimer* tick_timer,
                   StatisticsCalculator* stats)
      : PacketBuffer(max_number_of_packets, tick_timer, stats) {}
  ~MockPacketBuffer() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(void, Flush, (), (override));
  MOCK_METHOD(bool, Empty, (), (const, override));
  MOCK_METHOD(int, InsertPacket, (Packet && packet), (override));
  MOCK_METHOD(int,
              NextTimestamp,
              (uint32_t * next_timestamp),
              (const, override));
  MOCK_METHOD(int,
              NextHigherTimestamp,
              (uint32_t timestamp, uint32_t* next_timestamp),
              (const, override));
  MOCK_METHOD(const Packet*, PeekNextPacket, (), (const, override));
  MOCK_METHOD(std::optional<Packet>, GetNextPacket, (), (override));
  MOCK_METHOD(int, DiscardNextPacket, (), (override));
  MOCK_METHOD(void,
              DiscardOldPackets,
              (uint32_t timestamp_limit, uint32_t horizon_samples),
              (override));
  MOCK_METHOD(void,
              DiscardAllOldPackets,
              (uint32_t timestamp_limit),
              (override));
  MOCK_METHOD(size_t, NumPacketsInBuffer, (), (const, override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_PACKET_BUFFER_H_
