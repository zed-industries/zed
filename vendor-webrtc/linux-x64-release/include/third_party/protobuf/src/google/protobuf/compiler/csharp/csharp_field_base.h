// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_FIELD_BASE_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_FIELD_BASE_H__

#include <string>

#include "google/protobuf/compiler/code_generator.h"
#include "absl/container/flat_hash_map.h"
#include "absl/strings/ascii.h"
#include "absl/strings/escaping.h"
#include "absl/strings/str_replace.h"
#include "absl/strings/str_split.h"
#include "google/protobuf/compiler/csharp/csharp_source_generator_base.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

class FieldGeneratorBase : public SourceGeneratorBase {
 public:
  FieldGeneratorBase(const FieldDescriptor* descriptor,
                     int presenceIndex,
                     const Options* options);
  ~FieldGeneratorBase();

  FieldGeneratorBase(const FieldGeneratorBase&) = delete;
  FieldGeneratorBase& operator=(const FieldGeneratorBase&) = delete;

  virtual void GenerateCloningCode(io::Printer* printer) = 0;
  virtual void GenerateFreezingCode(io::Printer* printer);
  virtual void GenerateCodecCode(io::Printer* printer);
  virtual void GenerateExtensionCode(io::Printer* printer);
  virtual void GenerateMembers(io::Printer* printer) = 0;
  virtual void GenerateMergingCode(io::Printer* printer) = 0;
  virtual void GenerateParsingCode(io::Printer* printer) = 0;
  virtual void GenerateParsingCode(io::Printer* printer, bool use_parse_context);
  virtual void GenerateSerializationCode(io::Printer* printer) = 0;
  virtual void GenerateSerializationCode(io::Printer* printer, bool use_write_context);
  virtual void GenerateSerializedSizeCode(io::Printer* printer) = 0;

  virtual void WriteHash(io::Printer* printer) = 0;
  virtual void WriteEquals(io::Printer* printer) = 0;
  // Currently unused, as we use reflection to generate JSON
  virtual void WriteToString(io::Printer* printer) = 0;

 protected:
  const FieldDescriptor* descriptor_;
  const int presenceIndex_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;

  void AddDeprecatedFlag(io::Printer* printer);
  void AddNullCheck(io::Printer* printer);
  void AddNullCheck(io::Printer* printer, const std::string& name);

  void AddPublicMemberAttributes(io::Printer* printer);
  void SetCommonOneofFieldVariables(
      absl::flat_hash_map<absl::string_view, std::string>* variables);

  std::string oneof_property_name();
  std::string oneof_case_name(); 
  std::string oneof_name();
  std::string property_name();
  std::string name();
  std::string type_name();
  std::string type_name(const FieldDescriptor* descriptor);
  bool has_default_value();
  std::string default_value();
  std::string default_value(const FieldDescriptor* descriptor);
  std::string number();
  std::string capitalized_type_name();

 private:
  void SetCommonFieldVariables(
      absl::flat_hash_map<absl::string_view, std::string>* variables);
  std::string GetStringDefaultValueInternal(const FieldDescriptor* descriptor);
  std::string GetBytesDefaultValueInternal(const FieldDescriptor* descriptor);
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_FIELD_BASE_H__

