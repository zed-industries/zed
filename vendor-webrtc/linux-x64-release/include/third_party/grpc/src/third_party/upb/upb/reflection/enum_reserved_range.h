// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// IWYU pragma: private, include "upb/reflection/def.h"

#ifndef UPB_REFLECTION_ENUM_RESERVED_RANGE_H_
#define UPB_REFLECTION_ENUM_RESERVED_RANGE_H_

#include "upb/reflection/common.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

int32_t upb_EnumReservedRange_Start(const upb_EnumReservedRange* r);
int32_t upb_EnumReservedRange_End(const upb_EnumReservedRange* r);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_ENUM_RESERVED_RANGE_H_ */
