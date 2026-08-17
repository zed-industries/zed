// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_BASE_DESCRIPTOR_CONSTANTS_H_
#define UPB_BASE_DESCRIPTOR_CONSTANTS_H_

// Must be last.
#include "upb/port/def.inc"

// The types a field can have. Note that this list is not identical to the
// types defined in descriptor.proto, which gives INT32 and SINT32 separate
// types (we distinguish the two with the "integer encoding" enum below).
// This enum is an internal convenience only and has no meaning outside of upb.
typedef enum {
  kUpb_CType_Bool = 1,
  kUpb_CType_Float = 2,
  kUpb_CType_Int32 = 3,
  kUpb_CType_UInt32 = 4,
  kUpb_CType_Enum = 5,  // Enum values are int32. TODO: rename
  kUpb_CType_Message = 6,
  kUpb_CType_Double = 7,
  kUpb_CType_Int64 = 8,
  kUpb_CType_UInt64 = 9,
  kUpb_CType_String = 10,
  kUpb_CType_Bytes = 11
} upb_CType;

// The repeated-ness of each field; this matches descriptor.proto.
typedef enum {
  kUpb_Label_Optional = 1,
  kUpb_Label_Required = 2,
  kUpb_Label_Repeated = 3
} upb_Label;

// Descriptor types, as defined in descriptor.proto.
typedef enum {
  kUpb_FieldType_Double = 1,
  kUpb_FieldType_Float = 2,
  kUpb_FieldType_Int64 = 3,
  kUpb_FieldType_UInt64 = 4,
  kUpb_FieldType_Int32 = 5,
  kUpb_FieldType_Fixed64 = 6,
  kUpb_FieldType_Fixed32 = 7,
  kUpb_FieldType_Bool = 8,
  kUpb_FieldType_String = 9,
  kUpb_FieldType_Group = 10,
  kUpb_FieldType_Message = 11,
  kUpb_FieldType_Bytes = 12,
  kUpb_FieldType_UInt32 = 13,
  kUpb_FieldType_Enum = 14,
  kUpb_FieldType_SFixed32 = 15,
  kUpb_FieldType_SFixed64 = 16,
  kUpb_FieldType_SInt32 = 17,
  kUpb_FieldType_SInt64 = 18,
} upb_FieldType;

#define kUpb_FieldType_SizeOf 19

#ifdef __cplusplus
extern "C" {
#endif

// Convert from upb_FieldType to upb_CType
UPB_INLINE upb_CType upb_FieldType_CType(upb_FieldType field_type) {
  static const upb_CType c_type[] = {
      kUpb_CType_Double,   // kUpb_FieldType_Double
      kUpb_CType_Float,    // kUpb_FieldType_Float
      kUpb_CType_Int64,    // kUpb_FieldType_Int64
      kUpb_CType_UInt64,   // kUpb_FieldType_UInt64
      kUpb_CType_Int32,    // kUpb_FieldType_Int32
      kUpb_CType_UInt64,   // kUpb_FieldType_Fixed64
      kUpb_CType_UInt32,   // kUpb_FieldType_Fixed32
      kUpb_CType_Bool,     // kUpb_FieldType_Bool
      kUpb_CType_String,   // kUpb_FieldType_String
      kUpb_CType_Message,  // kUpb_FieldType_Group
      kUpb_CType_Message,  // kUpb_FieldType_Message
      kUpb_CType_Bytes,    // kUpb_FieldType_Bytes
      kUpb_CType_UInt32,   // kUpb_FieldType_UInt32
      kUpb_CType_Enum,     // kUpb_FieldType_Enum
      kUpb_CType_Int32,    // kUpb_FieldType_SFixed32
      kUpb_CType_Int64,    // kUpb_FieldType_SFixed64
      kUpb_CType_Int32,    // kUpb_FieldType_SInt32
      kUpb_CType_Int64,    // kUpb_FieldType_SInt64
  };

  // -1 here because the enum is one-based but the table is zero-based.
  return c_type[field_type - 1];
}

UPB_INLINE bool upb_FieldType_IsPackable(upb_FieldType field_type) {
  // clang-format off
  const unsigned kUnpackableTypes =
      (1 << kUpb_FieldType_String) |
      (1 << kUpb_FieldType_Bytes) |
      (1 << kUpb_FieldType_Message) |
      (1 << kUpb_FieldType_Group);
  // clang-format on
  return (1 << field_type) & ~kUnpackableTypes;
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_BASE_DESCRIPTOR_CONSTANTS_H_ */
