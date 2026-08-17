// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

#ifndef UTIL_STRING_UTIL_H_
#define UTIL_STRING_UTIL_H_

#include <string.h>

namespace base {

#if defined(_WIN32)
// Compare the two strings s1 and s2 without regard to case using
// the current locale; returns 0 if they are equal, 1 if s1 > s2, and -1 if
// s2 > s1 according to a lexicographic comparison.
inline int strcasecmp(const char* s1, const char* s2) {
  return _stricmp(s1, s2);
}
inline int strncasecmp(const char* s1, const char* s2, size_t n) {
  return _strnicmp(s1, s2, n);
}
#else
inline int strcasecmp(const char* s1, const char* s2) {
  return ::strcasecmp(s1, s2);
}
inline int strncasecmp(const char* s1, const char* s2, size_t n) {
  return ::strncasecmp(s1, s2, n);
}
#endif
}

#if !defined(__linux__)
inline void* memrchr(const void* s, int c, size_t n) {
  const unsigned char* p = (const unsigned char*) s;
  for (p += n; n > 0; n--) {
    if (*--p == c)
      return (void*) p;
  }
  return NULL;
}
#endif

#endif  // UTIL_STRING_UTIL_H_
