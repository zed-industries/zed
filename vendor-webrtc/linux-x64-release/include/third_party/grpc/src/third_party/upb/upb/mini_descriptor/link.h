// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Functions for linking MiniTables together once they are built from a
// MiniDescriptor.
//
// These functions have names like upb_MiniTable_Link() because they operate on
// MiniTables.  We put them here, rather than in the mini_table/ directory,
// because they are only needed when building MiniTables from MiniDescriptors.
// The interfaces in mini_table/ assume that MiniTables are immutable.

#ifndef UPB_MINI_DESCRIPTOR_LINK_H_
#define UPB_MINI_DESCRIPTOR_LINK_H_

#include "upb/base/status.h"
#include "upb/mem/arena.h"
#include "upb/mini_table/enum.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/message.h"
#include "upb/mini_table/sub.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Links a sub-message field to a MiniTable for that sub-message. If a
// sub-message field is not linked, it will be treated as an unknown field
// during parsing, and setting the field will not be allowed. It is possible
// to link the message field later, at which point it will no longer be treated
// as unknown. However there is no synchronization for this operation, which
// means parallel mutation requires external synchronization.
// Returns success/failure.
UPB_API bool upb_MiniTable_SetSubMessage(upb_MiniTable* table,
                                         upb_MiniTableField* field,
                                         const upb_MiniTable* sub);

// Links an enum field to a MiniTable for that enum.
// All enum fields must be linked prior to parsing.
// Returns success/failure.
UPB_API bool upb_MiniTable_SetSubEnum(upb_MiniTable* table,
                                      upb_MiniTableField* field,
                                      const upb_MiniTableEnum* sub);

// Returns a list of fields that require linking at runtime, to connect the
// MiniTable to its sub-messages and sub-enums.  The list of fields will be
// written to the `subs` array, which must have been allocated by the caller
// and must be large enough to hold a list of all fields in the message.
//
// The order of the fields returned by this function is significant: it matches
// the order expected by upb_MiniTable_Link() below.
//
// The return value packs the sub-message count and sub-enum count into a single
// integer like so:
//  return (msg_count << 16) | enum_count;
UPB_API uint32_t upb_MiniTable_GetSubList(const upb_MiniTable* mt,
                                          const upb_MiniTableField** subs);

// Links a message to its sub-messages and sub-enums.  The caller must pass
// arrays of sub-tables and sub-enums, in the same length and order as is
// returned by upb_MiniTable_GetSubList() above.  However, individual elements
// of the sub_tables may be NULL if those sub-messages were tree shaken.
//
// Returns false if either array is too short, or if any of the tables fails
// to link.
UPB_API bool upb_MiniTable_Link(upb_MiniTable* mt,
                                const upb_MiniTable** sub_tables,
                                size_t sub_table_count,
                                const upb_MiniTableEnum** sub_enums,
                                size_t sub_enum_count);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MINI_DESCRIPTOR_LINK_H_
