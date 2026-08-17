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

#ifndef GRPC_SRC_CORE_UTIL_UUID_V4_H
#define GRPC_SRC_CORE_UTIL_UUID_V4_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <string>

namespace grpc_core {

// Generates string in the UUIDv4 form. \a hi and \a lo are expected to be
// random numbers.
std::string GenerateUUIDv4(uint64_t hi, uint64_t lo);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_UUID_V4_H
