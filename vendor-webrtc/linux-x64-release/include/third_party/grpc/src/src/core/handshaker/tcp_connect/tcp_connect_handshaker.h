//
// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_HANDSHAKER_TCP_CONNECT_TCP_CONNECT_HANDSHAKER_H
#define GRPC_SRC_CORE_HANDSHAKER_TCP_CONNECT_TCP_CONNECT_HANDSHAKER_H

#include <grpc/support/port_platform.h>

#include "src/core/config/core_configuration.h"

// Indicates the address that the tcp connect handshaker should connect to.
#define GRPC_ARG_TCP_HANDSHAKER_RESOLVED_ADDRESS \
  "grpc.internal.tcp_handshaker_resolved_address"

// Whether the TCP connect handshaker should bind the endpoint to the pollset.
#define GRPC_ARG_TCP_HANDSHAKER_BIND_ENDPOINT_TO_POLLSET \
  "grpc.internal.tcp_handshaker_bind_endpoint_to_pollset"

namespace grpc_core {

// Register the TCP Connect handshaker into the configuration builder.
void RegisterTCPConnectHandshaker(CoreConfiguration::Builder* builder);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_TCP_CONNECT_TCP_CONNECT_HANDSHAKER_H
