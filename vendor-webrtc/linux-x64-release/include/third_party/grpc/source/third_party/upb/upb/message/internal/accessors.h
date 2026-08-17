// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_INTERNAL_ACCESSORS_H_
#define UPB_MESSAGE_INTERNAL_ACCESSORS_H_

#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include "upb/base/descriptor_constants.h"
#include "upb/base/internal/endian.h"
#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/internal/array.h"
#include "upb/message/internal/extension.h"
#include "upb/message/internal/map.h"
#include "upb/message/internal/message.h"
#include "upb/message/internal/tagged_ptr.h"
#include "upb/message/internal/types.h"
#include "upb/message/value.h"
#include "upb/mini_table/enum.h"
#include "upb/mini_table/extension.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/internal/field.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

#if defined(__GNUC__) && !defined(__clang__)
// GCC raises incorrect warnings in these functions.  It thinks that we are
// overrunning buffers, but we carefully write the functions in this file to
// guarantee that this is impossible.  GCC gets this wrong due it its failure
// to perform constant propagation as we expect:
//   - https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108217
//   - https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108226
//
// Unfortunately this also indicates that GCC is not optimizing away the
// switch() in cases where it should be, compromising the performance.
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Warray-bounds"
#pragma GCC diagnostic ignored "-Wstringop-overflow"
#if __GNUC__ >= 11
#pragma GCC diagnostic ignored "-Wstringop-overread"
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

// LINT.IfChange(presence_logic)

// Hasbit access ///////////////////////////////////////////////////////////////

UPB_INLINE bool UPB_PRIVATE(_upb_Message_GetHasbit)(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  const size_t offset = UPB_PRIVATE(_upb_MiniTableField_HasbitOffset)(f);
  const char mask = UPB_PRIVATE(_upb_MiniTableField_HasbitMask)(f);

  return (*UPB_PTR_AT(msg, offset, const char) & mask) != 0;
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_SetHasbit)(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  const size_t offset = UPB_PRIVATE(_upb_MiniTableField_HasbitOffset)(f);
  const char mask = UPB_PRIVATE(_upb_MiniTableField_HasbitMask)(f);

  (*UPB_PTR_AT(msg, offset, char)) |= mask;
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_ClearHasbit)(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  const size_t offset = UPB_PRIVATE(_upb_MiniTableField_HasbitOffset)(f);
  const char mask = UPB_PRIVATE(_upb_MiniTableField_HasbitMask)(f);

  (*UPB_PTR_AT(msg, offset, char)) &= ~mask;
}

// Oneof case access ///////////////////////////////////////////////////////////

UPB_INLINE uint32_t* UPB_PRIVATE(_upb_Message_OneofCasePtr)(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  return UPB_PTR_AT(msg, UPB_PRIVATE(_upb_MiniTableField_OneofOffset)(f),
                    uint32_t);
}

UPB_INLINE uint32_t UPB_PRIVATE(_upb_Message_GetOneofCase)(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  const uint32_t* ptr =
      UPB_PRIVATE(_upb_Message_OneofCasePtr)((struct upb_Message*)msg, f);

  return *ptr;
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_SetOneofCase)(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  uint32_t* ptr = UPB_PRIVATE(_upb_Message_OneofCasePtr)(msg, f);

  *ptr = upb_MiniTableField_Number(f);
}

// Returns true if the given field is the current oneof case.
// Does nothing if it is not the current oneof case.
UPB_INLINE bool UPB_PRIVATE(_upb_Message_ClearOneofCase)(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  uint32_t* ptr = UPB_PRIVATE(_upb_Message_OneofCasePtr)(msg, f);

  if (*ptr != upb_MiniTableField_Number(f)) return false;
  *ptr = 0;
  return true;
}

UPB_API_INLINE uint32_t upb_Message_WhichOneofFieldNumber(
    const struct upb_Message* message, const upb_MiniTableField* oneof_field) {
  UPB_ASSUME(upb_MiniTableField_IsInOneof(oneof_field));
  return UPB_PRIVATE(_upb_Message_GetOneofCase)(message, oneof_field);
}

