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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_REF_COUNTED_DNS_RESOLVER_INTERFACE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_REF_COUNTED_DNS_RESOLVER_INTERFACE_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "absl/strings/string_view.h"
#include "src/core/util/orphanable.h"

namespace grpc_event_engine::experimental {

class RefCountedDNSResolverInterface
    : public grpc_core::InternallyRefCounted<RefCountedDNSResolverInterface> {
 public:
  explicit RefCountedDNSResolverInterface(const char* trace = nullptr,
                                          intptr_t initial_refcount = 1)
      : grpc_core::InternallyRefCounted<RefCountedDNSResolverInterface>(
            trace, initial_refcount) {}

  virtual void LookupHostname(
      EventEngine::DNSResolver::LookupHostnameCallback on_resolved,
      absl::string_view name, absl::string_view default_port) = 0;

  virtual void LookupSRV(
      EventEngine::DNSResolver::LookupSRVCallback on_resolved,
      absl::string_view name) = 0;

  virtual void LookupTXT(
      EventEngine::DNSResolver::LookupTXTCallback on_resolved,
      absl::string_view name) = 0;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_REF_COUNTED_DNS_RESOLVER_INTERFACE_H
