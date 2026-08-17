// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_ACCESSORS_H_
#define UPB_MESSAGE_ACCESSORS_H_

#include <stdint.h>

#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/array.h"
#include "upb/message/internal/accessors.h"
#include "upb/message/map.h"
#include "upb/message/message.h"
#include "upb/message/tagged_ptr.h"
#include "upb/message/value.h"
#include "upb/mini_table/extension.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Functions ending in BaseField() take a (upb_MiniTableField*) argument
// and work only on non-extension fields.
//
// Functions ending in Extension() take a (upb_MiniTableExtension*) argument
// and work only on extensions.

UPB_API_INLINE void upb_Message_Clear(upb_Message* msg, const upb_MiniTable* m);

UPB_API_INLINE void upb_Message_ClearBaseField(upb_Message* msg,
                                               const upb_MiniTableField* f);

UPB_API_INLINE void upb_Message_ClearExtension(upb_Message* msg,
                                               const upb_MiniTableExtension* e);

UPB_API_INLINE void upb_Message_ClearOneof(upb_Message* msg,
                                           const upb_MiniTable* m,
                                           const upb_MiniTableField* f);

UPB_API_INLINE bool upb_Message_HasBaseField(const upb_Message* msg,
                                             const upb_MiniTableField* f);

UPB_API_INLINE bool upb_Message_HasExtension(const upb_Message* msg,
                                             const upb_MiniTableExtension* e);

UPB_API_INLINE upb_MessageValue
upb_Message_GetField(const upb_Message* msg, const upb_MiniTableField* f,
                     upb_MessageValue default_val);

UPB_API_INLINE upb_TaggedMessagePtr upb_Message_GetTaggedMessagePtr(
    const upb_Message* msg, const upb_MiniTableField* field,
    upb_Message* default_val);

UPB_API_INLINE const upb_Array* upb_Message_GetArray(
    const upb_Message* msg, const upb_MiniTableField* f);

UPB_API_INLINE bool upb_Message_GetBool(const upb_Message* msg,
                                        const upb_MiniTableField* f,
                                        bool default_val);

UPB_API_INLINE double upb_Message_GetDouble(const upb_Message* msg,
                                            const upb_MiniTableField* field,
                                            double default_val);

UPB_API_INLINE float upb_Message_GetFloat(const upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          float default_val);

UPB_API_INLINE int32_t upb_Message_GetInt32(const upb_Message* msg,
                                            const upb_MiniTableField* f,
                                            int32_t default_val);

UPB_API_INLINE int64_t upb_Message_GetInt64(const upb_Message* msg,
                                            const upb_MiniTableField* f,
                                            int64_t default_val);

UPB_API_INLINE const upb_Map* upb_Message_GetMap(const upb_Message* msg,
                                                 const upb_MiniTableField* f);

UPB_API_INLINE const upb_Message* upb_Message_GetMessage(
    const upb_Message* msg, const upb_MiniTableField* f);

UPB_API_INLINE upb_Array* upb_Message_GetMutableArray(
    upb_Message* msg, const upb_MiniTableField* f);

UPB_API_INLINE upb_Map* upb_Message_GetMutableMap(upb_Message* msg,
                                                  const upb_MiniTableField* f);

UPB_API_INLINE upb_Message* upb_Message_GetMutableMessage(
    upb_Message* msg, const upb_MiniTableField* f);

UPB_API_INLINE upb_Array* upb_Message_GetOrCreateMutableArray(
    upb_Message* msg, const upb_MiniTableField* f, upb_Arena* arena);

UPB_API_INLINE upb_Map* upb_Message_GetOrCreateMutableMap(
    upb_Message* msg, const upb_MiniTable* map_entry_mini_table,
    const upb_MiniTableField* f, upb_Arena* arena);

UPB_API_INLINE upb_Message* upb_Message_GetOrCreateMutableMessage(
    upb_Message* msg, const upb_MiniTable* mini_table,
    const upb_MiniTableField* f, upb_Arena* arena);

UPB_API_INLINE upb_StringView
upb_Message_GetString(const upb_Message* msg, const upb_MiniTableField* field,
                      upb_StringView default_val);

UPB_API_INLINE uint32_t upb_Message_GetUInt32(const upb_Message* msg,
                                              const upb_MiniTableField* f,
                                              uint32_t default_val);

