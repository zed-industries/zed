//
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
//

#ifndef GRPCPP_IMPL_PROTO_UTILS_H
#define GRPCPP_IMPL_PROTO_UTILS_H

#include <grpc/byte_buffer_reader.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/slice.h>
#include <grpcpp/impl/codegen/config_protobuf.h>
#include <grpcpp/impl/generic_serialize.h>
#include <grpcpp/impl/serialization_traits.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/proto_buffer_reader.h>
#include <grpcpp/support/proto_buffer_writer.h>
#include <grpcpp/support/slice.h>
#include <grpcpp/support/status.h>

#include <type_traits>

/// This header provides serialization and deserialization between gRPC
/// messages serialized using protobuf and the C++ objects they represent.

namespace grpc {

// This class provides a protobuf serializer. It translates between protobuf
// objects and grpc_byte_buffers. More information about SerializationTraits can
// be found in include/grpcpp/impl/codegen/serialization_traits.h.
template <class T>
class SerializationTraits<
    T, typename std::enable_if<
           std::is_base_of<grpc::protobuf::MessageLite, T>::value>::type> {
 public:
  static Status Serialize(const grpc::protobuf::MessageLite& msg,
                          ByteBuffer* bb, bool* own_buffer) {
    return GenericSerialize<ProtoBufferWriter, T>(msg, bb, own_buffer);
  }

  static Status Deserialize(ByteBuffer* buffer,
                            grpc::protobuf::MessageLite* msg) {
    return GenericDeserialize<ProtoBufferReader, T>(buffer, msg);
  }
};

}  // namespace grpc

#endif  // GRPCPP_IMPL_PROTO_UTILS_H
