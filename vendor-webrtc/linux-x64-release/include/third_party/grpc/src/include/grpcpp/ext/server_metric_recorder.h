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

#ifndef GRPCPP_EXT_SERVER_METRIC_RECORDER_H
#define GRPCPP_EXT_SERVER_METRIC_RECORDER_H

#include <grpcpp/impl/sync.h>
#include <grpcpp/support/string_ref.h>

#include <functional>
#include <map>
#include <memory>

namespace grpc_core {
struct BackendMetricData;
}  // namespace grpc_core

namespace grpc {
class BackendMetricState;

namespace experimental {
/// Records server wide metrics to be reported to the client.
/// Server implementation creates an instance and reports server metrics to it,
/// and then passes it to
/// ServerBuilder::experimental_type::EnableCallMetricRecording or
/// experimental::OrcaService that read metrics to include in the report.
class ServerMetricRecorder {
 public:
  // Factory method. Use this to create.
  static std::unique_ptr<ServerMetricRecorder> Create();
  /// Records the server CPU utilization in the range [0, infy).
  /// Values may be larger than 1.0 when the usage exceeds the reporter
  /// dependent notion of soft limits. Values outside of the valid range are
  /// rejected. Overrides the stored value when called again with a valid value.
  void SetCpuUtilization(double value);
  /// Records the server memory utilization in the range [0, 1].
  /// Values outside of the valid range are rejected.
  /// Overrides the stored value when called again with a valid value.
  void SetMemoryUtilization(double value);
  /// Records the application specific utilization in the range [0, infy].
  /// Values outside of the valid range are rejected.
  /// Overrides the stored value when called again with a valid value.
  void SetApplicationUtilization(double value);
  /// Records number of queries per second to the server in the range [0, infy).
  /// Values outside of the valid range are rejected.
  /// Overrides the stored value when called again with a valid value.
  void SetQps(double value);
  /// Records number of errors per second to the server in the range [0, infy).
  /// Values outside of the valid range are rejected.
  /// Overrides the stored value when called again with a valid value.
  void SetEps(double value);
  /// Records a named resource utilization value in the range [0, 1].
  /// Values outside of the valid range are rejected.
  /// Overrides the stored value when called again with the same name.
  /// The name string should remain valid while this utilization remains
  /// in this recorder. It is assumed that strings are common names that are
  /// global constants.
  void SetNamedUtilization(string_ref name, double value);
  /// Replaces all named resource utilization values. No range validation.
  /// The name strings should remain valid while utilization values remain
  /// in this recorder. It is assumed that strings are common names that are
  /// global constants.
  void SetAllNamedUtilization(std::map<string_ref, double> named_utilization);

  /// Clears the server CPU utilization if recorded.
  void ClearCpuUtilization();
  /// Clears the server memory utilization if recorded.
  void ClearMemoryUtilization();
  /// Clears the application specific utilization if recorded.
  void ClearApplicationUtilization();
  /// Clears number of queries per second to the server if recorded.
  void ClearQps();
  /// Clears number of errors per second to the server if recorded.
  void ClearEps();
  /// Clears a named utilization value if exists.
  void ClearNamedUtilization(string_ref name);

 private:
  // To access GetMetrics().
  friend class grpc::BackendMetricState;
  friend class OrcaService;

  struct BackendMetricDataState;

  // No direct creation, use the factory method Create() above.
  ServerMetricRecorder();

  // Updates the metric state by applying `updater` to the data and incrementing
  // the sequence number.
  void UpdateBackendMetricDataState(
      std::function<void(grpc_core::BackendMetricData*)> updater);

  grpc_core::BackendMetricData GetMetrics() const;
  // Returned metric data is guaranteed to be identical between two calls if the
  // sequence numbers match.
  std::shared_ptr<const BackendMetricDataState> GetMetricsIfChanged() const;

  mutable grpc::internal::Mutex mu_;
  std::shared_ptr<const BackendMetricDataState> metric_state_
      ABSL_GUARDED_BY(mu_);
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_EXT_SERVER_METRIC_RECORDER_H
