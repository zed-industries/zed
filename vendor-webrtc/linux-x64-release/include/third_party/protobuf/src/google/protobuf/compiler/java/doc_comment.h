// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_DOC_COMMENT_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_DOC_COMMENT_H__

#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/descriptor.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {
class Printer;  // printer.h
}
}  // namespace protobuf
}  // namespace google

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

enum FieldAccessorType {
  HAZZER,
  GETTER,
  SETTER,
  CLEARER,
  // Repeated
  LIST_COUNT,
  LIST_GETTER,
  LIST_INDEXED_GETTER,
  LIST_INDEXED_SETTER,
  LIST_ADDER,
  LIST_MULTI_ADDER
};

void WriteMessageDocComment(io::Printer* printer, const Descriptor* message,
                            Options options, bool kdoc = false);
void WriteFieldDocComment(io::Printer* printer, const FieldDescriptor* field,
                          Options options, bool kdoc = false);
void WriteFieldAccessorDocComment(io::Printer* printer,
                                  const FieldDescriptor* field,
                                  FieldAccessorType type, Options options,
                                  bool builder = false, bool kdoc = false);
void WriteFieldEnumValueAccessorDocComment(
    io::Printer* printer, const FieldDescriptor* field, FieldAccessorType type,
    Options options, bool builder = false, bool kdoc = false);
void WriteFieldStringBytesAccessorDocComment(
    io::Printer* printer, const FieldDescriptor* field, FieldAccessorType type,
    Options options, bool builder = false, bool kdoc = false);
void WriteEnumDocComment(io::Printer* printer, const EnumDescriptor* enum_,
                         Options options, bool kdoc = false);
void WriteEnumValueDocComment(io::Printer* printer,
                              const EnumValueDescriptor* value,
                              Options options);
void WriteServiceDocComment(io::Printer* printer,
                            const ServiceDescriptor* service, Options options);
void WriteMethodDocComment(io::Printer* printer, const MethodDescriptor* method,
                           Options options);

// Exposed for testing only.
// Also called by proto1-Java code generator.
PROTOC_EXPORT std::string EscapeJavadoc(absl::string_view input);

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_DOC_COMMENT_H__
