//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_DIRECTORY_READER_H
#define GRPC_SRC_CORE_UTIL_DIRECTORY_READER_H

#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/functional/function_ref.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace grpc_core {

class DirectoryReader {
 public:
  virtual ~DirectoryReader() = default;
  // Returns the name of the directory being read.
  virtual absl::string_view Name() const = 0;
  // Calls callback for each name in the directory except for "." and "..".
  // Returns non-OK if there was an error reading the directory.
  virtual absl::Status ForEach(
      absl::FunctionRef<void(absl::string_view)> callback) = 0;
};

std::unique_ptr<DirectoryReader> MakeDirectoryReader(
    absl::string_view filename);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_DIRECTORY_READER_H