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

#ifndef UTIL_CASE_INSENSITIVE_HASH_H_
#define UTIL_CASE_INSENSITIVE_HASH_H_

#include <ctype.h>
#include <stddef.h>
#ifndef _MSC_VER
#include <strings.h>
#endif

#include <string>

#include "util/basictypes.h"
#include "util/string_util.h"

// Functors for hashing c-strings with case-insensitive semantics.
struct CStringCaseHash {
  size_t operator()(const char *str) const {
    unsigned long hash_val = 0;
    while (*str) {
      hash_val = 5*hash_val + tolower(*str);
      str++;
    }
    return (size_t)hash_val;
  }
};

struct CStringCaseEqual {
  bool operator()(const char *str1, const char *str2) const {
    return !base::strcasecmp(str1, str2);
  }
};

// These functors, in addition to being case-insensitive, ignore all
// non-alphanumeric characters.  This is useful when we want all variants of
// a string -- where variants can differ in puncutation and whitespace -- to
// map to the same value.
struct CStringAlnumCaseHash {
  size_t operator()(const char *str) const {
    unsigned long hash_val = 0;
    while (*str) {
      if (isalnum(*str)) {
        hash_val = 5*hash_val + tolower(*str);
      }
      str++;
    }
    return (size_t)hash_val;
  }
};

struct CStringAlnumCaseEqual {
  bool operator()(const char *str1, const char *str2) const {
    while (true) {
      // Skip until each pointer is pointing to an alphanumeric char or '\0'
      while (!isalnum(*str1) && (*str1 != '\0')) {
        str1++;
      }
      while (!isalnum(*str2) && (*str2 != '\0')) {
        str2++;
      }
      if (tolower(*str1) != tolower(*str2)) {
        return false;       // mismatch on alphanumeric char or '\0'
      }
      if (*str1 == '\0') {  // in which case *str2 must be '\0' as well
        return true;        // reached '\0' in both strings without mismatch
      }
      str1++;
      str2++;
    }
  }
};

#endif  // UTIL_CASE_INSENSITIVE_HASH_H_
