// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_MESSAGE_DEF_INTERNAL_H_
#define UPB_REFLECTION_MESSAGE_DEF_INTERNAL_H_

#include "upb/reflection/message_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_MessageDef* _upb_MessageDef_At(const upb_MessageDef* m, int i);
bool _upb_MessageDef_InMessageSet(const upb_MessageDef* m);
bool _upb_MessageDef_Insert(upb_MessageDef* m, const char* name, size_t size,
                            upb_value v, upb_Arena* a);
void _upb_MessageDef_InsertField(upb_DefBuilder* ctx, upb_MessageDef* m,
                                 const upb_FieldDef* f);
bool _upb_MessageDef_IsValidExtensionNumber(const upb_MessageDef* m, int n);
void _upb_MessageDef_CreateMiniTable(upb_DefBuilder* ctx, upb_MessageDef* m);
void _upb_MessageDef_LinkMiniTable(upb_DefBuilder* ctx,
                                   const upb_MessageDef* m);
void _upb_MessageDef_Resolve(upb_DefBuilder* ctx, upb_MessageDef* m);

// Allocate and initialize an array of |n| message defs.
upb_MessageDef* _upb_MessageDefs_New(upb_DefBuilder* ctx, int n,
                                     const UPB_DESC(DescriptorProto*)
                                         const* protos,
                                     const UPB_DESC(FeatureSet*)
                                         parent_features,
                                     const upb_MessageDef* containing_type);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_MESSAGE_DEF_INTERNAL_H_ */
