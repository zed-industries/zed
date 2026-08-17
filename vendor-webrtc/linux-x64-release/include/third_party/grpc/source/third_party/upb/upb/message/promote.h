// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_PROMOTE_H_
#define UPB_MESSAGE_PROMOTE_H_

#include "upb/message/array.h"
#include "upb/message/map.h"
#include "upb/message/value.h"
#include "upb/wire/decode.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  kUpb_GetExtension_Ok,
  kUpb_GetExtension_NotPresent,
  kUpb_GetExtension_ParseError,
  kUpb_GetExtension_OutOfMemory,
} upb_GetExtension_Status;

typedef enum {
  kUpb_GetExtensionAsBytes_Ok,
  kUpb_GetExtensionAsBytes_NotPresent,
  kUpb_GetExtensionAsBytes_EncodeError,
} upb_GetExtensionAsBytes_Status;

// Returns a message value or promotes an unknown field to an extension.
//
// TODO: Only supports extension fields that are messages,
// expand support to include non-message types.
upb_GetExtension_Status upb_Message_GetOrPromoteExtension(
    upb_Message* msg, const upb_MiniTableExtension* ext_table,
    int decode_options, upb_Arena* arena, upb_MessageValue* value);

typedef enum {
  kUpb_FindUnknown_Ok,
  kUpb_FindUnknown_NotPresent,
  kUpb_FindUnknown_ParseError,
} upb_FindUnknown_Status;

typedef struct {
  upb_FindUnknown_Status status;
  // Start of unknown field data in message arena.
  const char* ptr;
  // Size of unknown field data.
  size_t len;
} upb_FindUnknownRet;

// Finds first occurrence of unknown data by tag id in message.
// A depth_limit of zero means to just use the upb default depth limit.
upb_FindUnknownRet upb_Message_FindUnknown(const upb_Message* msg,
                                           uint32_t field_number,
                                           int depth_limit);

typedef enum {
  kUpb_UnknownToMessage_Ok,
  kUpb_UnknownToMessage_ParseError,
  kUpb_UnknownToMessage_OutOfMemory,
  kUpb_UnknownToMessage_NotFound,
} upb_UnknownToMessage_Status;

typedef struct {
  upb_UnknownToMessage_Status status;
  upb_Message* message;
} upb_UnknownToMessageRet;

// Promotes an "empty" non-repeated message field in `parent` to a message of
// the correct type.
//
// Preconditions:
//
// 1. The message field must currently be in the "empty" state (this must have
//    been previously verified by the caller by calling
//    `upb_Message_GetTaggedMessagePtr()` and observing that the message is
//    indeed empty).
//
// 2. This `field` must have previously been linked.
//
// If the promotion succeeds, `parent` will have its data for `field` replaced
// by the promoted message, which is also returned in `*promoted`.  If the
// return value indicates an error status, `parent` and `promoted` are
// unchanged.
upb_DecodeStatus upb_Message_PromoteMessage(upb_Message* parent,
                                            const upb_MiniTable* mini_table,
                                            const upb_MiniTableField* field,
                                            int decode_options,
                                            upb_Arena* arena,
                                            upb_Message** promoted);

// Promotes any "empty" messages in this array to a message of the correct type
// `mini_table`.  This function should only be called for arrays of messages.
//
// If the return value indicates an error status, some but not all elements may
// have been promoted, but the array itself will not be corrupted.
upb_DecodeStatus upb_Array_PromoteMessages(upb_Array* arr,
                                           const upb_MiniTable* mini_table,
                                           int decode_options,
                                           upb_Arena* arena);

// Promotes any "empty" entries in this map to a message of the correct type
// `mini_table`.  This function should only be called for maps that have a
// message type as the map value.
//
// If the return value indicates an error status, some but not all elements may
// have been promoted, but the map itself will not be corrupted.
upb_DecodeStatus upb_Map_PromoteMessages(upb_Map* map,
                                         const upb_MiniTable* mini_table,
                                         int decode_options, upb_Arena* arena);

////////////////////////////////////////////////////////////////////////////////
// OLD promotion interfaces, will be removed!
////////////////////////////////////////////////////////////////////////////////

// Promotes unknown data inside message to a upb_Message parsing the unknown.
//
// The unknown data is removed from message after field value is set
// using upb_Message_SetMessage.
//
// WARNING!: See b/267655898
upb_UnknownToMessageRet upb_MiniTable_PromoteUnknownToMessage(
    upb_Message* msg, const upb_MiniTable* mini_table,
    const upb_MiniTableField* field, const upb_MiniTable* sub_mini_table,
    int decode_options, upb_Arena* arena);

// Promotes all unknown data that matches field tag id to repeated messages
// in upb_Array.
//
// The unknown data is removed from message after upb_Array is populated.
// Since repeated messages can't be packed we remove each unknown that
// contains the target tag id.
upb_UnknownToMessage_Status upb_MiniTable_PromoteUnknownToMessageArray(
    upb_Message* msg, const upb_MiniTableField* field,
    const upb_MiniTable* mini_table, int decode_options, upb_Arena* arena);

// Promotes all unknown data that matches field tag id to upb_Map.
//
// The unknown data is removed from message after upb_Map is populated.
// Since repeated messages can't be packed we remove each unknown that
// contains the target tag id.
upb_UnknownToMessage_Status upb_MiniTable_PromoteUnknownToMap(
    upb_Message* msg, const upb_MiniTable* mini_table,
    const upb_MiniTableField* field, int decode_options, upb_Arena* arena);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MESSAGE_PROMOTE_H_
