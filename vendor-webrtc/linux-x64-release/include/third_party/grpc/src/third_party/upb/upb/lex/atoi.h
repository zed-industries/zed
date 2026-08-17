// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_LEX_ATOI_H_
#define UPB_LEX_ATOI_H_

#include <stdint.h>

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// We use these hand-written routines instead of strto[u]l() because the "long
// long" variants aren't in c89. Also our version allows setting a ptr limit.
// Return the new position of the pointer after parsing the int, or NULL on
// integer overflow.

const char* upb_BufToUint64(const char* ptr, const char* end, uint64_t* val);
const char* upb_BufToInt64(const char* ptr, const char* end, int64_t* val,
                           bool* is_neg);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_LEX_ATOI_H_ */
