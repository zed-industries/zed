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

#ifndef GRPC_TEST_CPP_UTIL_BYTE_BUFFER_PROTO_HELPER_H
#define GRPC_TEST_CPP_UTIL_BYTE_BUFFER_PROTO_HELPER_H

#include <grpcpp/impl/codegen/config_protobuf.h>
#include <grpcpp/support/byte_buffer.h>

#include <memory>

namespace grpc {
namespace testing {

bool ParseFromByteBuffer(ByteBuffer* buffer, grpc::protobuf::Message* message);

std::unique_ptr<ByteBuffer> SerializeToByteBuffer(
    grpc::protobuf::Message* message);

bool SerializeToByteBufferInPlace(grpc::protobuf::Message* message,
                                  ByteBuffer* buffer);

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_UTIL_BYTE_BUFFER_PROTO_HELPER_H
