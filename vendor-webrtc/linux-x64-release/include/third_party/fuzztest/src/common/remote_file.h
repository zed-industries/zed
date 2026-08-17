// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// A very simple abstract API for working with potentially remote files.
// The implementation may use any file API, including the plain C FILE API, C++
// streams, or an actual API for dealing with remote files. The abstractions are
// the same as in the C FILE API.

#ifndef FUZZTEST_COMMON_REMOTE_FILE_H_
#define FUZZTEST_COMMON_REMOTE_FILE_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "./common/defs.h"
#ifndef CENTIPEDE_DISABLE_RIEGELI
#include "riegeli/bytes/reader.h"
#include "riegeli/bytes/writer.h"
#endif  // CENTIPEDE_DISABLE_RIEGELI

#if defined(__APPLE__)
#if (defined(__MAC_OS_X_VERSION_MIN_REQUIRED) &&       \
     __MAC_OS_X_VERSION_MIN_REQUIRED < __MAC_10_15) || \
    (defined(__IPHONE_OS_VERSION_MIN_REQUIRED) &&      \
     __IPHONE_OS_VERSION_MIN_REQUIRED < __IPHONE_13_0)
// std::filesystem requires macOS 10.15+ or iOS 13+.
// Use this macro to stub out code that depends on std::filesystem.
#define FUZZTEST_STUB_STD_FILESYSTEM
#endif
#endif

namespace fuzztest::internal {

// An opaque file handle.
struct RemoteFile {};

// Opens a (potentially remote) file 'file_path' and returns a handle to it.
// Supported modes: "r", "a", "w", same as in C FILE API.
absl::StatusOr<RemoteFile *> RemoteFileOpen(std::string_view file_path,
                                            const char *mode);

// Closes the file previously opened by RemoteFileOpen.
absl::Status RemoteFileClose(RemoteFile *absl_nonnull f);

// Adjusts the buffered I/O capacity for a file opened for writing. By default,
// the internal buffer of size `BUFSIZ` is used. May only be used after opening
// a file, but before performing any other operations on it. Violating this
// requirement in general can cause undefined behavior.
absl::Status RemoteFileSetWriteBufferSize(RemoteFile *absl_nonnull f,
                                          size_t size);

// Appends bytes from 'ba' to 'f'.
absl::Status RemoteFileAppend(RemoteFile *absl_nonnull f, const ByteArray &ba);

// Appends characters from 'contents' to 'f'.
absl::Status RemoteFileAppend(RemoteFile *absl_nonnull f,
                              const std::string &contents);

// Flushes the file's internal buffer. Some dynamic results of a running
// pipeline are consumed by itself (e.g. shard cross-pollination) and can be
// consumed by external processes (e.g. monitoring): for such files, call this
// API after every write to ensure that they are in a valid state.
absl::Status RemoteFileFlush(RemoteFile *absl_nonnull f);

// Reads all current contents of 'f' into 'ba'.
absl::Status RemoteFileRead(RemoteFile *absl_nonnull f, ByteArray &ba);

// Reads all current contents of 'f' into 'contents'.
absl::Status RemoteFileRead(RemoteFile *absl_nonnull f, std::string &contents);

// Creates a (potentially remote) directory 'dir_path', as well as any missing
// parent directories. No-op if the directory already exists.
absl::Status RemoteMkdir(std::string_view dir_path);

// Sets the contents of the file at 'path' to 'contents'.
absl::Status RemoteFileSetContents(std::string_view path,
                                   const ByteArray &contents);

// Sets the contents of the file at 'path' to 'contents'.
absl::Status RemoteFileSetContents(std::string_view path,
                                   const std::string &contents);

// Reads the contents of the file at 'path' into 'contents'.
absl::Status RemoteFileGetContents(std::string_view path, ByteArray &contents);

// Reads the contents of the file at 'path' into 'contents'.
absl::Status RemoteFileGetContents(std::string_view path,
                                   std::string &contents);

// Returns true if `path` exists.
bool RemotePathExists(std::string_view path);

// Returns true if `path` is a directory.
bool RemotePathIsDirectory(std::string_view path);

// Returns the size of the file at `path` in bytes. The file must exist.
absl::StatusOr<int64_t> RemoteFileGetSize(std::string_view path);

// Finds all files matching `glob` and appends them to `matches`.
//
// Returns Ok when matches were found, or NotFound when no matches were found.
// Otherwise returns a non-NotFound error.
//
// Note: The OSS implementation of this function fails on Windows, Android, and
// Fuchsia. Instead of using this function, consider whether your use case can
// be solved in a more specific way, e.g., by listing files in a directory and
// filtering based on some criterion.
[[deprecated("Do not use in new code.")]]
absl::Status RemoteGlobMatch(std::string_view glob,
                             std::vector<std::string> &matches);

// Lists all files within `path`, recursively expanding subdirectories if
// `recursively` is true. Does not return any directories. Returns an empty
// vector if `path` is an empty directory, or `path` does not exist. Returns
// `{path}` if `path` is a non-directory.
absl::StatusOr<std::vector<std::string>> RemoteListFiles(std::string_view path,
                                                         bool recursively);

// Renames a file from `from` to `to`.
absl::Status RemoteFileRename(std::string_view from, std::string_view to);

// Updates the last-modified time of `path` to the current time.
absl::Status RemotePathTouchExistingFile(std::string_view path);

// Deletes `path`. If `path` is a directory and `recursively` is true,
// recursively deletes all files and subdirectories within `path`.
absl::Status RemotePathDelete(std::string_view path, bool recursively);

#ifndef CENTIPEDE_DISABLE_RIEGELI
// Returns a reader for the file at `file_path`.
absl::StatusOr<std::unique_ptr<riegeli::Reader>> CreateRiegeliFileReader(
    std::string_view file_path);

// Returns a writer for the file at `file_path`.
// If `append` is `true`, writes will append to the end of the file if it
// exists. If `false, the file will be truncated to empty if it exists.
absl::StatusOr<std::unique_ptr<riegeli::Writer>> CreateRiegeliFileWriter(
    std::string_view file_path, bool append);
#endif  // CENTIPEDE_DISABLE_RIEGELI

}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_REMOTE_FILE_H_
