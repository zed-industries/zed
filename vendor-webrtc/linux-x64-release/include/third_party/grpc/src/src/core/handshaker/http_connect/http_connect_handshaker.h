//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_HTTP_CONNECT_HANDSHAKER_H
#define GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_HTTP_CONNECT_HANDSHAKER_H

#include <grpc/support/port_platform.h>

#include "src/core/config/core_configuration.h"

/// Channel arg indicating the server in HTTP CONNECT request (string).
/// The presence of this arg triggers the use of HTTP CONNECT.
#define GRPC_ARG_HTTP_CONNECT_SERVER "grpc.http_connect_server"

/// Channel arg indicating HTTP CONNECT headers (string).
/// Multiple headers are separated by newlines.  Key/value pairs are
/// separated by colons.
#define GRPC_ARG_HTTP_CONNECT_HEADERS "grpc.http_connect_headers"

namespace grpc_core {

// Register the HTTP Connect handshaker into the configuration builder.
void RegisterHttpConnectHandshaker(CoreConfiguration::Builder* builder);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_HANDSHAKER_HTTP_CONNECT_HTTP_CONNECT_HANDSHAKER_H
