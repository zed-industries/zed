// Copyright 2022 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_TCP_CLIENT_H
#define GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_TCP_CLIENT_H
#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/resolved_address.h"

namespace grpc_event_engine {
namespace experimental {

/// Attempts to use event engine to connect to the specified remote address and
/// invokes the on_connect callback asynchronously upon connection
/// establishment, failure or timeout. It returns a 64 bit connection handle
/// which can later be used to cancel an in progress connection attempt.
int64_t event_engine_tcp_client_connect(grpc_closure* on_connect,
                                        grpc_endpoint** endpoint,
                                        const EndpointConfig& config,
                                        const grpc_resolved_address* addr,
                                        grpc_core::Timestamp deadline);

/// Attempts to cancel an in progress connection attempt represented by the
/// passed in connection handle. It returns true if the cancellation attempt
/// succeeded. Otherwise it returns false.
bool event_engine_tcp_client_cancel_connect(int64_t connection_handle);

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_TCP_CLIENT_H
