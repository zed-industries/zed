// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_EXTENSION_LITE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_EXTENSION_LITE_H__

#include <string>

#include "google/protobuf/compiler/java/generator_factory.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/port.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// Generates code for a lite extension, which may be within the scope of some
// message or may be at file scope.  This is much simpler than FieldGenerator
// since extensions are just simple identifiers with interesting types.
class ImmutableExtensionLiteGenerator : public ExtensionGenerator {
 public:
  explicit ImmutableExtensionLiteGenerator(const FieldDescriptor* descriptor,
                                           Context* context);
  ImmutableExtensionLiteGenerator(const ImmutableExtensionLiteGenerator&) =
      delete;
  ImmutableExtensionLiteGenerator& operator=(
      const ImmutableExtensionLiteGenerator&) = delete;
  ~ImmutableExtensionLiteGenerator() override;

  void Generate(io::Printer* printer) override;

  // Returns an estimate of the number of bytes the printed code will compile to
  int GenerateNonNestedInitializationCode(io::Printer* printer) override;

  // Returns an estimate of the number of bytes the printed code will compile to
  int GenerateRegistrationCode(io::Printer* printer) override;

 private:
  const FieldDescriptor* descriptor_;
  ClassNameResolver* name_resolver_;
  std::string scope_;
  Context* context_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_EXTENSION_LITE_H__
