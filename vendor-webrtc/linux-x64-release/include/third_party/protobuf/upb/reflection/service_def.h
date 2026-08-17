// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_SERVICE_DEF_H_
#define UPB_REFLECTION_SERVICE_DEF_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

UPB_API const upb_FileDef* upb_ServiceDef_File(const upb_ServiceDef* s);
const upb_MethodDef* upb_ServiceDef_FindMethodByName(const upb_ServiceDef* s,
                                                     const char* name);
UPB_API const char* upb_ServiceDef_FullName(const upb_ServiceDef* s);
bool upb_ServiceDef_HasOptions(const upb_ServiceDef* s);
int upb_ServiceDef_Index(const upb_ServiceDef* s);
UPB_API const upb_MethodDef* upb_ServiceDef_Method(const upb_ServiceDef* s,
                                                   int i);
UPB_API int upb_ServiceDef_MethodCount(const upb_ServiceDef* s);
const char* upb_ServiceDef_Name(const upb_ServiceDef* s);
UPB_API const UPB_DESC(ServiceOptions) *
    upb_ServiceDef_Options(const upb_ServiceDef* s);
const UPB_DESC(FeatureSet) *
    upb_ServiceDef_ResolvedFeatures(const upb_ServiceDef* s);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_SERVICE_DEF_H_ */
