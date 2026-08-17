// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_ENUM_DEF_INTERNAL_H_
#define UPB_REFLECTION_ENUM_DEF_INTERNAL_H_

#include "upb/reflection/enum_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_EnumDef* _upb_EnumDef_At(const upb_EnumDef* e, int i);
bool _upb_EnumDef_Insert(upb_EnumDef* e, upb_EnumValueDef* v, upb_Arena* a);
const upb_MiniTableEnum* _upb_EnumDef_MiniTable(const upb_EnumDef* e);

// Allocate and initialize an array of |n| enum defs.
upb_EnumDef* _upb_EnumDefs_New(upb_DefBuilder* ctx, int n,
                               const UPB_DESC(EnumDescriptorProto*)
                                   const* protos,
                               const UPB_DESC(FeatureSet*) parent_features,
                               const upb_MessageDef* containing_type);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_DEF_INTERNAL_H_ */
