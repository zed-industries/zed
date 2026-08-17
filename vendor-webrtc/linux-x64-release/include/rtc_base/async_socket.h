/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_ASYNC_SOCKET_H_
#define RTC_BASE_ASYNC_SOCKET_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/third_party/sigslot/sigslot.h"

namespace webrtc {

class AsyncSocketAdapter : public Socket, public sigslot::has_slots<> {
 public:
  // Takes ownership of the passed in socket.
  // TODO(bugs.webrtc.org/6424): Change to unique_ptr here and in callers.
  explicit AsyncSocketAdapter(Socket* socket);

  SocketAddress GetLocalAddress() const override;
  SocketAddress GetRemoteAddress() const override;
  int Bind(const SocketAddress& addr) override;
  int Connect(const SocketAddress& addr) override;
  int Send(const void* pv, size_t cb) override;
  int SendTo(const void* pv, size_t cb, const SocketAddress& addr) override;
  int Recv(void* pv, size_t cb, int64_t* timestamp) override;
  int RecvFrom(void* pv,
               size_t cb,
               SocketAddress* paddr,
               int64_t* timestamp) override;
  int Listen(int backlog) override;
  Socket* Accept(SocketAddress* paddr) override;
  int Close() override;
  int GetError() const override;
  void SetError(int error) override;
  ConnState GetState() const override;
  int GetOption(Option opt, int* value) override;
  int SetOption(Option opt, int value) override;

 protected:
  virtual void OnConnectEvent(Socket* socket);
  virtual void OnReadEvent(Socket* socket);
  virtual void OnWriteEvent(Socket* socket);
  virtual void OnCloseEvent(Socket* socket, int err);

  Socket* GetSocket() const { return socket_.get(); }

 private:
  const std::unique_ptr<Socket> socket_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::AsyncSocketAdapter;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_ASYNC_SOCKET_H_