UPB_API_INLINE const upb_MiniTableField* upb_Message_WhichOneof(
    const struct upb_Message* msg, const upb_MiniTable* m,
    const upb_MiniTableField* f) {
  uint32_t field_number = upb_Message_WhichOneofFieldNumber(msg, f);
  if (field_number == 0) {
    // No field in the oneof is set.
    return NULL;
  }
  return upb_MiniTable_FindFieldByNumber(m, field_number);
}

// LINT.ThenChange(GoogleInternalName2)

// Returns false if the message is missing any of its required fields.
UPB_INLINE bool UPB_PRIVATE(_upb_Message_IsInitializedShallow)(
    const struct upb_Message* msg, const upb_MiniTable* m) {
  uint64_t bits;
  memcpy(&bits, msg + 1, sizeof(bits));
  bits = upb_BigEndian64(bits);
  return (UPB_PRIVATE(_upb_MiniTable_RequiredMask)(m) & ~bits) == 0;
}

UPB_INLINE void* UPB_PRIVATE(_upb_Message_MutableDataPtr)(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  return (char*)msg + f->UPB_ONLYBITS(offset);
}

UPB_INLINE const void* UPB_PRIVATE(_upb_Message_DataPtr)(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  return (const char*)msg + f->UPB_ONLYBITS(offset);
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_SetPresence)(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  if (UPB_PRIVATE(_upb_MiniTableField_HasHasbit)(f)) {
    UPB_PRIVATE(_upb_Message_SetHasbit)(msg, f);
  } else if (upb_MiniTableField_IsInOneof(f)) {
    UPB_PRIVATE(_upb_Message_SetOneofCase)(msg, f);
  }
}

UPB_INLINE void UPB_PRIVATE(_upb_MiniTableField_DataCopy)(
    const upb_MiniTableField* f, void* to, const void* from) {
  switch (UPB_PRIVATE(_upb_MiniTableField_GetRep)(f)) {
    case kUpb_FieldRep_1Byte:
      memcpy(to, from, 1);
      return;
    case kUpb_FieldRep_4Byte:
      memcpy(to, from, 4);
      return;
    case kUpb_FieldRep_8Byte:
      memcpy(to, from, 8);
      return;
    case kUpb_FieldRep_StringView: {
      memcpy(to, from, sizeof(upb_StringView));
      return;
    }
  }
  UPB_UNREACHABLE();
}

UPB_INLINE bool UPB_PRIVATE(_upb_MiniTableField_DataEquals)(
    const upb_MiniTableField* f, const void* a, const void* b) {
  switch (UPB_PRIVATE(_upb_MiniTableField_GetRep)(f)) {
    case kUpb_FieldRep_1Byte:
      return memcmp(a, b, 1) == 0;
    case kUpb_FieldRep_4Byte:
      return memcmp(a, b, 4) == 0;
    case kUpb_FieldRep_8Byte:
      return memcmp(a, b, 8) == 0;
    case kUpb_FieldRep_StringView: {
      const upb_StringView sa = *(const upb_StringView*)a;
      const upb_StringView sb = *(const upb_StringView*)b;
      return upb_StringView_IsEqual(sa, sb);
    }
  }
  UPB_UNREACHABLE();
}

UPB_INLINE void UPB_PRIVATE(_upb_MiniTableField_DataClear)(
    const upb_MiniTableField* f, void* val) {
  const char zero[16] = {0};
  UPB_PRIVATE(_upb_MiniTableField_DataCopy)(f, val, zero);
}

UPB_INLINE bool UPB_PRIVATE(_upb_MiniTableField_DataIsZero)(
    const upb_MiniTableField* f, const void* val) {
  const char zero[16] = {0};
  return UPB_PRIVATE(_upb_MiniTableField_DataEquals)(f, val, zero);
}

