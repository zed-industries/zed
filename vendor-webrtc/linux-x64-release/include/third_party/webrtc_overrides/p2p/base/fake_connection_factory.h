// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_FAKE_CONNECTION_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_FAKE_CONNECTION_H_

#include <cstdint>
#include <memory>
#include <string_view>
#include <vector>

#include "base/synchronization/waitable_event.h"
#include "third_party/webrtc/api/candidate.h"
#include "third_party/webrtc/api/task_queue/task_queue_base.h"
#include "third_party/webrtc/p2p/base/connection.h"
#include "third_party/webrtc/p2p/base/port_allocator.h"
#include "third_party/webrtc/p2p/base/port_interface.h"
#include "third_party/webrtc/rtc_base/socket_factory.h"
#include "third_party/webrtc/rtc_base/third_party/sigslot/sigslot.h"

namespace blink {

// Generates simulated connection objects for use in tests.
class FakeConnectionFactory : public sigslot::has_slots<> {
 public:
  // The factory must be initialized by calling Prepare(). readyEvent will be
  // signaled when the factory is ready to start creating connections.
  explicit FakeConnectionFactory(webrtc::TaskQueueBase* thread,
                                 base::WaitableEvent* readyEvent);

  // Start a port allocation session to generate port(s) from which connections
  // may be created.
  void Prepare(uint32_t allocator_flags = webrtc::kDefaultPortAllocatorFlags);

  // Create a connection to a remote candidate represented as the type, IP
  // address, port, and an optional candidate priority.
  webrtc::Connection* CreateConnection(webrtc::IceCandidateType type,
                                       std::string_view remote_ip,
                                       int remote_port,
                                       int priority = 0);

  // Count of created ports.
  int port_count() { return ports_.size(); }

 private:
  void OnPortReady(webrtc::PortAllocatorSession* session,
                   webrtc::PortInterface* port);

  webrtc::Candidate CreateUdpCandidate(webrtc::IceCandidateType type,
                                       std::string_view ip,
                                       int port,
                                       int priority,
                                       std::string_view ufrag = "");

  base::WaitableEvent* readyEvent_;

  std::unique_ptr<webrtc::SocketFactory> sf_;
  std::unique_ptr<webrtc::PortAllocator> allocator_;
  std::vector<std::unique_ptr<webrtc::PortAllocatorSession>> sessions_;
  std::vector<webrtc::PortInterface*> ports_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_FAKE_CONNECTION_H_
