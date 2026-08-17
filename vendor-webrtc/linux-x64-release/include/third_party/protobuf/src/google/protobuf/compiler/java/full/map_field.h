// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_H__

#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/full/field_generator.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class ImmutableMapFieldGenerator : public ImmutableFieldGenerator {
 public:
  explicit ImmutableMapFieldGenerator(const FieldDescriptor* descriptor,
                                      int messageBitIndex, int builderBitIndex,
                                      Context* context);
  ~ImmutableMapFieldGenerator() override = default;

  // implements ImmutableFieldGenerator ---------------------------------------
  int GetMessageBitIndex() const override;
  int GetBuilderBitIndex() const override;
  int GetNumBitsForMessage() const override;
  int GetNumBitsForBuilder() const override;
  void GenerateInterfaceMembers(io::Printer* printer) const override;
  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateInitializationCode(io::Printer* printer) const override;
  void GenerateBuilderClearCode(io::Printer* printer) const override;
  void GenerateMergingCode(io::Printer* printer) const override;
  void GenerateBuildingCode(io::Printer* printer) const override;
  void GenerateBuilderParsingCode(io::Printer* printer) const override;
  void GenerateSerializationCode(io::Printer* printer) const override;
  void GenerateSerializedSizeCode(io::Printer* printer) const override;
  void GenerateFieldBuilderInitializationCode(
      io::Printer* printer) const override;
  void GenerateEqualsCode(io::Printer* printer) const override;
  void GenerateHashCode(io::Printer* printer) const override;

  std::string GetBoxedType() const override;

 private:
  const FieldDescriptor* descriptor_;
  int message_bit_index_;
  int builder_bit_index_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;
  ClassNameResolver* name_resolver_;
  Context* context_;
  void SetMessageVariables(const FieldGeneratorInfo* info);
  void GenerateMapGetters(io::Printer* printer) const;
  void GenerateMessageMapBuilderMembers(io::Printer* printer) const;
  void GenerateMessageMapGetters(io::Printer* printer) const;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MAP_FIELD_H__