// Here we define universal getter/setter functions for message fields.
// These look very branchy and inefficient, but as long as the MiniTableField
// values are known at compile time, all the branches are optimized away and
// we are left with ideal code.  This can happen either through through
// literals or UPB_ASSUME():
//
//   // Via struct literals.
//   bool FooMessage_set_bool_field(const upb_Message* msg, bool val) {
//     const upb_MiniTableField field = {1, 0, 0, /* etc... */};
//     // All value in "field" are compile-time known.
//     upb_Message_SetBaseField(msg, &field, &value);
//   }
//
//   // Via UPB_ASSUME().
//   UPB_INLINE bool upb_Message_SetBool(upb_Message* msg,
//                                       const upb_MiniTableField* field,
//                                       bool value, upb_Arena* a) {
//     UPB_ASSUME(field->UPB_PRIVATE(descriptortype) == kUpb_FieldType_Bool);
//     UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(field) ==
//                kUpb_FieldRep_1Byte);
//     upb_Message_SetField(msg, field, &value, a);
//   }
//
// As a result, we can use these universal getters/setters for *all* message
// accessors: generated code, MiniTable accessors, and reflection.  The only
// exception is the binary encoder/decoder, which need to be a bit more clever
// about how they read/write the message data, for efficiency.
//
// These functions work on both extensions and non-extensions. If the field
// of a setter is known to be a non-extension, the arena may be NULL and the
// returned bool value may be ignored since it will always succeed.

UPB_API_INLINE bool upb_Message_HasBaseField(const struct upb_Message* msg,
                                             const upb_MiniTableField* field) {
  UPB_ASSERT(upb_MiniTableField_HasPresence(field));
  UPB_ASSUME(!upb_MiniTableField_IsExtension(field));
  if (upb_MiniTableField_IsInOneof(field)) {
    return UPB_PRIVATE(_upb_Message_GetOneofCase)(msg, field) ==
           upb_MiniTableField_Number(field);
  } else {
    return UPB_PRIVATE(_upb_Message_GetHasbit)(msg, field);
  }
}

UPB_API_INLINE bool upb_Message_HasExtension(const struct upb_Message* msg,
                                             const upb_MiniTableExtension* e) {
  UPB_ASSERT(upb_MiniTableField_HasPresence(&e->UPB_PRIVATE(field)));
  return UPB_PRIVATE(_upb_Message_Getext)(msg, e) != NULL;
}

UPB_FORCEINLINE void _upb_Message_GetNonExtensionField(
    const struct upb_Message* msg, const upb_MiniTableField* field,
    const void* default_val, void* val) {
  UPB_ASSUME(!upb_MiniTableField_IsExtension(field));
  if ((upb_MiniTableField_IsInOneof(field) ||
       !UPB_PRIVATE(_upb_MiniTableField_DataIsZero)(field, default_val)) &&
      !upb_Message_HasBaseField(msg, field)) {
    UPB_PRIVATE(_upb_MiniTableField_DataCopy)(field, val, default_val);
    return;
  }
  UPB_PRIVATE(_upb_MiniTableField_DataCopy)
  (field, val, UPB_PRIVATE(_upb_Message_DataPtr)(msg, field));
}

UPB_INLINE void _upb_Message_GetExtensionField(
    const struct upb_Message* msg, const upb_MiniTableExtension* mt_ext,
    const void* default_val, void* val) {
  const upb_Extension* ext = UPB_PRIVATE(_upb_Message_Getext)(msg, mt_ext);
  const upb_MiniTableField* f = &mt_ext->UPB_PRIVATE(field);
  UPB_ASSUME(upb_MiniTableField_IsExtension(f));

  if (ext) {
    UPB_PRIVATE(_upb_MiniTableField_DataCopy)(f, val, &ext->data);
  } else {
    UPB_PRIVATE(_upb_MiniTableField_DataCopy)(f, val, default_val);
  }
}

