// Copyright 2011 Google LLC
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

// Utility class for creating a temporary directory for unit tests
// that is deleted in the destructor.
#ifndef GOOGLE_BREAKPAD_COMMON_TESTS_AUTO_TEMPDIR
#define GOOGLE_BREAKPAD_COMMON_TESTS_AUTO_TEMPDIR

#include <dirent.h>
#include <sys/types.h>

#include <string>

#include "breakpad_googletest_includes.h"
#include "common/using_std_string.h"

#if !defined(__ANDROID__)
#define TEMPDIR "/tmp"
#else
#define TEMPDIR "/data/local/tmp"
#include "common/android/testing/mkdtemp.h"
#endif

namespace google_breakpad {

class AutoTempDir {
 public:
  AutoTempDir() {
    char temp_dir[] = TEMPDIR "/breakpad.XXXXXX";
    EXPECT_TRUE(mkdtemp(temp_dir) != nullptr);
    path_.assign(temp_dir);
  }

  ~AutoTempDir() {
    DeleteRecursively(path_);
  }

  const string& path() const {
    return path_;
  }

 private:
  void DeleteRecursively(const string& path) {
    // First remove any files in the dir
    DIR* dir = opendir(path.c_str());
    if (!dir)
      return;

    dirent* entry;
    while ((entry = readdir(dir)) != nullptr) {
      if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0)
        continue;
      string entry_path = path + "/" + entry->d_name;
      struct stat stats;
      EXPECT_TRUE(lstat(entry_path.c_str(), &stats) == 0);
      if (S_ISDIR(stats.st_mode))
        DeleteRecursively(entry_path);
      else
        EXPECT_TRUE(unlink(entry_path.c_str()) == 0);
    }
    EXPECT_TRUE(closedir(dir) == 0);
    EXPECT_TRUE(rmdir(path.c_str()) == 0);
  }

  // prevent copy construction and assignment
  AutoTempDir(const AutoTempDir&);
  AutoTempDir& operator=(const AutoTempDir&);

  string path_;
};

}  // namespace google_breakpad

#endif  // GOOGLE_BREAKPAD_COMMON_TESTS_AUTO_TEMPDIR
