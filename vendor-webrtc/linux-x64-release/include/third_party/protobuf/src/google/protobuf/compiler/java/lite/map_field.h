// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_LITE_H__

#include <cstdint>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/lite/field_generator.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class ImmutableMapFieldLiteGenerator : public ImmutableFieldLiteGenerator {
 public:
  explicit ImmutableMapFieldLiteGenerator(const FieldDescriptor* descriptor,
                                          int messageBitIndex,
                                          Context* context);
  ~ImmutableMapFieldLiteGenerator() override;

  // implements ImmutableFieldLiteGenerator ------------------------------------
  int GetNumBitsForMessage() const override;
  void GenerateInterfaceMembers(io::Printer* printer) const override;
  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateInitializationCode(io::Printer* printer) const override;
  void GenerateFieldInfo(io::Printer* printer,
                         std::vector<uint16_t>* output) const override;

  std::string GetBoxedType() const override;

 private:
  const FieldDescriptor* descriptor_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;
  Context* context_;
  ClassNameResolver* name_resolver_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_LITE_H__
