/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_DATA_CHANNEL_H_
#define API_TEST_MOCK_DATA_CHANNEL_H_

#include <cstdint>
#include <optional>
#include <string>

#include "absl/functional/any_invocable.h"
#include "api/data_channel_interface.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/scoped_refptr.h"
#include "rtc_base/ref_counted_object.h"
#include "test/gmock.h"

namespace webrtc {

class MockDataChannelInterface
    : public RefCountedObject<webrtc::DataChannelInterface> {
 public:
  static scoped_refptr<MockDataChannelInterface> Create() {
    return scoped_refptr<MockDataChannelInterface>(
        new MockDataChannelInterface());
  }

  MOCK_METHOD(void,
              RegisterObserver,
              (DataChannelObserver * observer),
              (override));
  MOCK_METHOD(void, UnregisterObserver, (), (override));
  MOCK_METHOD(std::string, label, (), (const, override));
  MOCK_METHOD(bool, reliable, (), (const, override));
  MOCK_METHOD(bool, ordered, (), (const, override));
  MOCK_METHOD(std::optional<int>, maxRetransmitsOpt, (), (const, override));
  MOCK_METHOD(std::optional<int>, maxPacketLifeTime, (), (const, override));
  MOCK_METHOD(std::string, protocol, (), (const, override));
  MOCK_METHOD(bool, negotiated, (), (const, override));
  MOCK_METHOD(int, id, (), (const, override));
  MOCK_METHOD(PriorityValue, priority, (), (const, override));
  MOCK_METHOD(DataState, state, (), (const, override));
  MOCK_METHOD(RTCError, error, (), (const, override));
  MOCK_METHOD(uint32_t, messages_sent, (), (const, override));
  MOCK_METHOD(uint64_t, bytes_sent, (), (const, override));
  MOCK_METHOD(uint32_t, messages_received, (), (const, override));
  MOCK_METHOD(uint64_t, bytes_received, (), (const, override));
  MOCK_METHOD(uint64_t, buffered_amount, (), (const, override));
  MOCK_METHOD(void, Close, (), (override));
  MOCK_METHOD(bool, Send, (const DataBuffer& buffer), (override));
  MOCK_METHOD(void,
              SendAsync,
              (DataBuffer buffer,
               absl::AnyInvocable<void(RTCError) &&> on_complete),
              (override));

 protected:
  MockDataChannelInterface() = default;
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_DATA_CHANNEL_H_
