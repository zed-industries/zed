// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MESSAGE_INTERNAL_MAP_ENTRY_H_
#define UPB_MESSAGE_INTERNAL_MAP_ENTRY_H_

#include <stdint.h>

#include "upb/base/string_view.h"
#include "upb/hash/common.h"
#include "upb/message/internal/types.h"

// Map entries aren't actually stored for map fields, they are only used during
// parsing. (It helps a lot if all map entry messages have the same layout.)
// The mini_table layout code will ensure that all map entries have this layout.
//
// Note that users can and do create map entries directly, which will also use
// this layout.

typedef struct {
  struct upb_Message message;
  // We only need 2 hasbits max, but due to alignment we'll use 8 bytes here,
  // and the uint64_t helps make this clear.
  uint64_t hasbits;
  union {
    upb_StringView str;  // For str/bytes.
    upb_value val;       // For all other types.
    double d[2];         // Padding for 32-bit builds.
  } k;
  union {
    upb_StringView str;  // For str/bytes.
    upb_value val;       // For all other types.
    double d[2];         // Padding for 32-bit builds.
  } v;
} upb_MapEntry;

#endif  // UPB_MESSAGE_INTERNAL_MAP_ENTRY_H_
