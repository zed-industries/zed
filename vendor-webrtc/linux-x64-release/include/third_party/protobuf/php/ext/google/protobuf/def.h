// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PHP_PROTOBUF_DEF_H_
#define PHP_PROTOBUF_DEF_H_

#include <php.h>

#include "php-upb.h"

// Initializes the Def module, which defines all of the descriptor classes.
void Def_ModuleInit();

// Creates a new DescriptorPool to wrap the given symtab, which must not be
// NULL.
void DescriptorPool_CreateWithSymbolTable(zval* zv, upb_DefPool* symtab);

upb_DefPool* DescriptorPool_GetSymbolTable();

// Returns true if the global descriptor pool already has the given filename.
bool DescriptorPool_HasFile(const char* filename);

// Adds the given descriptor with the given filename to the global pool.
void DescriptorPool_AddDescriptor(const char* filename, const char* data,
                                  int size);

typedef struct Descriptor {
  zend_object std;
  const upb_MessageDef* msgdef;
  zend_class_entry* class_entry;
} Descriptor;

// Gets or creates a Descriptor* for the given class entry, upb_MessageDef, or
// upb_FieldDef. The returned Descriptor* will live for the entire request,
// so no ref is necessary to keep it alive. The caller does *not* own a ref
// on the returned object.
Descriptor* Descriptor_GetFromClassEntry(zend_class_entry* ce);
Descriptor* Descriptor_GetFromMessageDef(const upb_MessageDef* m);
Descriptor* Descriptor_GetFromFieldDef(const upb_FieldDef* f);

// Packages up a upb_CType with a Descriptor, since many functions need
// both.
typedef struct {
  upb_CType type;
  const Descriptor* desc;  // When type == kUpb_CType_Message.
} TypeInfo;

static inline TypeInfo TypeInfo_Get(const upb_FieldDef* f) {
  TypeInfo ret = {upb_FieldDef_CType(f), Descriptor_GetFromFieldDef(f)};
  return ret;
}

static inline TypeInfo TypeInfo_FromType(upb_CType type) {
  TypeInfo ret = {type};
  return ret;
}

static inline bool TypeInfo_Eq(TypeInfo a, TypeInfo b) {
  if (a.type != b.type) return false;
  if (a.type == kUpb_CType_Message && a.desc != b.desc) return false;
  return true;
}

#endif  // PHP_PROTOBUF_DEF_H_
