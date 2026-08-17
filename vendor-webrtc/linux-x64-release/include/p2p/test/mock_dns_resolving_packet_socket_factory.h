/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_MOCK_DNS_RESOLVING_PACKET_SOCKET_FACTORY_H_
#define P2P_TEST_MOCK_DNS_RESOLVING_PACKET_SOCKET_FACTORY_H_

#include <functional>
#include <memory>

#include "api/async_dns_resolver.h"
#include "api/test/mock_async_dns_resolver.h"
#include "p2p/base/basic_packet_socket_factory.h"
#include "rtc_base/socket_factory.h"

namespace webrtc {

// A PacketSocketFactory implementation for tests that uses a mock DnsResolver
// and allows setting expectations on the resolver and results.
class MockDnsResolvingPacketSocketFactory : public BasicPacketSocketFactory {
 public:
  using Expectations =
      std::function<void(MockAsyncDnsResolver*, MockAsyncDnsResolverResult*)>;

  explicit MockDnsResolvingPacketSocketFactory(SocketFactory* socket_factory)
      : BasicPacketSocketFactory(socket_factory) {}

  std::unique_ptr<AsyncDnsResolverInterface> CreateAsyncDnsResolver() override {
    std::unique_ptr<MockAsyncDnsResolver> resolver =
        std::make_unique<MockAsyncDnsResolver>();
    if (expectations_) {
      expectations_(resolver.get(), &resolver_result_);
    }
    return resolver;
  }

  void SetExpectations(Expectations expectations) {
    expectations_ = expectations;
  }

 private:
  MockAsyncDnsResolverResult resolver_result_;
  Expectations expectations_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::MockDnsResolvingPacketSocketFactory;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_MOCK_DNS_RESOLVING_PACKET_SOCKET_FACTORY_H_
