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

#ifndef SRC_TOOLS_PROTO_MERGER_ALLOWLIST_H_
#define SRC_TOOLS_PROTO_MERGER_ALLOWLIST_H_

#include <map>
#include <set>
#include <string>
#include <vector>

#include "perfetto/base/status.h"

// We include this intentionally instead of forward declaring to allow
// for an easy find/replace transformation when moving to Google3.
#include <google/protobuf/descriptor.h>

namespace perfetto {
namespace proto_merger {

// Represents an allow-list for proto messages, fields and enums.
struct Allowlist {
  using Oneof = std::set<int>;
  struct Message {
    std::set<std::string> enums;
    std::map<std::string, Oneof> oneofs;
    std::set<int> fields;

    // Needs to be a std::map as std::unordered_map causes complaints about
    // self-referentiality from GCC.
    std::map<std::string, Message> nested_messages;
  };
  std::map<std::string, Message> messages;
  std::set<std::string> enums;
};

// Creates a Allowlist struct from a list of allowed fields rooted at the given
// descriptor.
base::Status AllowlistFromFieldList(
    const google::protobuf::Descriptor&,
    const std::vector<std::string>& allowed_fields,
    Allowlist& allowlist);

}  // namespace proto_merger
}  // namespace perfetto

#endif  // SRC_TOOLS_PROTO_MERGER_ALLOWLIST_H_
