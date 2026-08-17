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
#ifndef GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_CLOSURE_H
#define GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_CLOSURE_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/support/port_platform.h>

#include "absl/functional/any_invocable.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"

namespace grpc_event_engine {
namespace experimental {

/// Runs a grpc_closure inline with the specified error handle.
void RunEventEngineClosure(grpc_closure* closure, grpc_error_handle error);

/// Creates a callback that takes an error status argument.
absl::AnyInvocable<void(absl::Status)> GrpcClosureToStatusCallback(
    grpc_closure* closure);

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_SRC_CORE_LIB_IOMGR_EVENT_ENGINE_SHIMS_CLOSURE_H
