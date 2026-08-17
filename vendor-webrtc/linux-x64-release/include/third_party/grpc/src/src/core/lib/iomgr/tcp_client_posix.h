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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_POSIX_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_POSIX_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/ev_posix.h"
#include "src/core/lib/iomgr/socket_utils_posix.h"
#include "src/core/lib/iomgr/tcp_client.h"

// Create an endpoint from a connected grpc_fd.

// fd: a connected FD. Ownership is taken.
// config: may contain custom settings for the endpoint
// addr_str: destination address in printable format
// slice_allocator: ownership is taken by client.
// Returns: a new endpoint
//
grpc_endpoint* grpc_tcp_create_from_fd(
    grpc_fd* fd, const grpc_event_engine::experimental::EndpointConfig& config,
    absl::string_view addr_str);

// Return a configured, unbound, unconnected TCP client fd.

// options: may contain custom settings for the fd
// addr: the destination address
// mapped_addr: out parameter. addr mapped to an address appropriate to the
//   type of socket FD created. For example, if addr is IPv4 and dual stack
//   sockets are available, mapped_addr will be an IPv4-mapped IPv6 address
// fd: out parameter. The new FD
// Returns: error, if any. Out parameters are not set on error
//
grpc_error_handle grpc_tcp_client_prepare_fd(
    const grpc_core::PosixTcpOptions& options,
    const grpc_resolved_address* addr, grpc_resolved_address* mapped_addr,
    int* fd);

// Connect a configured TCP client fd.

// interested_parties: a set of pollsets that would be interested in this
//   connection being established (in order to continue their work
// closure: called when complete. On success, *ep will be set.
// fd: an FD returned from grpc_tcp_client_prepare_fd().
// options: may contain custom settings for the endpoint
// deadline: connection deadline
// ep: out parameter. Set before closure is called if successful
//
int64_t grpc_tcp_client_create_from_prepared_fd(
    grpc_pollset_set* interested_parties, grpc_closure* closure, const int fd,
    const grpc_event_engine::experimental::EndpointConfig& config,
    const grpc_resolved_address* addr, grpc_core::Timestamp deadline,
    grpc_endpoint** ep);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_POSIX_H