// NOTE: The default_val is only used for fields that support presence.
// For repeated/map fields, the resulting upb_Array*/upb_Map* can be NULL if a
// upb_Array/upb_Map has not been allocated yet. Array/map fields do not have
// presence, so this is semantically identical to a pointer to an empty
// array/map, and must be treated the same for all semantic purposes.
UPB_API_INLINE upb_MessageValue upb_Message_GetField(
    const struct upb_Message* msg, const upb_MiniTableField* field,
    upb_MessageValue default_val) {
  upb_MessageValue ret;
  if (upb_MiniTableField_IsExtension(field)) {
    _upb_Message_GetExtensionField(msg, (upb_MiniTableExtension*)field,
                                   &default_val, &ret);
  } else {
    _upb_Message_GetNonExtensionField(msg, field, &default_val, &ret);
  }
  return ret;
}

UPB_API_INLINE void upb_Message_SetBaseField(struct upb_Message* msg,
                                             const upb_MiniTableField* f,
                                             const void* val) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  UPB_ASSUME(!upb_MiniTableField_IsExtension(f));
  UPB_PRIVATE(_upb_Message_SetPresence)(msg, f);
  UPB_PRIVATE(_upb_MiniTableField_DataCopy)
  (f, UPB_PRIVATE(_upb_Message_MutableDataPtr)(msg, f), val);
}

UPB_API_INLINE bool upb_Message_SetExtension(struct upb_Message* msg,
                                             const upb_MiniTableExtension* e,
                                             const void* val, upb_Arena* a) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  UPB_ASSERT(a);
  upb_Extension* ext =
      UPB_PRIVATE(_upb_Message_GetOrCreateExtension)(msg, e, a);
  if (!ext) return false;
  UPB_PRIVATE(_upb_MiniTableField_DataCopy)
  (&e->UPB_PRIVATE(field), &ext->data, val);
  return true;
}

// Sets the value of the given field in the given msg. The return value is true
// if the operation completed successfully, or false if memory allocation
// failed.
UPB_INLINE bool UPB_PRIVATE(_upb_Message_SetField)(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   upb_MessageValue val,
                                                   upb_Arena* a) {
  if (upb_MiniTableField_IsExtension(f)) {
    const upb_MiniTableExtension* ext = (const upb_MiniTableExtension*)f;
    return upb_Message_SetExtension(msg, ext, &val, a);
  } else {
    upb_Message_SetBaseField(msg, f, &val);
    return true;
  }
}

UPB_API_INLINE const upb_Array* upb_Message_GetArray(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(f);
  upb_Array* ret;
  const upb_Array* default_val = NULL;
  _upb_Message_GetNonExtensionField(msg, f, &default_val, &ret);
  return ret;
}

UPB_API_INLINE bool upb_Message_GetBool(const struct upb_Message* msg,
                                        const upb_MiniTableField* f,
                                        bool default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Bool);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_1Byte);
  upb_MessageValue def;
  def.bool_val = default_val;
  return upb_Message_GetField(msg, f, def).bool_val;
}

UPB_API_INLINE double upb_Message_GetDouble(const struct upb_Message* msg,
                                            const upb_MiniTableField* f,
                                            double default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Double);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);

  upb_MessageValue def;
  def.double_val = default_val;
  return upb_Message_GetField(msg, f, def).double_val;
}

UPB_API_INLINE float upb_Message_GetFloat(const struct upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          float default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Float);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);

  upb_MessageValue def;
  def.float_val = default_val;
  return upb_Message_GetField(msg, f, def).float_val;
}

UPB_API_INLINE int32_t upb_Message_GetInt32(const struct upb_Message* msg,
                                            const upb_MiniTableField* f,
                                            int32_t default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Int32 ||
             upb_MiniTableField_CType(f) == kUpb_CType_Enum);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);

  upb_MessageValue def;
  def.int32_val = default_val;
  return upb_Message_GetField(msg, f, def).int32_val;
}

UPB_API_INLINE int64_t upb_Message_GetInt64(const struct upb_Message* msg,
                                            const upb_MiniTableField* f,
                                            int64_t default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Int64);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);

  upb_MessageValue def;
  def.int64_val = default_val;
  return upb_Message_GetField(msg, f, def).int64_val;
}

