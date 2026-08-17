// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
// Author: jonp@google.com (Jon Perlow)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_STRING_FIELD_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_STRING_FIELD_LITE_H__

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

class ImmutableStringFieldLiteGenerator : public ImmutableFieldLiteGenerator {
 public:
  explicit ImmutableStringFieldLiteGenerator(const FieldDescriptor* descriptor,
                                             int messageBitIndex,
                                             Context* context);
  ImmutableStringFieldLiteGenerator(const ImmutableStringFieldLiteGenerator&) =
      delete;
  ImmutableStringFieldLiteGenerator& operator=(
      const ImmutableStringFieldLiteGenerator&) = delete;
  ~ImmutableStringFieldLiteGenerator() override;

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
};

class ImmutableStringOneofFieldLiteGenerator
    : public ImmutableStringFieldLiteGenerator {
 public:
  ImmutableStringOneofFieldLiteGenerator(const FieldDescriptor* descriptor,
                                         int messageBitIndex, Context* context);
  ImmutableStringOneofFieldLiteGenerator(
      const ImmutableStringOneofFieldLiteGenerator&) = delete;
  ImmutableStringOneofFieldLiteGenerator& operator=(
      const ImmutableStringOneofFieldLiteGenerator&) = delete;
  ~ImmutableStringOneofFieldLiteGenerator() override;

 private:
  void GenerateMembers(io::Printer* printer) const override;
  void GenerateBuilderMembers(io::Printer* printer) const override;
  void GenerateFieldInfo(io::Printer* printer,
                         std::vector<uint16_t>* output) const override;
};

class RepeatedImmutableStringFieldLiteGenerator
    : public ImmutableFieldLiteGenerator {
 public:
  explicit RepeatedImmutableStringFieldLiteGenerator(
      const FieldDescriptor* descriptor, int messageBitIndex, Context* context);
  RepeatedImmutableStringFieldLiteGenerator(
      const RepeatedImmutableStringFieldLiteGenerator&) = delete;
  RepeatedImmutableStringFieldLiteGenerator& operator=(
      const RepeatedImmutableStringFieldLiteGenerator&) = delete;
  ~RepeatedImmutableStringFieldLiteGenerator() override;

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

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_STRING_FIELD_LITE_H__
