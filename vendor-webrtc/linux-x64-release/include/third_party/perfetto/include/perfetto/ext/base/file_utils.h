/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_FILE_UTILS_H_
#define INCLUDE_PERFETTO_EXT_BASE_FILE_UTILS_H_

#include <fcntl.h>  // For mode_t & O_RDONLY/RDWR. Exists also on Windows.
#include <stddef.h>

#include <optional>
#include <string>
#include <vector>

#include "perfetto/base/build_config.h"
#include "perfetto/base/export.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/utils.h"

namespace perfetto {
namespace base {

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
using FileOpenMode = int;
inline constexpr char kDevNull[] = "NUL";
#else
using FileOpenMode = mode_t;
inline constexpr char kDevNull[] = "/dev/null";
#endif

constexpr FileOpenMode kFileModeInvalid = static_cast<FileOpenMode>(-1);

bool ReadPlatformHandle(PlatformHandle, std::string* out);
bool ReadFileDescriptor(int fd, std::string* out);
bool ReadFileStream(FILE* f, std::string* out);
bool ReadFile(const std::string& path, std::string* out);

// A wrapper around read(2). It deals with Linux vs Windows includes. It also
// deals with handling EINTR. Has the same semantics of UNIX's read(2).
ssize_t Read(int fd, void* dst, size_t dst_size);

// Call write until all data is written or an error is detected.
//
// man 2 write:
//   If a write() is interrupted by a signal handler before any bytes are
//   written, then the call fails with the error EINTR; if it is
//   interrupted after at least one byte has been written, the call
//   succeeds, and returns the number of bytes written.
ssize_t WriteAll(int fd, const void* buf, size_t count);

ssize_t WriteAllHandle(PlatformHandle, const void* buf, size_t count);

ScopedFile OpenFile(const std::string& path,
                    int flags,
                    FileOpenMode = kFileModeInvalid);
ScopedFstream OpenFstream(const char* path, const char* mode);

// This is an alias for close(). It's to avoid leaking Windows.h in headers.
// Exported because ScopedFile is used in the /include/ext API by Chromium
// component builds.
int PERFETTO_EXPORT_COMPONENT CloseFile(int fd);

bool FlushFile(int fd);

// Returns true if mkdir succeeds, false if it fails (see errno in that case).
bool Mkdir(const std::string& path);

// Calls rmdir() on UNIX, _rmdir() on Windows.
bool Rmdir(const std::string& path);

// Wrapper around access(path, F_OK).
bool FileExists(const std::string& path);

// Gets the extension for a filename. If the file has two extensions, returns
// only the last one (foo.pb.gz => .gz). Returns empty string if there is no
// extension.
std::string GetFileExtension(const std::string& filename);

// Puts the path to all files under |dir_path| in |output|, recursively walking
// subdirectories. File paths are relative to |dir_path|. Only files are
// included, not directories. Path separator is always '/', even on windows (not
// '\').
base::Status ListFilesRecursive(const std::string& dir_path,
                                std::vector<std::string>& output);

// Sets |path|'s owner group to |group_name| and permission mode bits to
// |mode_bits|.
base::Status SetFilePermissions(const std::string& path,
                                const std::string& group_name,
                                const std::string& mode_bits);

// Returns the size of the file located at |path|, or nullopt in case of error.
std::optional<uint64_t> GetFileSize(const std::string& path);

// Returns the size of the open file |fd|, or nullopt in case of error.
std::optional<uint64_t> GetFileSize(PlatformHandle fd);

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_FILE_UTILS_H_
