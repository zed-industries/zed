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
#include <stdint.h>

#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/array.h"
#include "upb/message/internal/extension.h"
#include "upb/message/internal/message.h"
#include "upb/message/internal/types.h"
#include "upb/mini_table/extension.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct upb_Message upb_Message;

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new message with the given mini_table on the given arena.
UPB_API upb_Message* upb_Message_New(const upb_MiniTable* m, upb_Arena* arena);

//
// Unknown data may be stored non-contiguously. Each segment stores a block of
// unknown fields. To iterate over segments:
//
//   uintptr_t iter = kUpb_Message_UnknownBegin;
//   upb_StringView data;
//   while (upb_Message_NextUnknown(msg, &data, &iter)) {
//     // Use data
//   }
// Iterates in the order unknown fields were parsed.

#define kUpb_Message_UnknownBegin 0
#define kUpb_Message_ExtensionBegin 0

UPB_INLINE bool upb_Message_NextUnknown(const upb_Message* msg,
                                        upb_StringView* data, uintptr_t* iter);

UPB_INLINE bool upb_Message_HasUnknown(const upb_Message* msg);

// Returns a reference to the message's unknown data.
const char* upb_Message_GetUnknown(const upb_Message* msg, size_t* len);

// Removes a segment of unknown data from the message, advancing to the next
// segment.  Returns false if the removed segment was at the end of the last
// chunk.
//
// This must be done while iterating:
//
//   uintptr_t iter = kUpb_Message_UnknownBegin;
//   upb_StringView data;
//   // Iterate chunks
//   while (upb_Message_NextUnknown(msg, &data, &iter)) {
//     // Iterate within a chunk, deleting ranges
//     while (ShouldDeleteSubSegment(&data)) {
//       // Data now points to the region to be deleted
//       if (!upb_Message_DeleteUnknown(msg, &data, &iter)) return;
//       // If DeleteUnknown returned true, then data now points to the
//       // remaining unknown fields after the region that was just deleted.
//     }
//   }
//
// The range given in `data` must be contained inside the most recently
// returned region.
bool upb_Message_DeleteUnknown(upb_Message* msg, upb_StringView* data,
                               uintptr_t* iter);

// Returns the number of extensions present in this message.
size_t upb_Message_ExtensionCount(const upb_Message* msg);

// Iterates extensions in wire order
UPB_INLINE bool upb_Message_NextExtension(const upb_Message* msg,
                                          const upb_MiniTableExtension** out_e,
                                          upb_MessageValue* out_v,
                                          uintptr_t* iter);

// Iterates extensions in reverse wire order
UPB_INLINE bool UPB_PRIVATE(_upb_Message_NextExtensionReverse)(
    const struct upb_Message* msg, const upb_MiniTableExtension** out_e,
    upb_MessageValue* out_v, uintptr_t* iter);

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
