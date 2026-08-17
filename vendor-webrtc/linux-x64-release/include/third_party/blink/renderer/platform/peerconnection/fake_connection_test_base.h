// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_FAKE_CONNECTION_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_FAKE_CONNECTION_TEST_BASE_H_

#include <string_view>

#include "base/synchronization/waitable_event.h"
#include "base/test/task_environment.h"
#include "components/webrtc/thread_wrapper.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/webrtc/p2p/base/connection.h"
#include "third_party/webrtc_overrides/p2p/base/fake_connection_factory.h"

namespace blink {

// Base class for test fixtures that need to create fake webrtc connections
// using the fake connection factory.
class FakeConnectionTestBase : public ::testing::Test {
 protected:
  FakeConnectionTestBase() {
    webrtc::ThreadWrapper::EnsureForCurrentMessageLoop();
    EXPECT_NE(webrtc::ThreadWrapper::current(), nullptr);

    base::WaitableEvent ready(base::WaitableEvent::ResetPolicy::MANUAL,
                              base::WaitableEvent::InitialState::NOT_SIGNALED);
    connection_factory_ = std::make_unique<FakeConnectionFactory>(
        webrtc::ThreadWrapper::current(), &ready);
    connection_factory_->Prepare();
    ready.Wait();
  }

  const ::webrtc::Connection* GetConnection(std::string_view remote_ip,
                                            int remote_port) {
    return connection_factory_->CreateConnection(
        webrtc::IceCandidateType::kHost, remote_ip, remote_port);
  }

  ::base::test::SingleThreadTaskEnvironment env{
      ::base::test::TaskEnvironment::TimeSource::MOCK_TIME};

 private:
  std::unique_ptr<FakeConnectionFactory> connection_factory_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_FAKE_CONNECTION_TEST_BASE_H_
