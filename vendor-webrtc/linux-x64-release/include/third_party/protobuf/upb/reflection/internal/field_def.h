// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_FIELD_DEF_INTERNAL_H_
#define UPB_REFLECTION_FIELD_DEF_INTERNAL_H_

#include "upb/reflection/field_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_FieldDef* _upb_FieldDef_At(const upb_FieldDef* f, int i);

bool _upb_FieldDef_IsClosedEnum(const upb_FieldDef* f);
bool _upb_FieldDef_IsProto3Optional(const upb_FieldDef* f);
int _upb_FieldDef_LayoutIndex(const upb_FieldDef* f);
uint64_t _upb_FieldDef_Modifiers(const upb_FieldDef* f);
void _upb_FieldDef_Resolve(upb_DefBuilder* ctx, const char* prefix,
                           upb_FieldDef* f);
void _upb_FieldDef_BuildMiniTableExtension(upb_DefBuilder* ctx,
                                           const upb_FieldDef* f);

// Allocate and initialize an array of |n| extensions (field defs).
upb_FieldDef* _upb_Extensions_New(upb_DefBuilder* ctx, int n,
                                  const UPB_DESC(FieldDescriptorProto*)
                                      const* protos,
                                  const UPB_DESC(FeatureSet*) parent_features,
                                  const char* prefix, upb_MessageDef* m);

// Allocate and initialize an array of |n| field defs.
upb_FieldDef* _upb_FieldDefs_New(upb_DefBuilder* ctx, int n,
                                 const UPB_DESC(FieldDescriptorProto*)
                                     const* protos,
                                 const UPB_DESC(FeatureSet*) parent_features,
                                 const char* prefix, upb_MessageDef* m,
                                 bool* is_sorted);

// Allocate and return a list of pointers to the |n| field defs in |ff|,
// sorted by field number.
const upb_FieldDef** _upb_FieldDefs_Sorted(const upb_FieldDef* f, int n,
                                           upb_Arena* a);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_FIELD_DEF_INTERNAL_H_ */
