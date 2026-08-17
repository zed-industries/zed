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

// mkdtemp() wasn't declared in <stdlib.h> until NDK r9b due to a simple
// packaging bug (the function has always been implemented in all versions
// of the C library). This header is provided to build Breakpad with earlier
// NDK revisions (e.g. the one used by Chromium). It may be removed in the
// future once all major projects upgrade to use a more recent NDK.
//
// The reason this is inlined here is to avoid linking a new object file
// into each unit test program (i.e. keep build files simple).

#ifndef GOOGLE_BREAKPAD_COMMON_ANDROID_TESTING_MKDTEMP_H
#define GOOGLE_BREAKPAD_COMMON_ANDROID_TESTING_MKDTEMP_H

#include <assert.h>
#include <errno.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>

// Using a macro renaming trick here is necessary when building against
// NDK r9b. Otherwise the compiler will complain that calls to mkdtemp()
// are ambiguous.
#define mkdtemp breakpad_mkdtemp

namespace {

char* breakpad_mkdtemp(char* path) {
  if (path == nullptr) {
    errno = EINVAL;
    return nullptr;
  }

  // 'path' must be terminated with six 'X'
  const char kSuffix[] = "XXXXXX";
  const size_t kSuffixLen = strlen(kSuffix);
  char* path_end = path + strlen(path);

  if (static_cast<size_t>(path_end - path) < kSuffixLen ||
      memcmp(path_end - kSuffixLen, kSuffix, kSuffixLen) != 0) {
    errno = EINVAL;
    return nullptr;
  }

  // If 'path' contains a directory separator, check that it exists to
  // avoid looping later.
  char* sep = strrchr(path, '/');
  if (sep != nullptr) {
    struct stat st;
    int ret;
    *sep = '\0';  // temporarily zero-terminate the dirname.
    ret = stat(path, &st);
    *sep = '/';   // restore full path.
    if (ret < 0)
      return nullptr;
    if (!S_ISDIR(st.st_mode)) {
      errno = ENOTDIR;
      return nullptr;
    }
  }

  // Loop. On each iteration, replace the XXXXXX suffix with a random
  // number.
  int tries;
  for (tries = 128; tries > 0; tries--) {
    int random = rand() % 1000000;

    snprintf(path_end - kSuffixLen, kSuffixLen + 1, "%0d", random);
    if (mkdir(path, 0700) == 0)
      return path;  // Success

    if (errno != EEXIST)
      return nullptr;
  }

  assert(errno == EEXIST);
  return nullptr;
}

}  // namespace

#endif  // GOOGLE_BREAKPAD_COMMON_ANDROID_TESTING_MKDTEMP_H
