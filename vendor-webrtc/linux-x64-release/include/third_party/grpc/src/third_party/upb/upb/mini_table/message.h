// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_MESSAGE_H_
#define UPB_MINI_TABLE_MESSAGE_H_

#include "upb/mini_table/enum.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/internal/message.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct upb_MiniTable upb_MiniTable;

#ifdef __cplusplus
extern "C" {
#endif

UPB_API const upb_MiniTableField* upb_MiniTable_FindFieldByNumber(
    const upb_MiniTable* m, uint32_t number);

UPB_API_INLINE const upb_MiniTableField* upb_MiniTable_GetFieldByIndex(
    const upb_MiniTable* m, uint32_t index);

UPB_API_INLINE int upb_MiniTable_FieldCount(const upb_MiniTable* m);

// DEPRECATED: use upb_MiniTable_SubMessage() instead
// Returns the MiniTable for a message field, NULL if the field is unlinked.
UPB_API_INLINE const upb_MiniTable* upb_MiniTable_GetSubMessageTable(
    const upb_MiniTable* m, const upb_MiniTableField* f);

// Returns the MiniTable for a message field if it is a submessage, otherwise
// returns NULL.
//
// WARNING: if dynamic tree shaking is in use, the return value may be the
// "empty", zero-field placeholder message instead of the real message type.
// If the message is later linked, this function will begin returning the real
// message type.
UPB_API_INLINE const upb_MiniTable* upb_MiniTable_SubMessage(
    const upb_MiniTable* m, const upb_MiniTableField* f);

// Returns the MiniTable for a map field.  The given field must refer to a map.
UPB_API_INLINE const upb_MiniTable* upb_MiniTable_MapEntrySubMessage(
    const upb_MiniTable* m, const upb_MiniTableField* f);

// Returns the MiniTableEnum for a message field, NULL if the field is unlinked.
UPB_API_INLINE const upb_MiniTableEnum* upb_MiniTable_GetSubEnumTable(
    const upb_MiniTable* m, const upb_MiniTableField* f);

// Returns the MiniTableField for the key of a map.
UPB_API_INLINE const upb_MiniTableField* upb_MiniTable_MapKey(
    const upb_MiniTable* m);

// Returns the MiniTableField for the value of a map.
UPB_API_INLINE const upb_MiniTableField* upb_MiniTable_MapValue(
    const upb_MiniTable* m);

// Returns true if this MiniTable field is linked to a MiniTable for the
// sub-message.
UPB_API_INLINE bool upb_MiniTable_FieldIsLinked(const upb_MiniTable* m,
                                                const upb_MiniTableField* f);

// If this field is in a oneof, returns the first field in the oneof.
//
// Otherwise returns NULL.
//
// Usage:
//   const upb_MiniTableField* field = upb_MiniTable_GetOneof(m, f);
//   do {
//       ..
//   } while (upb_MiniTable_NextOneofField(m, &field);
//
const upb_MiniTableField* upb_MiniTable_GetOneof(const upb_MiniTable* m,
                                                 const upb_MiniTableField* f);

// Iterates to the next field in the oneof. If this is the last field in the
// oneof, returns false. The ordering of fields in the oneof is not
// guaranteed.
// REQUIRES: |f| is the field initialized by upb_MiniTable_GetOneof and updated
//           by prior upb_MiniTable_NextOneofField calls.
bool upb_MiniTable_NextOneofField(const upb_MiniTable* m,
                                  const upb_MiniTableField** f);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_MESSAGE_H_ */
