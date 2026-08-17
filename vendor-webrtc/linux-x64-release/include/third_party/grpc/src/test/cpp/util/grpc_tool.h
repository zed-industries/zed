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

#ifndef GRPC_TEST_CPP_UTIL_GRPC_TOOL_H
#define GRPC_TEST_CPP_UTIL_GRPC_TOOL_H

#include <grpcpp/support/config.h>

#include <functional>

#include "test/cpp/util/cli_credentials.h"

namespace grpc {
namespace testing {

typedef std::function<bool(const std::string&)> GrpcToolOutputCallback;

int GrpcToolMainLib(int argc, const char** argv, const CliCredentials& cred,
                    const GrpcToolOutputCallback& callback);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_GRPC_TOOL_H
