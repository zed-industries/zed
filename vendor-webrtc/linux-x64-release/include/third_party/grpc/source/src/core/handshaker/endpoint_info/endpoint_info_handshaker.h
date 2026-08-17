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

#ifndef GRPC_SRC_CORE_HANDSHAKER_ENDPOINT_INFO_ENDPOINT_INFO_HANDSHAKER_H
#define GRPC_SRC_CORE_HANDSHAKER_ENDPOINT_INFO_ENDPOINT_INFO_HANDSHAKER_H

#include <grpc/support/port_platform.h>

#include "src/core/config/core_configuration.h"

// Set by the handshaker to indicate the local address of the endpoint.
#define GRPC_ARG_ENDPOINT_LOCAL_ADDRESS "grpc.internal.endpoint_local_address"

// Set by the handshaker to indicate the peer address of the endpoint.
#define GRPC_ARG_ENDPOINT_PEER_ADDRESS "grpc.internal.endpoint_peer_address"

namespace grpc_core {

// Register the endpoint info handshaker into the configuration builder.
void RegisterEndpointInfoHandshaker(CoreConfiguration::Builder* builder);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_ENDPOINT_INFO_ENDPOINT_INFO_HANDSHAKER_H
