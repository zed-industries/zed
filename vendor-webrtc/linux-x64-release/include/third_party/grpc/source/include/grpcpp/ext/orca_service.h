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

#ifndef GRPCPP_EXT_ORCA_SERVICE_H
#define GRPCPP_EXT_ORCA_SERVICE_H

#include <grpc/event_engine/event_engine.h>
#include <grpcpp/ext/server_metric_recorder.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/server_builder.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/support/slice.h>
#include <grpcpp/support/status.h>

#include <cstdint>
#include <optional>

#include "absl/base/thread_annotations.h"
#include "absl/time/time.h"

namespace grpc {

namespace testing {
class OrcaServiceTest;
}  // namespace testing

namespace experimental {

// RPC service implementation for supplying out-of-band backend
// utilization metrics to clients.
class OrcaService : public Service {
 public:
  struct Options {
    // Minimum report interval.  If a client requests an interval lower
    // than this value, this value will be used instead.
    absl::Duration min_report_duration = absl::Seconds(30);

    Options() = default;
    Options& set_min_report_duration(absl::Duration duration) {
      min_report_duration = duration;
      return *this;
    }
  };

  // ServerMetricRecorder is required.
  OrcaService(ServerMetricRecorder* const server_metric_recorder,
              Options options);

 private:
  class ReactorHook {
   public:
    virtual ~ReactorHook() = default;
    virtual void OnFinish(grpc::Status status) = 0;
    virtual void OnStartWrite(const ByteBuffer* response) = 0;
  };

  class Reactor;
  friend class testing::OrcaServiceTest;

  Slice GetOrCreateSerializedResponse();

  const ServerMetricRecorder* const server_metric_recorder_;
  const absl::Duration min_report_duration_;
  grpc::internal::Mutex mu_;
  // Contains the last serialized metrics from server_metric_recorder_.
  std::optional<Slice> response_slice_ ABSL_GUARDED_BY(mu_);
  // The update sequence number of metrics serialized in response_slice_.
  std::optional<uint64_t> response_slice_seq_ ABSL_GUARDED_BY(mu_);
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_EXT_ORCA_SERVICE_H
