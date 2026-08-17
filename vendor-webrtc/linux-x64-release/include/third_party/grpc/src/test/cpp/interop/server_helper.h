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

#ifndef GRPC_TEST_CPP_INTEROP_SERVER_HELPER_H
#define GRPC_TEST_CPP_INTEROP_SERVER_HELPER_H

#include <grpc/compression.h>
#include <grpc/support/atm.h>
#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server.h>
#include <grpcpp/server_builder.h>
#include <grpcpp/server_context.h>

#include <condition_variable>
#include <memory>

namespace grpc {
namespace testing {

std::shared_ptr<ServerCredentials> CreateInteropServerCredentials();

class InteropServerContextInspector {
 public:
  explicit InteropServerContextInspector(const grpc::ServerContext& context);

  // Inspector methods, able to peek inside ServerContext, follow.
  std::shared_ptr<const AuthContext> GetAuthContext() const;
  bool IsCancelled() const;
  grpc_compression_algorithm GetCallCompressionAlgorithm() const;
  uint32_t GetEncodingsAcceptedByClient() const;
  bool WasCompressed() const;

 private:
  const grpc::ServerContext& context_;
};

namespace interop {

extern gpr_atm g_got_sigint;

struct ServerStartedCondition {
  std::mutex mutex;
  std::condition_variable condition;
  bool server_started = false;
};

/// Run gRPC interop server using port FLAGS_port.
///
/// \param creds The credentials associated with the server.
void RunServer(const std::shared_ptr<ServerCredentials>& creds);

/// Run gRPC interop server.
///
/// \param creds The credentials associated with the server.
/// \param port Port to use for the server.
/// \param server_started_condition (optional) Struct holding mutex, condition
///     variable, and condition used to notify when the server has started.
void RunServer(const std::shared_ptr<ServerCredentials>& creds, int port,
               ServerStartedCondition* server_started_condition);

/// Run gRPC interop server.
///
/// \param creds The credentials associated with the server.
/// \param server_options List of options to set when building the server.
void RunServer(
    const std::shared_ptr<ServerCredentials>& creds,
    std::unique_ptr<std::vector<std::unique_ptr<ServerBuilderOption>>>
        server_options);

/// Run gRPC interop server.
///
/// \param creds The credentials associated with the server.
/// \param port Port to use for the server.
/// \param server_options List of options to set when building the server.
/// \param server_started_condition (optional) Struct holding mutex, condition
//     variable, and condition used to notify when the server has started.
void RunServer(
    const std::shared_ptr<ServerCredentials>& creds, const int port,
    ServerStartedCondition* server_started_condition,
    std::unique_ptr<std::vector<std::unique_ptr<grpc::ServerBuilderOption>>>
        server_options);

}  // namespace interop
}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_SERVER_HELPER_H
