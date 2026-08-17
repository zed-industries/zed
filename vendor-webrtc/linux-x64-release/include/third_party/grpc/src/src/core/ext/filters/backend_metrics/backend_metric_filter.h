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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_BACKEND_METRICS_BACKEND_METRIC_FILTER_H
#define GRPC_SRC_CORE_EXT_FILTERS_BACKEND_METRICS_BACKEND_METRIC_FILTER_H

#include <grpc/support/port_platform.h>

#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "src/core/ext/filters/backend_metrics/backend_metric_provider.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"

namespace grpc_core {

class BackendMetricFilter : public ImplementChannelFilter<BackendMetricFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "backend_metric"; }

  static absl::StatusOr<std::unique_ptr<BackendMetricFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args);

  class Call {
   public:
    static inline const NoInterceptor OnClientInitialMetadata;
    static inline const NoInterceptor OnServerInitialMetadata;
    void OnServerTrailingMetadata(ServerMetadata& md);
    static inline const NoInterceptor OnClientToServerMessage;
    static inline const NoInterceptor OnClientToServerHalfClose;
    static inline const NoInterceptor OnServerToClientMessage;
    static inline const NoInterceptor OnFinalize;
  };
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_BACKEND_METRICS_BACKEND_METRIC_FILTER_H
