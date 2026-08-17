// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

/*
** Our memory representation for parsing tables and messages themselves.
** Functions in this file are used by generated code and possibly reflection.
**
** The definitions in this file are internal to upb.
**/

#ifndef UPB_MESSAGE_INTERNAL_MESSAGE_H_
#define UPB_MESSAGE_INTERNAL_MESSAGE_H_

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/internal/extension.h"
#include "upb/message/internal/types.h"
#include "upb/mini_table/extension.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

extern const float kUpb_FltInfinity;
extern const double kUpb_Infinity;
extern const double kUpb_NaN;

// Internal members of a upb_Message that track unknown fields and/or
// extensions. We can change this without breaking binary compatibility.

typedef struct upb_TaggedAuxPtr {
  uintptr_t ptr;
} upb_TaggedAuxPtr;

UPB_INLINE bool upb_TaggedAuxPtr_IsNull(upb_TaggedAuxPtr ptr) {
  return ptr.ptr == 0;
}

UPB_INLINE bool upb_TaggedAuxPtr_IsExtension(upb_TaggedAuxPtr ptr) {
  return ptr.ptr & 1;
}

UPB_INLINE bool upb_TaggedAuxPtr_IsUnknown(upb_TaggedAuxPtr ptr) {
  return (ptr.ptr != 0) && ((ptr.ptr & 1) == 0);
}

UPB_INLINE upb_Extension* upb_TaggedAuxPtr_Extension(upb_TaggedAuxPtr ptr) {
  UPB_ASSERT(upb_TaggedAuxPtr_IsExtension(ptr));
  return (upb_Extension*)(ptr.ptr & ~1ULL);
}

UPB_INLINE upb_StringView* upb_TaggedAuxPtr_UnknownData(upb_TaggedAuxPtr ptr) {
  UPB_ASSERT(!upb_TaggedAuxPtr_IsExtension(ptr));
  return (upb_StringView*)(ptr.ptr);
}

UPB_INLINE upb_TaggedAuxPtr upb_TaggedAuxPtr_Null(void) {
  upb_TaggedAuxPtr ptr;
  ptr.ptr = 0;
  return ptr;
}

UPB_INLINE upb_TaggedAuxPtr
upb_TaggedAuxPtr_MakeExtension(const upb_Extension* e) {
  upb_TaggedAuxPtr ptr;
  ptr.ptr = (uintptr_t)e | 1;
  return ptr;
}

UPB_INLINE upb_TaggedAuxPtr
upb_TaggedAuxPtr_MakeUnknownData(const upb_StringView* sv) {
  upb_TaggedAuxPtr ptr;
  ptr.ptr = (uintptr_t)sv;
  return ptr;
}

typedef struct upb_Message_Internal {
  // Total number of entries set in aux_data
  uint32_t size;
  uint32_t capacity;
  // Tagged pointers to upb_StringView or upb_Extension
  upb_TaggedAuxPtr aux_data[];
} upb_Message_Internal;

#ifdef UPB_TRACING_ENABLED
UPB_API void upb_Message_LogNewMessage(const upb_MiniTable* m,
                                       const upb_Arena* arena);
UPB_API void upb_Message_SetNewMessageTraceHandler(
    void (*handler)(const upb_MiniTable*, const upb_Arena*));
#endif  // UPB_TRACING_ENABLED

// Inline version upb_Message_New(), for internal use.
UPB_INLINE struct upb_Message* _upb_Message_New(const upb_MiniTable* m,
                                                upb_Arena* a) {
#ifdef UPB_TRACING_ENABLED
  upb_Message_LogNewMessage(m, a);
#endif  // UPB_TRACING_ENABLED

  const int size = m->UPB_PRIVATE(size);
  struct upb_Message* msg = (struct upb_Message*)upb_Arena_Malloc(a, size);
  if (UPB_UNLIKELY(!msg)) return NULL;
  memset(msg, 0, size);
  return msg;
}

// Discards the unknown fields for this message only.
void _upb_Message_DiscardUnknown_shallow(struct upb_Message* msg);

