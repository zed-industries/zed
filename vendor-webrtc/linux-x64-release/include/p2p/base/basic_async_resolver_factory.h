/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_BASIC_ASYNC_RESOLVER_FACTORY_H_
#define P2P_BASE_BASIC_ASYNC_RESOLVER_FACTORY_H_

#include <memory>

#include "absl/functional/any_invocable.h"
#include "api/async_dns_resolver.h"
#include "rtc_base/socket_address.h"

namespace webrtc {

// A factory that vends AsyncDnsResolver instances.
class BasicAsyncDnsResolverFactory final
    : public AsyncDnsResolverFactoryInterface {
 public:
  BasicAsyncDnsResolverFactory() = default;

  std::unique_ptr<webrtc::AsyncDnsResolverInterface> CreateAndResolve(
      const SocketAddress& addr,
      absl::AnyInvocable<void()> callback) override;

  std::unique_ptr<webrtc::AsyncDnsResolverInterface> CreateAndResolve(
      const SocketAddress& addr,
      int family,
      absl::AnyInvocable<void()> callback) override;

  std::unique_ptr<webrtc::AsyncDnsResolverInterface> Create() override;
};

}  // namespace webrtc

#endif  // P2P_BASE_BASIC_ASYNC_RESOLVER_FACTORY_H_
