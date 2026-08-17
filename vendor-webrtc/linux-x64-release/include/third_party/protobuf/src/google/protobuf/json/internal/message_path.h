// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_JSON_INTERNAL_MESSAGE_PATH_H__
#define GOOGLE_PROTOBUF_JSON_INTERNAL_MESSAGE_PATH_H__

#include <string>
#include <vector>

#include "absl/cleanup/cleanup.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace json_internal {
// A path in a Protobuf message, annotated specifically for producing nice
// errors.
class MessagePath {
 public:
  explicit MessagePath(absl::string_view message_root)
      : components_(
            {Component{FieldDescriptor::TYPE_MESSAGE, message_root, "", -1}}) {}

  // Pushes a new field name, along with an optional type name if it is
  // a message or enum.
  //
  // Returns an RAII object that will pop the field component on scope exit.
  auto Push(absl::string_view field_name, FieldDescriptor::Type type,
            absl::string_view type_name = "") {
    // -1 makes it so the first call to NextRepeated makes the index 0.
    components_.push_back(Component{type, type_name, field_name, -1});
    return absl::MakeCleanup([this] { components_.pop_back(); });
  }

  // Increments the index of this field, indicating it is a repeated field.
  //
  // The first time this is called, the field will be marked as repeated and
  // the index will become 0.
  void NextRepeated() { ++components_.back().repeated_index; }

  // Appends a description of the current state of the path to `out`.
  void Describe(std::string& out) const;

 private:
  struct Component {
    FieldDescriptor::Type type;
    absl::string_view type_name, field_name;
    int32_t repeated_index;
  };
  std::vector<Component> components_;
};
}  // namespace json_internal
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_JSON_INTERNAL_MESSAGE_PATH_H__
