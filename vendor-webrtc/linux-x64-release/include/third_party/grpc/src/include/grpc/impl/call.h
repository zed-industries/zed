// Copyright 2023 The gRPC Authors
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

#ifndef GRPC_IMPL_CALL_H
#define GRPC_IMPL_CALL_H

#include <grpc/grpc.h>
#include <grpc/support/port_platform.h>

#include "absl/functional/any_invocable.h"

// Run a callback in the call's EventEngine.
// Internal-only
void grpc_call_run_in_event_engine(const grpc_call* call,
                                   absl::AnyInvocable<void()> cb);

#endif /* GRPC_IMPL_CALL_H */
