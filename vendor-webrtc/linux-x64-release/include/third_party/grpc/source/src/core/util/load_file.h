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

#ifndef GRPC_SRC_CORE_UTIL_LOAD_FILE_H
#define GRPC_SRC_CORE_UTIL_LOAD_FILE_H

#include <grpc/support/port_platform.h>

#include <string>

#include "absl/status/statusor.h"
#include "src/core/lib/slice/slice.h"

namespace grpc_core {

// Loads the content of a file into a slice. add_null_terminator will add a NULL
// terminator if true.
absl::StatusOr<Slice> LoadFile(const std::string& filename,
                               bool add_null_terminator);
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_LOAD_FILE_H
