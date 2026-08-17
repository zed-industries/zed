// Copyright 2025 The gRPC Authors.
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

#ifndef GRPC_SRC_CORE_TELEMETRY_DEFAULT_TCP_TRACER_H
#define GRPC_SRC_CORE_TELEMETRY_DEFAULT_TCP_TRACER_H

#include <optional>

#include "absl/base/thread_annotations.h"
#include "absl/time/time.h"
#include "absl/types/optional.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/telemetry/tcp_tracer.h"
#include "src/core/util/sync.h"

namespace grpc_core {

class DefaultTcpTracer final : public TcpConnectionTracer {
 public:
  explicit DefaultTcpTracer(
      std::shared_ptr<GlobalStatsPluginRegistry::StatsPluginGroup>) {}
  ~DefaultTcpTracer() override = default;
  // Records per-connection metrics.
  void RecordConnectionMetrics(TcpConnectionMetrics metrics) override;

 private:
  Mutex mu_;
  TcpConnectionMetrics connection_metrics_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_TELEMETRY_DEFAULT_TCP_TRACER_H
