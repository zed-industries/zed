// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_COMPAT_H_
#define UPB_MINI_TABLE_COMPAT_H_

#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

// upb does not support mixing minitables from different sources but these
// functions are still used by some existing users so for now we make them
// available here. This may or may not change in the future so do not add
// them to new code.

#ifdef __cplusplus
extern "C" {
#endif

// Checks if memory layout of src is compatible with dst.
bool upb_MiniTable_Compatible(const upb_MiniTable* src,
                              const upb_MiniTable* dst);

typedef enum {
  kUpb_MiniTableEquals_NotEqual,
  kUpb_MiniTableEquals_Equal,
  kUpb_MiniTableEquals_OutOfMemory,
} upb_MiniTableEquals_Status;

// Checks equality of mini tables originating from different language runtimes.
upb_MiniTableEquals_Status upb_MiniTable_Equals(const upb_MiniTable* src,
                                                const upb_MiniTable* dst);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_COMPAT_H_ */
