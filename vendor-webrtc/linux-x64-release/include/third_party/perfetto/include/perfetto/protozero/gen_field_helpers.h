/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_PROTOZERO_GEN_FIELD_HELPERS_H_
#define INCLUDE_PERFETTO_PROTOZERO_GEN_FIELD_HELPERS_H_

#include "perfetto/protozero/message.h"
#include "perfetto/protozero/proto_decoder.h"
#include "perfetto/protozero/proto_utils.h"
#include "perfetto/protozero/scattered_heap_buffer.h"

namespace protozero {
namespace internal {
namespace gen_helpers {

// This file implements some helpers used by the protobuf generated code in the
// .gen.cc files.
//
// The .gen.cc generated protobuf implementation (as opposed to the .pbzero.h
// implementation) is not zero-copy and is not supposed to be used in fast
// paths, so most of these helpers are designed to reduce binary size.

void DeserializeString(const protozero::Field& field, std::string* dst);

// Read packed repeated elements (serialized as `wire_type`) from `field` into
// the `*dst` vector. Returns false if some bytes of `field` could not be
// interpreted correctly as `wire_type`.
template <proto_utils::ProtoWireType wire_type, typename CppType>
bool DeserializePackedRepeated(const protozero::Field& field,
                               std::vector<CppType>* dst) {
  bool parse_error = false;
  for (::protozero::PackedRepeatedFieldIterator<wire_type, CppType> rep(
           field.data(), field.size(), &parse_error);
       rep; ++rep) {
    dst->emplace_back(*rep);
  }
  return !parse_error;
}

extern template bool
DeserializePackedRepeated<proto_utils::ProtoWireType::kVarInt, uint64_t>(
    const protozero::Field& field,
    std::vector<uint64_t>* dst);

extern template bool
DeserializePackedRepeated<proto_utils::ProtoWireType::kVarInt, int64_t>(
    const protozero::Field& field,
    std::vector<int64_t>* dst);

extern template bool
DeserializePackedRepeated<proto_utils::ProtoWireType::kVarInt, uint32_t>(
    const protozero::Field& field,
    std::vector<uint32_t>* dst);

extern template bool
DeserializePackedRepeated<proto_utils::ProtoWireType::kVarInt, int32_t>(
    const protozero::Field& field,
    std::vector<int32_t>* dst);

// Serializers for different type of fields

void SerializeTinyVarInt(uint32_t field_id, bool value, Message* msg);

template <typename T>
void SerializeExtendedVarInt(uint32_t field_id, T value, Message* msg) {
  msg->AppendVarInt(field_id, value);
}

extern template void SerializeExtendedVarInt<uint64_t>(uint32_t field_id,
                                                       uint64_t value,
                                                       Message* msg);

extern template void SerializeExtendedVarInt<uint32_t>(uint32_t field_id,
                                                       uint32_t value,
                                                       Message* msg);

template <typename T>
void SerializeVarInt(uint32_t field_id, T value, Message* msg) {
  SerializeExtendedVarInt(
      field_id, proto_utils::ExtendValueForVarIntSerialization(value), msg);
}

template <typename T>
void SerializeSignedVarInt(uint32_t field_id, T value, Message* msg) {
  SerializeVarInt(field_id, proto_utils::ZigZagEncode(value), msg);
}

template <typename T>
void SerializeFixed(uint32_t field_id, T value, Message* msg) {
  msg->AppendFixed(field_id, value);
}

extern template void SerializeFixed<double>(uint32_t field_id,
                                            double value,
                                            Message* msg);

extern template void SerializeFixed<float>(uint32_t field_id,
                                           float value,
                                           Message* msg);

extern template void SerializeFixed<uint64_t>(uint32_t field_id,
                                              uint64_t value,
                                              Message* msg);

extern template void SerializeFixed<int64_t>(uint32_t field_id,
                                             int64_t value,
                                             Message* msg);

extern template void SerializeFixed<uint32_t>(uint32_t field_id,
                                              uint32_t value,
                                              Message* msg);

extern template void SerializeFixed<int32_t>(uint32_t field_id,
                                             int32_t value,
                                             Message* msg);

void SerializeString(uint32_t field_id, const std::string& value, Message* msg);

void SerializeUnknownFields(const std::string& unknown_fields, Message* msg);

// Wrapper around HeapBuffered that avoids inlining.
class MessageSerializer {
 public:
  MessageSerializer();
  ~MessageSerializer();

  Message* get() { return msg_.get(); }
  std::vector<uint8_t> SerializeAsArray();
  std::string SerializeAsString();

 private:
  HeapBuffered<Message> msg_;
};

// Wrapper about operator==() which reduces the binary size of generated protos.
// This is needed because std::string's operator== is inlined aggressively (even
// when optimizing for size). Having this layer of indirection with removes the
// overhead.
template <typename T>
bool EqualsField(const T& a, const T& b) {
  return a == b;
}
extern template bool EqualsField<std::string>(const std::string&,
                                              const std::string&);

}  // namespace gen_helpers
}  // namespace internal
}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_GEN_FIELD_HELPERS_H_
