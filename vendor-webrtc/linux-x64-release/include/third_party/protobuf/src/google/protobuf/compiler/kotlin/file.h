// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Generates Kotlin code for a given .proto file.

#ifndef GOOGLE_PROTOBUF_COMPILER_KOTLIN_FILE_H__
#define GOOGLE_PROTOBUF_COMPILER_KOTLIN_FILE_H__

#include <memory>
#include <string>
#include <vector>

#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/compiler/kotlin/message.h"

namespace google {
namespace protobuf {
class FileDescriptor;  // descriptor.h
namespace io {
class Printer;  // printer.h
}
namespace compiler {
class GeneratorContext;  // code_generator.h
namespace java {
class Context;             // context.h
class MessageGenerator;    // message.h
class GeneratorFactory;    // generator_factory.h
class ExtensionGenerator;  // extension.h
class ClassNameResolver;   // name_resolver.h
}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace kotlin {

// TODO: b/366047913 - Move away from these generator classes and more towards
// the "Context" model that the Kotlin Native & Rust code generators use:
// http://google3/third_party/kotlin/protobuf/generator/native/file.cc;l=149-170;rcl=642292300
class FileGenerator {
 public:
  FileGenerator(const FileDescriptor* file, const java::Options& options);
  FileGenerator(const FileGenerator&) = delete;
  FileGenerator& operator=(const FileGenerator&) = delete;
  ~FileGenerator() = default;

  std::string GetKotlinClassname();
  void Generate(io::Printer* printer);
  void GenerateSiblings(const std::string& package_dir,
                        GeneratorContext* generator_context,
                        std::vector<std::string>* file_list,
                        std::vector<std::string>* annotation_list);

  const std::string& java_package() { return java_package_; }

 private:
  const FileDescriptor* file_;
  std::string java_package_;

  std::vector<std::unique_ptr<MessageGenerator>> message_generators_;
  std::unique_ptr<java::Context> context_;
  java::ClassNameResolver* name_resolver_;
  const java::Options options_;
};

}  // namespace kotlin
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_KOTLIN_FILE_H__
