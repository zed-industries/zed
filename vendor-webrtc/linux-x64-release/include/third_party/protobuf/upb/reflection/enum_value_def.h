// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_ENUM_VALUE_DEF_H_
#define UPB_REFLECTION_ENUM_VALUE_DEF_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

const upb_EnumDef* upb_EnumValueDef_Enum(const upb_EnumValueDef* v);
const char* upb_EnumValueDef_FullName(const upb_EnumValueDef* v);
bool upb_EnumValueDef_HasOptions(const upb_EnumValueDef* v);
uint32_t upb_EnumValueDef_Index(const upb_EnumValueDef* v);
UPB_API const char* upb_EnumValueDef_Name(const upb_EnumValueDef* v);
UPB_API int32_t upb_EnumValueDef_Number(const upb_EnumValueDef* v);
const UPB_DESC(EnumValueOptions) *
    upb_EnumValueDef_Options(const upb_EnumValueDef* v);
const UPB_DESC(FeatureSet) *
    upb_EnumValueDef_ResolvedFeatures(const upb_EnumValueDef* e);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_VALUE_DEF_H_ */
