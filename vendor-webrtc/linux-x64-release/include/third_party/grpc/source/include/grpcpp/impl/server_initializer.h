//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPCPP_IMPL_SERVER_INITIALIZER_H
#define GRPCPP_IMPL_SERVER_INITIALIZER_H

#include <grpcpp/server.h>

#include <memory>
#include <vector>

namespace grpc {
class Server;
class Service;

class ServerInitializer {
 public:
  explicit ServerInitializer(grpc::Server* server) : server_(server) {}

  bool RegisterService(std::shared_ptr<grpc::Service> service) {
    if (!server_->RegisterService(nullptr, service.get())) {
      return false;
    }
    default_services_.push_back(service);
    return true;
  }

  const std::vector<std::string>* GetServiceList() {
    return &server_->services_;
  }

 private:
  grpc::Server* server_;
  std::vector<std::shared_ptr<grpc::Service> > default_services_;
};

}  // namespace grpc

#endif  // GRPCPP_IMPL_SERVER_INITIALIZER_H
