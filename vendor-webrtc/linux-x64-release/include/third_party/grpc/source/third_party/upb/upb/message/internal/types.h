// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_INTERNAL_TYPES_H_
#define UPB_MESSAGE_INTERNAL_TYPES_H_

#include <stdint.h>

// Must be last.
#include "upb/port/def.inc"

#define UPB_OPAQUE(x) x##_opaque

struct upb_Message {
  union {
    uintptr_t UPB_OPAQUE(internal);  // tagged pointer, low bit == frozen
    double d;  // Forces same size for 32-bit/64-bit builds
  };
};

#ifdef __cplusplus
extern "C" {
#endif

UPB_INLINE void UPB_PRIVATE(_upb_Message_ShallowFreeze)(
    struct upb_Message* msg) {
  msg->UPB_OPAQUE(internal) |= 1ULL;
}

UPB_API_INLINE bool upb_Message_IsFrozen(const struct upb_Message* msg) {
  return (msg->UPB_OPAQUE(internal) & 1ULL) != 0;
}

UPB_INLINE struct upb_Message_Internal* UPB_PRIVATE(_upb_Message_GetInternal)(
    const struct upb_Message* msg) {
  const uintptr_t tmp = msg->UPB_OPAQUE(internal) & ~1ULL;
  return (struct upb_Message_Internal*)tmp;
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_SetInternal)(
    struct upb_Message* msg, struct upb_Message_Internal* internal) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  msg->UPB_OPAQUE(internal) = (uintptr_t)internal;
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#undef UPB_OPAQUE

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_INTERNAL_TYPES_H_ */
