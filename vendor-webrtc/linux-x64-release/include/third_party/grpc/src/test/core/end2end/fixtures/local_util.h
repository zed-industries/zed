//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_TEST_CORE_END2END_FIXTURES_LOCAL_UTIL_H
#define GRPC_TEST_CORE_END2END_FIXTURES_LOCAL_UTIL_H

#include <grpc/grpc.h>
#include <grpc/grpc_security_constants.h>

#include <string>

#include "absl/functional/any_invocable.h"
#include "src/core/lib/channel/channel_args.h"
#include "test/core/end2end/end2end_tests.h"

class LocalTestFixture final : public grpc_core::CoreTestFixture {
 public:
  LocalTestFixture(std::string localaddr, grpc_local_connect_type type);

 private:
  grpc_server* MakeServer(
      const grpc_core::ChannelArgs& args, grpc_completion_queue* cq,
      absl::AnyInvocable<void(grpc_server*)>& pre_server_start) override;
  grpc_channel* MakeClient(const grpc_core::ChannelArgs& args,
                           grpc_completion_queue* cq) override;

  std::string localaddr_;
  grpc_local_connect_type type_;
};

#endif  // GRPC_TEST_CORE_END2END_FIXTURES_LOCAL_UTIL_H
