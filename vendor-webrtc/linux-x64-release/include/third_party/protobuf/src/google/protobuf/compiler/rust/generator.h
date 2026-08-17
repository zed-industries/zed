// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_GENERATOR_H__

#include <cstdint>
#include <string>

#include "google/protobuf/compiler/code_generator.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

class PROTOC_EXPORT RustGenerator final
    : public google::protobuf::compiler::CodeGenerator {
 public:
  RustGenerator() = default;
  RustGenerator(const RustGenerator&) = delete;
  RustGenerator& operator=(const RustGenerator&) = delete;
  ~RustGenerator() override = default;

  bool Generate(const FileDescriptor* file, const std::string& parameter,
                GeneratorContext* generator_context,
                std::string* error) const override;

  uint64_t GetSupportedFeatures() const override {
    return FEATURE_PROTO3_OPTIONAL | FEATURE_SUPPORTS_EDITIONS;
  }
  Edition GetMinimumEdition() const override { return Edition::EDITION_PROTO2; }
  Edition GetMaximumEdition() const override { return Edition::EDITION_2023; }
};

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_GENERATOR_H__
