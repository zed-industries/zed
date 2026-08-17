//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_WINDOWS_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_WINDOWS_H
//
// Low level TCP "bottom half" implementation, for use by transports built on
// top of a TCP connection.

// Note that this file does not (yet) include APIs for creating the socket in
// the first place.

// All calls passing slice transfer ownership of a slice refcount unless
// otherwise specified.
//

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_WINSOCK_SOCKET
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/socket_windows.h"

// Create a tcp endpoint given a winsock handle.
// Takes ownership of the handle.
//
grpc_endpoint* grpc_tcp_create(grpc_winsocket* socket,
                               absl::string_view peer_string);

grpc_error_handle grpc_tcp_prepare_socket(SOCKET sock);

grpc_error_handle grpc_tcp_set_non_block(SOCKET sock);

#endif

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_WINDOWS_H
