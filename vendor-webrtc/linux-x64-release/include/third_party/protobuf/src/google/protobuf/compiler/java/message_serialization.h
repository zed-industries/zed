// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_SERIALIZATION_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_SERIALIZATION_H__

#include <algorithm>
#include <cstddef>
#include <vector>

#include "absl/types/span.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// Generates code to serialize a single extension range.
void GenerateSerializeExtensionRange(io::Printer* printer,
                                     const Descriptor::ExtensionRange* range);

// Generates code to serialize all fields and extension ranges for the specified
// message descriptor, sorting serialization calls in increasing order by field
// number.
inline void GenerateSerializeFieldsAndExtensions(
    io::Printer* printer,
    const std::vector<const FieldGenerator*>& field_generators,
    const Descriptor* descriptor, const FieldDescriptor** sorted_fields) {
  std::vector<const Descriptor::ExtensionRange*> sorted_extensions;
  sorted_extensions.reserve(descriptor->extension_range_count());
  for (int i = 0; i < descriptor->extension_range_count(); ++i) {
    sorted_extensions.push_back(descriptor->extension_range(i));
  }
  std::sort(sorted_extensions.begin(), sorted_extensions.end(),
            ExtensionRangeOrdering());

  std::size_t range_idx = 0;

  // Merge the fields and the extension ranges, both sorted by field number.
  for (int i = 0; i < descriptor->field_count(); ++i) {
    const FieldDescriptor* field = sorted_fields[i];

    // Collapse all extension ranges up until the next field. This leads to
    // shorter and more efficient codegen for messages containing a large
    // number of extension ranges without fields in between them.
    const Descriptor::ExtensionRange* range = nullptr;
    while (range_idx < sorted_extensions.size() &&
           sorted_extensions[range_idx]->end_number() <= field->number()) {
      range = sorted_extensions[range_idx++];
    }

    if (range != nullptr) {
      GenerateSerializeExtensionRange(printer, range);
    }
    size_t idx = static_cast<size_t>(field->index());
    field_generators[idx]->GenerateSerializationCode(printer);
  }

  // After serializing all fields, serialize any remaining extensions via a
  // single writeUntil call.
  if (range_idx < sorted_extensions.size()) {
    GenerateSerializeExtensionRange(printer, sorted_extensions.back());
  }
}

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_SERIALIZATION_H__