// Adds unknown data (serialized protobuf data) to the given message. The data
// must represent one or more complete and well formed proto fields.
// If alias is set, will keep a view to the provided data; otherwise a copy is
// made.
bool UPB_PRIVATE(_upb_Message_AddUnknown)(struct upb_Message* msg,
                                          const char* data, size_t len,
                                          upb_Arena* arena, bool alias);

// Adds unknown data (serialized protobuf data) to the given message.
// The data is copied into the message instance. Data when concatenated together
// must represent one or more complete and well formed proto fields, but the
// individual spans may point only to partial fields.
bool UPB_PRIVATE(_upb_Message_AddUnknownV)(struct upb_Message* msg,
                                           upb_Arena* arena,
                                           upb_StringView data[], size_t count);

// Ensures at least one slot is available in the aux_data of this message.
// Returns false if a reallocation is needed to satisfy the request, and fails.
bool UPB_PRIVATE(_upb_Message_ReserveSlot)(struct upb_Message* msg,
                                           upb_Arena* arena);

#define kUpb_Message_UnknownBegin 0
#define kUpb_Message_ExtensionBegin 0

UPB_INLINE bool upb_Message_NextUnknown(const struct upb_Message* msg,
                                        upb_StringView* data, uintptr_t* iter) {
  const upb_Message_Internal* in = UPB_PRIVATE(_upb_Message_GetInternal)(msg);
  size_t i = *iter;
  if (in) {
    while (i < in->size) {
      upb_TaggedAuxPtr tagged_ptr = in->aux_data[i++];
      if (upb_TaggedAuxPtr_IsUnknown(tagged_ptr)) {
        *data = *upb_TaggedAuxPtr_UnknownData(tagged_ptr);
        *iter = i;
        return true;
      }
    }
  }
  data->size = 0;
  data->data = NULL;
  *iter = i;
  return false;
}

UPB_INLINE bool upb_Message_HasUnknown(const struct upb_Message* msg) {
  upb_StringView data;
  uintptr_t iter = kUpb_Message_UnknownBegin;
  return upb_Message_NextUnknown(msg, &data, &iter);
}

UPB_INLINE bool upb_Message_NextExtension(const struct upb_Message* msg,
                                          const upb_MiniTableExtension** out_e,
                                          upb_MessageValue* out_v,
                                          uintptr_t* iter) {
  const upb_Message_Internal* in = UPB_PRIVATE(_upb_Message_GetInternal)(msg);
  uintptr_t i = *iter;
  if (in) {
    while (i < in->size) {
      upb_TaggedAuxPtr tagged_ptr = in->aux_data[i++];
      if (upb_TaggedAuxPtr_IsExtension(tagged_ptr)) {
        const upb_Extension* ext = upb_TaggedAuxPtr_Extension(tagged_ptr);

        // Empty repeated fields or maps semantically don't exist.
        if (UPB_PRIVATE(_upb_Extension_IsEmpty)(ext)) continue;

        *out_e = ext->ext;
        *out_v = ext->data;
        *iter = i;
        return true;
      }
    }
  }
  *iter = i;

  return false;
}

UPB_INLINE bool UPB_PRIVATE(_upb_Message_NextExtensionReverse)(
    const struct upb_Message* msg, const upb_MiniTableExtension** out_e,
    upb_MessageValue* out_v, uintptr_t* iter) {
  upb_Message_Internal* in = UPB_PRIVATE(_upb_Message_GetInternal)(msg);
  if (!in) return false;
  uintptr_t i = *iter;
  uint32_t size = in->size;
  while (i < size) {
    upb_TaggedAuxPtr tagged_ptr = in->aux_data[size - 1 - i];
    i++;
    if (!upb_TaggedAuxPtr_IsExtension(tagged_ptr)) {
      continue;
    }
    const upb_Extension* ext = upb_TaggedAuxPtr_Extension(tagged_ptr);

    // Empty repeated fields or maps semantically don't exist.
    if (UPB_PRIVATE(_upb_Extension_IsEmpty)(ext)) continue;

    *out_e = ext->ext;
    *out_v = ext->data;
    *iter = i;
    return true;
  }
  *iter = i;
  return false;
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_INTERNAL_MESSAGE_H_ */
