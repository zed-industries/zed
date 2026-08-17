// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_COMPILER_HBP_GEN_NAMES_H_
#define PROTOBUF_COMPILER_HBP_GEN_NAMES_H_

#include <string>

#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/compiler/hpb/context.h"

namespace google::protobuf::hpb_generator {

namespace protobuf = ::proto2;

inline constexpr absl::string_view kNoPackageNamePrefix = "hpb_";

std::string ClassName(const protobuf::Descriptor* descriptor);
std::string QualifiedClassName(const protobuf::Descriptor* descriptor);
std::string QualifiedInternalClassName(const protobuf::Descriptor* descriptor);

std::string CppSourceFilename(const google::protobuf::FileDescriptor* file);
std::string ForwardingHeaderFilename(const google::protobuf::FileDescriptor* file);
std::string UpbCFilename(const google::protobuf::FileDescriptor* file);
std::string CppHeaderFilename(const google::protobuf::FileDescriptor* file);

void WriteStartNamespace(const protobuf::FileDescriptor* file, Context& ctx);
void WriteEndNamespace(const protobuf::FileDescriptor* file, Context& ctx);

std::string CppConstType(const protobuf::FieldDescriptor* field);
std::string CppTypeParameterName(const protobuf::FieldDescriptor* field);

std::string MessageBaseType(const protobuf::FieldDescriptor* field,
                            bool is_const);
// Generate hpb::Ptr<const Model> to be used in accessors as public
// signatures.
std::string MessagePtrConstType(const protobuf::FieldDescriptor* field,
                                bool is_const);
std::string MessageCProxyType(const protobuf::FieldDescriptor* field,
                              bool is_const);
std::string MessageProxyType(const protobuf::FieldDescriptor* field,
                             bool is_const);

}  // namespace protobuf
}  // namespace google::hpb_generator

#endif  // PROTOBUF_COMPILER_HBP_GEN_NAMES_H_
