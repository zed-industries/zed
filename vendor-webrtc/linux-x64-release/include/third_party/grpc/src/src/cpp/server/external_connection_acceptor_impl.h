//
//
// Copyright 2019 gRPC authors.
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

#ifndef GRPC_SRC_CPP_SERVER_EXTERNAL_CONNECTION_ACCEPTOR_IMPL_H
#define GRPC_SRC_CPP_SERVER_EXTERNAL_CONNECTION_ACCEPTOR_IMPL_H

#include <grpcpp/security/server_credentials.h>
#include <grpcpp/server_builder.h>
#include <grpcpp/support/channel_arguments.h>

#include <memory>
#include <string>

#include "src/core/lib/iomgr/tcp_server.h"
#include "src/core/util/sync.h"

namespace grpc {
namespace internal {

class ExternalConnectionAcceptorImpl
    : public std::enable_shared_from_this<ExternalConnectionAcceptorImpl> {
 public:
  ExternalConnectionAcceptorImpl(
      const std::string& name,
      ServerBuilder::experimental_type::ExternalConnectionType type,
      std::shared_ptr<ServerCredentials> creds);
  // Should only be called once.
  std::unique_ptr<experimental::ExternalConnectionAcceptor> GetAcceptor();

  void HandleNewConnection(
      experimental::ExternalConnectionAcceptor::NewConnectionParameters* p);

  void Shutdown();

  void Start();

  const char* name() { return name_.c_str(); }

  ServerCredentials* GetCredentials() { return creds_.get(); }

  void SetToChannelArgs(grpc::ChannelArguments* args);

 private:
  const std::string name_;
  std::shared_ptr<ServerCredentials> creds_;
  grpc_core::TcpServerFdHandler* handler_ = nullptr;  // not owned
  grpc_core::Mutex mu_;
  bool has_acceptor_ = false;
  bool started_ = false;
  bool shutdown_ = false;
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_EXTERNAL_CONNECTION_ACCEPTOR_IMPL_H