UPB_INLINE void UPB_PRIVATE(_upb_Message_AssertMapIsUntagged)(
    const struct upb_Message* msg, const upb_MiniTableField* field) {
  UPB_UNUSED(msg);
  UPB_PRIVATE(_upb_MiniTableField_CheckIsMap)(field);
#ifndef NDEBUG
  uintptr_t default_val = 0;
  uintptr_t tagged;
  _upb_Message_GetNonExtensionField(msg, field, &default_val, &tagged);
  UPB_ASSERT(!upb_TaggedMessagePtr_IsEmpty(tagged));
#endif
}

UPB_API_INLINE const struct upb_Map* upb_Message_GetMap(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  UPB_PRIVATE(_upb_MiniTableField_CheckIsMap)(f);
  UPB_PRIVATE(_upb_Message_AssertMapIsUntagged)(msg, f);
  struct upb_Map* ret;
  const struct upb_Map* default_val = NULL;
  _upb_Message_GetNonExtensionField(msg, f, &default_val, &ret);
  return ret;
}

UPB_API_INLINE uintptr_t upb_Message_GetTaggedMessagePtr(
    const struct upb_Message* msg, const upb_MiniTableField* f,
    struct upb_Message* default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Message);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             UPB_SIZE(kUpb_FieldRep_4Byte, kUpb_FieldRep_8Byte));
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  uintptr_t tagged;
  _upb_Message_GetNonExtensionField(msg, f, &default_val, &tagged);
  return tagged;
}

// For internal use only; users cannot set tagged messages because only the
// parser and the message copier are allowed to directly create an empty
// message.
UPB_INLINE void UPB_PRIVATE(_upb_Message_SetTaggedMessagePtr)(
    struct upb_Message* msg, const upb_MiniTableField* f,
    uintptr_t sub_message) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Message);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             UPB_SIZE(kUpb_FieldRep_4Byte, kUpb_FieldRep_8Byte));
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  upb_Message_SetBaseField(msg, f, &sub_message);
}

UPB_API_INLINE const struct upb_Message* upb_Message_GetMessage(
    const struct upb_Message* msg, const upb_MiniTableField* f) {
  uintptr_t tagged = upb_Message_GetTaggedMessagePtr(msg, f, NULL);
  return upb_TaggedMessagePtr_GetNonEmptyMessage(tagged);
}

UPB_API_INLINE upb_Array* upb_Message_GetMutableArray(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(f);
  return (upb_Array*)upb_Message_GetArray(msg, f);
}

UPB_API_INLINE struct upb_Map* upb_Message_GetMutableMap(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  return (struct upb_Map*)upb_Message_GetMap(msg, f);
}

UPB_API_INLINE struct upb_Message* upb_Message_GetMutableMessage(
    struct upb_Message* msg, const upb_MiniTableField* f) {
  return (struct upb_Message*)upb_Message_GetMessage(msg, f);
}

UPB_API_INLINE upb_Array* upb_Message_GetOrCreateMutableArray(
    struct upb_Message* msg, const upb_MiniTableField* f, upb_Arena* arena) {
  UPB_ASSERT(arena);
  UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(f);
  upb_Array* array = upb_Message_GetMutableArray(msg, f);
  if (!array) {
    array = UPB_PRIVATE(_upb_Array_New)(
        arena, 4, UPB_PRIVATE(_upb_MiniTableField_ElemSizeLg2)(f));
    // Check again due to: https://godbolt.org/z/7WfaoKG1r
    UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(f);
    upb_MessageValue val;
    val.array_val = array;
    UPB_PRIVATE(_upb_Message_SetField)(msg, f, val, arena);
  }
  return array;
}

