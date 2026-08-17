// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_DESCRIPTOR_INTERNAL_DECODER_H_
#define UPB_MINI_DESCRIPTOR_INTERNAL_DECODER_H_

#include "upb/base/status.h"
#include "upb/mini_descriptor/internal/base92.h"

// Must be last.
#include "upb/port/def.inc"

// upb_MdDecoder: used internally for decoding MiniDescriptors for messages,
// extensions, and enums.
typedef struct {
  const char* end;
  upb_Status* status;
  jmp_buf err;
} upb_MdDecoder;

UPB_PRINTF(2, 3)
UPB_NORETURN UPB_INLINE void upb_MdDecoder_ErrorJmp(upb_MdDecoder* d,
                                                    const char* fmt, ...) {
  if (d->status) {
    va_list argp;
    upb_Status_SetErrorMessage(d->status, "Error building mini table: ");
    va_start(argp, fmt);
    upb_Status_VAppendErrorFormat(d->status, fmt, argp);
    va_end(argp);
  }
  UPB_LONGJMP(d->err, 1);
}

UPB_INLINE void upb_MdDecoder_CheckOutOfMemory(upb_MdDecoder* d,
                                               const void* ptr) {
  if (!ptr) upb_MdDecoder_ErrorJmp(d, "Out of memory");
}

UPB_INLINE const char* upb_MdDecoder_DecodeBase92Varint(
    upb_MdDecoder* d, const char* ptr, char first_ch, uint8_t min, uint8_t max,
    uint32_t* out_val) {
  ptr = _upb_Base92_DecodeVarint(ptr, d->end, first_ch, min, max, out_val);
  if (!ptr) upb_MdDecoder_ErrorJmp(d, "Overlong varint");
  return ptr;
}

#include "upb/port/undef.inc"

#endif  // UPB_MINI_DESCRIPTOR_INTERNAL_DECODER_H_
