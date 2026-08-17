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

#ifndef SRC_PROTOZERO_FILTERING_FILTER_UTIL_H_
#define SRC_PROTOZERO_FILTERING_FILTER_UTIL_H_

#include <stdint.h>

#include <list>
#include <map>
#include <optional>
#include <set>
#include <string>

// We include this intentionally instead of forward declaring to allow
// for an easy find/replace transformation when moving to Google3.
#include <google/protobuf/descriptor.h>

namespace protozero {

// Parses a .proto message definition, recursing into its sub-messages, and
// builds up a set of Messages and Field definitions.
// Depends on libprotobuf-full and should be used only in host tools.
// See the //tools/proto_filter for an executable that wraps this class with
// a cmdline interface.
class FilterUtil {
 public:
  FilterUtil();
  ~FilterUtil();

  // Loads a message schema from a .proto file, recursing into nested types.
  // Args:
  // proto_file: path to the .proto file.
  // root_message: fully qualified message name (e.g., perfetto.protos.Trace).
  //     If empty, the first message in the file will be used.
  // proto_dir_path: the root for .proto includes. If empty uses CWD.
  // passthrough: an optional set of fields that should be transparently passed
  //     through without recursing further.
  //     Syntax: "perfetto.protos.TracePacket:trace_config"
  // filter_string_fields: an optional set of fields that should be treated as
  //     string fields which need to be filtered.
  //     Syntax: same as passthrough
  bool LoadMessageDefinition(
      const std::string& proto_file,
      const std::string& root_message,
      const std::string& proto_dir_path,
      const std::set<std::string>& passthrough_fields = {},
      const std::set<std::string>& filter_string_fields = {});

  // Deduplicates leaf messages having the same sets of field ids.
  // It changes the internal state and affects the behavior of next calls to
  // GenerateFilterBytecode() and PrintAsText().
  void Dedupe();

  // Generates the filter bytecode for the root message previously loaded by
  // LoadMessageDefinition() using FilterBytecodeGenerator.
  // The returned string is a binary-encoded proto message of type
  // perfetto.protos.ProtoFilter (see proto_filter.proto).
  std::string GenerateFilterBytecode();

  // Prints the list of messages and fields onto stdout in a diff-friendly text
  // format. Example:
  // PowerRails                 2 message  energy_data     PowerRails.EnergyData
  // PowerRails.RailDescriptor  1 uint32   index
  // If the optional bytecode filter is given, only the fields allowed by the
  // bytecode are printed.
  void PrintAsText(std::optional<std::string> filter_bytecode = {});

  // Resolves an array of field ids into a dot-concatenated field names.
  // E.g., [2,5,1] -> ".trace.packet.timestamp".
  std::string LookupField(const uint32_t* field_ids, size_t num_fields);

  // Like the above but the array of field is passed as a buffer containing
  // varints, e.g. "\x02\x05\0x01".
  std::string LookupField(const std::string& varint_encoded_path);

  void set_print_stream_for_testing(FILE* stream) { print_stream_ = stream; }

 private:
  struct Message {
    struct Field {
      std::string name;
      std::string type;  // "uint32", "string", "message"
      bool filter_string;
      // Only when type == "message". Note that when using Dedupe() this can
      // be aliased against a different submessage which happens to have the
      // same set of field ids.
      Message* nested_type = nullptr;

      bool is_simple() const {
        return nested_type == nullptr && !filter_string;
      }
    };
    std::string full_name;  // e.g., "perfetto.protos.Foo.Bar";
    std::map<uint32_t /*field_id*/, Field> fields;

    // True if at least one field has a non-null |nestd_type|.
    bool has_nested_fields = false;

    // True if at least one field has |filter_string|==true.
    bool has_filter_string_fields = false;
  };

  using DescriptorsByNameMap = std::map<std::string, Message*>;
  Message* ParseProtoDescriptor(const google::protobuf::Descriptor*,
                                DescriptorsByNameMap*);

  // list<> because pointers need to be stable.
  std::list<Message> descriptors_;
  std::set<std::string> passthrough_fields_;
  std::set<std::string> filter_string_fields_;

  // Used only for debugging aid, to print out an error message when the user
  // specifies a field to pass through but it doesn't exist.
  std::set<std::string> passthrough_fields_seen_;
  std::set<std::string> filter_string_fields_seen_;

  FILE* print_stream_ = stdout;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_FILTER_UTIL_H_
