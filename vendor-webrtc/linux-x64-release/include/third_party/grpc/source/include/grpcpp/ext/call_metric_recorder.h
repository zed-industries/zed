//
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
//

#ifndef GRPCPP_EXT_CALL_METRIC_RECORDER_H
#define GRPCPP_EXT_CALL_METRIC_RECORDER_H

#include <grpcpp/impl/sync.h>
#include <grpcpp/support/slice.h>

#include <memory>
#include <string>

#include "absl/strings/string_view.h"

namespace grpc {
namespace experimental {

/// Records call metrics for the purpose of load balancing.
/// During an RPC, call \a ServerContext::ExperimentalGetCallMetricRecorder()
/// method to retrieve the recorder for the current call.
class CallMetricRecorder {
 public:
  virtual ~CallMetricRecorder() = default;

  /// Records a call metric measurement for CPU utilization.
  /// Multiple calls to this method will override the stored value.
  /// Values may be larger than 1.0 when the usage exceeds the reporter
  /// dependent notion of soft limits.
  /// Values outside of the valid range [0, infy] are ignored.
  virtual CallMetricRecorder& RecordCpuUtilizationMetric(double value) = 0;

  /// Records a call metric measurement for memory utilization.
  /// Multiple calls to this method will override the stored value.
  /// Values outside of the valid range [0, 1] are ignored.
  virtual CallMetricRecorder& RecordMemoryUtilizationMetric(double value) = 0;

  /// Records a call metric measurement for application specific utilization.
  /// Multiple calls to this method will override the stored value.
  /// Values may be larger than 1.0 when the usage exceeds the reporter
  /// dependent notion of soft limits.
  /// Values outside of the valid range [0, infy] are ignored.
  virtual CallMetricRecorder& RecordApplicationUtilizationMetric(
      double value) = 0;

  /// Records a call metric measurement for queries per second.
  /// Multiple calls to this method will override the stored value.
  /// Values outside of the valid range [0, infy) are ignored.
  virtual CallMetricRecorder& RecordQpsMetric(double value) = 0;

  /// Records a call metric measurement for errors per second.
  /// Multiple calls to this method will override the stored value.
  /// Values outside of the valid range [0, infy) are ignored.
  virtual CallMetricRecorder& RecordEpsMetric(double value) = 0;

  /// Records a call metric measurement for utilization.
  /// Multiple calls to this method with the same name will
  /// override the corresponding stored value. The lifetime of the
  /// name string needs to be longer than the lifetime of the RPC
  /// itself, since it's going to be sent as trailers after the RPC
  /// finishes. It is assumed the strings are common names that
  /// are global constants.
  /// Values outside of the valid range [0, 1] are ignored.
  virtual CallMetricRecorder& RecordUtilizationMetric(string_ref name,
                                                      double value) = 0;

  /// Records a call metric measurement for request cost.
  /// Multiple calls to this method with the same name will
  /// override the corresponding stored value. The lifetime of the
  /// name string needs to be longer than the lifetime of the RPC
  /// itself, since it's going to be sent as trailers after the RPC
  /// finishes. It is assumed the strings are common names that
  /// are global constants.
  virtual CallMetricRecorder& RecordRequestCostMetric(string_ref name,
                                                      double value) = 0;

  /// Records an application-specific opaque metric measurement.
  /// Multiple calls to this method with the same name will
  /// override the corresponding stored value. The lifetime of the
  /// name string needs to be longer than the lifetime of the RPC
  /// itself, since it's going to be sent as trailers after the RPC
  /// finishes. It is assumed the strings are common names that
  /// are global constants.
  virtual CallMetricRecorder& RecordNamedMetric(string_ref name,
                                                double value) = 0;
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_EXT_CALL_METRIC_RECORDER_H
