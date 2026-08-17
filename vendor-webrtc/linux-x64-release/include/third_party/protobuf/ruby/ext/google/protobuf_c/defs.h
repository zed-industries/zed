// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef RUBY_PROTOBUF_DEFS_H_
#define RUBY_PROTOBUF_DEFS_H_

#include "protobuf.h"
#include "ruby-upb.h"

// -----------------------------------------------------------------------------
// TypeInfo
// -----------------------------------------------------------------------------

// This bundles a upb_CType and msgdef/enumdef when appropriate. This is
// convenient for functions that need type information but cannot necessarily
// assume a upb_FieldDef will be available.
//
// For example, Google::Protobuf::Map and Google::Protobuf::RepeatedField can
// be constructed with type information alone:
//
//   # RepeatedField will internally store the type information in a TypeInfo.
//   Google::Protobuf::RepeatedField.new(:message, FooMessage)

typedef struct {
  upb_CType type;
  union {
    const upb_MessageDef* msgdef;  // When type == kUpb_CType_Message
    const upb_EnumDef* enumdef;    // When type == kUpb_CType_Enum
  } def;
} TypeInfo;

static inline TypeInfo TypeInfo_get(const upb_FieldDef* f) {
  TypeInfo ret = {upb_FieldDef_CType(f), {NULL}};
  switch (ret.type) {
    case kUpb_CType_Message:
      ret.def.msgdef = upb_FieldDef_MessageSubDef(f);
      break;
    case kUpb_CType_Enum:
      ret.def.enumdef = upb_FieldDef_EnumSubDef(f);
      break;
    default:
      break;
  }
  return ret;
}

TypeInfo TypeInfo_FromClass(int argc, VALUE* argv, int skip_arg,
                            VALUE* type_class, VALUE* init_arg);

static inline TypeInfo TypeInfo_from_type(upb_CType type) {
  TypeInfo ret = {type};
  assert(type != kUpb_CType_Message && type != kUpb_CType_Enum);
  return ret;
}

// -----------------------------------------------------------------------------
// Other utilities
// -----------------------------------------------------------------------------

VALUE Descriptor_DefToClass(const upb_MessageDef* m);

// Returns the underlying msgdef, enumdef, or symtab (respectively) for the
// given Descriptor, EnumDescriptor, or DescriptorPool Ruby object.
const upb_EnumDef* EnumDescriptor_GetEnumDef(VALUE enum_desc_rb);
const upb_DefPool* DescriptorPool_GetSymtab(VALUE desc_pool_rb);
const upb_MessageDef* Descriptor_GetMsgDef(VALUE desc_rb);

// Returns a upb field type for the given Ruby symbol
// (eg. :float => kUpb_CType_Float).
upb_CType ruby_to_fieldtype(VALUE type);

// The singleton generated pool (a DescriptorPool object).
extern VALUE generated_pool;

// Call at startup to register all types in this module.
void Defs_register(VALUE module);

#endif  // RUBY_PROTOBUF_DEFS_H_
