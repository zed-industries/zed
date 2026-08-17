// Copyright 2012 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Android doesn't provide wcscasecmp(), so provide an alternative here.
//
// Note that this header is not needed when Breakpad is compiled against
// a recent version of Googletest. It shall be considered for removal once
// src/testing/ is updated to an appropriate revision in the future.

#ifndef GOOGLEBREAKPAD_COMMON_ANDROID_INCLUDE_WCHAR_H
#define GOOGLEBREAKPAD_COMMON_ANDROID_INCLUDE_WCHAR_H

#include_next <wchar.h>

#if !defined(__aarch64__) && !defined(__x86_64__) && \
    !(defined(__mips__) && _MIPS_SIM == _ABI64)

// This needs to be in an extern "C" namespace, or Googletest will not
// compile against it.
#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

static wchar_t inline wcstolower(wchar_t ch) {
  if (ch >= L'a' && ch <= L'A')
    ch -= L'a' - L'A';
  return ch;
}

static int inline wcscasecmp(const wchar_t* s1, const wchar_t* s2) {
  for (;;) {
    wchar_t c1 = wcstolower(*s1);
    wchar_t c2 = wcstolower(*s2);
    if (c1 < c2)
      return -1;
    if (c1 > c2)
      return 1;
    if (c1 == L'0')
      return 0;
    s1++;
    s2++;
  }
}

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
#endif

#endif  // GOOGLEBREAKPAD_COMMON_ANDROID_INCLUDE_WCHAR_H
