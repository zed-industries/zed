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

#ifndef GRPC_SRC_CORE_UTIL_JSON_JSON_WRITER_H
#define GRPC_SRC_CORE_UTIL_JSON_JSON_WRITER_H

#include <grpc/support/port_platform.h>

#include <string>

#include "src/core/util/json/json.h"

namespace grpc_core {

// Dumps JSON from value to string form.
std::string JsonDump(const Json& json, int indent = 0);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_JSON_JSON_WRITER_H
