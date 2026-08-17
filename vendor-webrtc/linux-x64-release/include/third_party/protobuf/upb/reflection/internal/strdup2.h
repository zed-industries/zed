// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_INTERNAL_STRDUP2_H_
#define UPB_REFLECTION_INTERNAL_STRDUP2_H_

#include <stddef.h>

#include "upb/mem/arena.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Variant that works with a length-delimited rather than NULL-delimited string,
// as supported by strtable.
char* upb_strdup2(const char* s, size_t len, upb_Arena* a);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_INTERNAL_STRDUP2_H_ */
