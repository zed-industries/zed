/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_SERVER_SOCKET_ADAPTERS_H_
#define RTC_BASE_SERVER_SOCKET_ADAPTERS_H_

#include "rtc_base/socket_adapters.h"

namespace webrtc {

// Interface for implementing proxy server sockets.
class AsyncProxyServerSocket : public BufferedReadAdapter {
 public:
  AsyncProxyServerSocket(Socket* socket, size_t buffer_size);
  ~AsyncProxyServerSocket() override;
  sigslot::signal2<AsyncProxyServerSocket*, const SocketAddress&>
      SignalConnectRequest;
  virtual void SendConnectResult(int err, const SocketAddress& addr) = 0;
};

// Implements a socket adapter that performs the server side of a
// fake SSL handshake. Used when implementing a relay server that does "ssltcp".
class AsyncSSLServerSocket : public BufferedReadAdapter {
 public:
  explicit AsyncSSLServerSocket(Socket* socket);

  AsyncSSLServerSocket(const AsyncSSLServerSocket&) = delete;
  AsyncSSLServerSocket& operator=(const AsyncSSLServerSocket&) = delete;

 protected:
  void ProcessInput(char* data, size_t* len) override;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::AsyncProxyServerSocket;
using ::webrtc::AsyncSSLServerSocket;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_SERVER_SOCKET_ADAPTERS_H_
