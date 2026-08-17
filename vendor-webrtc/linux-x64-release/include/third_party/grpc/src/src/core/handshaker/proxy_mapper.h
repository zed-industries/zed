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

#ifndef GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_H
#define GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/resolved_address.h"

namespace grpc_core {

class ProxyMapperInterface {
 public:
  virtual ~ProxyMapperInterface() = default;

  /// Determines the proxy name to resolve for \a server_uri.
  /// If no proxy is needed, returns nullopt.
  /// Otherwise, updates \a args and returns the name to resolve.
  virtual std::optional<std::string> MapName(absl::string_view server_uri,
                                             ChannelArgs* args) = 0;

  /// Determines the proxy address to use to contact \a address.
  /// If no proxy is needed, returns nullopt.
  /// Otherwise, updates \a args, and returns a new address.
  virtual std::optional<grpc_resolved_address> MapAddress(
      const grpc_resolved_address& address, ChannelArgs* args) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_PROXY_MAPPER_H
