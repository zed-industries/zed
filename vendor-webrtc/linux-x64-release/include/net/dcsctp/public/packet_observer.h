/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_PACKET_OBSERVER_H_
#define NET_DCSCTP_PUBLIC_PACKET_OBSERVER_H_

#include <stdint.h>

#include "api/array_view.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// A PacketObserver can be attached to a socket and will be called for
// all sent and received packets.
class PacketObserver {
 public:
  virtual ~PacketObserver() = default;
  // Called when a packet is sent, with the current time (in milliseconds) as
  // `now`, and the packet payload as `payload`.
  virtual void OnSentPacket(TimeMs now,
                            webrtc::ArrayView<const uint8_t> payload) = 0;

  // Called when a packet is received, with the current time (in milliseconds)
  // as `now`, and the packet payload as `payload`.
  virtual void OnReceivedPacket(TimeMs now,
                                webrtc::ArrayView<const uint8_t> payload) = 0;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_PACKET_OBSERVER_H_
