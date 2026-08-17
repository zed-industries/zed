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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_POSIX_H
//
// Low level TCP "bottom half" implementation, for use by transports built on
// top of a TCP connection.

// Note that this file does not (yet) include APIs for creating the socket in
// the first place.

// All calls passing slice transfer ownership of a slice refcount unless
// otherwise specified.
//

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/buffer_list.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/ev_posix.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/iomgr/socket_utils_posix.h"

/// [DEPRECATED] Create a tcp endpoint given a file descriptor and a read slice
/// size. Takes ownership of \a fd. This is a legacy API, intended for iomgr use
/// only, it will not support EventEngine.
grpc_endpoint* grpc_tcp_create(grpc_fd* fd,
                               const grpc_core::PosixTcpOptions& options,
                               absl::string_view peer_string);

/// Create a tcp endpoint given a file descriptor and a read slice size.
/// Takes ownership of \a fd.
grpc_endpoint* grpc_tcp_create(
    grpc_fd* fd, const grpc_event_engine::experimental::EndpointConfig& config,
    absl::string_view peer_string);

/// Return the tcp endpoint's fd, or -1 if this is not available. Does not
/// release the fd. Requires: \a ep must be a tcp endpoint.
int grpc_tcp_fd(grpc_endpoint* ep);

/// Destroy the tcp endpoint without closing its fd. *fd will be set and done
/// will be called when the endpoint is destroyed. Requires: \a ep must be a tcp
/// endpoint and fd must not be NULL.
void grpc_tcp_destroy_and_release_fd(grpc_endpoint* ep, int* fd,
                                     grpc_closure* done);

#ifdef GRPC_POSIX_SOCKET_TCP

void grpc_tcp_posix_init();

void grpc_tcp_posix_shutdown();

#endif  // GRPC_POSIX_SOCKET_TCP

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_POSIX_H
