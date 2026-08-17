// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_TAGGED_PTR_H_
#define UPB_MINI_TABLE_TAGGED_PTR_H_

#include <stdint.h>

#include "upb/message/internal/tagged_ptr.h"
#include "upb/message/message.h"

// Must be last.
#include "upb/port/def.inc"

// When a upb_Message* is stored in a message, array, or map, it is stored in a
// tagged form. If the tag bit is set, the referenced upb_Message is of type
// _kUpb_MiniTable_Empty (a sentinel message type with no fields) instead of
// that field's true message type. This forms the basis of what we call
// "dynamic tree shaking."
//
// See the documentation for kUpb_DecodeOption_ExperimentalAllowUnlinked for
// more information.

typedef uintptr_t upb_TaggedMessagePtr;

#ifdef __cplusplus
extern "C" {
#endif

// Users who enable unlinked sub-messages must use this to test whether a
// message is empty before accessing it. If a message is empty, it must be
// first promoted using the interfaces in message/promote.h.
UPB_API_INLINE bool upb_TaggedMessagePtr_IsEmpty(upb_TaggedMessagePtr ptr);

UPB_API_INLINE upb_Message* upb_TaggedMessagePtr_GetNonEmptyMessage(
    upb_TaggedMessagePtr ptr);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_TAGGED_PTR_H_ */
