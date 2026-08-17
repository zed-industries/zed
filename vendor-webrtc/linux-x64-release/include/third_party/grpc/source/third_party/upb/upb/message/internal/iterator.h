// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef THIRD_PARTY_UPB_UPB_MESSAGE_INTERNAL_ITERATOR_H_
#define THIRD_PARTY_UPB_UPB_MESSAGE_INTERNAL_ITERATOR_H_

#include <stddef.h>

#include "upb/message/message.h"
#include "upb/message/value.h"
#include "upb/mini_table/extension.h"
#include "upb/mini_table/field.h"
#include "upb/mini_table/message.h"

// Must be last.
#include "upb/port/def.inc"

#define kUpb_BaseField_Begin ((size_t)-1)
#define kUpb_Extension_Begin ((size_t)-1)

bool UPB_PRIVATE(_upb_Message_NextBaseField)(const upb_Message* msg,
                                             const upb_MiniTable* m,
                                             const upb_MiniTableField** out_f,
                                             upb_MessageValue* out_v,
                                             size_t* iter);

bool UPB_PRIVATE(_upb_Message_NextExtension)(
    const upb_Message* msg, const upb_MiniTable* m,
    const upb_MiniTableExtension** out_e, upb_MessageValue* out_v,
    size_t* iter);
#endif  // THIRD_PARTY_UPB_UPB_MESSAGE_INTERNAL_ITERATOR_H_
