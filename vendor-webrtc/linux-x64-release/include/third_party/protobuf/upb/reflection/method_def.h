// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_METHOD_DEF_H_
#define UPB_REFLECTION_METHOD_DEF_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

UPB_API bool upb_MethodDef_ClientStreaming(const upb_MethodDef* m);
const char* upb_MethodDef_FullName(const upb_MethodDef* m);
bool upb_MethodDef_HasOptions(const upb_MethodDef* m);
int upb_MethodDef_Index(const upb_MethodDef* m);
UPB_API const upb_MessageDef* upb_MethodDef_InputType(const upb_MethodDef* m);
UPB_API const char* upb_MethodDef_Name(const upb_MethodDef* m);
UPB_API const UPB_DESC(MethodOptions) *
    upb_MethodDef_Options(const upb_MethodDef* m);
const UPB_DESC(FeatureSet) *
    upb_MethodDef_ResolvedFeatures(const upb_MethodDef* m);
UPB_API const upb_MessageDef* upb_MethodDef_OutputType(const upb_MethodDef* m);
UPB_API bool upb_MethodDef_ServerStreaming(const upb_MethodDef* m);
UPB_API const upb_ServiceDef* upb_MethodDef_Service(const upb_MethodDef* m);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_METHOD_DEF_H_ */
