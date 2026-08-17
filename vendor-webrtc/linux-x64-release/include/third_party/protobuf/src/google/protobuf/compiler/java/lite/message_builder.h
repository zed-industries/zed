// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: dweis@google.com (Daniel Weis)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_LITE_H__

#include <string>

#include "absl/container/btree_map.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/compiler/java/lite/field_generator.h"

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

class MessageBuilderLiteGenerator {
 public:
  explicit MessageBuilderLiteGenerator(const Descriptor* descriptor,
                                       Context* context);
  MessageBuilderLiteGenerator(const MessageBuilderLiteGenerator&) = delete;
  MessageBuilderLiteGenerator& operator=(const MessageBuilderLiteGenerator&) =
      delete;
  virtual ~MessageBuilderLiteGenerator();

  virtual void Generate(io::Printer* printer);

 private:
  void GenerateCommonBuilderMethods(io::Printer* printer);

  const Descriptor* descriptor_;
  Context* context_;
  ClassNameResolver* name_resolver_;
  FieldGeneratorMap<ImmutableFieldLiteGenerator> field_generators_;
  absl::btree_map<int, const OneofDescriptor*> oneofs_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_BUILDER_LITE_H__
