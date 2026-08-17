// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_ENUM_RESERVED_RANGE_INTERNAL_H_
#define UPB_REFLECTION_ENUM_RESERVED_RANGE_INTERNAL_H_

#include "upb/reflection/enum_reserved_range.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

upb_EnumReservedRange* _upb_EnumReservedRange_At(const upb_EnumReservedRange* r,
                                                 int i);

// Allocate and initialize an array of |n| reserved ranges owned by |e|.
upb_EnumReservedRange* _upb_EnumReservedRanges_New(
    upb_DefBuilder* ctx, int n,
    const UPB_DESC(EnumDescriptorProto_EnumReservedRange*) const* protos,
    const upb_EnumDef* e);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_RESERVED_RANGE_INTERNAL_H_ */
