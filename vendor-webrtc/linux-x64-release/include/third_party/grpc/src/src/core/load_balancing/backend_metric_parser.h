//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_PARSER_H
#define GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_PARSER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include "absl/strings/string_view.h"
#include "src/core/load_balancing/backend_metric_data.h"

namespace grpc_core {

class BackendMetricAllocatorInterface {
 public:
  virtual ~BackendMetricAllocatorInterface() = default;

  virtual BackendMetricData* AllocateBackendMetricData() = 0;

  virtual char* AllocateString(size_t size) = 0;
};

// Parses the serialized load report and populates out.
// Returns false on error.
const BackendMetricData* ParseBackendMetricData(
    absl::string_view serialized_load_report,
    BackendMetricAllocatorInterface* allocator);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_PARSER_H
