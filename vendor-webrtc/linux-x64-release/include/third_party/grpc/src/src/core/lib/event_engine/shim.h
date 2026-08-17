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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_SHIM_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_SHIM_H

// Platform-specific configuration for use of the EventEngine shims.

#include <grpc/support/port_platform.h>

namespace grpc_event_engine::experimental {

// Returns true if the EventEngine client shim should be used.
// This is based on the experiment state, and compile-time configurations that
// may disable the poller in some builds.
bool UseEventEngineClient();

// Returns true if the EventEngine listener shim should be used.
// This is based on the experiment state, and compile-time configurations that
// may disable the poller in some builds.
bool UseEventEngineListener();

// Returns true if the pollset alternative experiment should be used.
// This is based on the experiment state, and compile-time configurations that
// may disable the poller in some builds.
bool UsePollsetAlternative();

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_SHIM_H
