// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_INTERNAL_FIELD_H_
#define UPB_MINI_TABLE_INTERNAL_FIELD_H_

#include <stddef.h>
#include <stdint.h>

#include "upb/base/descriptor_constants.h"
#include "upb/mini_table/internal/size_log2.h"

// Must be last.
#include "upb/port/def.inc"

// LINT.IfChange(struct_definition)
struct upb_MiniTableField {
  uint32_t UPB_ONLYBITS(number);
  uint16_t UPB_ONLYBITS(offset);
  int16_t presence;  // If >0, hasbit_index.  If <0, ~oneof_index

  // Indexes into `upb_MiniTable.subs`
  // Will be set to `kUpb_NoSub` if `descriptortype` != MESSAGE/GROUP/ENUM
  uint16_t UPB_PRIVATE(submsg_index);

  uint8_t UPB_PRIVATE(descriptortype);

  // upb_FieldMode | upb_LabelFlags | (upb_FieldRep << kUpb_FieldRep_Shift)
  uint8_t UPB_ONLYBITS(mode);
};

#define kUpb_NoSub ((uint16_t) - 1)

typedef enum {
  kUpb_FieldMode_Map = 0,
  kUpb_FieldMode_Array = 1,
  kUpb_FieldMode_Scalar = 2,
} upb_FieldMode;

// Mask to isolate the upb_FieldMode from field.mode.
#define kUpb_FieldMode_Mask 3

// Extra flags on the mode field.
typedef enum {
  kUpb_LabelFlags_IsPacked = 4,
  kUpb_LabelFlags_IsExtension = 8,
  // Indicates that this descriptor type is an "alternate type":
  //   - for Int32, this indicates that the actual type is Enum (but was
  //     rewritten to Int32 because it is an open enum that requires no check).
  //   - for Bytes, this indicates that the actual type is String (but does
  //     not require any UTF-8 check).
  kUpb_LabelFlags_IsAlternate = 16,
} upb_LabelFlags;

// Note: we sort by this number when calculating layout order.
typedef enum {
  kUpb_FieldRep_1Byte = 0,
  kUpb_FieldRep_4Byte = 1,
  kUpb_FieldRep_StringView = 2,
  kUpb_FieldRep_8Byte = 3,

  kUpb_FieldRep_NativePointer =
      UPB_SIZE(kUpb_FieldRep_4Byte, kUpb_FieldRep_8Byte),
  kUpb_FieldRep_Max = kUpb_FieldRep_8Byte,
} upb_FieldRep;

#define kUpb_FieldRep_Shift 6

