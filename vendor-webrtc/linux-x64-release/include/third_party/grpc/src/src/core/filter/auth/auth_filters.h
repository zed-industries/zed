//
//
// Copyright 2015 gRPC authors.
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

#ifndef GRPC_SRC_CORE_FILTER_AUTH_AUTH_FILTERS_H
#define GRPC_SRC_CORE_FILTER_AUTH_AUTH_FILTERS_H

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>

#include "absl/status/statusor.h"
#include "src/core/call/status_util.h"
#include "src/core/credentials/call/call_credentials.h"
#include "src/core/credentials/transport/security_connector.h"
#include "src/core/credentials/transport/transport_credentials.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/channel/promise_based_filter.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

// Handles calling out to credentials to fill in metadata per call.
class ClientAuthFilter final : public ImplementChannelFilter<ClientAuthFilter> {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "client-auth-filter"; }

  ClientAuthFilter(
      RefCountedPtr<grpc_channel_security_connector> security_connector,
      RefCountedPtr<grpc_auth_context> auth_context);

  static absl::StatusOr<std::unique_ptr<ClientAuthFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args);

 private:
  // These methods are early in the declaration order as auto return type is
  // needed by Call.
  auto GetMetadataFromCreds(RefCountedPtr<grpc_call_credentials> creds,
                            ClientMetadataHandle md) {
    return Map(creds->GetRequestMetadata(std::move(md), &args_),
               [](absl::StatusOr<ClientMetadataHandle> new_metadata) mutable {
                 if (!new_metadata.ok()) {
                   return absl::StatusOr<ClientMetadataHandle>(
                       MaybeRewriteIllegalStatusCode(new_metadata.status(),
                                                     "call credentials"));
                 }
                 return new_metadata;
               });
  }

  auto GetCallCredsMetadata(ClientMetadataHandle md) {
    auto creds = GetCallCreds();
    return If(
        creds.ok(),
        [this, &creds, md = std::move(md)]() mutable {
          return If(
              *creds != nullptr,
              [this, &creds, &md]() {
                return GetMetadataFromCreds(std::move(*creds), std::move(md));
              },
              [&md]() {
                return Immediate(
                    absl::StatusOr<ClientMetadataHandle>(std::move(md)));
              });
        },
        [&creds]() {
          return Immediate(
              absl::StatusOr<ClientMetadataHandle>(creds.status()));
        });
  }

 public:
  class Call {
   public:
    auto OnClientInitialMetadata(ClientMetadataHandle md,
                                 ClientAuthFilter* filter) {
      filter->InstallContext();
      auto* host = md->get_pointer(HttpAuthorityMetadata());
      return AssertResultType<absl::StatusOr<ClientMetadataHandle>>(If(
          host == nullptr,
          [&md]() mutable -> absl::StatusOr<ClientMetadataHandle> {
            return std::move(md);
          },
          [filter, host, &md]() mutable {
            return TrySeq(
                filter->args_.security_connector->CheckCallHost(
                    host->as_string_view(), filter->args_.auth_context.get()),
                [filter, md = std::move(md)]() mutable {
                  return filter->GetCallCredsMetadata(std::move(md));
                });
          }));
    }
    static const inline NoInterceptor OnServerInitialMetadata;
    static const inline NoInterceptor OnClientToServerMessage;
    static const inline NoInterceptor OnClientToServerHalfClose;
    static const inline NoInterceptor OnServerToClientMessage;
    static const inline NoInterceptor OnServerTrailingMetadata;
    static const inline NoInterceptor OnFinalize;
  };

 private:
  void InstallContext();
  absl::StatusOr<RefCountedPtr<grpc_call_credentials>> GetCallCreds();

  // Contains refs to security connector and auth context.
  grpc_call_credentials::GetRequestMetadataArgs args_;
};

class LegacyClientAuthFilter final : public ChannelFilter {
 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "client-auth-filter"; }

  LegacyClientAuthFilter(
      RefCountedPtr<grpc_channel_security_connector> security_connector,
      RefCountedPtr<grpc_auth_context> auth_context);

  static absl::StatusOr<std::unique_ptr<ClientAuthFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args);

  // Construct a promise for one call.
  ArenaPromise<ServerMetadataHandle> MakeCallPromise(
      CallArgs call_args, NextPromiseFactory next_promise_factory) override;

 private:
  ArenaPromise<absl::StatusOr<CallArgs>> GetCallCredsMetadata(
      CallArgs call_args);

  // Contains refs to security connector and auth context.
  grpc_call_credentials::GetRequestMetadataArgs args_;
};

class ServerAuthFilter final : public ImplementChannelFilter<ServerAuthFilter> {
 private:
  class RunApplicationCode {
   public:
    RunApplicationCode(ServerAuthFilter* filter, ClientMetadata& metadata);

    RunApplicationCode(const RunApplicationCode&) = delete;
    RunApplicationCode& operator=(const RunApplicationCode&) = delete;
    RunApplicationCode(RunApplicationCode&& other) noexcept
        : state_(std::exchange(other.state_, nullptr)) {}
    RunApplicationCode& operator=(RunApplicationCode&& other) noexcept {
      state_ = std::exchange(other.state_, nullptr);
      return *this;
    }

    Poll<absl::Status> operator()();

   private:
    // Called from application code.
    static void OnMdProcessingDone(void* user_data,
                                   const grpc_metadata* consumed_md,
                                   size_t num_consumed_md,
                                   const grpc_metadata* response_md,
                                   size_t num_response_md,
                                   grpc_status_code status,
                                   const char* error_details);

    struct State;
    State* state_;
  };

 public:
  static const grpc_channel_filter kFilter;

  static absl::string_view TypeName() { return "server-auth"; }

  ServerAuthFilter(RefCountedPtr<grpc_server_credentials> server_credentials,
                   RefCountedPtr<grpc_auth_context> auth_context);

  static absl::StatusOr<std::unique_ptr<ServerAuthFilter>> Create(
      const ChannelArgs& args, ChannelFilter::Args);

  class Call {
   public:
    explicit Call(ServerAuthFilter* filter);
    auto OnClientInitialMetadata(ClientMetadata& md, ServerAuthFilter* filter) {
      return If(
          filter->server_credentials_ == nullptr ||
              filter->server_credentials_->auth_metadata_processor().process ==
                  nullptr,
          ImmediateOkStatus(),
          [filter, md = &md]() { return RunApplicationCode(filter, *md); });
    }
    static inline const NoInterceptor OnServerInitialMetadata;
    static inline const NoInterceptor OnClientToServerMessage;
    static inline const NoInterceptor OnClientToServerHalfClose;
    static inline const NoInterceptor OnServerToClientMessage;
    static inline const NoInterceptor OnServerTrailingMetadata;
    static inline const NoInterceptor OnFinalize;
  };

 private:
  ArenaPromise<absl::StatusOr<CallArgs>> GetCallCredsMetadata(
      CallArgs call_args);

  RefCountedPtr<grpc_server_credentials> server_credentials_;
  RefCountedPtr<grpc_auth_context> auth_context_;
};

}  // namespace grpc_core

// Exposed for testing purposes only.
// Check if the channel's security level is higher or equal to
// that of call credentials to make a decision whether the transfer
// of call credentials should be allowed or not.
bool grpc_check_security_level(grpc_security_level channel_level,
                               grpc_security_level call_cred_level);

#endif  // GRPC_SRC_CORE_FILTER_AUTH_AUTH_FILTERS_H
