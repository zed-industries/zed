// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_SOURCE_GENERATOR_BASE_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_SOURCE_GENERATOR_BASE_H__

#include <string>

#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

struct Options;

class SourceGeneratorBase {
 protected:
  SourceGeneratorBase(const Options* options);
  virtual ~SourceGeneratorBase();

  SourceGeneratorBase(const SourceGeneratorBase&) = delete;
  SourceGeneratorBase& operator=(const SourceGeneratorBase&) = delete;

  std::string class_access_level();
  const Options* options();

  // Write any attributes used to decorate generated function members (methods and properties).
  // Should not be used to decorate types.
  void WriteGeneratedCodeAttributes(io::Printer* printer);

 private:
  const Options *options_;
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_SOURCE_GENERATOR_BASE_H__

