// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
// https://developers.google.com/protocol-buffers/
///
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_INTERNAL_SIZE_LOG2_H_
#define UPB_MINI_TABLE_INTERNAL_SIZE_LOG2_H_

#include <stddef.h>
#include <stdint.h>

#include "upb/base/descriptor_constants.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Return the log2 of the storage size in bytes for a upb_CType
UPB_INLINE int UPB_PRIVATE(_upb_CType_SizeLg2)(upb_CType c_type) {
  static const int8_t size[] = {
      0,               // kUpb_CType_Bool
      2,               // kUpb_CType_Float
      2,               // kUpb_CType_Int32
      2,               // kUpb_CType_UInt32
      2,               // kUpb_CType_Enum
      UPB_SIZE(2, 3),  // kUpb_CType_Message
      3,               // kUpb_CType_Double
      3,               // kUpb_CType_Int64
      3,               // kUpb_CType_UInt64
      UPB_SIZE(3, 4),  // kUpb_CType_String
      UPB_SIZE(3, 4),  // kUpb_CType_Bytes
  };

  // -1 here because the enum is one-based but the table is zero-based.
  return size[c_type - 1];
}

// Return the log2 of the storage size in bytes for a upb_FieldType
UPB_INLINE int UPB_PRIVATE(_upb_FieldType_SizeLg2)(upb_FieldType field_type) {
  static const int8_t size[] = {
      3,               // kUpb_FieldType_Double
      2,               // kUpb_FieldType_Float
      3,               // kUpb_FieldType_Int64
      3,               // kUpb_FieldType_UInt64
      2,               // kUpb_FieldType_Int32
      3,               // kUpb_FieldType_Fixed64
      2,               // kUpb_FieldType_Fixed32
      0,               // kUpb_FieldType_Bool
      UPB_SIZE(3, 4),  // kUpb_FieldType_String
      UPB_SIZE(2, 3),  // kUpb_FieldType_Group
      UPB_SIZE(2, 3),  // kUpb_FieldType_Message
      UPB_SIZE(3, 4),  // kUpb_FieldType_Bytes
      2,               // kUpb_FieldType_UInt32
      2,               // kUpb_FieldType_Enum
      2,               // kUpb_FieldType_SFixed32
      3,               // kUpb_FieldType_SFixed64
      2,               // kUpb_FieldType_SInt32
      3,               // kUpb_FieldType_SInt64
  };

  // -1 here because the enum is one-based but the table is zero-based.
  return size[field_type - 1];
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_INTERNAL_SIZE_LOG2_H_ */
