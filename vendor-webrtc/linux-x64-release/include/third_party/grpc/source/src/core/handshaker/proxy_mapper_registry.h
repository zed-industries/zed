//
//
// Copyright 2017 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_REGISTRY_H
#define GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "src/core/handshaker/proxy_mapper.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/resolved_address.h"

namespace grpc_core {

class ProxyMapperRegistry {
  using ProxyMapperList = std::vector<std::unique_ptr<ProxyMapperInterface>>;

 public:
  class Builder {
   public:
    /// Registers a new proxy mapper.
    /// If \a at_start is true, the new mapper will be at the beginning of
    /// the list.  Otherwise, it will be added to the end.
    void Register(bool at_start, std::unique_ptr<ProxyMapperInterface> mapper);

    ProxyMapperRegistry Build();

   private:
    ProxyMapperList mappers_;
  };

  ~ProxyMapperRegistry() = default;
  ProxyMapperRegistry(const ProxyMapperRegistry&) = delete;
  ProxyMapperRegistry& operator=(const ProxyMapperRegistry&) = delete;
  ProxyMapperRegistry(ProxyMapperRegistry&&) = default;
  ProxyMapperRegistry& operator=(ProxyMapperRegistry&&) = default;

  std::optional<std::string> MapName(absl::string_view server_uri,
                                     ChannelArgs* args) const;

  std::optional<grpc_resolved_address> MapAddress(
      const grpc_resolved_address& address, ChannelArgs* args) const;

 private:
  ProxyMapperRegistry() = default;

  ProxyMapperList mappers_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_REGISTRY_H
