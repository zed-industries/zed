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

#ifndef GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_H
#define GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>

#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/iomgr/resolve_address.h"
#include "src/core/lib/resource_quota/memory_quota.h"

typedef struct grpc_tcp_client_vtable {
  int64_t (*connect)(
      grpc_closure* on_connect, grpc_endpoint** endpoint,
      grpc_pollset_set* interested_parties,
      const grpc_event_engine::experimental::EndpointConfig& config,
      const grpc_resolved_address* addr, grpc_core::Timestamp deadline);
  bool (*cancel_connect)(int64_t connection_handle);
} grpc_tcp_client_vtable;

// Asynchronously connect to an address (specified as (addr, len)), and call
// cb with arg and the completed connection when done (or call cb with arg and
// NULL on failure).
// interested_parties points to a set of pollsets that would be interested
// in this connection being established (in order to continue their work). It
// returns a handle to the connect operation which can be used to cancel the
// connection attempt.
int64_t grpc_tcp_client_connect(
    grpc_closure* on_connect, grpc_endpoint** endpoint,
    grpc_pollset_set* interested_parties,
    const grpc_event_engine::experimental::EndpointConfig& config,
    const grpc_resolved_address* addr, grpc_core::Timestamp deadline);

// Returns true if a connect attempt corresponding to the provided handle
// is successfully cancelled. Otherwise it returns false. If the connect
// attempt is successfully cancelled, then the on_connect closure passed to
// grpc_tcp_client_connect will not be executed. Its upto the caller to free
// up any resources that may have been allocated to create the closure.
bool grpc_tcp_client_cancel_connect(int64_t connection_handle);

extern void grpc_tcp_client_global_init();

void grpc_set_tcp_client_impl(grpc_tcp_client_vtable* impl);

#endif  // GRPC_SRC_CORE_LIB_IOMGR_TCP_CLIENT_H
