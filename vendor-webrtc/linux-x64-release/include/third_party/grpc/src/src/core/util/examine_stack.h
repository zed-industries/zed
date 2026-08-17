//
//
// Copyright 2020 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_EXAMINE_STACK_H
#define GRPC_SRC_CORE_UTIL_EXAMINE_STACK_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

namespace grpc_core {

typedef std::string (*gpr_current_stack_trace_func)();

// Returns a current_stack_trace_provider.
gpr_current_stack_trace_func GetCurrentStackTraceProvider();

// Sets current_stack_trace_provider which provides a current-stack trace.
void SetCurrentStackTraceProvider(
    gpr_current_stack_trace_func current_stack_trace_provider);

// Returns the current stack trace as a string via current_stack_trace_provider
// If current_stack_trace_provider is not set, it returns std::nullopt.
std::optional<std::string> GetCurrentStackTrace();

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_EXAMINE_STACK_H
