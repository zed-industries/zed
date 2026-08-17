// Copyright 2023 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_RESOLVER_DNS_EVENT_ENGINE_EVENT_ENGINE_CLIENT_CHANNEL_RESOLVER_H
#define GRPC_SRC_CORE_RESOLVER_DNS_EVENT_ENGINE_EVENT_ENGINE_CLIENT_CHANNEL_RESOLVER_H
#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/resolver/resolver.h"
#include "src/core/resolver/resolver_factory.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/uri.h"

namespace grpc_core {

class EventEngineClientChannelDNSResolverFactory final
    : public ResolverFactory {
 public:
  absl::string_view scheme() const override { return "dns"; }
  bool IsValidUri(const URI& uri) const override;
  OrphanablePtr<Resolver> CreateResolver(ResolverArgs args) const override;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_DNS_EVENT_ENGINE_EVENT_ENGINE_CLIENT_CHANNEL_RESOLVER_H
