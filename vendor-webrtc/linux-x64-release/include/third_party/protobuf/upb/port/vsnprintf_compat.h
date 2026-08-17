// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_PORT_VSNPRINTF_COMPAT_H_
#define UPB_PORT_VSNPRINTF_COMPAT_H_

// Must be last.
#include "upb/port/def.inc"

UPB_INLINE int _upb_vsnprintf(char* buf, size_t size, const char* fmt,
                              va_list ap) {
#if defined(__MINGW64__) || defined(__MINGW32__) || defined(_MSC_VER)
  // The msvc runtime has a non-conforming vsnprintf() that requires the
  // following compatibility code to become conformant.
  int n = -1;
  if (size != 0) n = _vsnprintf_s(buf, size, _TRUNCATE, fmt, ap);
  if (n == -1) n = _vscprintf(fmt, ap);
  return n;
#else
  return vsnprintf(buf, size, fmt, ap);
#endif
}

#include "upb/port/undef.inc"

#endif  // UPB_PORT_VSNPRINTF_COMPAT_H_
