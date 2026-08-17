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

#ifndef GRPC_TEST_CORE_UTIL_HTTP_CLIENT_HTTPCLI_TEST_UTIL_H
#define GRPC_TEST_CORE_UTIL_HTTP_CLIENT_HTTPCLI_TEST_UTIL_H

#include <grpc/support/port_platform.h>

#include "src/core/util/subprocess.h"

namespace grpc_core {
namespace testing {

struct HttpRequestTestServer {
  gpr_subprocess* server;
  int port;
};

HttpRequestTestServer StartHttpRequestTestServer(int argc, char** argv,
                                                 bool use_ssl);

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_UTIL_HTTP_CLIENT_HTTPCLI_TEST_UTIL_H
