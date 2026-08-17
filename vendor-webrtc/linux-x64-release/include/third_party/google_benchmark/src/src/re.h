// Copyright 2015 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BENCHMARK_RE_H_
#define BENCHMARK_RE_H_

#include "internal_macros.h"

// clang-format off

#if !defined(HAVE_STD_REGEX) && \
    !defined(HAVE_GNU_POSIX_REGEX) && \
    !defined(HAVE_POSIX_REGEX)
  // No explicit regex selection; detect based on builtin hints.
  #if defined(BENCHMARK_OS_LINUX) || defined(BENCHMARK_OS_APPLE)
    #define HAVE_POSIX_REGEX 1
  #elif __cplusplus >= 199711L
    #define HAVE_STD_REGEX 1
  #endif
#endif

// Prefer C regex libraries when compiling w/o exceptions so that we can
// correctly report errors.
#if defined(BENCHMARK_HAS_NO_EXCEPTIONS) && \
    defined(HAVE_STD_REGEX) && \
    (defined(HAVE_GNU_POSIX_REGEX) || defined(HAVE_POSIX_REGEX))
  #undef HAVE_STD_REGEX
#endif

#if defined(HAVE_STD_REGEX)
  #include <regex>
#elif defined(HAVE_GNU_POSIX_REGEX)
  #include <gnuregex.h>
#elif defined(HAVE_POSIX_REGEX)
  #include <regex.h>
#else
#error No regular expression backend was found!
#endif

// clang-format on

#include <string>

#include "check.h"

namespace benchmark {

// A wrapper around the POSIX regular expression API that provides automatic
// cleanup
class Regex {
 public:
  Regex() : init_(false) {}

  ~Regex();

  // Compile a regular expression matcher from spec.  Returns true on success.
  //
  // On failure (and if error is not nullptr), error is populated with a human
  // readable error message if an error occurs.
  bool Init(const std::string& spec, std::string* error);

  // Returns whether str matches the compiled regular expression.
  bool Match(const std::string& str);

 private:
  bool init_;
// Underlying regular expression object
#if defined(HAVE_STD_REGEX)
  std::regex re_;
#elif defined(HAVE_POSIX_REGEX) || defined(HAVE_GNU_POSIX_REGEX)
  regex_t re_;
#else
#error No regular expression backend implementation available
#endif
};

#if defined(HAVE_STD_REGEX)

inline bool Regex::Init(const std::string& spec, std::string* error) {
#ifdef BENCHMARK_HAS_NO_EXCEPTIONS
  ((void)error);  // suppress unused warning
#else
  try {
#endif
  re_ = std::regex(spec, std::regex_constants::extended);
  init_ = true;
#ifndef BENCHMARK_HAS_NO_EXCEPTIONS
}
catch (const std::regex_error& e) {
  if (error) {
    *error = e.what();
  }
}
#endif
return init_;
}

inline Regex::~Regex() {}

inline bool Regex::Match(const std::string& str) {
  if (!init_) {
    return false;
  }
  return std::regex_search(str, re_);
}

#else
inline bool Regex::Init(const std::string& spec, std::string* error) {
  int ec = regcomp(&re_, spec.c_str(), REG_EXTENDED | REG_NOSUB);
  if (ec != 0) {
    if (error) {
      size_t needed = regerror(ec, &re_, nullptr, 0);
      char* errbuf = new char[needed];
      regerror(ec, &re_, errbuf, needed);

      // regerror returns the number of bytes necessary to null terminate
      // the string, so we move that when assigning to error.
      BM_CHECK_NE(needed, 0);
      error->assign(errbuf, needed - 1);

      delete[] errbuf;
    }

    return false;
  }

  init_ = true;
  return true;
}

inline Regex::~Regex() {
  if (init_) {
    regfree(&re_);
  }
}

inline bool Regex::Match(const std::string& str) {
  if (!init_) {
    return false;
  }
  return regexec(&re_, str.c_str(), 0, nullptr, 0) == 0;
}
#endif

}  // end namespace benchmark

#endif  // BENCHMARK_RE_H_
