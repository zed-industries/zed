// Copyright 2023 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_TEST_FILE_UTIL_H
#define OPENSSL_HEADER_CRYPTO_TEST_FILE_UTIL_H

#include <stdio.h>

#include <memory>
#include <set>
#include <string>
#include <string_view>
#include <utility>

#include <openssl/span.h>

#if defined(OPENSSL_WINDOWS)
#include <io.h>
#else
#include <unistd.h>
#endif


BSSL_NAMESPACE_BEGIN

struct FileDeleter {
  void operator()(FILE *f) const {
    if (f != nullptr) {
      fclose(f);
    }
  }
};

using ScopedFILE = std::unique_ptr<FILE, FileDeleter>;

class ScopedFD {
 public:
  ScopedFD() = default;
  explicit ScopedFD(int fd) : fd_(fd) {}
  ~ScopedFD() { reset(); }

  ScopedFD(ScopedFD &&other) { *this = std::move(other); }
  ScopedFD &operator=(ScopedFD other) {
    reset(other.release());
    return *this;
  }

  bool is_valid() const { return fd_ >= 0; }
  int get() const { return fd_; }

  int release() { return std::exchange(fd_, -1); }
  void reset(int fd = -1) {
    if (is_valid()) {
#if defined(OPENSSL_WINDOWS)
      _close(fd_);
#else
      close(fd_);
#endif
    }
    fd_ = fd;
  }

 private:
  int fd_ = -1;
};

// SkipTempFileTests returns true and prints a warning if tests involving
// temporary files should be skipped because of platform issues.
bool SkipTempFileTests();

// TemporaryFile manages a temporary file for testing.
class TemporaryFile {
 public:
  TemporaryFile() = default;
  ~TemporaryFile();

  TemporaryFile(TemporaryFile &other) { *this = std::move(other); }
  TemporaryFile& operator=(TemporaryFile&&other) {
    // Ensure |path_| is empty so it doesn't try to delete the File.
    path_ = std::exchange(other.path_, {});
    return *this;
  }

  // Init initializes the temporary file with the specified content. It returns
  // true on success and false on error. On error, callers should call
  // |IgnoreTempFileErrors| to determine whether to ignore the error.
  bool Init(bssl::Span<const uint8_t> content = {});
  bool Init(std::string_view content) {
    return Init(bssl::StringAsBytes(content));
  }

  // Open opens the file as a |FILE| with the specified mode.
  ScopedFILE Open(const char *mode) const;

  // Open opens the file as a file descriptor with the specified flags.
  ScopedFD OpenFD(int flags) const;

  // path returns the path to the temporary file.
  const std::string &path() const { return path_; }

 private:
  std::string path_;
};

// TemporaryDirectory manages a temporary directory for testing.
class TemporaryDirectory {
 public:
  TemporaryDirectory() = default;
  ~TemporaryDirectory();

  TemporaryDirectory(TemporaryDirectory &other) { *this = std::move(other); }
  TemporaryDirectory& operator=(TemporaryDirectory&&other) {
    // Ensure |other_| is empty so it doesn't try to delete the directory.
    path_ = std::exchange(other.path_, {});
    files_ = std::exchange(other.files_, {});
    return *this;
  }

  // Init initializes the temporary directory. It returns true on success and
  // false on error. On error, callers should call |IgnoreTempFileErrors| to
  // determine whether to ignore the error.
  bool Init();

  // path returns the path to the temporary directory.
  const std::string &path() const { return path_; }

  // AddFile adds a file to the temporary directory with the specified content.
  // It returns true on success and false on error. Subdirectories in the
  // temporary directory are not currently supported.
  bool AddFile(const std::string &filename, bssl::Span<const uint8_t> content);
  bool AddFile(const std::string &filename, std::string_view content) {
    return AddFile(filename, bssl::StringAsBytes(content));
  }

  // GetFilePath returns the path to the specified file within the temporary
  // directory.
  std::string GetFilePath(const std::string &filename) {
#if defined(OPENSSL_WINDOWS)
    return path_ + '\\' + filename;
#else
    return path_ + '/' + filename;
#endif
  }

 private:
  std::string path_;
  std::set<std::string> files_;
};

BSSL_NAMESPACE_END

#endif  // OPENSSL_HEADER_CRYPTO_TEST_FILE_UTIL_H
