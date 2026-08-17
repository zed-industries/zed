// Copyright 2016 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_IJAR_PLATFORM_UTILS_H_
#define THIRD_PARTY_IJAR_PLATFORM_UTILS_H_

#include <stdlib.h>

#include <sys/stat.h>
#include <sys/types.h>
#include <string>

#include "third_party/ijar/common.h"

namespace devtools_ijar {

// Platform-independent stat data.
struct Stat {
  // Total size of the file in bytes.
  int total_size;
  // The Unix file mode from the stat.st_mode field.
  mode_t file_mode;
  // True if this is a directory.
  bool is_directory;
};

// Converts a Stat object to ZIP attributes.
inline u4 stat_to_zipattr(const Stat& file_stat) {
  return (((u4)file_stat.file_mode) << 16) |
         (file_stat.is_directory != 0 ? 0x10 : 0);
}

// Writes stat data into `result` about the file under `path`.
// Returns true if file is found and can be stat'ed.
// Returns false if the file is not found or cannot be stat'ed.
// Doesn't report any errors because it can also be used to simply check if a
// file exists.
bool stat_file(const char* path, Stat* result);

// Writes `size` bytes from `data` into file under `path`.
// The file is created or overwritten and is set to have `perm` permissions.
// Returns true upon success: file is created and all data is written.
// Returns false upon failure and reports the error to stderr.
bool write_file(const char* path, unsigned int perm, const void* data,
                size_t size);

// Reads at most `size` bytes into `buffer` from the file under `path`.
// Returns true upon success: file is opened and all data is read.
// Returns false upon failure and reports the error to stderr.
bool read_file(const char* path, void* buffer, size_t size);

// Returns the current working directory.
// Returns the empty string upon failure and reports the error to stderr.
std::string get_cwd();

// Do a recursive mkdir of all folders of path except the last path
// segment (if path ends with a / then the last path segment is empty).
// All folders are created using "perm" for creation mode, and are writable and
// openable by the current user.
// Returns true if all directories were created and permissions set.
// Returns false upon failure and reports the error to stderr.
bool make_dirs(const char* path, unsigned int perm);

}  // namespace devtools_ijar

#endif  // THIRD_PARTY_IJAR_PLATFORM_UTILS_H_
