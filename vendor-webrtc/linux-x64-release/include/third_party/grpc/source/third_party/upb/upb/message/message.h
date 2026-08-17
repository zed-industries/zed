// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Public APIs for message operations that do not depend on the schema.
//
// MiniTable-based accessors live in accessors.h.

#ifndef UPB_MESSAGE_MESSAGE_H_
#define UPB_MESSAGE_MESSAGE_H_

#include <stddef.h>

#include "upb/mem/arena.h"
#include "upb/message/internal/message.h"
#include "upb/message/internal/types.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct upb_Message upb_Message;

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new message with the given mini_table on the given arena.
UPB_API upb_Message* upb_Message_New(const upb_MiniTable* m, upb_Arena* arena);

// Returns a reference to the message's unknown data.
const char* upb_Message_GetUnknown(const upb_Message* msg, size_t* len);

// Removes partial unknown data from message.
void upb_Message_DeleteUnknown(upb_Message* msg, const char* data, size_t len);

// Returns the number of extensions present in this message.
size_t upb_Message_ExtensionCount(const upb_Message* msg);

// Mark a message and all of its descendents as frozen/immutable.
UPB_API void upb_Message_Freeze(upb_Message* msg, const upb_MiniTable* m);

// Returns whether a message has been frozen.
UPB_API_INLINE bool upb_Message_IsFrozen(const upb_Message* msg);

#ifdef UPB_TRACING_ENABLED
UPB_API void upb_Message_LogNewMessage(const upb_MiniTable* m,
                                       const upb_Arena* arena);

UPB_API void upb_Message_SetNewMessageTraceHandler(
    void (*handler)(const upb_MiniTable* m, const upb_Arena* arena));
#endif  // UPB_TRACING_ENABLED

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_MESSAGE_H_ */
