// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_ONEOF_DEF_H_
#define UPB_REFLECTION_ONEOF_DEF_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

UPB_API const upb_MessageDef* upb_OneofDef_ContainingType(
    const upb_OneofDef* o);
UPB_API const upb_FieldDef* upb_OneofDef_Field(const upb_OneofDef* o, int i);
UPB_API int upb_OneofDef_FieldCount(const upb_OneofDef* o);
const char* upb_OneofDef_FullName(const upb_OneofDef* o);
bool upb_OneofDef_HasOptions(const upb_OneofDef* o);
uint32_t upb_OneofDef_Index(const upb_OneofDef* o);
bool upb_OneofDef_IsSynthetic(const upb_OneofDef* o);
const upb_FieldDef* upb_OneofDef_LookupName(const upb_OneofDef* o,
                                            const char* name);
const upb_FieldDef* upb_OneofDef_LookupNameWithSize(const upb_OneofDef* o,
                                                    const char* name,
                                                    size_t size);
const upb_FieldDef* upb_OneofDef_LookupNumber(const upb_OneofDef* o,
                                              uint32_t num);
UPB_API const char* upb_OneofDef_Name(const upb_OneofDef* o);
int upb_OneofDef_numfields(const upb_OneofDef* o);
const UPB_DESC(OneofOptions*) upb_OneofDef_Options(const upb_OneofDef* o);
const UPB_DESC(FeatureSet*)
    upb_OneofDef_ResolvedFeatures(const upb_OneofDef* o);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ONEOF_DEF_H_ */
