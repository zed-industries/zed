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

#ifndef GRPC_TEST_CPP_INTEROP_HTTP2_CLIENT_H
#define GRPC_TEST_CPP_INTEROP_HTTP2_CLIENT_H

#include <grpc/grpc.h>
#include <grpcpp/channel.h>

#include <memory>

#include "src/proto/grpc/testing/messages.pb.h"
#include "src/proto/grpc/testing/test.grpc.pb.h"

namespace grpc {
namespace testing {

class Http2Client {
 public:
  explicit Http2Client(const std::shared_ptr<Channel>& channel);
  ~Http2Client() {}

  bool DoRstAfterHeader();
  bool DoRstAfterData();
  bool DoRstDuringData();
  bool DoGoaway();
  bool DoPing();
  bool DoMaxStreams();

 private:
  class ServiceStub {
   public:
    explicit ServiceStub(const std::shared_ptr<Channel>& channel);

    TestService::Stub* Get();

   private:
    std::unique_ptr<TestService::Stub> stub_;
    std::shared_ptr<Channel> channel_;
  };

  void MaxStreamsWorker(const std::shared_ptr<grpc::Channel>& channel);
  bool AssertStatusCode(const Status& s, StatusCode expected_code);
  Status SendUnaryCall(SimpleResponse* response);
  SimpleRequest BuildDefaultRequest();
  ServiceStub serviceStub_;
  std::shared_ptr<Channel> channel_;
  SimpleRequest defaultRequest_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_HTTP2_CLIENT_H
