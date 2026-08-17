// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_PHP_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_PHP_GENERATOR_H__

#include <cstdint>
#include <string>

#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/compiler/php/names.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace php {

struct Options;

class PROTOC_EXPORT Generator : public CodeGenerator {
 public:
  virtual bool Generate(
      const FileDescriptor* file,
      const std::string& parameter,
      GeneratorContext* generator_context,
      std::string* error) const override;

  bool GenerateAll(const std::vector<const FileDescriptor*>& files,
                   const std::string& parameter,
                   GeneratorContext* generator_context,
                   std::string* error) const override;

  uint64_t GetSupportedFeatures() const override {
    return Feature::FEATURE_PROTO3_OPTIONAL;
  }

  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }
  std::vector<const FieldDescriptor*> GetFeatureExtensions() const override {
    return {};
  }

 private:
  bool Generate(
      const FileDescriptor* file,
      const Options& options,
      GeneratorContext* generator_context,
      std::string* error) const;
};

inline bool IsWrapperType(const FieldDescriptor* descriptor) {
  return descriptor->cpp_type() == FieldDescriptor::CPPTYPE_MESSAGE &&
      descriptor->message_type()->file()->name() == "google/protobuf/wrappers.proto";
}

}  // namespace php
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_PHP_GENERATOR_H__
