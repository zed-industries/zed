/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_INCLUDE_RECOVERED_PACKET_RECEIVER_H_
#define MODULES_RTP_RTCP_INCLUDE_RECOVERED_PACKET_RECEIVER_H_

#include "modules/rtp_rtcp/source/rtp_packet_received.h"

namespace webrtc {

// Callback interface for packets recovered by FlexFEC or ULPFEC. In
// the FlexFEC case, the implementation should be able to demultiplex
// the recovered RTP packets based on SSRC.
class RecoveredPacketReceiver {
 public:
  virtual void OnRecoveredPacket(const RtpPacketReceived& packet) = 0;

 protected:
  virtual ~RecoveredPacketReceiver() = default;
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_INCLUDE_RECOVERED_PACKET_RECEIVER_H_
