// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// These functions are only used by generated code.

#ifndef UPB_MESSAGE_MAP_GENCODE_UTIL_H_
#define UPB_MESSAGE_MAP_GENCODE_UTIL_H_

#include "upb/message/internal/map.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Message map operations, these get the map from the message first.

UPB_INLINE void _upb_msg_map_key(const void* msg, void* key, size_t size) {
  const upb_tabent* ent = (const upb_tabent*)msg;
  uint32_t u32len;
  upb_StringView k;
  k.data = upb_tabstr(ent->key, &u32len);
  k.size = u32len;
  _upb_map_fromkey(k, key, size);
}

UPB_INLINE void _upb_msg_map_value(const void* msg, void* val, size_t size) {
  const upb_tabent* ent = (const upb_tabent*)msg;
  upb_value v = {ent->val.val};
  _upb_map_fromvalue(v, val, size);
}

UPB_INLINE void _upb_msg_map_set_value(void* msg, const void* val,
                                       size_t size) {
  upb_tabent* ent = (upb_tabent*)msg;
  // This is like _upb_map_tovalue() except the entry already exists
  // so we can reuse the allocated upb_StringView for string fields.
  if (size == UPB_MAPTYPE_STRING) {
    upb_StringView* strp = (upb_StringView*)(uintptr_t)ent->val.val;
    memcpy(strp, val, sizeof(*strp));
  } else {
    memcpy(&ent->val.val, val, size);
  }
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_MAP_GENCODE_UTIL_H_ */
