// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: dweis@google.com (Daniel Weis)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_H__

#include <memory>
#include <string>
#include <vector>

#include "absl/container/btree_map.h"
#include "google/protobuf/compiler/java/full/field_generator.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {
class Context;            // context.h
class ClassNameResolver;  // name_resolver.h
}  // namespace java
}  // namespace compiler
namespace io {
class Printer;  // printer.h
}
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class MessageBuilderGenerator {
 public:
  explicit MessageBuilderGenerator(const Descriptor* descriptor,
                                   Context* context);
  MessageBuilderGenerator(const MessageBuilderGenerator&) = delete;
  MessageBuilderGenerator& operator=(const MessageBuilderGenerator&) = delete;
  virtual ~MessageBuilderGenerator();

  virtual void Generate(io::Printer* printer);

 private:
  void GenerateCommonBuilderMethods(io::Printer* printer);
  void GenerateBuildPartial(io::Printer* printer);
  int GenerateBuildPartialPiece(io::Printer* printer, int piece,
                                int first_field);
  int GenerateBuildPartialPieceWithoutPresence(io::Printer* printer, int piece,
                                               int first_field);
  void GenerateDescriptorMethods(io::Printer* printer);
  void GenerateBuilderParsingMethods(io::Printer* printer);
  void GenerateBuilderFieldParsingCases(io::Printer* printer);
  void GenerateBuilderFieldParsingCase(io::Printer* printer,
                                       const FieldDescriptor* field);
  void GenerateBuilderPackedFieldParsingCase(io::Printer* printer,
                                             const FieldDescriptor* field);
  void GenerateIsInitialized(io::Printer* printer);

  const Descriptor* descriptor_;
  Context* context_;
  ClassNameResolver* name_resolver_;
  FieldGeneratorMap<ImmutableFieldGenerator> field_generators_;
  absl::btree_map<int, const OneofDescriptor*> oneofs_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_H__
