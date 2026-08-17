/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_MOCK_DATA_CHANNEL_H_
#define PC_TEST_MOCK_DATA_CHANNEL_H_

#include <cstdint>
#include <string>
#include <utility>

#include "pc/sctp_data_channel.h"
#include "rtc_base/thread.h"
#include "rtc_base/weak_ptr.h"
#include "test/gmock.h"

namespace webrtc {

class MockSctpDataChannel : public SctpDataChannel {
 public:
  MockSctpDataChannel(WeakPtr<SctpDataChannelControllerInterface> controller,
                      int id,
                      DataState state)
      : MockSctpDataChannel(std::move(controller),
                            id,
                            "MockSctpDataChannel",
                            state,
                            "someProtocol",
                            0,
                            0,
                            0,
                            0) {}
  MockSctpDataChannel(
      WeakPtr<SctpDataChannelControllerInterface> controller,
      int id,
      const std::string& label,
      DataState state,
      const std::string& protocol,
      uint32_t messages_sent,
      uint64_t bytes_sent,
      uint32_t messages_received,
      uint64_t bytes_received,
      const InternalDataChannelInit& config = InternalDataChannelInit(),
      Thread* signaling_thread = Thread::Current(),
      Thread* network_thread = Thread::Current())
      : SctpDataChannel(config,
                        std::move(controller),
                        label,
                        false,
                        signaling_thread,
                        network_thread) {
    EXPECT_CALL(*this, id()).WillRepeatedly(::testing::Return(id));
    EXPECT_CALL(*this, state()).WillRepeatedly(::testing::Return(state));
    EXPECT_CALL(*this, protocol()).WillRepeatedly(::testing::Return(protocol));
    EXPECT_CALL(*this, messages_sent())
        .WillRepeatedly(::testing::Return(messages_sent));
    EXPECT_CALL(*this, bytes_sent())
        .WillRepeatedly(::testing::Return(bytes_sent));
    EXPECT_CALL(*this, messages_received())
        .WillRepeatedly(::testing::Return(messages_received));
    EXPECT_CALL(*this, bytes_received())
        .WillRepeatedly(::testing::Return(bytes_received));
  }
  MOCK_METHOD(int, id, (), (const, override));
  MOCK_METHOD(DataState, state, (), (const, override));
  MOCK_METHOD(std::string, protocol, (), (const, override));
  MOCK_METHOD(uint32_t, messages_sent, (), (const, override));
  MOCK_METHOD(uint64_t, bytes_sent, (), (const, override));
  MOCK_METHOD(uint32_t, messages_received, (), (const, override));
  MOCK_METHOD(uint64_t, bytes_received, (), (const, override));
};

}  // namespace webrtc

#endif  // PC_TEST_MOCK_DATA_CHANNEL_H_
