// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_CRASH_H
#define GRPC_SRC_CORE_UTIL_CRASH_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"
#include "src/core/util/debug_location.h"

namespace grpc_core {

// Crash the program after printing `message`.
// ::grpc_core:: prefix to SourceLocation is required to work around a symbol
// mismatch bug on MSVC.
[[noreturn]] void Crash(absl::string_view message,
                        ::grpc_core::SourceLocation location = {});

[[noreturn]] void CrashWithStdio(absl::string_view message,
                                 ::grpc_core::SourceLocation location = {});

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_CRASH_H
