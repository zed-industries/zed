// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_ENUM_H_
#define UPB_MINI_TABLE_ENUM_H_

#include <stdint.h>

#include "upb/mini_table/internal/enum.h"

// Must be last
#include "upb/port/def.inc"

typedef struct upb_MiniTableEnum upb_MiniTableEnum;

#ifdef __cplusplus
extern "C" {
#endif

// Validates enum value against range defined by enum mini table.
UPB_API_INLINE bool upb_MiniTableEnum_CheckValue(const upb_MiniTableEnum* e,
                                                 uint32_t val);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_ENUM_H_ */
