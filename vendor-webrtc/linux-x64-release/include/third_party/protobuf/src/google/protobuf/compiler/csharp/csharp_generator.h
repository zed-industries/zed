// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Generates C# code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_GENERATOR_H__

#include <string>

#include "google/protobuf/compiler/code_generator.h"

#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

// CodeGenerator implementation which generates a C# source file and
// header.  If you create your own protocol compiler binary and you want
// it to support C# output, you can do so by registering an instance of this
// CodeGenerator with the CommandLineInterface in your main() function.
class PROTOC_EXPORT Generator : public CodeGenerator {
 public:
  Generator();
  ~Generator();
  bool Generate(
    const FileDescriptor* file,
    const std::string& parameter,
    GeneratorContext* generator_context,
    std::string* error) const override;
  uint64_t GetSupportedFeatures() const override;
  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }
  using CodeGenerator::GetEdition;
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_GENERATOR_H__
