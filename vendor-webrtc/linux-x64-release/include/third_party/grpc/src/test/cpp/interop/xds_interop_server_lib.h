//
//
// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CPP_INTEROP_XDS_INTEROP_SERVER_LIB_H
#define GRPC_TEST_CPP_INTEROP_XDS_INTEROP_SERVER_LIB_H
#include <grpcpp/server.h>

#include <optional>

#include "absl/strings/string_view.h"

namespace grpc {
namespace testing {

// Exposed for the tests
std::optional<grpc::Status> GetStatusForRpcBehaviorMetadata(
    absl::string_view header_value, absl::string_view hostname);

void RunServer(bool secure_mode, int port, const int maintenance_port,
               absl::string_view hostname, absl::string_view server_id,
               const std::function<void(Server*)>& server_callback);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_INTEROP_XDS_INTEROP_SERVER_LIB_H
