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
// Provides a mechanism for mapping a descriptor to the
// fully-qualified name of the corresponding C# class.

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_NAMES_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_NAMES_H__

#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Descriptor;
class EnumDescriptor;
class FileDescriptor;
class ServiceDescriptor;

namespace compiler {
namespace csharp {

// Requires:
//   descriptor != NULL
//
// Returns:
//   The namespace to use for given file descriptor.
std::string PROTOC_EXPORT GetFileNamespace(const FileDescriptor* descriptor);

// Requires:
//   descriptor != NULL
//
// Returns:
//   The fully-qualified C# class name.
std::string PROTOC_EXPORT GetClassName(const Descriptor* descriptor);

// Requires:
//   descriptor != NULL
//
// Returns:
//   The fully-qualified C# enum class name.
std::string GetClassName(const EnumDescriptor* descriptor);

// Requires:
//   descriptor != NULL
//
// Returns:
//   The unqualified name of the C# class that provides access to the file
//   descriptor. Proto compiler generates
//   such class for each .proto file processed.
std::string GetReflectionClassUnqualifiedName(const FileDescriptor* descriptor);

// Gets unqualified name of the extension class
// Requires:
//   descriptor != NULL
//
// Returns:
//   The unqualified name of the generated C# extensions class that provide
//   access to extensions. Proto compiler generates such class for each
//   .proto file processed that contains extensions.
std::string GetExtensionClassUnqualifiedName(const FileDescriptor* descriptor);

// Requires:
//   descriptor != NULL
//
// Returns:
//   The fully-qualified name of the C# class that provides access to the file
//   descriptor. Proto compiler generates such class for each .proto file
//   processed.
std::string PROTOC_EXPORT
GetReflectionClassName(const FileDescriptor* descriptor);

// Generates output file name for given file descriptor. If generate_directories
// is true, the output file will be put under directory corresponding to file's
// namespace. base_namespace can be used to strip some of the top level
// directories. E.g. for file with namespace "Bar.Foo" and base_namespace="Bar",
// the resulting file will be put under directory "Foo" (and not "Bar/Foo").
//
// Requires:
//   descriptor != NULL
//   error != NULL
//
//  Returns:
//    The file name to use as output file for given file descriptor. In case
//    of failure, this function will return empty string and error parameter
//    will contain the error message.
std::string PROTOC_EXPORT GetOutputFile(const FileDescriptor* descriptor,
                                        absl::string_view file_extension,
                                        bool generate_directories,
                                        absl::string_view base_namespace,
                                        std::string* error);

std::string UnderscoresToPascalCase(absl::string_view input);

// Note that we wouldn't normally want to export this (we're not expecting
// it to be used outside libprotoc itself) but this exposes it for testing.
std::string PROTOC_EXPORT UnderscoresToCamelCase(absl::string_view input,
                                                 bool cap_next_letter,
                                                 bool preserve_period);

inline std::string UnderscoresToCamelCase(absl::string_view input,
                                          bool cap_next_letter) {
  return UnderscoresToCamelCase(input, cap_next_letter, false);
}

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_NAMES_H__
