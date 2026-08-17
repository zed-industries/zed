// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_EXTENSION_RANGE_INTERNAL_H_
#define UPB_REFLECTION_EXTENSION_RANGE_INTERNAL_H_

#include "upb/reflection/extension_range.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_ExtensionRange* _upb_ExtensionRange_At(const upb_ExtensionRange* r, int i);

// Allocate and initialize an array of |n| extension ranges owned by |m|.
upb_ExtensionRange* _upb_ExtensionRanges_New(
    upb_DefBuilder* ctx, int n,
    const UPB_DESC(DescriptorProto_ExtensionRange*) const* protos,
    const UPB_DESC(FeatureSet*) parent_features, const upb_MessageDef* m);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_EXTENSION_RANGE_INTERNAL_H_ */
