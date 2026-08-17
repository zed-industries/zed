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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_SERVER_AUTHZ_FILTER_H
#define GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_SERVER_AUTHZ_FILTER_H

#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/security/authorization/authorization_policy_provider.h"
#include "src/core/lib/security/authorization/evaluate_args.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/transport/auth_context.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class GrpcServerAuthzFilter final
    : public ImplementChannelFilter<GrpcServerAuthzFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "grpc-server-authz"; }

  static absl::StatusOr<std::unique_ptr<GrpcServerAuthzFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args);

  GrpcServerAuthzFilter(
      RefCountedPtr<grpc_auth_context> auth_context, const ChannelArgs& args,
      RefCountedPtr<grpc_authorization_policy_provider> provider);

  class Call {
   public:
    absl::Status OnClientInitialMetadata(ClientMetadata& md,
                                         GrpcServerAuthzFilter* filter);
    static inline const NoInterceptor OnServerInitialMetadata;
    static inline const NoInterceptor OnServerTrailingMetadata;
    static inline const NoInterceptor OnClientToServerMessage;
    static inline const NoInterceptor OnClientToServerHalfClose;
    static inline const NoInterceptor OnServerToClientMessage;
    static inline const NoInterceptor OnFinalize;
  };

 private:
  bool IsAuthorized(ClientMetadata& initial_metadata);

  RefCountedPtr<grpc_auth_context> auth_context_;
  EvaluateArgs::PerChannelArgs per_channel_evaluate_args_;
  RefCountedPtr<grpc_authorization_policy_provider> provider_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SECURITY_AUTHORIZATION_GRPC_SERVER_AUTHZ_FILTER_H
