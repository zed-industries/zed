// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_INTERNAL_FILE_H_
#define UPB_MINI_TABLE_INTERNAL_FILE_H_

// Must be last.
#include "upb/port/def.inc"

struct upb_MiniTableFile {
  const struct upb_MiniTable** UPB_PRIVATE(msgs);
  const struct upb_MiniTableEnum** UPB_PRIVATE(enums);
  const struct upb_MiniTableExtension** UPB_PRIVATE(exts);
  int UPB_PRIVATE(msg_count);
  int UPB_PRIVATE(enum_count);
  int UPB_PRIVATE(ext_count);
};

#ifdef __cplusplus
extern "C" {
#endif

UPB_API_INLINE int upb_MiniTableFile_EnumCount(
    const struct upb_MiniTableFile* f) {
  return f->UPB_PRIVATE(enum_count);
}

UPB_API_INLINE int upb_MiniTableFile_ExtensionCount(
    const struct upb_MiniTableFile* f) {
  return f->UPB_PRIVATE(ext_count);
}

UPB_API_INLINE int upb_MiniTableFile_MessageCount(
    const struct upb_MiniTableFile* f) {
  return f->UPB_PRIVATE(msg_count);
}

UPB_API_INLINE const struct upb_MiniTableEnum* upb_MiniTableFile_Enum(
    const struct upb_MiniTableFile* f, int i) {
  UPB_ASSERT(i < f->UPB_PRIVATE(enum_count));
  return f->UPB_PRIVATE(enums)[i];
}

UPB_API_INLINE const struct upb_MiniTableExtension* upb_MiniTableFile_Extension(
    const struct upb_MiniTableFile* f, int i) {
  UPB_ASSERT(i < f->UPB_PRIVATE(ext_count));
  return f->UPB_PRIVATE(exts)[i];
}

UPB_API_INLINE const struct upb_MiniTable* upb_MiniTableFile_Message(
    const struct upb_MiniTableFile* f, int i) {
  UPB_ASSERT(i < f->UPB_PRIVATE(msg_count));
  return f->UPB_PRIVATE(msgs)[i];
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_INTERNAL_FILE_H_ */
