/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_SOCKET_MOCK_CONTEXT_H_
#define NET_DCSCTP_SOCKET_MOCK_CONTEXT_H_

#include <cstdint>
#include <optional>

#include "absl/strings/string_view.h"
#include "net/dcsctp/packet/sctp_packet.h"
#include "net/dcsctp/public/dcsctp_options.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/socket/context.h"
#include "net/dcsctp/socket/mock_dcsctp_socket_callbacks.h"
#include "test/gmock.h"

namespace dcsctp {

class MockContext : public Context {
 public:
  static constexpr TSN MyInitialTsn() { return TSN(990); }
  static constexpr TSN PeerInitialTsn() { return TSN(10); }
  static constexpr VerificationTag PeerVerificationTag() {
    return VerificationTag(0x01234567);
  }

  explicit MockContext(MockDcSctpSocketCallbacks* callbacks)
      : callbacks_(*callbacks) {
    ON_CALL(*this, is_connection_established)
        .WillByDefault(testing::Return(true));
    ON_CALL(*this, my_initial_tsn)
        .WillByDefault(testing::Return(MyInitialTsn()));
    ON_CALL(*this, peer_initial_tsn)
        .WillByDefault(testing::Return(PeerInitialTsn()));
    ON_CALL(*this, callbacks).WillByDefault(testing::ReturnRef(callbacks_));
    ON_CALL(*this, current_rto)
        .WillByDefault(testing::Return(webrtc::TimeDelta::Millis(123)));
    ON_CALL(*this, Send).WillByDefault([this](SctpPacket::Builder& builder) {
      callbacks_.SendPacketWithStatus(builder.Build());
    });
  }

  MOCK_METHOD(bool, is_connection_established, (), (const, override));
  MOCK_METHOD(TSN, my_initial_tsn, (), (const, override));
  MOCK_METHOD(TSN, peer_initial_tsn, (), (const, override));
  MOCK_METHOD(DcSctpSocketCallbacks&, callbacks, (), (const, override));

  MOCK_METHOD(void, ObserveRTT, (webrtc::TimeDelta rtt), (override));
  MOCK_METHOD(webrtc::TimeDelta, current_rto, (), (const, override));
  MOCK_METHOD(bool,
              IncrementTxErrorCounter,
              (absl::string_view reason),
              (override));
  MOCK_METHOD(void, ClearTxErrorCounter, (), (override));
  MOCK_METHOD(bool, HasTooManyTxErrors, (), (const, override));
  SctpPacket::Builder PacketBuilder() const override {
    return SctpPacket::Builder(PeerVerificationTag(), options_);
  }
  MOCK_METHOD(void, Send, (SctpPacket::Builder & builder), (override));

  DcSctpOptions options_;
  MockDcSctpSocketCallbacks& callbacks_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_SOCKET_MOCK_CONTEXT_H_
