// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef FILE_UTILS_H_
#define FILE_UTILS_H_

#include <dirent.h>
#include <sys/types.h>
#include <unistd.h>

#include <functional>
#include <map>
#include <memory>

#include "logging.h"

namespace file_utils {

// RAII classes for auto-releasing fd/dirs.
template <typename RESOURCE_TYPE, int (*CLOSE_FN)(RESOURCE_TYPE)>
struct ScopedResource {
  explicit ScopedResource(RESOURCE_TYPE r) : r_(r) { CHECK(r); }
  ~ScopedResource() { CLOSE_FN(r_); }
  RESOURCE_TYPE r_;
};

using ScopedFD = ScopedResource<int, close>;
using ScopedDir = ScopedResource<DIR*, closedir>;

// Invokes predicate(pid) for each folder in |proc_path|/[0-9]+ which has
// a numeric name (typically pids and tids).
void ForEachPidInProcPath(const char* proc_path,
                          std::function<void(int)> predicate);

// Reads the contents of |path| fully into |buf| up to |length| chars.
// |buf| is guaranteed to be null terminated.
ssize_t ReadFile(const char* path, char* buf, size_t length);

// Reads a single-line file, stripping out any \0, \r, \n and replacing
// non-printable charcters with '?'. |buf| is guaranteed to be null terminated.
bool ReadFileTrimmed(const char* path, char* buf, size_t length);

// Convenience wrappers for /proc/|pid|/|proc_file| paths.
ssize_t ReadProcFile(int pid, const char* proc_file, char* buf, size_t length);
bool ReadProcFileTrimmed(int pid,
                         const char* proc_file,
                         char* buf,
                         size_t length);

// Takes a C string buffer and chunks it into lines without creating any
// copies. It modifies the original buffer, by replacing \n with \0.
class LineReader {
 public:
  LineReader(char* buf, size_t size);
  ~LineReader();

  const char* NextLine();

 private:
  char* ptr_;
  char* end_;
};

}  // namespace file_utils

#endif  // FILE_UTILS_H_