UPB_INLINE struct upb_Map* _upb_Message_GetOrCreateMutableMap(
    struct upb_Message* msg, const upb_MiniTableField* field, size_t key_size,
    size_t val_size, upb_Arena* arena) {
  UPB_PRIVATE(_upb_MiniTableField_CheckIsMap)(field);
  UPB_PRIVATE(_upb_Message_AssertMapIsUntagged)(msg, field);
  struct upb_Map* map = NULL;
  struct upb_Map* default_map_value = NULL;
  _upb_Message_GetNonExtensionField(msg, field, &default_map_value, &map);
  if (!map) {
    map = _upb_Map_New(arena, key_size, val_size);
    // Check again due to: https://godbolt.org/z/7WfaoKG1r
    UPB_PRIVATE(_upb_MiniTableField_CheckIsMap)(field);
    upb_Message_SetBaseField(msg, field, &map);
  }
  return map;
}

UPB_API_INLINE struct upb_Map* upb_Message_GetOrCreateMutableMap(
    struct upb_Message* msg, const upb_MiniTable* map_entry_mini_table,
    const upb_MiniTableField* f, upb_Arena* arena) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Message);
  const upb_MiniTableField* map_entry_key_field =
      &map_entry_mini_table->UPB_ONLYBITS(fields)[0];
  const upb_MiniTableField* map_entry_value_field =
      &map_entry_mini_table->UPB_ONLYBITS(fields)[1];
  return _upb_Message_GetOrCreateMutableMap(
      msg, f, _upb_Map_CTypeSize(upb_MiniTableField_CType(map_entry_key_field)),
      _upb_Map_CTypeSize(upb_MiniTableField_CType(map_entry_value_field)),
      arena);
}

UPB_API_INLINE struct upb_Message* upb_Message_GetOrCreateMutableMessage(
    struct upb_Message* msg, const upb_MiniTable* mini_table,
    const upb_MiniTableField* f, upb_Arena* arena) {
  UPB_ASSERT(arena);
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Message);
  UPB_ASSUME(!upb_MiniTableField_IsExtension(f));
  struct upb_Message* sub_message =
      *UPB_PTR_AT(msg, f->UPB_ONLYBITS(offset), struct upb_Message*);
  if (!sub_message) {
    const upb_MiniTable* sub_mini_table =
        upb_MiniTable_SubMessage(mini_table, f);
    UPB_ASSERT(sub_mini_table);
    sub_message = _upb_Message_New(sub_mini_table, arena);
    *UPB_PTR_AT(msg, f->UPB_ONLYBITS(offset), struct upb_Message*) =
        sub_message;
    UPB_PRIVATE(_upb_Message_SetPresence)(msg, f);
  }
  return sub_message;
}

UPB_API_INLINE upb_StringView
upb_Message_GetString(const struct upb_Message* msg,
                      const upb_MiniTableField* f, upb_StringView default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_String ||
             upb_MiniTableField_CType(f) == kUpb_CType_Bytes);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             kUpb_FieldRep_StringView);

  upb_MessageValue def;
  def.str_val = default_val;
  return upb_Message_GetField(msg, f, def).str_val;
}

UPB_API_INLINE uint32_t upb_Message_GetUInt32(const struct upb_Message* msg,
                                              const upb_MiniTableField* f,
                                              uint32_t default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_UInt32);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);

  upb_MessageValue def;
  def.uint32_val = default_val;
  return upb_Message_GetField(msg, f, def).uint32_val;
}

UPB_API_INLINE uint64_t upb_Message_GetUInt64(const struct upb_Message* msg,
                                              const upb_MiniTableField* f,
                                              uint64_t default_val) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_UInt64);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);

  upb_MessageValue def;
  def.uint64_val = default_val;
  return upb_Message_GetField(msg, f, def).uint64_val;
}

// BaseField Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE void upb_Message_SetBaseFieldBool(struct upb_Message* msg,
                                                 const upb_MiniTableField* f,
                                                 bool value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Bool);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_1Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldDouble(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   double value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Double);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldFloat(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  float value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Float);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldInt32(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  int32_t value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Int32 ||
             upb_MiniTableField_CType(f) == kUpb_CType_Enum);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldInt64(struct upb_Message* msg,
                                                  const upb_MiniTableField* f,
                                                  int64_t value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Int64);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldMessage(struct upb_Message* msg,
                                                    const upb_MiniTableField* f,
                                                    struct upb_Message* value) {
  UPB_PRIVATE(_upb_Message_SetTaggedMessagePtr)
  (msg, f, UPB_PRIVATE(_upb_TaggedMessagePtr_Pack)(value, false));
}

