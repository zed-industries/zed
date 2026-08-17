/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_TEST_ECHO_SERVER_H_
#define RTC_BASE_TEST_ECHO_SERVER_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <memory>

#include "absl/algorithm/container.h"
#include "absl/memory/memory.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/async_tcp_socket.h"
#include "rtc_base/network/received_packet.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/third_party/sigslot/sigslot.h"
#include "rtc_base/thread.h"

namespace webrtc {

// A test echo server, echoes back any packets sent to it.
// Useful for unit tests.
class TestEchoServer : public sigslot::has_slots<> {
 public:
  TestEchoServer(Thread* thread, const SocketAddress& addr);
  ~TestEchoServer() override;

  TestEchoServer(const TestEchoServer&) = delete;
  TestEchoServer& operator=(const TestEchoServer&) = delete;

  SocketAddress address() const { return server_socket_->GetLocalAddress(); }

 private:
  void OnAccept(Socket* socket) {
    Socket* raw_socket = socket->Accept(nullptr);
    if (raw_socket) {
      AsyncTCPSocket* packet_socket = new AsyncTCPSocket(raw_socket);
      packet_socket->RegisterReceivedPacketCallback(
          [&](AsyncPacketSocket* socket, const ReceivedIpPacket& packet) {
            OnPacket(socket, packet);
          });
      packet_socket->SubscribeCloseEvent(
          this, [this](AsyncPacketSocket* s, int err) { OnClose(s, err); });
      client_sockets_.push_back(packet_socket);
    }
  }
  void OnPacket(AsyncPacketSocket* socket, const ReceivedIpPacket& packet) {
    AsyncSocketPacketOptions options;
    socket->Send(packet.payload().data(), packet.payload().size(), options);
  }
  void OnClose(AsyncPacketSocket* socket, int err) {
    ClientList::iterator it = absl::c_find(client_sockets_, socket);
    client_sockets_.erase(it);
    // `OnClose` is triggered by socket Close callback, deleting `socket` while
    // processing that callback might be unsafe.
    Thread::Current()->PostTask([socket = absl::WrapUnique(socket)] {});
  }

  typedef std::list<AsyncTCPSocket*> ClientList;
  std::unique_ptr<Socket> server_socket_;
  ClientList client_sockets_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::TestEchoServer;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_TEST_ECHO_SERVER_H_
