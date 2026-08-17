// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__

#include <memory>
#include <string>
#include <vector>

#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/port.h"

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
namespace java {

class FileGenerator {
 public:
  FileGenerator(const FileDescriptor* file, const Options& options,
                bool immutable_api = true);
  FileGenerator(const FileGenerator&) = delete;
  FileGenerator& operator=(const FileGenerator&) = delete;
  ~FileGenerator();

  // Checks for problems that would otherwise lead to cryptic compile errors.
  // Returns true if there are no problems, or writes an error description to
  // the given string and returns false otherwise.
  bool Validate(std::string* error);

  void Generate(io::Printer* printer);

  // If we aren't putting everything into one file, this will write all the
  // files other than the outer file (i.e. one for each message, enum, and
  // service type).
  void GenerateSiblings(const std::string& package_dir,
                        GeneratorContext* generator_context,
                        std::vector<std::string>* file_list,
                        std::vector<std::string>* annotation_list);

  const std::string& java_package() { return java_package_; }
  const std::string& classname() { return classname_; }

 private:
  void GenerateDescriptorInitializationCodeForImmutable(io::Printer* printer);

  bool ShouldIncludeDependency(const FileDescriptor* descriptor,
                               bool immutable_api_);

  const FileDescriptor* file_;
  std::string java_package_;
  std::string classname_;

  std::vector<std::unique_ptr<MessageGenerator>> message_generators_;
  std::vector<std::unique_ptr<ExtensionGenerator>> extension_generators_;
  std::unique_ptr<Context> context_;
  std::unique_ptr<GeneratorFactory> generator_factory_;
  ClassNameResolver* name_resolver_;
  const Options options_;
  bool immutable_api_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_FILE_H__
