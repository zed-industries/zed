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

#ifndef GRPC_TEST_CPP_INTEROP_ISTIO_ECHO_SERVER_LIB_H
#define GRPC_TEST_CPP_INTEROP_ISTIO_ECHO_SERVER_LIB_H

#include "src/proto/grpc/testing/istio_echo.grpc.pb.h"

namespace grpc {
namespace testing {

class EchoTestServiceImpl : public proto::EchoTestService::Service {
 public:
  EchoTestServiceImpl(std::string hostname, std::string service_version,
                      std::string forwarding_address);

  grpc::Status Echo(grpc::ServerContext* context,
                    const proto::EchoRequest* request,
                    proto::EchoResponse* response) override;

  grpc::Status ForwardEcho(grpc::ServerContext* /*context*/,
                           const proto::ForwardEchoRequest* request,
                           proto::ForwardEchoResponse* response) override;

 private:
  std::string hostname_;
  std::string service_version_;
  std::string forwarding_address_;
  std::unique_ptr<proto::EchoTestService::Stub> forwarding_stub_;
  // The following fields are not set yet. But we may need them later.
  //  int port_;
  //  std::string cluster_;
  //  std::string istio_version_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_ISTIO_ECHO_SERVER_LIB_H
