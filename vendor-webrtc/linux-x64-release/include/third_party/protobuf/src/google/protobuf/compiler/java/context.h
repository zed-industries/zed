// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_CONTEXT_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_CONTEXT_H__

#include <memory>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "google/protobuf/compiler/java/field_common.h"
#include "google/protobuf/compiler/java/helpers.h"
#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/port.h"

namespace google {
namespace protobuf {
class FileDescriptor;
class FieldDescriptor;
class OneofDescriptor;
class Descriptor;
class EnumDescriptor;
namespace compiler {
namespace java {
class ClassNameResolver;  // name_resolver.h
}
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// A context object holds the information that is shared among all code
// generators.
class Context {
 public:
  Context(const FileDescriptor* file, const Options& options);
  Context(const Context&) = delete;
  Context& operator=(const Context&) = delete;
  ~Context();

  // Get the name resolver associated with this context. The resolver
  // can be used to map descriptors to Java class names.
  ClassNameResolver* GetNameResolver() const;

  // Get the FieldGeneratorInfo for a given field.
  const FieldGeneratorInfo* GetFieldGeneratorInfo(
      const FieldDescriptor* field) const;

  // Get the OneofGeneratorInfo for a given oneof.
  const OneofGeneratorInfo* GetOneofGeneratorInfo(
      const OneofDescriptor* oneof) const;

  const Options& options() const { return options_; }

  // Enforces all the files (including transitive dependencies) to use
  // LiteRuntime.

  bool EnforceLite() const { return options_.enforce_lite; }

  // Does this message class have generated parsing, serialization, and other
  // standard methods for which reflection-based fallback implementations exist?
  bool HasGeneratedMethods(const Descriptor* descriptor) const;

 private:
  void InitializeFieldGeneratorInfo(const FileDescriptor* file);
  void InitializeFieldGeneratorInfoForMessage(const Descriptor* message);
  void InitializeFieldGeneratorInfoForFields(
      const std::vector<const FieldDescriptor*>& fields);

  std::unique_ptr<ClassNameResolver> name_resolver_;
  absl::flat_hash_map<const FieldDescriptor*, FieldGeneratorInfo>
      field_generator_info_map_;
  absl::flat_hash_map<const OneofDescriptor*, OneofGeneratorInfo>
      oneof_generator_info_map_;
  Options options_;
};

template <typename Descriptor>
void MaybePrintGeneratedAnnotation(Context* context, io::Printer* printer,
                                   Descriptor* descriptor, bool immutable,
                                   const std::string& suffix = "") {
  if (IsOwnFile(descriptor, immutable)) {
    PrintGeneratedAnnotation(printer, '$',
                             context->options().annotate_code
                                 ? AnnotationFileName(descriptor, suffix)
                                 : "",
                             context->options());
  }
}


}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_CONTEXT_H__
