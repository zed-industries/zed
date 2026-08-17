/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_FAKE_NETWORK_SOCKET_SERVER_H_
#define TEST_NETWORK_FAKE_NETWORK_SOCKET_SERVER_H_

#include <vector>

#include "api/units/time_delta.h"
#include "rtc_base/event.h"
#include "rtc_base/ip_address.h"
#include "rtc_base/socket.h"
#include "rtc_base/socket_server.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "test/network/network_emulation.h"

namespace webrtc {
namespace test {
class FakeNetworkSocket;

// FakeNetworkSocketServer must outlive any sockets it creates.
class FakeNetworkSocketServer : public SocketServer {
 public:
  explicit FakeNetworkSocketServer(EndpointsContainer* endpoints_controller);
  ~FakeNetworkSocketServer() override;

  // webrtc::SocketFactory methods:
  Socket* CreateSocket(int family, int type) override;

  // webrtc::SocketServer methods:
  // Called by the network thread when this server is installed, kicking off the
  // message handler loop.
  void SetMessageQueue(Thread* thread) override;
  bool Wait(webrtc::TimeDelta max_wait_duration, bool process_io) override;
  void WakeUp() override;

 protected:
  friend class FakeNetworkSocket;
  EmulatedEndpointImpl* GetEndpointNode(const IPAddress& ip);
  void Unregister(FakeNetworkSocket* socket);

 private:
  const EndpointsContainer* endpoints_container_;
  Event wakeup_;
  Thread* thread_ = nullptr;

  Mutex lock_;
  std::vector<FakeNetworkSocket*> sockets_ RTC_GUARDED_BY(lock_);
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_NETWORK_FAKE_NETWORK_SOCKET_SERVER_H_
