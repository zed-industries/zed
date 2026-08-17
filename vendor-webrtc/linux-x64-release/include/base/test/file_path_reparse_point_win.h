// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_FILE_PATH_REPARSE_POINT_WIN_H_
#define BASE_TEST_FILE_PATH_REPARSE_POINT_WIN_H_

#include <optional>

#include "base/files/file_path.h"
#include "base/win/scoped_handle.h"

namespace base::test {

// Manages a reparse point for a test.
class FilePathReparsePoint {
 public:
  // Creates a reparse point from |source| (an empty directory) to |target|.
  static std::optional<FilePathReparsePoint> Create(const FilePath& source,
                                                    const FilePath& target);
  FilePathReparsePoint(const FilePathReparsePoint&) = delete;
  FilePathReparsePoint& operator=(const FilePathReparsePoint&) = delete;

  // Move operations.
  FilePathReparsePoint(FilePathReparsePoint&& other);
  FilePathReparsePoint& operator=(FilePathReparsePoint&& other);

  ~FilePathReparsePoint();

 private:
  FilePathReparsePoint(const FilePath& source, const FilePath& target);
  bool IsValid() const { return created_; }
  bool SetReparsePoint(HANDLE source, const FilePath& target_path);
  bool DeleteReparsePoint(HANDLE source);

  win::ScopedHandle dir_;
  bool created_;
};

}  // namespace base::test

#endif  // BASE_TEST_FILE_PATH_REPARSE_POINT_WIN_H_
