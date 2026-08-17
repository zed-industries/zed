// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.
//
// Generates C++ code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_GENERATOR_H__

#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/cpp_features.pb.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace cpp {
// CodeGenerator implementation which generates a C++ source file and
// header.  If you create your own protocol compiler binary and you want
// it to support C++ output, you can do so by registering an instance of this
// CodeGenerator with the CommandLineInterface in your main() function.
class PROTOC_EXPORT CppGenerator : public CodeGenerator {
 public:
  CppGenerator() = default;
  CppGenerator(const CppGenerator&) = delete;
  CppGenerator& operator=(const CppGenerator&) = delete;
  ~CppGenerator() override = default;

  enum class Runtime {
    kGoogle3,     // Use the internal google3 runtime.
    kOpensource,  // Use the open-source runtime.

    // Use the open-source runtime with google3 #include paths.  We make these
    // absolute to avoid ambiguity, so the runtime will be #included like:
    //   #include "third_party/protobuf/.../google/protobuf/message.h"
    kOpensourceGoogle3
  };

  void set_opensource_runtime(bool opensource) {
    opensource_runtime_ = opensource;
  }

  // If set to a non-empty string, generated code will do:
  //   #include "<BASE>/google/protobuf/message.h"
  // instead of:
  //   #include "google/protobuf/message.h"
  // This has no effect if opensource_runtime = false.
  void set_runtime_include_base(std::string base) {
    runtime_include_base_ = std::move(base);
  }

  bool Generate(const FileDescriptor* file, const std::string& parameter,
                GeneratorContext* generator_context,
                std::string* error) const override;

  uint64_t GetSupportedFeatures() const override {
    return FEATURE_PROTO3_OPTIONAL | FEATURE_SUPPORTS_EDITIONS;
  }

  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }

  std::vector<const FieldDescriptor*> GetFeatureExtensions() const override {
    return {GetExtensionReflection(pb::cpp)};
  }

  using CodeGenerator::GetEdition;
  using CodeGenerator::GetResolvedSourceFeatures;

 private:
  bool opensource_runtime_ = google::protobuf::internal::IsOss();
  std::string runtime_include_base_;

  absl::Status ValidateFeatures(const FileDescriptor* file) const;
};
}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_GENERATOR_H__
