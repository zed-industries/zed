// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_METHOD_DEF_INTERNAL_H_
#define UPB_REFLECTION_METHOD_DEF_INTERNAL_H_

#include "upb/reflection/method_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_MethodDef* _upb_MethodDef_At(const upb_MethodDef* m, int i);

// Allocate and initialize an array of |n| method defs owned by |s|.
upb_MethodDef* _upb_MethodDefs_New(upb_DefBuilder* ctx, int n,
                                   const UPB_DESC(MethodDescriptorProto*)
                                       const* protos,
                                   const UPB_DESC(FeatureSet*) parent_features,
                                   upb_ServiceDef* s);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_METHOD_DEF_INTERNAL_H_ */
