// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_DEF_TYPE_H_
#define UPB_REFLECTION_DEF_TYPE_H_

#include "upb/hash/common.h"

// Must be last.
#include "upb/port/def.inc"

// Inside a symtab we store tagged pointers to specific def types.
typedef enum {
  UPB_DEFTYPE_MASK = 7,

  // Only inside symtab table.
  UPB_DEFTYPE_EXT = 0,
  UPB_DEFTYPE_MSG = 1,
  UPB_DEFTYPE_ENUM = 2,
  UPB_DEFTYPE_ENUMVAL = 3,
  UPB_DEFTYPE_SERVICE = 4,

  // Only inside message table.
  UPB_DEFTYPE_FIELD = 0,
  UPB_DEFTYPE_ONEOF = 1,
} upb_deftype_t;

#ifdef __cplusplus
extern "C" {
#endif

// Our 3-bit pointer tagging requires all pointers to be multiples of 8.
// The arena will always yield 8-byte-aligned addresses, however we put
// the defs into arrays. For each element in the array to be 8-byte-aligned,
// the sizes of each def type must also be a multiple of 8.
//
// If any of these asserts fail, we need to add or remove padding on 32-bit
// machines (64-bit machines will have 8-byte alignment already due to
// pointers, which all of these structs have).
UPB_INLINE void _upb_DefType_CheckPadding(size_t size) {
  UPB_ASSERT((size & UPB_DEFTYPE_MASK) == 0);
}

upb_deftype_t _upb_DefType_Type(upb_value v);

upb_value _upb_DefType_Pack(const void* ptr, upb_deftype_t type);

const void* _upb_DefType_Unpack(upb_value v, upb_deftype_t type);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_DEF_TYPE_H_ */
