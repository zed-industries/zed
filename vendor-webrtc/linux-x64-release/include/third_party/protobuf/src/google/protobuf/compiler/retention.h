// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_COMPILER_RETENTION_H__
#define GOOGLE_PROTOBUF_COMPILER_RETENTION_H__

#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"

// Must appear last
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace compiler {

// Returns a FileDescriptorProto for this file, with all RETENTION_SOURCE
// options stripped out. If include_source_code_info is true, this function
// will also populate the source code info but strip out the parts of it
// corresponding to source-retention options.
PROTOC_EXPORT FileDescriptorProto StripSourceRetentionOptions(
    const FileDescriptor& file, bool include_source_code_info = false);
PROTOC_EXPORT void StripSourceRetentionOptions(const DescriptorPool& pool,
                                               FileDescriptorProto& file_proto);
PROTOC_EXPORT DescriptorProto
StripSourceRetentionOptions(const Descriptor& message);
PROTOC_EXPORT DescriptorProto::ExtensionRange StripSourceRetentionOptions(
    const Descriptor& message, const Descriptor::ExtensionRange& range);
PROTOC_EXPORT EnumDescriptorProto
StripSourceRetentionOptions(const EnumDescriptor& enm);
PROTOC_EXPORT FieldDescriptorProto
StripSourceRetentionOptions(const FieldDescriptor& field);
PROTOC_EXPORT OneofDescriptorProto
StripSourceRetentionOptions(const OneofDescriptor& oneof);

// The following functions take a descriptor and strip all source-retention
// options from just the local entity (e.g. message, enum, field). Most code
// generators should not need these functions, but they are sometimes useful if
// you need to strip the options on a single entity rather than handling the
// entire file at once.
PROTOC_EXPORT EnumOptions
StripLocalSourceRetentionOptions(const EnumDescriptor& descriptor);
PROTOC_EXPORT EnumValueOptions
StripLocalSourceRetentionOptions(const EnumValueDescriptor& descriptor);
PROTOC_EXPORT FieldOptions
StripLocalSourceRetentionOptions(const FieldDescriptor& descriptor);
PROTOC_EXPORT FileOptions
StripLocalSourceRetentionOptions(const FileDescriptor& descriptor);
PROTOC_EXPORT MessageOptions
StripLocalSourceRetentionOptions(const Descriptor& descriptor);
PROTOC_EXPORT ExtensionRangeOptions StripLocalSourceRetentionOptions(
    const Descriptor& descriptor, const Descriptor::ExtensionRange& range);
PROTOC_EXPORT MethodOptions
StripLocalSourceRetentionOptions(const MethodDescriptor& descriptor);
PROTOC_EXPORT OneofOptions
StripLocalSourceRetentionOptions(const OneofDescriptor& descriptor);
PROTOC_EXPORT ServiceOptions
StripLocalSourceRetentionOptions(const ServiceDescriptor& descriptor);

}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMPILER_RETENTION_H__
