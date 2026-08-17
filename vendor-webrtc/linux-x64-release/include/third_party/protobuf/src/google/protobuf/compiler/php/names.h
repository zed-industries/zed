// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_PHP_NAMES_H__
#define GOOGLE_PROTOBUF_COMPILER_PHP_NAMES_H__

#include "google/protobuf/descriptor.h"

#include <string>

#include "absl/strings/string_view.h"
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {
namespace php {

// Whether or not a name is reserved.
PROTOC_EXPORT bool IsReservedName(absl::string_view name);

// A prefix to stick in front of reserved names to avoid clashes.
PROTOC_EXPORT std::string ReservedNamePrefix(absl::string_view classname,
                                             const FileDescriptor* file);

// A prefix to stick in front of all class names.
PROTOC_EXPORT std::string ClassNamePrefix(absl::string_view classname,
                                          const Descriptor* desc);
PROTOC_EXPORT std::string ClassNamePrefix(absl::string_view classname,
                                          const EnumDescriptor* desc);

// To skip reserved keywords in php, some generated classname are prefixed.
// Other code generators may need following API to figure out the actual
// classname.
PROTOC_EXPORT std::string GeneratedClassName(const Descriptor* desc);
PROTOC_EXPORT std::string GeneratedClassName(const EnumDescriptor* desc);
PROTOC_EXPORT std::string GeneratedClassName(const ServiceDescriptor* desc);

}  // namespace php
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_PHP_NAMES_H__
