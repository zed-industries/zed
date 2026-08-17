// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_CPP_NAMES_H__
#define GOOGLE_PROTOBUF_COMPILER_CPP_NAMES_H__

#include <string>

#include "absl/strings/string_view.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Descriptor;
class EnumDescriptor;
class EnumValueDescriptor;
class FieldDescriptor;
class FileDescriptor;

namespace compiler {
namespace cpp {

// Returns the fully qualified C++ namespace.
//
// For example, if you had:
//   package foo.bar;
//   message Baz { message Moo {} }
// Then the qualified namespace for Moo would be:
//   ::foo::bar
PROTOC_EXPORT std::string Namespace(const FileDescriptor* d);
PROTOC_EXPORT std::string Namespace(const Descriptor* d);
PROTOC_EXPORT std::string Namespace(const FieldDescriptor* d);
PROTOC_EXPORT std::string Namespace(const EnumDescriptor* d);

// Returns the unqualified C++ name.
//
// For example, if you had:
//   package foo.bar;
//   message Baz { message Moo {} }
// Then the non-qualified version would be:
//   Baz_Moo
PROTOC_EXPORT std::string ClassName(const Descriptor* descriptor);
PROTOC_EXPORT std::string ClassName(const EnumDescriptor* enum_descriptor);

// Returns the fully qualified C++ name.
//
// For example, if you had:
//   package foo.bar;
//   message Baz { message Moo {} }
// Then the qualified ClassName for Moo would be:
//   ::foo::bar::Baz_Moo
PROTOC_EXPORT std::string QualifiedClassName(const Descriptor* d);
PROTOC_EXPORT std::string QualifiedClassName(const EnumDescriptor* d);
PROTOC_EXPORT std::string QualifiedExtensionName(const FieldDescriptor* d);

// Get the (unqualified) name that should be used for this field in C++ code.
// The name is coerced to lower-case to emulate proto1 behavior.  People
// should be using lowercase-with-underscores style for proto field names
// anyway, so normally this just returns field->name().
PROTOC_EXPORT std::string FieldName(const FieldDescriptor* field);

// Requires that this field is in a oneof. Returns the (unqualified) case
// constant for this field.
PROTOC_EXPORT std::string OneofCaseConstantName(const FieldDescriptor* field);
// Returns the quafilied case constant for this field.
PROTOC_EXPORT std::string QualifiedOneofCaseConstantName(
    const FieldDescriptor* field);

// Get the (unqualified) name that should be used for this enum value in C++
// code.
PROTOC_EXPORT std::string EnumValueName(const EnumValueDescriptor* enum_value);

// Strips ".proto" or ".protodevel" from the end of a filename.
PROTOC_EXPORT std::string StripProto(absl::string_view filename);

}  // namespace cpp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CPP_NAMES_H__
