// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_DESCRIPTOR_BUILD_ENUM_H_
#define UPB_MINI_DESCRIPTOR_BUILD_ENUM_H_

#include "upb/base/status.h"
#include "upb/mem/arena.h"
#include "upb/mini_table/enum.h"

// Must be last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

// Builds a upb_MiniTableEnum from an enum mini descriptor.
// The mini descriptor must be for an enum, not a message.
UPB_API upb_MiniTableEnum* upb_MiniTableEnum_Build(const char* data, size_t len,
                                                   upb_Arena* arena,
                                                   upb_Status* status);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_MINI_DESCRIPTOR_BUILD_ENUM_H_
