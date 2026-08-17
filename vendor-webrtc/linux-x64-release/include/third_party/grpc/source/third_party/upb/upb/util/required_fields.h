// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_UTIL_REQUIRED_FIELDS_H_
#define UPB_UTIL_REQUIRED_FIELDS_H_

#include <stddef.h>

#include "upb/reflection/def.h"
#include "upb/reflection/message.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// A FieldPath can be encoded as an array of upb_FieldPathEntry, in the
// following format:
//    { {.field = f1}, {.field = f2} }                      # f1.f2
//    { {.field = f1}, {.index = 5}, {.field = f2} }        # f1[5].f2
//    { {.field = f1}, {.key = "abc"}, {.field = f2} }      # f1["abc"].f2
//
// Users must look at the type of `field` to know if an index or map key
// follows.
//
// A field path may be NULL-terminated, in which case a NULL field indicates
// the end of the field path.
typedef union {
  const upb_FieldDef* field;
  size_t array_index;
  upb_MessageValue map_key;
} upb_FieldPathEntry;

// Writes a string representing `*path` to `buf` in the following textual
// format:
//    foo.bar                    # Regular fields
//    repeated_baz[2].bar        # Repeated field
//    int32_msg_map[5].bar       # Integer-keyed map
//    string_msg_map["abc"]      # String-keyed map
//    bool_msg_map[true]         # Bool-keyed map
//
// The input array `*path` must be NULL-terminated.  The pointer `*path` will be
// updated to point to one past the terminating NULL pointer of the input array.
//
// The output buffer `buf` will always be NULL-terminated. If the output data
// (including NULL terminator) exceeds `size`, the result will be truncated.
// Returns the string length of the data we attempted to write, excluding the
// terminating NULL.
size_t upb_FieldPath_ToText(upb_FieldPathEntry** path, char* buf, size_t size);

// Checks whether `msg` or any of its children has unset required fields,
// returning `true` if any are found.  `msg` may be NULL, in which case the
// message will be treated as empty.
//
// When this function returns true, `fields` is updated (if non-NULL) to point
// to a heap-allocated array encoding the field paths of the required fields
// that are missing.  Each path is terminated with {.field = NULL}, and a final
// {.field = NULL} terminates the list of paths.  The caller is responsible for
// freeing this array.
bool upb_util_HasUnsetRequired(const upb_Message* msg, const upb_MessageDef* m,
                               const upb_DefPool* ext_pool,
                               upb_FieldPathEntry** fields);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_UTIL_REQUIRED_FIELDS_H_ */
