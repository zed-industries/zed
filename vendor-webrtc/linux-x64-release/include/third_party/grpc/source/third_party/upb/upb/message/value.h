// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Users should include array.h or map.h instead.
// IWYU pragma: private, include "upb/message/array.h"

#ifndef UPB_MESSAGE_VALUE_H_
#define UPB_MESSAGE_VALUE_H_

#include <stdint.h>
#include <string.h>

#include "upb/base/string_view.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

typedef union {
  bool bool_val;
  float float_val;
  double double_val;
  int32_t int32_val;
  int64_t int64_val;
  uint32_t uint32_val;
  uint64_t uint64_val;
  const struct upb_Array* array_val;
  const struct upb_Map* map_val;
  const struct upb_Message* msg_val;
  upb_StringView str_val;

  // EXPERIMENTAL: A tagged upb_Message*.  Users must use this instead of
  // msg_val if unlinked sub-messages may possibly be in use.  See the
  // documentation in kUpb_DecodeOption_ExperimentalAllowUnlinked for more
  // information.
  uintptr_t tagged_msg_val;  // upb_TaggedMessagePtr
} upb_MessageValue;

UPB_API_INLINE upb_MessageValue upb_MessageValue_Zero(void) {
  upb_MessageValue zero;
  memset(&zero, 0, sizeof(zero));
  return zero;
}

typedef union {
  struct upb_Array* array;
  struct upb_Map* map;
  struct upb_Message* msg;
} upb_MutableMessageValue;

UPB_API_INLINE upb_MutableMessageValue upb_MutableMessageValue_Zero(void) {
  upb_MutableMessageValue zero;
  memset(&zero, 0, sizeof(zero));
  return zero;
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_VALUE_H_ */
