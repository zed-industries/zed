// Copyright 2023 The gRPC Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_NATIVE_POSIX_DNS_RESOLVER_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_NATIVE_POSIX_DNS_RESOLVER_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/strings/string_view.h"
#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_POSIX_SOCKET_RESOLVE_ADDRESS

#include <grpc/event_engine/event_engine.h>

namespace grpc_event_engine::experimental {

absl::StatusOr<std::vector<EventEngine::ResolvedAddress>>
LookupHostnameBlocking(absl::string_view name, absl::string_view default_port);

// An asynchronous DNS resolver which uses the native platform's getaddrinfo
// API. Only supports A/AAAA records.
class NativePosixDNSResolver : public EventEngine::DNSResolver {
 public:
  explicit NativePosixDNSResolver(std::shared_ptr<EventEngine> event_engine);

  void LookupHostname(
      EventEngine::DNSResolver::LookupHostnameCallback on_resolved,
      absl::string_view name, absl::string_view default_port) override;

  void LookupSRV(EventEngine::DNSResolver::LookupSRVCallback on_resolved,
                 absl::string_view name) override;

  void LookupTXT(EventEngine::DNSResolver::LookupTXTCallback on_resolved,
                 absl::string_view name) override;

 private:
  std::shared_ptr<EventEngine> event_engine_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_POSIX_SOCKET_RESOLVE_ADDRESS
#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_NATIVE_POSIX_DNS_RESOLVER_H
