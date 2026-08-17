//
// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_FILTERS_FAULT_INJECTION_FAULT_INJECTION_FILTER_H
#define GRPC_SRC_CORE_EXT_FILTERS_FAULT_INJECTION_FAULT_INJECTION_FILTER_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/random/random.h"
#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// This channel filter is intended to be used by the dynamic filters, instead
// of the ordinary channel stack. The fault injection filter fetches fault
// injection policy from the method config of service config returned by the
// resolver, and enforces the fault injection policy.
class FaultInjectionFilter
    : public ImplementChannelFilter<FaultInjectionFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "fault_injection_filter"; }

  static absl::StatusOr<std::unique_ptr<FaultInjectionFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args filter_args);

  explicit FaultInjectionFilter(ChannelFilter::Args filter_args);

  // Construct a promise for one call.
  class Call {
   public:
    ArenaPromise<absl::Status> OnClientInitialMetadata(
        ClientMetadata& md, FaultInjectionFilter* filter);
    static inline const NoInterceptor OnServerInitialMetadata;
    static inline const NoInterceptor OnServerTrailingMetadata;
    static inline const NoInterceptor OnClientToServerMessage;
    static inline const NoInterceptor OnClientToServerHalfClose;
    static inline const NoInterceptor OnServerToClientMessage;
    static inline const NoInterceptor OnFinalize;
  };

 private:
  class InjectionDecision;
  InjectionDecision MakeInjectionDecision(
      const ClientMetadata& initial_metadata);

  // The relative index of instances of the same filter.
  size_t index_;
  const size_t service_config_parser_index_;
  Mutex mu_;
  absl::InsecureBitGen abort_rand_generator_ ABSL_GUARDED_BY(mu_);
  absl::InsecureBitGen delay_rand_generator_ ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_FILTERS_FAULT_INJECTION_FAULT_INJECTION_FILTER_H
