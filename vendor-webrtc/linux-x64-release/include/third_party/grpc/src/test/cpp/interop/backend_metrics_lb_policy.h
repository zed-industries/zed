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

#ifndef GRPC_TEST_CPP_INTEROP_BACKEND_METRICS_LB_POLICY_H
#define GRPC_TEST_CPP_INTEROP_BACKEND_METRICS_LB_POLICY_H

#include <grpc/support/port_platform.h>
#include <grpcpp/support/channel_arguments.h>

#include "src/core/config/core_configuration.h"
#include "src/proto/grpc/testing/messages.pb.h"

namespace grpc {
namespace testing {
class LoadReportTracker {
 public:
  // A load report, or nullopt if the call had no load report.
  using LoadReportEntry = std::optional<TestOrcaReport>;

  ChannelArguments GetChannelArguments();
  void ResetCollectedLoadReports();
  void RecordPerRpcLoadReport(
      const grpc_core::BackendMetricData* backend_metric_data);
  void RecordOobLoadReport(const grpc_core::BackendMetricData& oob_metric_data);
  // Returns the next per-RPC load report, or nullopt if the queue is empty.
  std::optional<LoadReportEntry> GetNextLoadReport();
  LoadReportEntry WaitForOobLoadReport(
      const std::function<bool(const TestOrcaReport&)>& predicate,
      absl::Duration poll_timeout, size_t max_attempts);

 private:
  std::deque<LoadReportEntry> per_rpc_load_reports_
      ABSL_GUARDED_BY(load_reports_mu_);
  std::deque<TestOrcaReport> oob_load_reports_
      ABSL_GUARDED_BY(load_reports_mu_);
  grpc_core::Mutex load_reports_mu_;
  grpc_core::CondVar load_reports_cv_ ABSL_GUARDED_BY(load_reports_mu_);
};

void RegisterBackendMetricsLbPolicy(
    grpc_core::CoreConfiguration::Builder* builder);
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_BACKEND_METRICS_LB_POLICY_H
