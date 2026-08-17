// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_TEXT_ENCODE_DEBUG_H_
#define UPB_TEXT_ENCODE_DEBUG_H_

#include <stddef.h>

#include "upb/message/message.h"
#include "upb/mini_table/message.h"
#include "upb/text/options.h"  // IWYU pragma: export

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

/* Encodes the given |msg| to a pseudo-text format: Instead of printing field
 * name to value entries, it will print field number to value entries; much like
 * how unknown fields are printed in upb_TextEncode in this directory's
 * encode.h. |mt| should correspond to the |msg|'s minitable.
 *
 * Output is placed in the given buffer, and always NULL-terminated.  The output
 * size (excluding NULL) is returned.  This means that a return value >= |size|
 * implies that the output was truncated.  (These are the same semantics as
 * snprintf()). */
UPB_API size_t upb_DebugString(const upb_Message* msg, const upb_MiniTable* mt,
                               int options, char* buf, size_t size);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_TEXT_ENCODE_DEBUG_H_ */
