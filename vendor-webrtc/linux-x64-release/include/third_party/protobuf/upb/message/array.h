// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_ARRAY_H_
#define UPB_MESSAGE_ARRAY_H_

#include <stddef.h>

#include "upb/base/descriptor_constants.h"
#include "upb/mem/arena.h"
#include "upb/message/internal/array.h"
#include "upb/message/value.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct upb_Array upb_Array;

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new array on the given arena that holds elements of this type.
UPB_API upb_Array* upb_Array_New(upb_Arena* a, upb_CType type);

// Returns the number of elements in the array.
UPB_API_INLINE size_t upb_Array_Size(const upb_Array* arr);

// Returns the given element, which must be within the array's current size.
UPB_API upb_MessageValue upb_Array_Get(const upb_Array* arr, size_t i);

// Returns a mutating pointer to the given element, which must be within the
// array's current size.
UPB_API upb_MutableMessageValue upb_Array_GetMutable(upb_Array* arr, size_t i);

// Sets the given element, which must be within the array's current size.
UPB_API void upb_Array_Set(upb_Array* arr, size_t i, upb_MessageValue val);

// Appends an element to the array. Returns false on allocation failure.
UPB_API bool upb_Array_Append(upb_Array* array, upb_MessageValue val,
                              upb_Arena* arena);

// Moves elements within the array using memmove().
// Like memmove(), the source and destination elements may be overlapping.
UPB_API void upb_Array_Move(upb_Array* array, size_t dst_idx, size_t src_idx,
                            size_t count);

// Inserts one or more empty elements into the array.
// Existing elements are shifted right.
// The new elements have undefined state and must be set with `upb_Array_Set()`.
// REQUIRES: `i <= upb_Array_Size(arr)`
UPB_API bool upb_Array_Insert(upb_Array* array, size_t i, size_t count,
                              upb_Arena* arena);

// Deletes one or more elements from the array.
// Existing elements are shifted left.
// REQUIRES: `i + count <= upb_Array_Size(arr)`
UPB_API void upb_Array_Delete(upb_Array* array, size_t i, size_t count);

// Reserves |size| elements of storage for the array.
UPB_API_INLINE bool upb_Array_Reserve(struct upb_Array* array, size_t size,
                                      upb_Arena* arena);

// Changes the size of a vector. New elements are initialized to NULL/0.
// Returns false on allocation failure.
UPB_API bool upb_Array_Resize(upb_Array* array, size_t size, upb_Arena* arena);

// Returns pointer to array data.
UPB_API_INLINE const void* upb_Array_DataPtr(const upb_Array* arr);

// Returns mutable pointer to array data.
UPB_API_INLINE void* upb_Array_MutableDataPtr(upb_Array* arr);

// Mark an array and all of its descendents as frozen/immutable.
// If the array elements are messages then |m| must point to the minitable for
// those messages. Otherwise |m| must be NULL.
UPB_API void upb_Array_Freeze(upb_Array* arr, const upb_MiniTable* m);

// Returns whether an array has been frozen.
UPB_API_INLINE bool upb_Array_IsFrozen(const upb_Array* arr);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_ARRAY_H_ */
