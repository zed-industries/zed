// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_NAME_RESOLVER_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_NAME_RESOLVER_H__

#include <string>

#include "absl/container/flat_hash_map.h"
#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/port.h"

// Must be last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
class Descriptor;
class EnumDescriptor;
class FieldDescriptor;
class FileDescriptor;
class ServiceDescriptor;

namespace compiler {
namespace java {

// Indicates how closely the two class names match.
enum NameEquality { NO_MATCH, EXACT_EQUAL, EQUAL_IGNORE_CASE };

// Used to get the Java class related names for a given descriptor. It caches
// the results to avoid redundant calculation across multiple name queries.
// Thread-safety note: This class is *not* thread-safe.
class PROTOC_EXPORT ClassNameResolver {
 public:
  explicit ClassNameResolver(const Options& options = {}) : options_(options) {}
  ~ClassNameResolver() = default;

  ClassNameResolver(const ClassNameResolver&) = delete;
  ClassNameResolver& operator=(const ClassNameResolver&) = delete;

  // Gets the unqualified outer class name for the file.
  std::string GetFileClassName(const FileDescriptor* file, bool immutable);
  std::string GetFileClassName(const FileDescriptor* file, bool immutable,
                               bool kotlin);
  // Gets the unqualified immutable outer class name of a file.
  std::string GetFileImmutableClassName(const FileDescriptor* file);
  // Gets the unqualified default immutable outer class name of a file
  // (converted from the proto file's name).
  std::string GetFileDefaultImmutableClassName(const FileDescriptor* file);

  // Check whether there is any type defined in the proto file that has
  // the given class name.
  bool HasConflictingClassName(const FileDescriptor* file,
                               absl::string_view classname,
                               NameEquality equality_mode);

  // Gets the name of the outer class that holds descriptor information.
  // Descriptors are shared between immutable messages and mutable messages.
  // Since both of them are generated optionally, the descriptors need to be
  // put in another common place.
  std::string GetDescriptorClassName(const FileDescriptor* file);

  // Gets the fully-qualified class name corresponding to the given descriptor.
  std::string GetClassName(const Descriptor* descriptor, bool immutable);
  std::string GetClassName(const Descriptor* descriptor, bool immutable,
                           bool kotlin);
  std::string GetClassName(const EnumDescriptor* descriptor, bool immutable);
  std::string GetClassName(const EnumDescriptor* descriptor, bool immutable,
                           bool kotlin);
  std::string GetClassName(const ServiceDescriptor* descriptor, bool immutable);
  std::string GetClassName(const ServiceDescriptor* descriptor, bool immutable,
                           bool kotlin);
  std::string GetClassName(const FileDescriptor* descriptor, bool immutable);
  std::string GetClassName(const FileDescriptor* descriptor, bool immutable,
                           bool kotlin);

  template <class DescriptorType>
  std::string GetImmutableClassName(const DescriptorType* descriptor) {
    return GetClassName(descriptor, true);
  }
  template <class DescriptorType>
  std::string GetMutableClassName(const DescriptorType* descriptor) {
    return GetClassName(descriptor, false);
  }

  // Gets the fully qualified name of an extension identifier.
  std::string GetExtensionIdentifierName(const FieldDescriptor* descriptor,
                                         bool immutable);
  std::string GetExtensionIdentifierName(const FieldDescriptor* descriptor,
                                         bool immutable, bool kotlin);

  // Gets the fully qualified name for generated classes in Java convention.
  // Nested classes will be separated using '$' instead of '.'
  // For example:
  //   com.package.OuterClass$OuterMessage$InnerMessage
  std::string GetJavaImmutableClassName(const Descriptor* descriptor);
  std::string GetJavaImmutableClassName(const EnumDescriptor* descriptor);
  std::string GetJavaImmutableClassName(const ServiceDescriptor* descriptor);
  std::string GetKotlinFactoryName(const Descriptor* descriptor);
  std::string GetKotlinExtensionsClassName(const Descriptor* descriptor);
  std::string GetKotlinExtensionsClassNameEscaped(const Descriptor* descriptor);
  std::string GetJavaMutableClassName(const Descriptor* descriptor);
  std::string GetJavaMutableClassName(const EnumDescriptor* descriptor);
  std::string GetJavaMutableClassName(const ServiceDescriptor* descriptor);
  // Gets the outer class and the actual class for downgraded mutable messages.
  std::string GetDowngradedFileClassName(const FileDescriptor* file);
  std::string GetDowngradedClassName(const Descriptor* descriptor);

  // Get the full name of a Java class by prepending the Java package name
  // or outer class name.
  std::string GetClassFullName(absl::string_view name_without_package,
                               const FileDescriptor* file, bool immutable,
                               bool is_own_file);
  std::string GetClassFullName(absl::string_view name_without_package,
                               const FileDescriptor* file, bool immutable,
                               bool is_own_file, bool kotlin);

  Options options_;

 private:
  // Get the Java Class style full name of a message.
  std::string GetJavaClassFullName(absl::string_view name_without_package,
                                   const FileDescriptor* file, bool immutable);
  std::string GetJavaClassFullName(absl::string_view name_without_package,
                                   const FileDescriptor* file, bool immutable,
                                   bool kotlin);
  // Caches the result to provide better performance.
  absl::flat_hash_map<const FileDescriptor*, std::string>
      file_immutable_outer_class_names_;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_NAME_RESOLVER_H__
