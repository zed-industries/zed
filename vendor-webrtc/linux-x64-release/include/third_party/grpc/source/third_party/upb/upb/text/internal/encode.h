// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_TEXT_ENCODE_INTERNAL_H_
#define UPB_TEXT_ENCODE_INTERNAL_H_

#include <stdarg.h>
#include <string.h>

#include "upb/base/descriptor_constants.h"
#include "upb/base/string_view.h"
#include "upb/message/array.h"
#include "upb/message/internal/map_sorter.h"
#include "upb/port/vsnprintf_compat.h"
#include "upb/text/options.h"
#include "upb/wire/eps_copy_input_stream.h"
#include "utf8_range.h"

// Must be last.
#include "upb/port/def.inc"

typedef struct {
  char *buf, *ptr, *end;
  size_t overflow;
  int indent_depth;
  int options;
  const struct upb_DefPool* ext_pool;
  _upb_mapsorter sorter;
} txtenc;

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_PutBytes)(txtenc* e,
                                                      const void* data,
                                                      size_t len) {
  size_t have = e->end - e->ptr;
  if (UPB_LIKELY(have >= len)) {
    memcpy(e->ptr, data, len);
    e->ptr += len;
  } else {
    if (have) {
      memcpy(e->ptr, data, have);
      e->ptr += have;
    }
    e->overflow += (len - have);
  }
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_PutStr)(txtenc* e,
                                                    const char* str) {
  UPB_PRIVATE(_upb_TextEncode_PutBytes)(e, str, strlen(str));
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_Printf)(txtenc* e, const char* fmt,
                                                    ...) {
  size_t n;
  size_t have = e->end - e->ptr;
  va_list args;

  va_start(args, fmt);
  n = _upb_vsnprintf(e->ptr, have, fmt, args);
  va_end(args);

  if (UPB_LIKELY(have > n)) {
    e->ptr += n;
  } else {
    e->ptr = UPB_PTRADD(e->ptr, have);
    e->overflow += (n - have);
  }
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_Indent)(txtenc* e) {
  if ((e->options & UPB_TXTENC_SINGLELINE) == 0) {
    int i = e->indent_depth;
    while (i-- > 0) {
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "  ");
    }
  }
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_EndField)(txtenc* e) {
  if (e->options & UPB_TXTENC_SINGLELINE) {
    UPB_PRIVATE(_upb_TextEncode_PutStr)(e, " ");
  } else {
    UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\n");
  }
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_Escaped)(txtenc* e,
                                                     unsigned char ch) {
  switch (ch) {
    case '\n':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\n");
      break;
    case '\r':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\r");
      break;
    case '\t':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\t");
      break;
    case '\"':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\\"");
      break;
    case '\'':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\'");
      break;
    case '\\':
      UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\\\\");
      break;
    default:
      UPB_PRIVATE(_upb_TextEncode_Printf)(e, "\\%03o", ch);
      break;
  }
}

// Returns true if `ch` needs to be escaped in TextFormat, independent of any
// UTF-8 validity issues.
UPB_INLINE bool UPB_PRIVATE(_upb_DefinitelyNeedsEscape)(unsigned char ch) {
  if (ch < 32) return true;
  switch (ch) {
    case '\"':
    case '\'':
    case '\\':
    case 127:
      return true;
  }
  return false;
}

UPB_INLINE bool UPB_PRIVATE(_upb_AsciiIsPrint)(unsigned char ch) {
  return ch >= 32 && ch < 127;
}

// Returns true if this is a high byte that requires UTF-8 validation.  If the
// UTF-8 validation fails, we must escape the byte.
UPB_INLINE bool UPB_PRIVATE(_upb_NeedsUtf8Validation)(unsigned char ch) {
  return ch > 127;
}

