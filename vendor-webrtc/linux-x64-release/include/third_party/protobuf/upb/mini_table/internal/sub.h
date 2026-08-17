// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_INTERNAL_SUB_H_
#define UPB_MINI_TABLE_INTERNAL_SUB_H_

// Must be last.
#include "upb/port/def.inc"

typedef union {
  const struct upb_MiniTable* const* UPB_PRIVATE(submsg);
  const struct upb_MiniTableEnum* UPB_PRIVATE(subenum);
} upb_MiniTableSubInternal;

union upb_MiniTableSub {
  const struct upb_MiniTable* UPB_PRIVATE(submsg);
  const struct upb_MiniTableEnum* UPB_PRIVATE(subenum);
};

#ifdef __cplusplus
extern "C" {
#endif

UPB_API_INLINE union upb_MiniTableSub upb_MiniTableSub_FromEnum(
    const struct upb_MiniTableEnum* subenum) {
  union upb_MiniTableSub out;
  out.UPB_PRIVATE(subenum) = subenum;
  return out;
}

UPB_API_INLINE union upb_MiniTableSub upb_MiniTableSub_FromMessage(
    const struct upb_MiniTable* submsg) {
  union upb_MiniTableSub out;
  out.UPB_PRIVATE(submsg) = submsg;
  return out;
}

UPB_API_INLINE const struct upb_MiniTableEnum* upb_MiniTableSub_Enum(
    const union upb_MiniTableSub sub) {
  return sub.UPB_PRIVATE(subenum);
}

UPB_API_INLINE const struct upb_MiniTable* upb_MiniTableSub_Message(
    const union upb_MiniTableSub sub) {
  return sub.UPB_PRIVATE(submsg);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_INTERNAL_SUB_H_ */
