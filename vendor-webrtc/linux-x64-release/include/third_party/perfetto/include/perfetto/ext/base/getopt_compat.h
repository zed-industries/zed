/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_GETOPT_COMPAT_H_
#define INCLUDE_PERFETTO_EXT_BASE_GETOPT_COMPAT_H_

#include <cstddef>  // For std::nullptr_t

// No translation units other than base/getopt.h and getopt_compat_unittest.cc
// should directly include this file. Use base/getopt.h instead.

namespace perfetto {
namespace base {
namespace getopt_compat {

// A tiny getopt() replacement for Windows, which doesn't have <getopt.h>.
// This implementation is based on the subset of features that we use in the
// Perfetto codebase. It doesn't even try to deal with the full surface of GNU's
// getopt().
// Limitations:
// - getopt_long_only() is not supported.
// - optional_argument is not supported. That is extremely subtle and caused us
//   problems in the past with GNU's getopt.
// - It does not reorder non-option arguments. It behaves like MacOS getopt, or
//   GNU's when POSIXLY_CORRECT=1.
// - Doesn't expose optopt or opterr.
// - option.flag and longindex are not supported and must be nullptr.

enum {
  no_argument = 0,
  required_argument = 1,
};

struct option {
  const char* name;
  int has_arg;
  std::nullptr_t flag;  // Only nullptr is supported.
  int val;
};

extern char* optarg;
extern int optind;
extern int optopt;
extern int opterr;

int getopt_long(int argc,
                char** argv,
                const char* shortopts,
                const option* longopts,
                std::nullptr_t /*longindex is not supported*/);

int getopt(int argc, char** argv, const char* shortopts);

}  // namespace getopt_compat
}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_GETOPT_COMPAT_H_