UPB_API_INLINE uint64_t upb_Message_GetUInt64(const upb_Message* msg,
                                              const upb_MiniTableField* f,
                                              uint64_t default_val);

UPB_API_INLINE void upb_Message_SetClosedEnum(
    upb_Message* msg, const upb_MiniTable* msg_mini_table,
    const upb_MiniTableField* f, int32_t value);

// BaseField Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE void upb_Message_SetBaseField(upb_Message* msg,
                                             const upb_MiniTableField* f,
                                             const void* val);

UPB_API_INLINE void upb_Message_SetBaseFieldBool(struct upb_Message* msg,
                                                 const upb_MiniTableField* f,
                                                 bool value);

UPB_API_INLINE void upb_Message_SetBaseFieldDouble(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   double value);

UPB_API_INLINE void upb_Message_SetBaseFieldFloat(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  float value);

UPB_API_INLINE void upb_Message_SetBaseFieldInt32(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  int32_t value);

UPB_API_INLINE void upb_Message_SetBaseFieldInt64(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  int64_t value);

UPB_API_INLINE void upb_Message_SetBaseFieldMessage(struct upb_Message* msg,
                                                    const upb_MiniTableField* f,
                                                    upb_Message* value);

UPB_API_INLINE void upb_Message_SetBaseFieldString(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   upb_StringView value);

UPB_API_INLINE void upb_Message_SetBaseFieldUInt32(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   uint32_t value);

UPB_API_INLINE void upb_Message_SetBaseFieldUInt64(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   uint64_t value);

// Extension Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE bool upb_Message_SetExtension(upb_Message* msg,
                                             const upb_MiniTableExtension* e,
                                             const void* value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionBool(
    struct upb_Message* msg, const upb_MiniTableExtension* e, bool value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionDouble(
    struct upb_Message* msg, const upb_MiniTableExtension* e, double value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionFloat(
    struct upb_Message* msg, const upb_MiniTableExtension* e, float value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionInt32(
    struct upb_Message* msg, const upb_MiniTableExtension* e, int32_t value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionInt64(
    struct upb_Message* msg, const upb_MiniTableExtension* e, int64_t value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionString(
    struct upb_Message* msg, const upb_MiniTableExtension* e,
    upb_StringView value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionUInt32(
    struct upb_Message* msg, const upb_MiniTableExtension* e, uint32_t value,
    upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetExtensionUInt64(
    struct upb_Message* msg, const upb_MiniTableExtension* e, uint64_t value,
    upb_Arena* a);

// Universal Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE bool upb_Message_SetBool(upb_Message* msg,
                                        const upb_MiniTableField* f, bool value,
                                        upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetDouble(upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          double value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetFloat(upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         float value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetInt32(upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         int32_t value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetInt64(upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         int64_t value, upb_Arena* a);

// Unlike the other similarly-named setters, this function can only be
// called on base fields. Prefer upb_Message_SetBaseFieldMessage().
UPB_API_INLINE void upb_Message_SetMessage(upb_Message* msg,
                                           const upb_MiniTableField* f,
                                           upb_Message* value);

UPB_API_INLINE bool upb_Message_SetString(upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          upb_StringView value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetUInt32(upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          uint32_t value, upb_Arena* a);

UPB_API_INLINE bool upb_Message_SetUInt64(upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          uint64_t value, upb_Arena* a);

////////////////////////////////////////////////////////////////////////////////

UPB_API_INLINE void* upb_Message_ResizeArrayUninitialized(
    upb_Message* msg, const upb_MiniTableField* f, size_t size,
    upb_Arena* arena);

UPB_API_INLINE uint32_t upb_Message_WhichOneofFieldNumber(
    const upb_Message* message, const upb_MiniTableField* oneof_field);

// For a field `f` which is in a oneof, return the field of that
// oneof that is actually set (or NULL if none).
UPB_API_INLINE const upb_MiniTableField* upb_Message_WhichOneof(
    const upb_Message* msg, const upb_MiniTable* m,
    const upb_MiniTableField* f);

// Updates a map entry given an entry message.
bool upb_Message_SetMapEntry(upb_Map* map, const upb_MiniTable* mini_table,
                             const upb_MiniTableField* field,
                             upb_Message* map_entry_message, upb_Arena* arena);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MESSAGE_ACCESSORS_H_