#ifdef __cplusplus
extern "C" {
#endif

UPB_INLINE upb_FieldMode
UPB_PRIVATE(_upb_MiniTableField_Mode)(const struct upb_MiniTableField* f) {
  return (upb_FieldMode)(f->UPB_ONLYBITS(mode) & kUpb_FieldMode_Mask);
}

UPB_INLINE upb_FieldRep
UPB_PRIVATE(_upb_MiniTableField_GetRep)(const struct upb_MiniTableField* f) {
  return (upb_FieldRep)(f->UPB_ONLYBITS(mode) >> kUpb_FieldRep_Shift);
}

UPB_API_INLINE bool upb_MiniTableField_IsArray(
    const struct upb_MiniTableField* f) {
  return UPB_PRIVATE(_upb_MiniTableField_Mode)(f) == kUpb_FieldMode_Array;
}

UPB_API_INLINE bool upb_MiniTableField_IsMap(
    const struct upb_MiniTableField* f) {
  return UPB_PRIVATE(_upb_MiniTableField_Mode)(f) == kUpb_FieldMode_Map;
}

UPB_API_INLINE bool upb_MiniTableField_IsScalar(
    const struct upb_MiniTableField* f) {
  return UPB_PRIVATE(_upb_MiniTableField_Mode)(f) == kUpb_FieldMode_Scalar;
}

UPB_INLINE bool UPB_PRIVATE(_upb_MiniTableField_IsAlternate)(
    const struct upb_MiniTableField* f) {
  return (f->UPB_ONLYBITS(mode) & kUpb_LabelFlags_IsAlternate) != 0;
}

UPB_API_INLINE bool upb_MiniTableField_IsExtension(
    const struct upb_MiniTableField* f) {
  return (f->UPB_ONLYBITS(mode) & kUpb_LabelFlags_IsExtension) != 0;
}

UPB_API_INLINE bool upb_MiniTableField_IsPacked(
    const struct upb_MiniTableField* f) {
  return (f->UPB_ONLYBITS(mode) & kUpb_LabelFlags_IsPacked) != 0;
}

UPB_API_INLINE upb_FieldType
upb_MiniTableField_Type(const struct upb_MiniTableField* f) {
  const upb_FieldType type = (upb_FieldType)f->UPB_PRIVATE(descriptortype);
  if (UPB_PRIVATE(_upb_MiniTableField_IsAlternate)(f)) {
    if (type == kUpb_FieldType_Int32) return kUpb_FieldType_Enum;
    if (type == kUpb_FieldType_Bytes) return kUpb_FieldType_String;
    UPB_ASSERT(false);
  }
  return type;
}

UPB_API_INLINE
upb_CType upb_MiniTableField_CType(const struct upb_MiniTableField* f) {
  return upb_FieldType_CType(upb_MiniTableField_Type(f));
}

UPB_INLINE bool UPB_PRIVATE(_upb_MiniTableField_HasHasbit)(
    const struct upb_MiniTableField* f) {
  return f->presence > 0;
}

UPB_INLINE char UPB_PRIVATE(_upb_MiniTableField_HasbitMask)(
    const struct upb_MiniTableField* f) {
  UPB_ASSERT(UPB_PRIVATE(_upb_MiniTableField_HasHasbit)(f));
  const size_t index = f->presence;
  return 1 << (index % 8);
}

UPB_INLINE size_t UPB_PRIVATE(_upb_MiniTableField_HasbitOffset)(
    const struct upb_MiniTableField* f) {
  UPB_ASSERT(UPB_PRIVATE(_upb_MiniTableField_HasHasbit)(f));
  const size_t index = f->presence;
  return index / 8;
}

UPB_API_INLINE bool upb_MiniTableField_IsClosedEnum(
    const struct upb_MiniTableField* f) {
  return f->UPB_PRIVATE(descriptortype) == kUpb_FieldType_Enum;
}

UPB_API_INLINE bool upb_MiniTableField_IsInOneof(
    const struct upb_MiniTableField* f) {
  return f->presence < 0;
}

UPB_API_INLINE bool upb_MiniTableField_IsSubMessage(
    const struct upb_MiniTableField* f) {
  return f->UPB_PRIVATE(descriptortype) == kUpb_FieldType_Message ||
         f->UPB_PRIVATE(descriptortype) == kUpb_FieldType_Group;
}

UPB_API_INLINE bool upb_MiniTableField_HasPresence(
    const struct upb_MiniTableField* f) {
  if (upb_MiniTableField_IsExtension(f)) {
    return upb_MiniTableField_IsScalar(f);
  } else {
    return f->presence != 0;
  }
}

UPB_API_INLINE uint32_t
upb_MiniTableField_Number(const struct upb_MiniTableField* f) {
  return f->UPB_ONLYBITS(number);
}

UPB_INLINE uint16_t
UPB_PRIVATE(_upb_MiniTableField_Offset)(const struct upb_MiniTableField* f) {
  return f->UPB_ONLYBITS(offset);
}

UPB_INLINE size_t UPB_PRIVATE(_upb_MiniTableField_OneofOffset)(
    const struct upb_MiniTableField* f) {
  UPB_ASSERT(upb_MiniTableField_IsInOneof(f));
  return ~(ptrdiff_t)f->presence;
}

UPB_INLINE void UPB_PRIVATE(_upb_MiniTableField_CheckIsArray)(
    const struct upb_MiniTableField* f) {
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             kUpb_FieldRep_NativePointer);
  UPB_ASSUME(upb_MiniTableField_IsArray(f));
  UPB_ASSUME(f->presence == 0);
}

UPB_INLINE void UPB_PRIVATE(_upb_MiniTableField_CheckIsMap)(
    const struct upb_MiniTableField* f) {
  UPB_ASSUME(UPB_PRIVATE(_upb_MiniTableField_GetRep)(f) ==
             kUpb_FieldRep_NativePointer);
  UPB_ASSUME(upb_MiniTableField_IsMap(f));
  UPB_ASSUME(f->presence == 0);
}

UPB_INLINE size_t UPB_PRIVATE(_upb_MiniTableField_ElemSizeLg2)(
    const struct upb_MiniTableField* f) {
  const upb_FieldType field_type = upb_MiniTableField_Type(f);
  return UPB_PRIVATE(_upb_FieldType_SizeLg2)(field_type);
}

// LINT.ThenChange(//depot/google3/third_party/upb/bits/typescript/mini_table_field.ts)

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_INTERNAL_FIELD_H_ */
