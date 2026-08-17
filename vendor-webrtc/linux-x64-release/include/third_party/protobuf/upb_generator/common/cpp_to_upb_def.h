// Copyright (c) 2024 Google LLC
// All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_GENERATOR_COMMON_DESC_HELPERS_H_
#define THIRD_PARTY_UPB_UPB_GENERATOR_COMMON_DESC_HELPERS_H_

#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/descriptor.upb.h"
#include "google/protobuf/descriptor.h"
#include "upb/mem/arena.hpp"
#include "upb/mini_table/field.h"
#include "upb/reflection/def.hpp"

namespace upb::generator {

using google::protobuf::Descriptor;
using google::protobuf::EnumDescriptor;
using google::protobuf::FieldDescriptor;
using google::protobuf::FileDescriptor;

// Add a filedesc to defpool, and all its dependencies.
void AddFile(const FileDescriptor* file, upb::DefPool* pool);

// Given a Descriptor, returns a MessageDefPtr.
// This will fail if the message is not in the defpool.
// Files can be added to the defpool using AddFile.
upb::MessageDefPtr FindMessageDef(upb::DefPool& pool,
                                  const Descriptor* descriptor);

// Given an EnumDescriptor, returns an EnumDefPtr.
// This will fail if the enum is not in the defpool.
upb::EnumDefPtr FindEnumDef(upb::DefPool& pool,
                            const EnumDescriptor* enum_descriptor);

// Given a FieldDescriptor, returns a FieldDefPtr.
// This will fail if the field is not in the defpool.
// This is for non-extension fields. For extensions, use FindExtensionDef.
upb::FieldDefPtr FindBaseFieldDef(upb::DefPool& pool,
                                  const FieldDescriptor* field);

// Given a FieldDescriptor, returns a FieldDefPtr.
// This will fail if the field is not in the defpool.
// This is solely for extension fields.
upb::FieldDefPtr FindExtensionDef(upb::DefPool& pool,
                                  const FieldDescriptor* field);

// Looks up a FieldDescriptor from a upb_MiniTableField.
// This will fail if the field is not in the message.
const FieldDescriptor* FindFieldDescriptor(const Descriptor* message,
                                           const upb_MiniTableField* field_def);

}  // namespace upb::generator

#endif  // THIRD_PARTY_UPB_UPB_GENERATOR_COMMON_DESC_HELPERS_H_
