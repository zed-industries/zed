//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_CLIENT_LOAD_REPORTING_FILTER_H
#define GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_CLIENT_LOAD_REPORTING_FILTER_H

#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/load_balancing/grpclb/grpclb_client_stats.h"

namespace grpc_core {

class ClientLoadReportingFilter final
    : public ImplementChannelFilter<ClientLoadReportingFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "client_load_reporting"; }

  class Call {
   public:
    void OnClientInitialMetadata(ClientMetadata& client_initial_metadata);
    void OnServerInitialMetadata(ServerMetadata& server_initial_metadata);
    void OnServerTrailingMetadata(ServerMetadata& server_trailing_metadata);
    static inline const NoInterceptor OnServerToClientMessage;
    static inline const NoInterceptor OnClientToServerMessage;
    static inline const NoInterceptor OnClientToServerHalfClose;
    static inline const NoInterceptor OnFinalize;

   private:
    RefCountedPtr<GrpcLbClientStats> client_stats_;
    bool saw_initial_metadata_ = false;
  };

  static absl::StatusOr<std::unique_ptr<ClientLoadReportingFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LOAD_BALANCING_GRPCLB_CLIENT_LOAD_REPORTING_FILTER_H
