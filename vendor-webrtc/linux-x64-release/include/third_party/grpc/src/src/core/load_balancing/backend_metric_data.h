//
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
//

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_DATA_H
#define GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_DATA_H

#include <grpc/support/port_platform.h>

#include <map>

#include "absl/strings/string_view.h"

namespace grpc_core {

// Represents backend metrics reported by the backend to the client.
struct BackendMetricData {
  /// CPU utilization expressed as a fraction of available CPU resources.
  double cpu_utilization = -1;
  /// Memory utilization expressed as a fraction of available memory
  /// resources.
  double mem_utilization = -1;
  /// Application specific utilization expressed as a fraction of available
  /// resources.
  double application_utilization = -1;
  /// Total queries per second being served by the backend across all services.
  double qps = -1;
  /// Total errors per second reported by the backend across all services.
  double eps = -1;
  /// Application-specific requests cost metrics.  Metric names are
  /// determined by the application.  Each value is an absolute cost
  /// (e.g. 3487 bytes of storage) associated with the request.
  std::map<absl::string_view, double> request_cost;
  /// Application-specific resource utilization metrics.  Metric names
  /// are determined by the application.  Each value is expressed as a
  /// fraction of total resources available.
  std::map<absl::string_view, double> utilization;
  /// Application-specific opaque metrics.  Metric names are determined by the
  /// the application.  Each value is an opaque measurement.
  std::map<absl::string_view, double> named_metrics;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_BACKEND_METRIC_DATA_H
