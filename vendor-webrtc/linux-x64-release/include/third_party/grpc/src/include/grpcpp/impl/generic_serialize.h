// Copyright 2024 gRPC authors.
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

#ifndef GRPCPP_IMPL_GENERIC_SERIALIZE_H
#define GRPCPP_IMPL_GENERIC_SERIALIZE_H

#include <grpc/byte_buffer_reader.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/slice.h>
#include <grpcpp/impl/codegen/config_protobuf.h>
#include <grpcpp/impl/serialization_traits.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/proto_buffer_reader.h>
#include <grpcpp/support/proto_buffer_writer.h>
#include <grpcpp/support/slice.h>
#include <grpcpp/support/status.h>

#include <type_traits>

#include "absl/log/absl_check.h"

/// This header provides serialization and deserialization between gRPC
/// messages serialized using protobuf and the C++ objects they represent.

namespace grpc {

// ProtoBufferWriter must be a subclass of ::protobuf::io::ZeroCopyOutputStream.
template <class ProtoBufferWriter, class T>
Status GenericSerialize(const grpc::protobuf::MessageLite& msg, ByteBuffer* bb,
                        bool* own_buffer) {
  static_assert(std::is_base_of<protobuf::io::ZeroCopyOutputStream,
                                ProtoBufferWriter>::value,
                "ProtoBufferWriter must be a subclass of "
                "::protobuf::io::ZeroCopyOutputStream");
  *own_buffer = true;
  int byte_size = static_cast<int>(msg.ByteSizeLong());
  if (static_cast<size_t>(byte_size) <= GRPC_SLICE_INLINED_SIZE) {
    Slice slice(byte_size);
    // We serialize directly into the allocated slices memory
    ABSL_CHECK(slice.end() == msg.SerializeWithCachedSizesToArray(
                                  const_cast<uint8_t*>(slice.begin())));
    ByteBuffer tmp(&slice, 1);
    bb->Swap(&tmp);

    return grpc::Status::OK;
  }
  ProtoBufferWriter writer(bb, kProtoBufferWriterMaxBufferLength, byte_size);
  protobuf::io::CodedOutputStream cs(&writer);
  msg.SerializeWithCachedSizes(&cs);
  return !cs.HadError()
             ? grpc::Status::OK
             : Status(StatusCode::INTERNAL, "Failed to serialize message");
}

// BufferReader must be a subclass of ::protobuf::io::ZeroCopyInputStream.
template <class ProtoBufferReader, class T>
Status GenericDeserialize(ByteBuffer* buffer,
                          grpc::protobuf::MessageLite* msg) {
  static_assert(std::is_base_of<protobuf::io::ZeroCopyInputStream,
                                ProtoBufferReader>::value,
                "ProtoBufferReader must be a subclass of "
                "::protobuf::io::ZeroCopyInputStream");
  if (buffer == nullptr) {
    return Status(StatusCode::INTERNAL, "No payload");
  }
  Status result = grpc::Status::OK;
  {
    ProtoBufferReader reader(buffer);
    if (!reader.status().ok()) {
      return reader.status();
    }
    if (!msg->ParseFromZeroCopyStream(&reader)) {
      result = Status(StatusCode::INTERNAL, msg->InitializationErrorString());
    }
  }
  buffer->Clear();
  return result;
}

}  // namespace grpc

#endif  // GRPCPP_IMPL_GENERIC_SERIALIZE_H
