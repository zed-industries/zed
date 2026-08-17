/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef CALL_PACKET_RECEIVER_H_
#define CALL_PACKET_RECEIVER_H_

#include "absl/functional/any_invocable.h"
#include "api/media_types.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

class PacketReceiver {
 public:
  // Demux RTCP packets. Must be called on the worker thread.
  virtual void DeliverRtcpPacket(CopyOnWriteBuffer packet) = 0;

  // Invoked once when a packet is received that can not be demuxed.
  // If the method returns true, a new attempt is made to demux the packet.
  using OnUndemuxablePacketHandler =
      absl::AnyInvocable<bool(const RtpPacketReceived& parsed_packet)>;

  // Must be called on the worker thread.
  // If `media_type` is not Audio or Video, packets may be used for BWE
  // calculations but are not demuxed.
  virtual void DeliverRtpPacket(
      MediaType media_type,
      RtpPacketReceived packet,
      OnUndemuxablePacketHandler undemuxable_packet_handler) = 0;

 protected:
  virtual ~PacketReceiver() {}
};

}  // namespace webrtc

#endif  // CALL_PACKET_RECEIVER_H_
