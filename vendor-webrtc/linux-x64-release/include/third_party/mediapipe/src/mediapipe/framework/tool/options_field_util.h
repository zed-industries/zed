#ifndef MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_FIELD_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_FIELD_UTIL_H_

#include <string>
#include <vector>

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/advanced_proto_inc.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/tool/field_data.pb.h"
#include "mediapipe/framework/tool/options_registry.h"

namespace mediapipe {

namespace tool {

// Utility to read and write Packet data from protobuf fields.
namespace options_field_util {

// A protobuf field and index description.
struct FieldPathEntry {
  const FieldDescriptor* field = nullptr;
  int index = -1;
  std::string extension_type;
};

// A chain of nested protobuf fields and indexes.
using FieldPath = std::vector<FieldPathEntry>;

// Writes a field value into protobuf field.
absl::Status SetField(const FieldPath& field_path, const FieldData& value,
                      FieldData* message_data);

// Reads a field value from a protobuf field.
absl::StatusOr<FieldData> GetField(const FieldData& message_data,
                                   const FieldPath& field_path);

// Reads one or all FieldData values from a protobuf field.
absl::StatusOr<std::vector<FieldData>> GetFieldValues(
    const FieldData& message_data, const FieldPath& field_path);

// Writes FieldData values into a protobuf field.
absl::Status SetFieldValues(FieldData& message_data,
                            const FieldPath& field_path,
                            const std::vector<FieldData>& values);

// Merges FieldData values into a protobuf field.
absl::Status MergeFieldValues(FieldData& message_data,
                              const FieldPath& field_path,
                              const std::vector<FieldData>& values);

// Deserializes a packet containing a MessageLite value.
absl::StatusOr<Packet> ReadMessage(const std::string& value,
                                   const std::string& type_name);

// Merge two options protobuf field values.
absl::StatusOr<FieldData> MergeMessages(const FieldData& base,
                                        const FieldData& over);

// Returns the requested options protobuf for a graph.
absl::StatusOr<FieldData> GetNodeOptions(const FieldData& message_data,
                                         const std::string& extension_type);

// Returns the requested options protobuf for a graph node.
absl::StatusOr<FieldData> GetGraphOptions(const FieldData& message_data,
                                          const std::string& extension_type);

// Sets the node_options field in a Node, and clears the options field.
void SetOptionsMessage(const FieldData& node_options,
                       CalculatorGraphConfig::Node* node);

// Serialize a MessageLite to a FieldData.
FieldData AsFieldData(const proto_ns::MessageLite& message);

// Constructs a Packet for a FieldData proto.
absl::StatusOr<Packet> AsPacket(const FieldData& data);

// Constructs a FieldData proto for a Packet.
absl::StatusOr<FieldData> AsFieldData(Packet packet);

// Returns the protobuf type-url for a protobuf type-name.
std::string TypeUrl(absl::string_view type_name);

// Returns the protobuf type-name for a protobuf type-url.
std::string ParseTypeUrl(absl::string_view type_url);

}  // namespace options_field_util
}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_FIELD_UTIL_H_
