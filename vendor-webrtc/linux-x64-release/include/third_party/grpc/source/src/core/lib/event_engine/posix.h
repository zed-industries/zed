// Copyright 2022 gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/support/port_platform.h>

#include "src/core/lib/event_engine/extensions/can_track_errors.h"
#include "src/core/lib/event_engine/extensions/chaotic_good_extension.h"
#include "src/core/lib/event_engine/extensions/supports_fd.h"
#include "src/core/lib/event_engine/query_extensions.h"

namespace grpc_event_engine::experimental {

/// This defines an interface that posix specific EventEngines endpoints
/// may implement to support additional chaotic good related functionality.
class PosixEndpointWithChaoticGoodSupport
    : public ExtendedType<EventEngine::Endpoint, ChaoticGoodExtension,
                          EndpointSupportsFdExtension,
                          EndpointCanTrackErrorsExtension> {};

/// This defines an interface that posix specific EventEngines endpoints
/// may implement to support additional file descriptor related functionality.
class PosixEndpointWithFdSupport
    : public ExtendedType<EventEngine::Endpoint, EndpointSupportsFdExtension,
                          EndpointCanTrackErrorsExtension> {};

/// Defines an interface that posix EventEngine listeners may implement to
/// support additional file descriptor related functionality.
class PosixListenerWithFdSupport
    : public ExtendedType<EventEngine::Listener, ListenerSupportsFdExtension> {
};

/// Defines an interface that posix EventEngines may implement to
/// support additional file descriptor related functionality.
class PosixEventEngineWithFdSupport
    : public ExtendedType<EventEngine, EventEngineSupportsFdExtension> {};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_H
