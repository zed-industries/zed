// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: xiaofeng@google.com (Feng Xiao)
//
// Generators that generate shared code between immutable API and mutable API.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_SHARED_CODE_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_SHARED_CODE_GENERATOR_H__

#include <memory>
#include <string>
#include <vector>

#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/port.h"

namespace google {
namespace protobuf {
class FileDescriptor;  // descriptor.h
namespace compiler {
class GeneratorContext;  // code_generator.h
namespace java {
class ClassNameResolver;  // name_resolver.h
}
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

// A generator that generates code that are shared between immutable API
// and mutable API. Currently only descriptors are shared.
class SharedCodeGenerator {
 public:
  SharedCodeGenerator(const FileDescriptor* file, const Options& options);
  SharedCodeGenerator(const SharedCodeGenerator&) = delete;
  SharedCodeGenerator& operator=(const SharedCodeGenerator&) = delete;
  ~SharedCodeGenerator();

  void Generate(GeneratorContext* generator_context,
                std::vector<std::string>* file_list,
                std::vector<std::string>* annotation_file_list);

  void GenerateDescriptors(io::Printer* printer);

 private:
  std::unique_ptr<ClassNameResolver> name_resolver_;
  const FileDescriptor* file_;
  const Options options_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_SHARED_CODE_GENERATOR_H__
