// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_H__

#include <memory>
#include <string>
#include <vector>

#include "google/protobuf/compiler/java/generator_factory.h"
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

class ImmutableMessageGenerator : public MessageGenerator {
 public:
  ImmutableMessageGenerator(const Descriptor* descriptor, Context* context);
  ImmutableMessageGenerator(const ImmutableMessageGenerator&) = delete;
  ImmutableMessageGenerator& operator=(const ImmutableMessageGenerator&) =
      delete;
  ~ImmutableMessageGenerator() override;

  void Generate(io::Printer* printer) override;
  void GenerateInterface(io::Printer* printer) override;
  void GenerateExtensionRegistrationCode(io::Printer* printer) override;
  void GenerateStaticVariables(io::Printer* printer,
                               int* bytecode_estimate) override;

  // Returns an estimate of the number of bytes the printed code will compile to
  int GenerateStaticVariableInitializers(io::Printer* printer) override;

 private:
  void GenerateFieldAccessorTable(io::Printer* printer, int* bytecode_estimate);

  // Returns an estimate of the number of bytes the printed code will compile to
  int GenerateFieldAccessorTableInitializer(io::Printer* printer);

  void GenerateMessageSerializationMethods(io::Printer* printer);
  void GenerateParseFromMethods(io::Printer* printer);

  void GenerateBuilder(io::Printer* printer);
  void GenerateIsInitialized(io::Printer* printer);
  void GenerateDescriptorMethods(io::Printer* printer);
  void GenerateInitializers(io::Printer* printer);
  void GenerateEqualsAndHashCode(io::Printer* printer);
  void GenerateParser(io::Printer* printer);
  void GenerateParsingConstructor(io::Printer* printer);
  void GenerateMutableCopy(io::Printer* printer);
  void GenerateAnyMethods(io::Printer* printer);

  Context* context_;
  ClassNameResolver* name_resolver_;
  FieldGeneratorMap<ImmutableFieldGenerator> field_generators_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_MESSAGE_H__
