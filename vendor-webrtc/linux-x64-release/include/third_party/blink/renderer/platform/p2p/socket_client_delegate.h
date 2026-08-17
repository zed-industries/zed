// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_DELEGATE_H_

#include "base/time/time.h"
#include "net/base/ip_endpoint.h"
#include "services/network/public/cpp/p2p_socket_type.h"

namespace blink {

// TODO(crbug.com/787254): Consider eliminating this pure virtual class now
// that it has moved to blink/renderer.
class P2PSocketClientDelegate {
 public:
  virtual ~P2PSocketClientDelegate() {}

  // Called after the socket has been opened with the local endpoint address
  // as argument. Please note that in the precence of multiple interfaces,
  // you should not rely on the local endpoint address if possible.
  virtual void OnOpen(const net::IPEndPoint& local_address,
                      const net::IPEndPoint& remote_address) = 0;

  // Called once for each Send() call after the send is complete.
  virtual void OnSendComplete(
      const network::P2PSendPacketMetrics& send_metrics) = 0;

  // Called if an non-retryable error occurs.
  virtual void OnError() = 0;

  // Called when data is received on the socket.
  virtual void OnDataReceived(const net::IPEndPoint& address,
                              base::span<const uint8_t> data,
                              const base::TimeTicks& timestamp,
                              webrtc::EcnMarking ecn) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_SOCKET_CLIENT_DELEGATE_H_
