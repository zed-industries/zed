// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_FIELD_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_FIELD_LITE_H__

#include <cstdint>
#include <string>

#include "absl/container/flat_hash_map.h"
#include "google/protobuf/compiler/java/lite/field_generator.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {
class Context;            // context.h
class ClassNameResolver;  // name_resolver.h
}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class ImmutableEnumFieldLiteGenerator : public ImmutableFieldLiteGenerator {
 public:
  explicit ImmutableEnumFieldLiteGenerator(const FieldDescriptor* descriptor,
                                           int messageBitIndex,
                                           Context* context);
  ImmutableEnumFieldLiteGenerator(const ImmutableEnumFieldLiteGenerator&) =
      delete;
  ImmutableEnumFieldLiteGenerator& operator=(
      const ImmutableEnumFieldLiteGenerator&) = delete;
  ~ImmutableEnumFieldLiteGenerator() override;

  // implements ImmutableFieldLiteGenerator
  // ------------------------------------
  int GetNumBitsForMessage() const override;
  void GenerateInterfaceMembers(io::Printer* printer) const override;
  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateInitializationCode(io::Printer* printer) const override;
  void GenerateFieldInfo(io::Printer* printer,
                         std::vector<uint16_t>* output) const override;

  std::string GetBoxedType() const override;

 protected:
  const FieldDescriptor* descriptor_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;
  const int messageBitIndex_;
  Context* context_;
  ClassNameResolver* name_resolver_;
};

class ImmutableEnumOneofFieldLiteGenerator
    : public ImmutableEnumFieldLiteGenerator {
 public:
  ImmutableEnumOneofFieldLiteGenerator(const FieldDescriptor* descriptor,
                                       int messageBitIndex, Context* context);
  ImmutableEnumOneofFieldLiteGenerator(
      const ImmutableEnumOneofFieldLiteGenerator&) = delete;
  ImmutableEnumOneofFieldLiteGenerator& operator=(
      const ImmutableEnumOneofFieldLiteGenerator&) = delete;
  ~ImmutableEnumOneofFieldLiteGenerator() override;

  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateFieldInfo(io::Printer* printer,
                         std::vector<uint16_t>* output) const override;
};

class RepeatedImmutableEnumFieldLiteGenerator
    : public ImmutableFieldLiteGenerator {
 public:
  explicit RepeatedImmutableEnumFieldLiteGenerator(
      const FieldDescriptor* descriptor, int messageBitIndex, Context* context);
  RepeatedImmutableEnumFieldLiteGenerator(
      const RepeatedImmutableEnumFieldLiteGenerator&) = delete;
  RepeatedImmutableEnumFieldLiteGenerator& operator=(
      const RepeatedImmutableEnumFieldLiteGenerator&) = delete;
  ~RepeatedImmutableEnumFieldLiteGenerator() override;

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

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_ENUM_FIELD_LITE_H__
