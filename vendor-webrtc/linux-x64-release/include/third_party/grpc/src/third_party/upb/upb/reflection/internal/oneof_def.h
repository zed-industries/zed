// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_ONEOF_DEF_INTERNAL_H_
#define UPB_REFLECTION_ONEOF_DEF_INTERNAL_H_

#include "upb/reflection/oneof_def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_OneofDef* _upb_OneofDef_At(const upb_OneofDef* o, int i);
void _upb_OneofDef_Insert(upb_DefBuilder* ctx, upb_OneofDef* o,
                          const upb_FieldDef* f, const char* name, size_t size);

// Allocate and initialize an array of |n| oneof defs owned by |m|.
upb_OneofDef* _upb_OneofDefs_New(upb_DefBuilder* ctx, int n,
                                 const UPB_DESC(OneofDescriptorProto*)
                                     const* protos,
                                 const UPB_DESC(FeatureSet*) parent_features,
                                 upb_MessageDef* m);

size_t _upb_OneofDefs_Finalize(upb_DefBuilder* ctx, upb_MessageDef* m);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ONEOF_DEF_INTERNAL_H_ */
