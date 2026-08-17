/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef CALL_RTP_PACKET_SINK_INTERFACE_H_
#define CALL_RTP_PACKET_SINK_INTERFACE_H_

namespace webrtc {

class RtpPacketReceived;

// This class represents a receiver of already parsed RTP packets.
class RtpPacketSinkInterface {
 public:
  virtual ~RtpPacketSinkInterface() = default;
  virtual void OnRtpPacket(const RtpPacketReceived& packet) = 0;
};

}  // namespace webrtc

#endif  // CALL_RTP_PACKET_SINK_INTERFACE_H_
