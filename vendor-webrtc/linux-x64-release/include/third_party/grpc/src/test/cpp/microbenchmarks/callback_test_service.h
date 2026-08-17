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

#ifndef GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_TEST_SERVICE_H
#define GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_TEST_SERVICE_H

#include <benchmark/benchmark.h>

#include <condition_variable>
#include <memory>
#include <mutex>
#include <sstream>

#include "src/proto/grpc/testing/echo.grpc.pb.h"
#include "test/cpp/util/string_ref_helper.h"

namespace grpc {
namespace testing {

const char* const kServerMessageSize = "server_message_size";

class CallbackStreamingTestService : public EchoTestService::CallbackService {
 public:
  CallbackStreamingTestService() {}

  ServerUnaryReactor* Echo(CallbackServerContext* context,
                           const EchoRequest* request,
                           EchoResponse* response) override;

  ServerBidiReactor<EchoRequest, EchoResponse>* BidiStream(
      CallbackServerContext* context) override;
};
}  // namespace testing
}  // namespace grpc
#endif  // GRPC_TEST_CPP_MICROBENCHMARKS_CALLBACK_TEST_SERVICE_H
