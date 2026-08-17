/*
 *  Copyright 2021 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_ASYNC_DNS_RESOLVER_H_
#define API_ASYNC_DNS_RESOLVER_H_

#include <memory>

#include "absl/functional/any_invocable.h"
#include "rtc_base/socket_address.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// This interface defines the methods to resolve a hostname asynchronously.
// The AsyncDnsResolverInterface class encapsulates a single name query.
//
// Usage:
//   std::unique_ptr<AsyncDnsResolverInterface> resolver =
//        factory->Create(address-to-be-resolved, [r = resolver.get()]() {
//     if (r->result.GetResolvedAddress(AF_INET, &addr) {
//       // success
//     } else {
//       // failure
//       error = r->result().GetError();
//     }
//     // Release resolver.
//     resolver_list.erase(std::remove_if(resolver_list.begin(),
//     resolver_list.end(),
//                         [](refptr) { refptr.get() == r; });
//   });
//   resolver_list.push_back(std::move(resolver));

class AsyncDnsResolverResult {
 public:
  virtual ~AsyncDnsResolverResult() = default;
  // Returns true iff the address from `Start` was successfully resolved.
  // If the address was successfully resolved, sets `addr` to a copy of the
  // address from `Start` with the IP address set to the top most resolved
  // address of `family` (`addr` will have both hostname and the resolved ip).
  virtual bool GetResolvedAddress(int family, SocketAddress* addr) const = 0;
  // Returns error from resolver.
  virtual int GetError() const = 0;
};

// The API for a single name query.
// The constructor, destructor and all functions must be called from
// the same sequence, and the callback will also be called on that sequence.
// The class guarantees that the callback will not be called if the
// resolver's destructor has been called.
class RTC_EXPORT AsyncDnsResolverInterface {
 public:
  virtual ~AsyncDnsResolverInterface() = default;

  // Start address resolution of the hostname in `addr`.
  virtual void Start(const SocketAddress& addr,
                     absl::AnyInvocable<void()> callback) = 0;
  // Start address resolution of the hostname in `addr` matching `family`.
  virtual void Start(const SocketAddress& addr,
                     int family,
                     absl::AnyInvocable<void()> callback) = 0;
  virtual const AsyncDnsResolverResult& result() const = 0;
};

// An abstract factory for creating AsyncDnsResolverInterfaces. This allows
// client applications to provide WebRTC with their own mechanism for
// performing DNS resolution.
class AsyncDnsResolverFactoryInterface {
 public:
  virtual ~AsyncDnsResolverFactoryInterface() = default;

  // Creates an AsyncDnsResolver and starts resolving the name. The callback
  // will be called when resolution is finished.
  // The callback will be called on the sequence that the caller runs on.
  virtual std::unique_ptr<webrtc::AsyncDnsResolverInterface> CreateAndResolve(
      const SocketAddress& addr,
      absl::AnyInvocable<void()> callback) = 0;
  // Creates an AsyncDnsResolver and starts resolving the name to an address
  // matching the specified family. The callback will be called when resolution
  // is finished. The callback will be called on the sequence that the caller
  // runs on.
  virtual std::unique_ptr<webrtc::AsyncDnsResolverInterface> CreateAndResolve(
      const SocketAddress& addr,
      int family,
      absl::AnyInvocable<void()> callback) = 0;
  // Creates an AsyncDnsResolver and does not start it.
  // For backwards compatibility, will be deprecated and removed.
  // One has to do a separate Start() call on the
  // resolver to start name resolution.
  virtual std::unique_ptr<webrtc::AsyncDnsResolverInterface> Create() = 0;
};

}  // namespace webrtc

#endif  // API_ASYNC_DNS_RESOLVER_H_
