// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_MAP_H_
#define UPB_MESSAGE_MAP_H_

#include <stddef.h>

#include "upb/base/descriptor_constants.h"
#include "upb/mem/arena.h"
#include "upb/message/internal/map.h"
#include "upb/message/value.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct upb_Map upb_Map;

#ifdef __cplusplus
extern "C" {
#endif

// Creates a new map on the given arena with the given key/value size.
UPB_API upb_Map* upb_Map_New(upb_Arena* a, upb_CType key_type,
                             upb_CType value_type);

// Returns the number of entries in the map.
UPB_API size_t upb_Map_Size(const upb_Map* map);

// Stores a value for the given key into |*val| (or the zero value if the key is
// not present). Returns whether the key was present. The |val| pointer may be
// NULL, in which case the function tests whether the given key is present.
UPB_API bool upb_Map_Get(const upb_Map* map, upb_MessageValue key,
                         upb_MessageValue* val);

// Removes all entries in the map.
UPB_API void upb_Map_Clear(upb_Map* map);

// Sets the given key to the given value, returning whether the key was inserted
// or replaced. If the key was inserted, then any existing iterators will be
// invalidated.
UPB_API upb_MapInsertStatus upb_Map_Insert(upb_Map* map, upb_MessageValue key,
                                           upb_MessageValue val,
                                           upb_Arena* arena);

// Sets the given key to the given value. Returns false if memory allocation
// failed. If the key is newly inserted, then any existing iterators will be
// invalidated.
UPB_API_INLINE bool upb_Map_Set(upb_Map* map, upb_MessageValue key,
                                upb_MessageValue val, upb_Arena* arena) {
  return upb_Map_Insert(map, key, val, arena) !=
         kUpb_MapInsertStatus_OutOfMemory;
}

// Deletes this key from the table. Returns true if the key was present.
// If present and |val| is non-NULL, stores the deleted value.
UPB_API bool upb_Map_Delete(upb_Map* map, upb_MessageValue key,
                            upb_MessageValue* val);

// Map iteration:
//
// size_t iter = kUpb_Map_Begin;
// upb_MessageValue key, val;
// while (upb_Map_Next(map, &key, &val, &iter)) {
//   ...
// }

#define kUpb_Map_Begin ((size_t) - 1)

// Advances to the next entry. Returns false if no more entries are present.
// Otherwise returns true and populates both *key and *value.
UPB_API bool upb_Map_Next(const upb_Map* map, upb_MessageValue* key,
                          upb_MessageValue* val, size_t* iter);

// Sets the value for the entry pointed to by iter.
// WARNING: this does not currently work for string values!
UPB_API void upb_Map_SetEntryValue(upb_Map* map, size_t iter,
                                   upb_MessageValue val);

// DEPRECATED iterator, slated for removal.

/* Map iteration:
 *
 * size_t iter = kUpb_Map_Begin;
 * while (upb_MapIterator_Next(map, &iter)) {
 *   upb_MessageValue key = upb_MapIterator_Key(map, iter);
 *   upb_MessageValue val = upb_MapIterator_Value(map, iter);
 * }
 */

// Advances to the next entry. Returns false if no more entries are present.
UPB_API bool upb_MapIterator_Next(const upb_Map* map, size_t* iter);

// Returns true if the iterator still points to a valid entry, or false if the
// iterator is past the last element. It is an error to call this function with
// kUpb_Map_Begin (you must call next() at least once first).
UPB_API bool upb_MapIterator_Done(const upb_Map* map, size_t iter);

// Returns the key and value for this entry of the map.
UPB_API upb_MessageValue upb_MapIterator_Key(const upb_Map* map, size_t iter);
UPB_API upb_MessageValue upb_MapIterator_Value(const upb_Map* map, size_t iter);

// Mark a map and all of its descendents as frozen/immutable.
// If the map values are messages then |m| must point to the minitable for
// those messages. Otherwise |m| must be NULL.
UPB_API void upb_Map_Freeze(upb_Map* map, const upb_MiniTable* m);

// Returns whether a map has been frozen.
UPB_API_INLINE bool upb_Map_IsFrozen(const upb_Map* map);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MESSAGE_MAP_H_ */
