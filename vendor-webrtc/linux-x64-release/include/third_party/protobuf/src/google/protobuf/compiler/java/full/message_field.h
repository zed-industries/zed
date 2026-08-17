// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_H__

#include <string>

#include "google/protobuf/compiler/java/full/field_generator.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

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

class ImmutableMessageFieldGenerator : public ImmutableFieldGenerator {
 public:
  explicit ImmutableMessageFieldGenerator(const FieldDescriptor* descriptor,
                                          int messageBitIndex,
                                          int builderBitIndex,
                                          Context* context);
  ImmutableMessageFieldGenerator(const ImmutableMessageFieldGenerator&) =
      delete;
  ImmutableMessageFieldGenerator& operator=(
      const ImmutableMessageFieldGenerator&) = delete;
  ~ImmutableMessageFieldGenerator() override;

  // implements ImmutableFieldGenerator
  // ---------------------------------------
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

 protected:
  const FieldDescriptor* descriptor_;
  int message_bit_index_;
  int builder_bit_index_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;
  ClassNameResolver* name_resolver_;
  Context* context_;

  virtual void PrintNestedBuilderCondition(
      io::Printer* printer, const char* regular_case,
      const char* nested_builder_case) const;
  virtual void PrintNestedBuilderFunction(
      io::Printer* printer, const char* method_prototype,
      const char* regular_case, const char* nested_builder_case,
      const char* trailing_code,
      absl::optional<io::AnnotationCollector::Semantic> semantic =
          absl::nullopt) const;
};

class ImmutableMessageOneofFieldGenerator
    : public ImmutableMessageFieldGenerator {
 public:
  ImmutableMessageOneofFieldGenerator(const FieldDescriptor* descriptor,
                                      int messageBitIndex, int builderBitIndex,
                                      Context* context);
  ImmutableMessageOneofFieldGenerator(
      const ImmutableMessageOneofFieldGenerator&) = delete;
  ImmutableMessageOneofFieldGenerator& operator=(
      const ImmutableMessageOneofFieldGenerator&) = delete;
  ~ImmutableMessageOneofFieldGenerator() override;

  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateBuilderClearCode(io::Printer* printer) const override;
  void GenerateBuildingCode(io::Printer* printer) const override;
  void GenerateMergingCode(io::Printer* printer) const override;
  void GenerateBuilderParsingCode(io::Printer* printer) const override;
  void GenerateSerializationCode(io::Printer* printer) const override;
  void GenerateSerializedSizeCode(io::Printer* printer) const override;
};

class RepeatedImmutableMessageFieldGenerator
    : public ImmutableMessageFieldGenerator {
 public:
  explicit RepeatedImmutableMessageFieldGenerator(
      const FieldDescriptor* descriptor, int messageBitIndex,
      int builderBitIndex, Context* context);
  RepeatedImmutableMessageFieldGenerator(
      const RepeatedImmutableMessageFieldGenerator&) = delete;
  RepeatedImmutableMessageFieldGenerator& operator=(
      const RepeatedImmutableMessageFieldGenerator&) = delete;
  ~RepeatedImmutableMessageFieldGenerator() override;

  // implements ImmutableFieldGenerator ---------------------------------------
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

 protected:
  void PrintNestedBuilderCondition(
      io::Printer* printer, const char* regular_case,
      const char* nested_builder_case) const override;
  void PrintNestedBuilderFunction(
      io::Printer* printer, const char* method_prototype,
      const char* regular_case, const char* nested_builder_case,
      const char* trailing_code,
      absl::optional<io::AnnotationCollector::Semantic> semantic =
          absl::nullopt) const override;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_H__
