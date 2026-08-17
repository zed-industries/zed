// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_LITE_H__

#include <cstdint>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
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

class ImmutableMessageFieldLiteGenerator : public ImmutableFieldLiteGenerator {
 public:
  explicit ImmutableMessageFieldLiteGenerator(const FieldDescriptor* descriptor,
                                              int messageBitIndex,
                                              Context* context);
  ImmutableMessageFieldLiteGenerator(
      const ImmutableMessageFieldLiteGenerator&) = delete;
  ImmutableMessageFieldLiteGenerator& operator=(
      const ImmutableMessageFieldLiteGenerator&) = delete;
  ~ImmutableMessageFieldLiteGenerator() override;

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
  ClassNameResolver* name_resolver_;
  Context* context_;

 private:
};

class ImmutableMessageOneofFieldLiteGenerator
    : public ImmutableMessageFieldLiteGenerator {
 public:
  ImmutableMessageOneofFieldLiteGenerator(const FieldDescriptor* descriptor,
                                          int messageBitIndex,
                                          Context* context);
  ImmutableMessageOneofFieldLiteGenerator(
      const ImmutableMessageOneofFieldLiteGenerator&) = delete;
  ImmutableMessageOneofFieldLiteGenerator& operator=(
      const ImmutableMessageOneofFieldLiteGenerator&) = delete;
  ~ImmutableMessageOneofFieldLiteGenerator() override;

  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateFieldInfo(io::Printer* printer,
                         std::vector<uint16_t>* output) const override;

};

class RepeatedImmutableMessageFieldLiteGenerator
    : public ImmutableFieldLiteGenerator {
 public:
  explicit RepeatedImmutableMessageFieldLiteGenerator(
      const FieldDescriptor* descriptor, int messageBitIndex, Context* context);
  RepeatedImmutableMessageFieldLiteGenerator(
      const RepeatedImmutableMessageFieldLiteGenerator&) = delete;
  RepeatedImmutableMessageFieldLiteGenerator& operator=(
      const RepeatedImmutableMessageFieldLiteGenerator&) = delete;
  ~RepeatedImmutableMessageFieldLiteGenerator() override;

  // implements ImmutableFieldLiteGenerator ------------------------------------
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
  ClassNameResolver* name_resolver_;
  Context* context_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_FIELD_LITE_H__
