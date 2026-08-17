// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_PROTO_UTIL_LITE_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_PROTO_UTIL_LITE_H_

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "mediapipe/framework/port/advanced_proto_lite_inc.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/tool/field_data.pb.h"

namespace mediapipe {
namespace tool {

// TODO: Replace this class with a namespace following Google style.
class ProtoUtilLite {
 public:
  // Defines field types and tag formats.
  using WireFormatLite = proto_ns::internal::WireFormatLite;

  // The serialized value for a protobuf field.
  using FieldValue = std::string;

  // The serialized data type for a protobuf field.
  using FieldType = WireFormatLite::FieldType;

  // A field-id and index, or a map-id and key, or both.
  struct ProtoPathEntry {
    ProtoPathEntry(int id, int index) : field_id(id), index(index) {}
    ProtoPathEntry(int id, int key_id, FieldType key_type, FieldValue key_value)
        : map_id(id),
          key_id(key_id),
          key_type(key_type),
          key_value(std::move(key_value)) {}
    bool operator==(const ProtoPathEntry& o) const {
      return field_id == o.field_id && index == o.index && map_id == o.map_id &&
             key_id == o.key_id && key_type == o.key_type &&
             key_value == o.key_value;
    }
    int field_id = -1;
    int index = -1;
    int map_id = -1;
    int key_id = -1;
    FieldType key_type = FieldType::MAX_FIELD_TYPE;
    FieldValue key_value;
  };

  // Defines a sequence of nested field-number field-index pairs.
  using ProtoPath = std::vector<ProtoPathEntry>;

  class FieldAccess {
   public:
    // Provides access to a certain protobuf field.
    FieldAccess(uint32_t field_id, FieldType field_type);

    // Specifies the original serialized protobuf message.
    absl::Status SetMessage(const FieldValue& message);

    // Returns the serialized protobuf message with updated field values.
    void GetMessage(FieldValue* result);

    // Returns the serialized values of the protobuf field.
    std::vector<FieldValue>* mutable_field_values();

    uint32_t field_id() const { return field_id_; }

   private:
    uint32_t field_id_;
    FieldType field_type_;
    std::string message_;
    std::vector<FieldValue> field_values_;
  };

  // Replace a range of field values nested within a protobuf.
  // Starting at the proto_path index, "length" values are replaced.
  static absl::Status ReplaceFieldRange(
      FieldValue* message, ProtoPath proto_path, int length,
      FieldType field_type, const std::vector<FieldValue>& field_values);

  // Retrieve a range of field values nested within a protobuf.
  // Starting at the proto_path index, "length" values are retrieved.
  static absl::Status GetFieldRange(const FieldValue& message,
                                    ProtoPath proto_path, int length,
                                    FieldType field_type,
                                    std::vector<FieldValue>* field_values);

  // Returns the number of field values in a repeated protobuf field.
  static absl::Status GetFieldCount(const FieldValue& message,
                                    ProtoPath proto_path, FieldType field_type,
                                    int* field_count);

  // Serialize one or more protobuf field values from text.
  static absl::Status Serialize(const std::vector<std::string>& text_values,
                                FieldType field_type,
                                std::vector<FieldValue>* result);

  // Deserialize one or more protobuf field values to text.
  static absl::Status Deserialize(const std::vector<FieldValue>& field_values,
                                  FieldType field_type,
                                  std::vector<std::string>* result);

  // Write a protobuf field value from a typed FieldData value.
  static absl::Status WriteValue(const mediapipe::FieldData& value,
                                 FieldType field_type,
                                 std::string* field_bytes);

  // Read a protobuf field value into a typed FieldData value.
  static absl::Status ReadValue(absl::string_view field_bytes,
                                FieldType field_type,
                                absl::string_view message_type,
                                mediapipe::FieldData* result);

  // Returns the protobuf type-url for a protobuf type-name.
  static std::string TypeUrl(absl::string_view type_name);

  // Returns the protobuf type-name for a protobuf type-url.
  static std::string ParseTypeUrl(absl::string_view type_url);
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_PROTO_UTIL_LITE_H_
