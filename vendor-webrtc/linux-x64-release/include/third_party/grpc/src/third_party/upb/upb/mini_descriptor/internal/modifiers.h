// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_DESCRIPTOR_INTERNAL_MODIFIERS_H_
#define UPB_MINI_DESCRIPTOR_INTERNAL_MODIFIERS_H_

// Must be last.
#include "upb/port/def.inc"

typedef enum {
  kUpb_FieldModifier_IsRepeated = 1 << 0,
  kUpb_FieldModifier_IsPacked = 1 << 1,
  kUpb_FieldModifier_IsClosedEnum = 1 << 2,
  kUpb_FieldModifier_IsProto3Singular = 1 << 3,
  kUpb_FieldModifier_IsRequired = 1 << 4,
  kUpb_FieldModifier_ValidateUtf8 = 1 << 5,
} kUpb_FieldModifier;

// These modifiers are also used on the wire.
typedef enum {
  kUpb_MessageModifier_ValidateUtf8 = 1 << 0,
  kUpb_MessageModifier_DefaultIsPacked = 1 << 1,
  kUpb_MessageModifier_IsExtendable = 1 << 2,
} kUpb_MessageModifier;

#include "upb/port/undef.inc"

#endif  // UPB_MINI_DESCRIPTOR_INTERNAL_MODIFIERS_H_
