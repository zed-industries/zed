/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_H_
#define NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_H_

#include <vector>

#include "net/dcsctp/public/dcsctp_socket.h"
#include "test/gmock.h"

namespace dcsctp {

class MockDcSctpSocket : public DcSctpSocketInterface {
 public:
  MOCK_METHOD(void,
              ReceivePacket,
              (webrtc::ArrayView<const uint8_t> data),
              (override));

  MOCK_METHOD(void, HandleTimeout, (TimeoutID timeout_id), (override));

  MOCK_METHOD(void, Connect, (), (override));

  MOCK_METHOD(void,
              RestoreFromState,
              (const DcSctpSocketHandoverState&),
              (override));

  MOCK_METHOD(void, Shutdown, (), (override));

  MOCK_METHOD(void, Close, (), (override));

  MOCK_METHOD(SocketState, state, (), (const, override));

  MOCK_METHOD(const DcSctpOptions&, options, (), (const, override));

  MOCK_METHOD(void, SetMaxMessageSize, (size_t max_message_size), (override));

  MOCK_METHOD(void,
              SetStreamPriority,
              (StreamID stream_id, StreamPriority priority),
              (override));

  MOCK_METHOD(StreamPriority,
              GetStreamPriority,
              (StreamID stream_id),
              (const, override));

  MOCK_METHOD(SendStatus,
              Send,
              (DcSctpMessage message, const SendOptions& send_options),
              (override));

  MOCK_METHOD(std::vector<SendStatus>,
              SendMany,
              (webrtc::ArrayView<DcSctpMessage> messages,
               const SendOptions& send_options),
              (override));

  MOCK_METHOD(ResetStreamsStatus,
              ResetStreams,
              (webrtc::ArrayView<const StreamID> outgoing_streams),
              (override));

  MOCK_METHOD(size_t, buffered_amount, (StreamID stream_id), (const, override));

  MOCK_METHOD(size_t,
              buffered_amount_low_threshold,
              (StreamID stream_id),
              (const, override));

  MOCK_METHOD(void,
              SetBufferedAmountLowThreshold,
              (StreamID stream_id, size_t bytes),
              (override));

  MOCK_METHOD(std::optional<Metrics>, GetMetrics, (), (const, override));

  MOCK_METHOD(HandoverReadinessStatus,
              GetHandoverReadiness,
              (),
              (const, override));
  MOCK_METHOD(std::optional<DcSctpSocketHandoverState>,
              GetHandoverStateAndClose,
              (),
              (override));
};

}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_MOCK_DCSCTP_SOCKET_H_
