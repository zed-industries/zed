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

#ifndef GRPC_SRC_CORE_UTIL_ENV_H
#define GRPC_SRC_CORE_UTIL_ENV_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

namespace grpc_core {

// Gets the environment variable value with the specified name. */
std::optional<std::string> GetEnv(const char* name);

// Sets the environment with the specified name to the specified value.
void SetEnv(const char* name, const char* value);
inline void SetEnv(const char* name, const std::string& value) {
  SetEnv(name, value.c_str());
}

// Deletes the variable name from the environment.
void UnsetEnv(const char* name);

template <typename T>
void SetOrUnsetEnv(const char* name, const std::optional<T>& value) {
  if (value.has_value()) {
    SetEnv(name, value.value());
  } else {
    UnsetEnv(name);
  }
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_ENV_H
