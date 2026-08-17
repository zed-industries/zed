// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_ENUM_VALUE_DEF_INTERNAL_H_
#define UPB_REFLECTION_ENUM_VALUE_DEF_INTERNAL_H_

#include "upb/reflection/enum_value_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_EnumValueDef* _upb_EnumValueDef_At(const upb_EnumValueDef* v, int i);

// Allocate and initialize an array of |n| enum value defs owned by |e|.
upb_EnumValueDef* _upb_EnumValueDefs_New(
    upb_DefBuilder* ctx, const char* prefix, int n,
    const UPB_DESC(EnumValueDescriptorProto*) const* protos,
    const UPB_DESC(FeatureSet*) parent_features, upb_EnumDef* e,
    bool* is_sorted);

const upb_EnumValueDef** _upb_EnumValueDefs_Sorted(const upb_EnumValueDef* v,
                                                   int n, upb_Arena* a);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_VALUE_DEF_INTERNAL_H_ */
