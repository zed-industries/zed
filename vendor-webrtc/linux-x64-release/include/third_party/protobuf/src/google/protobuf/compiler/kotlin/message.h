// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_KOTLIN_MESSAGE_H__
#define GOOGLE_PROTOBUF_COMPILER_KOTLIN_MESSAGE_H__

#include "absl/container/btree_map.h"
#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/compiler/java/name_resolver.h"
#include "google/protobuf/compiler/kotlin/field.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace kotlin {

class MessageGenerator {
 public:
  MessageGenerator(const Descriptor* descriptor, java::Context* context);
  MessageGenerator(const MessageGenerator&) = delete;
  MessageGenerator& operator=(const MessageGenerator&) = delete;
  virtual ~MessageGenerator() = default;

  void Generate(io::Printer* printer) const;
  void GenerateMembers(io::Printer* printer) const;
  void GenerateTopLevelMembers(io::Printer* printer) const;

 private:
  java::Context* context_;
  java::ClassNameResolver* name_resolver_;
  const Descriptor* descriptor_;
  absl::btree_map<int, const OneofDescriptor*> oneofs_;
  bool lite_;
  bool jvm_dsl_;

  java::FieldGeneratorMap<FieldGenerator> field_generators_;

  void GenerateExtensions(io::Printer* printer) const;
  void GenerateOrNull(io::Printer* printer) const;
  void GenerateFieldMembers(io::Printer* printer) const;
};
}  // namespace kotlin
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_KOTLIN_MESSAGE_H__
