// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_JSON_ENCODE_H_
#define UPB_JSON_ENCODE_H_

#include "upb/reflection/def.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

enum {
  /* When set, emits 0/default values.  TODO: proto3 only? */
  upb_JsonEncode_EmitDefaults = 1 << 0,

  /* When set, use normal (snake_case) field names instead of JSON (camelCase)
     names. */
  upb_JsonEncode_UseProtoNames = 1 << 1,

  /* When set, emits enums as their integer values instead of as their names. */
  upb_JsonEncode_FormatEnumsAsIntegers = 1 << 2
};

/* Encodes the given |msg| to JSON format.  The message's reflection is given in
 * |m|.  The DefPool in |ext_pool| is used to find extensions (if NULL,
 * extensions will not be printed).
 *
 * Output is placed in the given buffer, and always NULL-terminated.  The output
 * size (excluding NULL) is returned.  This means that a return value >= |size|
 * implies that the output was truncated.  (These are the same semantics as
 * snprintf()). */
UPB_API size_t upb_JsonEncode(const upb_Message* msg, const upb_MessageDef* m,
                              const upb_DefPool* ext_pool, int options,
                              char* buf, size_t size, upb_Status* status);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_JSONENCODE_H_ */
