/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VOIP_VOIP_NETWORK_H_
#define API_VOIP_VOIP_NETWORK_H_

#include <cstdint>

#include "api/array_view.h"
#include "api/voip/voip_base.h"

namespace webrtc {

// VoipNetwork interface provides any network related interfaces such as
// processing received RTP/RTCP packet from remote endpoint. This interface
// requires a ChannelId created via VoipBase interface.
class VoipNetwork {
 public:
  // The data received from the network including RTP header is passed here.
  // Returns following VoipResult;
  //  kOk - received RTP packet is processed.
  //  kInvalidArgument - `channel_id` is invalid.
  virtual VoipResult ReceivedRTPPacket(ChannelId channel_id,
                                       ArrayView<const uint8_t> rtp_packet) = 0;

  // The data received from the network including RTCP header is passed here.
  // Returns following VoipResult;
  //  kOk - received RTCP packet is processed.
  //  kInvalidArgument - `channel_id` is invalid.
  virtual VoipResult ReceivedRTCPPacket(
      ChannelId channel_id,
      ArrayView<const uint8_t> rtcp_packet) = 0;

 protected:
  virtual ~VoipNetwork() = default;
};

}  // namespace webrtc

#endif  // API_VOIP_VOIP_NETWORK_H_