UPB_API_INLINE void upb_Message_SetBaseFieldString(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   upb_StringView value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_String ||
             upb_MiniTableField_CType(f) == kUpb_CType_Bytes);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             kUpb_FieldRep_StringView);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldUInt32(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   uint32_t value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_UInt32);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetBaseFieldUInt64(struct upb_Message* msg,
                                                   const upb_MiniTableField* f,
                                                   uint64_t value) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_UInt64);
  UPB_ASSUME(upb_MiniTableField_IsScalar(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_8Byte);
  upb_Message_SetBaseField(msg, f, &value);
}

UPB_API_INLINE void upb_Message_SetClosedEnum(struct upb_Message* msg,
                                              const upb_MiniTable* m,
                                              const upb_MiniTableField* f,
                                              int32_t value) {
  UPB_ASSERT(upb_MiniTableField_IsClosedEnum(f));
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) == kUpb_FieldRep_4Byte);
  UPB_ASSERT(
      upb_MiniTableEnum_CheckValue(upb_MiniTable_GetSubEnumTable(m, f), value));
  upb_Message_SetBaseField(msg, f, &value);
}

// Extension Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE bool upb_Message_SetExtensionBool(
    struct upb_Message* msg, const upb_MiniTableExtension* e, bool value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_Bool);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_1Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionDouble(
    struct upb_Message* msg, const upb_MiniTableExtension* e, double value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_Double);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_8Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionFloat(
    struct upb_Message* msg, const upb_MiniTableExtension* e, float value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_Float);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_4Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionInt32(
    struct upb_Message* msg, const upb_MiniTableExtension* e, int32_t value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_Int32 ||
             upb_MiniTableExtension_CType(e) == kUpb_CType_Enum);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_4Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionInt64(
    struct upb_Message* msg, const upb_MiniTableExtension* e, int64_t value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_Int64);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_8Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionString(
    struct upb_Message* msg, const upb_MiniTableExtension* e,
    upb_StringView value, upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_String ||
             upb_MiniTableExtension_CType(e) == kUpb_CType_Bytes);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_StringView);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionUInt32(
    struct upb_Message* msg, const upb_MiniTableExtension* e, uint32_t value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_UInt32);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_4Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

UPB_API_INLINE bool upb_Message_SetExtensionUInt64(
    struct upb_Message* msg, const upb_MiniTableExtension* e, uint64_t value,
    upb_Arena* a) {
  UPB_ASSUME(upb_MiniTableExtension_CType(e) == kUpb_CType_UInt64);
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableExtension_GetRep)(e) ==
             kUpb_FieldRep_8Byte);
  return upb_Message_SetExtension(msg, e, &value, a);
}

// Universal Setters ///////////////////////////////////////////////////////////

