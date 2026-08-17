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

#ifndef GRPC_TEST_CORE_TSI_ALTS_FAKE_HANDSHAKER_FAKE_HANDSHAKER_SERVER_H
#define GRPC_TEST_CORE_TSI_ALTS_FAKE_HANDSHAKER_FAKE_HANDSHAKER_SERVER_H

#include <grpcpp/grpcpp.h>

#include <memory>
#include <string>

namespace grpc {
namespace gcp {

std::unique_ptr<grpc::Service> CreateFakeHandshakerService(
    const std::string& peer_identity);

}  // namespace gcp
}  // namespace grpc

#endif  // GRPC_TEST_CORE_TSI_ALTS_FAKE_HANDSHAKER_FAKE_HANDSHAKER_SERVER_H
