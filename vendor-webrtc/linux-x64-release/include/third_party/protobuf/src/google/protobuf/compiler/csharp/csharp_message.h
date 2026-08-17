// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_MESSAGE_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_MESSAGE_H__

#include <string>
#include <vector>

#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/compiler/csharp/csharp_source_generator_base.h"
#include "google/protobuf/compiler/csharp/csharp_helpers.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

class FieldGeneratorBase;

class MessageGenerator : public SourceGeneratorBase {
 public:
  MessageGenerator(const Descriptor* descriptor, const Options* options);
  ~MessageGenerator();

  MessageGenerator(const MessageGenerator&) = delete;
  MessageGenerator& operator=(const MessageGenerator&) = delete;

  void GenerateCloningCode(io::Printer* printer);
  void GenerateFreezingCode(io::Printer* printer);
  void GenerateFrameworkMethods(io::Printer* printer);
  void Generate(io::Printer* printer);

 private:
  const Descriptor* descriptor_;
  std::vector<const FieldDescriptor*> fields_by_number_;
  int has_bit_field_count_;
  bool has_extension_ranges_;

  void GenerateMessageSerializationMethods(io::Printer* printer);
  void GenerateWriteToBody(io::Printer* printer, bool use_write_context);
  void GenerateMergingMethods(io::Printer* printer);
  void GenerateMainParseLoop(io::Printer* printer, bool use_parse_context);

  int GetPresenceIndex(const FieldDescriptor* descriptor);
  FieldGeneratorBase* CreateFieldGeneratorInternal(
      const FieldDescriptor* descriptor);

  bool HasNestedGeneratedTypes();

  void AddDeprecatedFlag(io::Printer* printer);
  void AddSerializableAttribute(io::Printer* printer);

  std::string class_name();
  std::string full_class_name();

  // field descriptors sorted by number
  const std::vector<const FieldDescriptor*>& fields_by_number();
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_MESSAGE_H__