UPB_API_INLINE bool upb_Message_SetBool(struct upb_Message* msg,
                                        const upb_MiniTableField* f, bool value,
                                        upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionBool(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldBool(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetDouble(struct upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          double value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionDouble(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldDouble(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetFloat(struct upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         float value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionFloat(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldFloat(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetInt32(struct upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         int32_t value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionInt32(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldInt32(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetInt64(struct upb_Message* msg,
                                         const upb_MiniTableField* f,
                                         int64_t value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionInt64(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldInt64(msg, f, value), true);
}

// Sets the value of a message-typed field. The mini_tables of `msg` and
// `value` must have been linked for this to work correctly.
UPB_API_INLINE void upb_Message_SetMessage(struct upb_Message* msg,
                                           const upb_MiniTableField* f,
                                           struct upb_Message* value) {
  UPB_ASSERT(!upb_MiniTableField_IsExtension(f));
  upb_Message_SetBaseFieldMessage(msg, f, value);
}

// Sets the value of a `string` or `bytes` field. The bytes of the value are not
// copied, so it is the caller's responsibility to ensure that they remain valid
// for the lifetime of `msg`. That might be done by copying them into the given
// arena, or by fusing that arena with the arena the bytes live in, for example.
UPB_API_INLINE bool upb_Message_SetString(struct upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          upb_StringView value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionString(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldString(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetUInt32(struct upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          uint32_t value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionUInt32(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldUInt32(msg, f, value), true);
}

UPB_API_INLINE bool upb_Message_SetUInt64(struct upb_Message* msg,
                                          const upb_MiniTableField* f,
                                          uint64_t value, upb_Arena* a) {
  return upb_MiniTableField_IsExtension(f)
             ? upb_Message_SetExtensionUInt64(
                   msg, (const upb_MiniTableExtension*)f, value, a)
             : (upb_Message_SetBaseFieldUInt64(msg, f, value), true);
}

UPB_API_INLINE void upb_Message_Clear(struct upb_Message* msg,
                                      const upb_MiniTable* m) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  upb_Message_Internal* in = UPB_PRIVATE(_upb_Message_GetInternal)(msg);
  memset(msg, 0, m->UPB_PRIVATE(size));
  if (in) {
    // Reset the internal buffer to empty.
    in->unknown_end = sizeof(upb_Message_Internal);
    in->ext_begin = in->size;
    UPB_PRIVATE(_upb_Message_SetInternal)(msg, in);
  }
}

UPB_API_INLINE void upb_Message_ClearBaseField(struct upb_Message* msg,
                                               const upb_MiniTableField* f) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  if (UPB_PRIVATE(_upb_MiniTableField_HasHasbit)(f)) {
    UPB_PRIVATE(_upb_Message_ClearHasbit)(msg, f);
  } else if (upb_MiniTableField_IsInOneof(f)) {
    uint32_t* ptr = UPB_PRIVATE(_upb_Message_OneofCasePtr)(msg, f);
    if (*ptr != upb_MiniTableField_Number(f)) return;
    *ptr = 0;
  }
  const char zeros[16] = {0};
  UPB_PRIVATE(_upb_MiniTableField_DataCopy)
  (f, UPB_PRIVATE(_upb_Message_MutableDataPtr)(msg, f), zeros);
}

UPB_API_INLINE void upb_Message_ClearExtension(
    struct upb_Message* msg, const upb_MiniTableExtension* e) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  upb_Message_Internal* in = UPB_PRIVATE(_upb_Message_GetInternal)(msg);
  if (!in) return;
  const upb_Extension* base = UPB_PTR_AT(in, in->ext_begin, upb_Extension);
  upb_Extension* ext = (upb_Extension*)UPB_PRIVATE(_upb_Message_Getext)(msg, e);
  if (ext) {
    *ext = *base;
    in->ext_begin += sizeof(upb_Extension);
  }
}

UPB_API_INLINE void upb_Message_ClearOneof(struct upb_Message* msg,
                                           const upb_MiniTable* m,
                                           const upb_MiniTableField* f) {
  UPB_ASSERT(!upb_Message_IsFrozen(msg));
  uint32_t field_number = upb_Message_WhichOneofFieldNumber(msg, f);
  if (field_number == 0) {
    // No field in the oneof is set.
    return;
  }

  const upb_MiniTableField* field =
      upb_MiniTable_FindFieldByNumber(m, field_number);
  upb_Message_ClearBaseField(msg, field);
}

UPB_API_INLINE void* upb_Message_ResizeArrayUninitialized(
    struct upb_Message* msg, const upb_MiniTableField* f, size_t size,
    upb_Arena* arena) {
  UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(f);
  upb_Array* arr = upb_Message_GetOrCreateMutableArray(msg, f, arena);
  if (!arr || !UPB_PRIVATE(_upb_Array_ResizeUninitialized)(arr, size, arena)) {
    return NULL;
  }
  return upb_Array_MutableDataPtr(arr);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#if defined(__GNUC__) && !defined(__clang__)
#pragma GCC diagnostic pop
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MESSAGE_INTERNAL_ACCESSORS_H_
