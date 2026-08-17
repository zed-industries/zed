/*
 * Copyright (C) 2021 The Android Open Source Project
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

#include "perfetto/protozero/message.h"
#include "perfetto/protozero/proto_utils.h"

#ifndef INCLUDE_PERFETTO_PROTOZERO_FIELD_WRITER_H_
#define INCLUDE_PERFETTO_PROTOZERO_FIELD_WRITER_H_

namespace protozero {
namespace internal {

template <proto_utils::ProtoSchemaType proto_schema_type>
struct FieldWriter {
  static_assert(proto_schema_type != proto_utils::ProtoSchemaType::kMessage,
                "FieldWriter can't be used with nested messages");
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kDouble> {
  inline static void Append(Message& message, uint32_t field_id, double value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kFloat> {
  inline static void Append(Message& message, uint32_t field_id, float value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kBool> {
  inline static void Append(Message& message, uint32_t field_id, bool value) {
    message.AppendTinyVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kInt32> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int32_t value) {
    message.AppendVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kInt64> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int64_t value) {
    message.AppendVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kUint32> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            uint32_t value) {
    message.AppendVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kUint64> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            uint64_t value) {
    message.AppendVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kSint32> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int32_t value) {
    message.AppendSignedVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kSint64> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int64_t value) {
    message.AppendSignedVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kFixed32> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            uint32_t value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kFixed64> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            uint64_t value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kSfixed32> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int32_t value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kSfixed64> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            int64_t value) {
    message.AppendFixed(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kEnum> {
  template <typename EnumType>
  inline static void Append(Message& message,
                            uint32_t field_id,
                            EnumType value) {
    message.AppendVarInt(field_id, value);
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kString> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            const char* data,
                            size_t size) {
    message.AppendBytes(field_id, data, size);
  }

  inline static void Append(Message& message,
                            uint32_t field_id,
                            const std::string& value) {
    message.AppendBytes(field_id, value.data(), value.size());
  }
};

template <>
struct FieldWriter<proto_utils::ProtoSchemaType::kBytes> {
  inline static void Append(Message& message,
                            uint32_t field_id,
                            const uint8_t* data,
                            size_t size) {
    message.AppendBytes(field_id, data, size);
  }

  inline static void Append(Message& message,
                            uint32_t field_id,
                            const std::string& value) {
    message.AppendBytes(field_id, value.data(), value.size());
  }
};

}  // namespace internal
}  // namespace protozero

#endif  // INCLUDE_PERFETTO_PROTOZERO_FIELD_WRITER_H_
