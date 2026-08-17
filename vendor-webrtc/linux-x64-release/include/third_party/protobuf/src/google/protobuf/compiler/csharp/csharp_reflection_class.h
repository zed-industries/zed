// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_REFLECTION_CLASS_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_REFLECTION_CLASS_H__

#include <string>

#include "google/protobuf/compiler/code_generator.h"
#include "google/protobuf/compiler/csharp/csharp_source_generator_base.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

class ReflectionClassGenerator : public SourceGeneratorBase {
 public:
  ReflectionClassGenerator(const FileDescriptor* file, const Options* options);
  ~ReflectionClassGenerator();

  ReflectionClassGenerator(const ReflectionClassGenerator&) = delete;
  ReflectionClassGenerator& operator=(const ReflectionClassGenerator&) = delete;

  void Generate(io::Printer* printer);

 private:
  const FileDescriptor* file_;

  std::string namespace_;
  std::string reflectionClassname_;
  std::string extensionClassname_;

  void WriteIntroduction(io::Printer* printer);
  void WriteDescriptor(io::Printer* printer);
  void WriteGeneratedCodeInfo(const Descriptor* descriptor,
                              io::Printer* printer,
                              bool last);
};

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_REFLECTION_CLASS_H__
