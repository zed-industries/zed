// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Generates Kotlin code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_KOTLIN_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_KOTLIN_GENERATOR_H__

#include <cstdint>
#include <string>
#include <vector>

#include "google/protobuf/compiler/java/java_features.pb.h"
#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/descriptor.pb.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace kotlin {

// CodeGenerator implementation which generates Kotlin code.  If you create your
// own protocol compiler binary and you want it to support Kotlin output, you
// can do so by registering an instance of this CodeGenerator with the
// CommandLineInterface in your main() function.
class PROTOC_EXPORT KotlinGenerator : public CodeGenerator {
 public:
  KotlinGenerator();
  KotlinGenerator(const KotlinGenerator&) = delete;
  KotlinGenerator& operator=(const KotlinGenerator&) = delete;
  ~KotlinGenerator() override;

  // implements CodeGenerator ----------------------------------------
  bool Generate(const FileDescriptor* file, const std::string& parameter,
                GeneratorContext* context, std::string* error) const override;

  uint64_t GetSupportedFeatures() const override;

  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }

  std::vector<const FieldDescriptor*> GetFeatureExtensions() const override {
    return {GetExtensionReflection(pb::java)};
  }

  using CodeGenerator::GetResolvedSourceFeatures;
};

}  // namespace kotlin
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_KOTLIN_GENERATOR_H__
