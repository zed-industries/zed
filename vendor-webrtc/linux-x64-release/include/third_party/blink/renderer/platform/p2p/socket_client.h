// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_H_

#include <stdint.h>

#include "net/base/ip_endpoint.h"
#include "services/network/public/cpp/p2p_socket_type.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class P2PSocketClientDelegate;

// P2P socket that routes all calls over IPC.
//
// TODO(crbug.com/787254): Verify whether this class is still needed
// now that all its clients are in Blink.
class P2PSocketClient {
 public:
  virtual ~P2PSocketClient() {}

  // Send the |data| to the |address| using Differentiated Services Code Point
  // |dscp|. Return value is the unique packet_id for this packet.
  virtual uint64_t Send(const net::IPEndPoint& address,
                        base::span<const uint8_t> data,
                        const webrtc::AsyncSocketPacketOptions& options) = 0;

  // Call to complete sending of any batched packets.
  virtual void FlushBatch() = 0;

  virtual void SetOption(network::P2PSocketOption option, int value) = 0;

  // Must be called before the socket is destroyed.
  virtual void Close() = 0;

  virtual int GetSocketID() const = 0;
  virtual void SetDelegate(P2PSocketClientDelegate* delegate) = 0;

 protected:
  P2PSocketClient() {}
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_H_
