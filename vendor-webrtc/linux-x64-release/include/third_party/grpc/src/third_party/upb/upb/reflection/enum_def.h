// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_ENUM_DEF_H_
#define UPB_REFLECTION_ENUM_DEF_H_

#include "upb/base/string_view.h"
#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

bool upb_EnumDef_CheckNumber(const upb_EnumDef* e, int32_t num);
const upb_MessageDef* upb_EnumDef_ContainingType(const upb_EnumDef* e);
int32_t upb_EnumDef_Default(const upb_EnumDef* e);
UPB_API const upb_FileDef* upb_EnumDef_File(const upb_EnumDef* e);
const upb_EnumValueDef* upb_EnumDef_FindValueByName(const upb_EnumDef* e,
                                                    const char* name);
UPB_API const upb_EnumValueDef* upb_EnumDef_FindValueByNameWithSize(
    const upb_EnumDef* e, const char* name, size_t size);
UPB_API const upb_EnumValueDef* upb_EnumDef_FindValueByNumber(
    const upb_EnumDef* e, int32_t num);
UPB_API const char* upb_EnumDef_FullName(const upb_EnumDef* e);
bool upb_EnumDef_HasOptions(const upb_EnumDef* e);
bool upb_EnumDef_IsClosed(const upb_EnumDef* e);
bool upb_EnumDef_IsSpecifiedAsClosed(const upb_EnumDef* e);

// Creates a mini descriptor string for an enum, returns true on success.
bool upb_EnumDef_MiniDescriptorEncode(const upb_EnumDef* e, upb_Arena* a,
                                      upb_StringView* out);

const char* upb_EnumDef_Name(const upb_EnumDef* e);
const UPB_DESC(EnumOptions) * upb_EnumDef_Options(const upb_EnumDef* e);
const UPB_DESC(FeatureSet) * upb_EnumDef_ResolvedFeatures(const upb_EnumDef* e);

upb_StringView upb_EnumDef_ReservedName(const upb_EnumDef* e, int i);
int upb_EnumDef_ReservedNameCount(const upb_EnumDef* e);

const upb_EnumReservedRange* upb_EnumDef_ReservedRange(const upb_EnumDef* e,
                                                       int i);
int upb_EnumDef_ReservedRangeCount(const upb_EnumDef* e);

UPB_API const upb_EnumValueDef* upb_EnumDef_Value(const upb_EnumDef* e, int i);
UPB_API int upb_EnumDef_ValueCount(const upb_EnumDef* e);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_DEF_H_ */