// Returns the number of bytes in the prefix of `val` that do not need escaping.
// This is like utf8_range::SpanStructurallyValid(), except that it also
// terminates at any ASCII char that needs to be escaped in TextFormat (any char
// that has `DefinitelyNeedsEscape(ch) == true`).
//
// If we could get a variant of utf8_range::SpanStructurallyValid() that could
// terminate on any of these chars, that might be more efficient, but it would
// be much more complicated to modify that heavily SIMD code.
UPB_INLINE size_t UPB_PRIVATE(_SkipPassthroughBytes)(const char* ptr,
                                                     size_t size) {
  for (size_t i = 0; i < size; i++) {
    unsigned char uc = ptr[i];
    if (UPB_PRIVATE(_upb_DefinitelyNeedsEscape)(uc)) return i;
    if (UPB_PRIVATE(_upb_NeedsUtf8Validation)(uc)) {
      // Find the end of this region of consecutive high bytes, so that we only
      // give high bytes to the UTF-8 checker.  This avoids needing to perform
      // a second scan of the ASCII characters looking for characters that
      // need escaping.
      //
      // We assume that high bytes are less frequent than plain, printable ASCII
      // bytes, so we accept the double-scan of high bytes.
      size_t end = i + 1;
      for (; end < size; end++) {
        if (!UPB_PRIVATE(_upb_NeedsUtf8Validation)(ptr[end])) break;
      }
      size_t n = end - i;
      size_t ok = utf8_range_ValidPrefix(ptr + i, n);
      if (ok != n) return i + ok;
      i += ok - 1;
    }
  }
  return size;
}

UPB_INLINE void UPB_PRIVATE(_upb_HardenedPrintString)(txtenc* e,
                                                      const char* ptr,
                                                      size_t len) {
  // Print as UTF-8, while guarding against any invalid UTF-8 in the string
  // field.
  //
  // If in the future we have a guaranteed invariant that invalid UTF-8 will
  // never be present, we could avoid the UTF-8 check here.
  UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\"");
  const char* end = ptr + len;
  while (ptr < end) {
    size_t n = UPB_PRIVATE(_SkipPassthroughBytes)(ptr, end - ptr);
    if (n != 0) {
      UPB_PRIVATE(_upb_TextEncode_PutBytes)(e, ptr, n);
      ptr += n;
      if (ptr == end) break;
    }

    // If repeated calls to CEscape() and PrintString() are expensive, we could
    // consider batching them, at the cost of some complexity.
    UPB_PRIVATE(_upb_TextEncode_Escaped)(e, *ptr);
    ptr++;
  }
  UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\"");
}

UPB_INLINE void UPB_PRIVATE(_upb_TextEncode_Bytes)(txtenc* e,
                                                   upb_StringView data) {
  const char* ptr = data.data;
  const char* end = ptr + data.size;
  UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\"");
  for (; ptr < end; ptr++) {
    unsigned char uc = *ptr;
    if (UPB_PRIVATE(_upb_AsciiIsPrint)(uc)) {
      UPB_PRIVATE(_upb_TextEncode_PutBytes)(e, ptr, 1);
    } else {
      UPB_PRIVATE(_upb_TextEncode_Escaped)(e, uc);
    }
  }
  UPB_PRIVATE(_upb_TextEncode_PutStr)(e, "\"");
}

UPB_INLINE size_t UPB_PRIVATE(_upb_TextEncode_Nullz)(txtenc* e, size_t size) {
  size_t ret = e->ptr - e->buf + e->overflow;

  if (size > 0) {
    if (e->ptr == e->end) e->ptr--;
    *e->ptr = '\0';
  }

  return ret;
}

const char* UPB_PRIVATE(_upb_TextEncode_Unknown)(txtenc* e, const char* ptr,
                                                 upb_EpsCopyInputStream* stream,
                                                 int groupnum);

// Must not be called for ctype = kUpb_CType_Enum, as they require different
// handling depending on whether or not we're doing reflection-based encoding.
void UPB_PRIVATE(_upb_TextEncode_Scalar)(txtenc* e, upb_MessageValue val,
                                         upb_CType ctype);

#include "upb/port/undef.inc"

#endif  // UPB_TEXT_ENCODE_INTERNAL_H_
