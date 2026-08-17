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

#ifndef SRC_TOOLS_PROTO_MERGER_PROTO_FILE_H_
#define SRC_TOOLS_PROTO_MERGER_PROTO_FILE_H_

#include <string>
#include <vector>

// We include this intentionally instead of forward declaring to allow
// for an easy find/replace transformation when moving to Google3.
#include <google/protobuf/descriptor.h>

namespace perfetto {
namespace proto_merger {

// Simplified representation of the components of a .proto file.
struct ProtoFile {
  struct Option {
    std::string key;
    std::string value;
  };
  struct Member {
    std::vector<std::string> leading_comments;
    std::vector<std::string> trailing_comments;
  };
  struct Enum : Member {
    struct Value : Member {
      std::string name;
      int number;
      std::vector<Option> options;
    };
    std::string name;
    std::vector<Value> values;

    std::vector<Value> deleted_values;
  };
  struct Field : Member {
    bool is_repeated;
    std::string packageless_type;
    std::string type;
    std::string name;
    int number;
    std::vector<Option> options;
  };
  struct Oneof : Member {
    std::string name;
    std::vector<Field> fields;

    std::vector<Field> deleted_fields;
  };
  struct Message : Member {
    std::string name;
    std::vector<Enum> enums;
    std::vector<Message> nested_messages;
    std::vector<Oneof> oneofs;
    std::vector<Field> fields;

    std::vector<Enum> deleted_enums;
    std::vector<Message> deleted_nested_messages;
    std::vector<Oneof> deleted_oneofs;
    std::vector<Field> deleted_fields;
  };

  std::string preamble;

  std::vector<Message> messages;
  std::vector<Enum> enums;

  std::vector<Message> deleted_messages;
  std::vector<Enum> deleted_enums;
};

// Creates a ProtoFile struct from a libprotobuf-full descriptor clas.
ProtoFile ProtoFileFromDescriptor(std::string preamble,
                                  const google::protobuf::FileDescriptor&);

}  // namespace proto_merger
}  // namespace perfetto

#endif  // SRC_TOOLS_PROTO_MERGER_PROTO_FILE_H_
