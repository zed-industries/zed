//
// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_METRICS_H
#define GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_METRICS_H

#include <grpc/support/port_platform.h>

#include "absl/strings/string_view.h"

namespace grpc_core {

// An interface for XdsClient to report metrics.
class XdsMetricsReporter {
 public:
  virtual ~XdsMetricsReporter() = default;

  virtual void ReportResourceUpdates(absl::string_view xds_server,
                                     absl::string_view resource_type,
                                     uint64_t num_valid,
                                     uint64_t num_invalid) = 0;

  virtual void ReportServerFailure(absl::string_view xds_server) = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_XDS_XDS_CLIENT_XDS_METRICS_H
