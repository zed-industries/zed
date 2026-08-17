//
// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_XDS_HTTP_PROXY_MAPPER_H
#define GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_XDS_HTTP_PROXY_MAPPER_H

#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "src/core/config/core_configuration.h"
#include "src/core/handshaker/proxy_mapper.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/resolved_address.h"

namespace grpc_core {

class XdsHttpProxyMapper final : public ProxyMapperInterface {
 public:
  std::optional<std::string> MapName(absl::string_view /*server_uri*/,
                                     ChannelArgs* /*args*/) override {
    return std::nullopt;
  }

  std::optional<grpc_resolved_address> MapAddress(
      const grpc_resolved_address& address, ChannelArgs* args) override;
};

void RegisterXdsHttpProxyMapper(CoreConfiguration::Builder* builder);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_XDS_HTTP_PROXY_MAPPER_H
