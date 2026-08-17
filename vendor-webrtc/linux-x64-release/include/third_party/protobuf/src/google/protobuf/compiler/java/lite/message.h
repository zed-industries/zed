// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: dweis@google.com (Daniel Weis)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_LITE_H__

#include <memory>
#include <vector>

#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/compiler/java/generator_factory.h"
#include "google/protobuf/compiler/java/lite/field_generator.h"
#include "google/protobuf/compiler/java/name_resolver.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class ImmutableMessageLiteGenerator : public MessageGenerator {
 public:
  ImmutableMessageLiteGenerator(const Descriptor* descriptor, Context* context);
  ImmutableMessageLiteGenerator(const ImmutableMessageLiteGenerator&) = delete;
  ImmutableMessageLiteGenerator& operator=(
      const ImmutableMessageLiteGenerator&) = delete;
  ~ImmutableMessageLiteGenerator() override;

  void Generate(io::Printer* printer) override;
  void GenerateInterface(io::Printer* printer) override;
  void GenerateExtensionRegistrationCode(io::Printer* printer) override;
  void GenerateStaticVariables(io::Printer* printer,
                               int* bytecode_estimate) override;
  int GenerateStaticVariableInitializers(io::Printer* printer) override;

 private:
  void GenerateParseFromMethods(io::Printer* printer);

  void GenerateBuilder(io::Printer* printer);
  void GenerateDynamicMethodNewBuilder(io::Printer* printer);
  void GenerateInitializers(io::Printer* printer);
  void GenerateParser(io::Printer* printer);
  void GenerateConstructor(io::Printer* printer);
  void GenerateDynamicMethodNewBuildMessageInfo(io::Printer* printer);

  Context* context_;
  ClassNameResolver* name_resolver_;
  FieldGeneratorMap<ImmutableFieldLiteGenerator> field_generators_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_LITE_H__
