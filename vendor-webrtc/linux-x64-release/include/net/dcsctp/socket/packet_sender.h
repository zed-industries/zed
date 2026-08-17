/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_SOCKET_PACKET_SENDER_H_
#define NET_DCSCTP_SOCKET_PACKET_SENDER_H_

#include "net/dcsctp/packet/sctp_packet.h"
#include "net/dcsctp/public/dcsctp_socket.h"

namespace dcsctp {

// The PacketSender sends packets to the network using the provided callback
// interface. When an attempt to send a packet is made, the `on_sent_packet`
// callback will be triggered.
class PacketSender {
 public:
  PacketSender(DcSctpSocketCallbacks& callbacks,
               std::function<void(webrtc::ArrayView<const uint8_t>,
                                  SendPacketStatus)> on_sent_packet);

  // Sends the packet, and returns true if it was sent successfully.
  bool Send(SctpPacket::Builder& builder, bool write_checksum = true);

 private:
  DcSctpSocketCallbacks& callbacks_;

  // Callback that will be triggered for every send attempt, indicating the
  // status of the operation.
  std::function<void(webrtc::ArrayView<const uint8_t>, SendPacketStatus)>
      on_sent_packet_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_SOCKET_PACKET_SENDER_H_
