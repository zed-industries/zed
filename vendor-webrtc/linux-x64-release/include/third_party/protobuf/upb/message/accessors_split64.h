// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_ACCESSORS_SPLIT64_H_
#define UPB_MESSAGE_ACCESSORS_SPLIT64_H_

#include "upb/message/accessors.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// JavaScript doesn't directly support 64-bit ints so we must split them.

UPB_API_INLINE uint32_t upb_Message_GetInt64Hi(const upb_Message* msg,
                                               const upb_MiniTableField* field,
                                               uint32_t default_value) {
  return (uint32_t)(upb_Message_GetInt64(msg, field, default_value) >> 32);
}

UPB_API_INLINE uint32_t upb_Message_GetInt64Lo(const upb_Message* msg,
                                               const upb_MiniTableField* field,
                                               uint32_t default_value) {
  return (uint32_t)upb_Message_GetInt64(msg, field, default_value);
}

UPB_API_INLINE bool upb_Message_SetInt64Split(upb_Message* msg,
                                              const upb_MiniTableField* field,
                                              uint32_t hi, uint32_t lo,
                                              upb_Arena* arena) {
  return upb_Message_SetInt64(msg, field, ((int64_t)hi << 32) | lo, arena);
}

UPB_API_INLINE uint32_t upb_Message_GetUInt64Hi(const upb_Message* msg,
                                                const upb_MiniTableField* field,
                                                uint32_t default_value) {
  return (uint32_t)(upb_Message_GetUInt64(msg, field, default_value) >> 32);
}

UPB_API_INLINE uint32_t upb_Message_GetUInt64Lo(const upb_Message* msg,
                                                const upb_MiniTableField* field,
                                                uint32_t default_value) {
  return (uint32_t)upb_Message_GetUInt64(msg, field, default_value);
}

UPB_API_INLINE bool upb_Message_SetUInt64Split(upb_Message* msg,
                                               const upb_MiniTableField* field,
                                               uint32_t hi, uint32_t lo,
                                               upb_Arena* arena) {
  return upb_Message_SetUInt64(msg, field, ((uint64_t)hi << 32) | lo, arena);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MESSAGE_ACCESSORS_SPLIT64_H_
