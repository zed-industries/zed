// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
//  Based on original Protocol Buffers design by
//  Sanjay Ghemawat, Jeff Dean, and others.

#ifndef GOOGLE_PROTOBUF_COMPILER_CSHARP_HELPERS_H__
#define GOOGLE_PROTOBUF_COMPILER_CSHARP_HELPERS_H__

#include <string>

#include "google/protobuf/compiler/code_generator.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/csharp/names.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/io/printer.h"
#include "google/protobuf/port.h"
#include "google/protobuf/port_def.inc"
#include "google/protobuf/stubs/common.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace csharp {

struct Options;
class FieldGeneratorBase;

// TODO: start using this enum.
enum CSharpType {
  CSHARPTYPE_INT32 = 1,
  CSHARPTYPE_INT64 = 2,
  CSHARPTYPE_UINT32 = 3,
  CSHARPTYPE_UINT64 = 4,
  CSHARPTYPE_FLOAT = 5,
  CSHARPTYPE_DOUBLE = 6,
  CSHARPTYPE_BOOL = 7,
  CSHARPTYPE_STRING = 8,
  CSHARPTYPE_BYTESTRING = 9,
  CSHARPTYPE_MESSAGE = 10,
  CSHARPTYPE_ENUM = 11,
  MAX_CSHARPTYPE = 11
};

// Converts field type to corresponding C# type.
CSharpType GetCSharpType(FieldDescriptor::Type type);

std::string GetFieldName(const FieldDescriptor* descriptor);

std::string GetFieldConstantName(const FieldDescriptor* field);

std::string GetPropertyName(const FieldDescriptor* descriptor);

std::string GetOneofCaseName(const FieldDescriptor* descriptor);

int GetFixedSize(FieldDescriptor::Type type);

// Note that we wouldn't normally want to export this (we're not expecting
// it to be used outside libprotoc itself) but this exposes it for testing.
std::string PROTOC_EXPORT GetEnumValueName(absl::string_view enum_name,
                                           absl::string_view enum_value_name);

// TODO: perhaps we could move this to strutil
std::string StringToBase64(absl::string_view input);

std::string FileDescriptorToBase64(const FileDescriptor* descriptor);

FieldGeneratorBase* CreateFieldGenerator(const FieldDescriptor* descriptor,
                                         int presenceIndex,
                                         const Options* options);

std::string GetFullExtensionName(const FieldDescriptor* descriptor);

bool IsNullable(const FieldDescriptor* descriptor);

// Determines whether the given message is a map entry message,
// i.e. one implicitly created by protoc due to a map<key, value> field.
inline bool IsMapEntryMessage(const Descriptor* descriptor) {
  return descriptor->options().map_entry();
}

// Determines whether we're generating code for the proto representation of
// descriptors etc, for use in the runtime. This is the only type which is
// allowed to use proto2 syntax, and it generates internal classes.
inline bool IsDescriptorProto(const FileDescriptor* descriptor) {
  return descriptor->name() == "google/protobuf/descriptor.proto" ||
         descriptor->name() == "net/proto2/proto/descriptor.proto";
}

// Determines whether the given message is an options message within descriptor.proto.
inline bool IsDescriptorOptionMessage(const Descriptor* descriptor) {
  if (!IsDescriptorProto(descriptor->file())) {
    return false;
  }
  const absl::string_view name = descriptor->name();
  return name == "FileOptions" ||
      name == "MessageOptions" ||
      name == "FieldOptions" ||
      name == "OneofOptions" ||
      name == "EnumOptions" ||
      name == "EnumValueOptions" ||
      name == "ServiceOptions" ||
      name == "MethodOptions";
}

inline bool IsWrapperType(const FieldDescriptor* descriptor) {
  return descriptor->type() == FieldDescriptor::TYPE_MESSAGE &&
      descriptor->message_type()->file()->name() == "google/protobuf/wrappers.proto";
}

inline bool SupportsPresenceApi(const FieldDescriptor* descriptor) {
  // Unlike most languages, we don't generate Has/Clear members for message
  // types, because they can always be set to null in C#. They're not really
  // needed for oneof fields in proto2 either, as everything can be done via
  // oneof case, but we follow the convention from other languages.
  if (descriptor->type() == FieldDescriptor::TYPE_MESSAGE) {
    return false;
  }

  return descriptor->has_presence();
}

inline bool RequiresPresenceBit(const FieldDescriptor* descriptor) {
  return SupportsPresenceApi(descriptor) &&
    !IsNullable(descriptor) &&
    !descriptor->is_extension() &&
    !descriptor->real_containing_oneof();
}

}  // namespace csharp
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_CSHARP_HELPERS_H__
