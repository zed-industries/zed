// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_
#define BASE_FILES_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_

#include <optional>
#include <set>
#include <vector>

#include "base/base_export.h"
#include "base/gtest_prod_util.h"

namespace base {

class FilePath;

using FileWriteBlockedForTestingFunctionPtr = void (*)(const FilePath&);

// Utility class for production code to check if writing to special directories
// is blocked for tests.
class BASE_EXPORT BlockTestsWritingToSpecialDirs {
 public:
  static bool CanWriteToPath(const FilePath& path);

  BlockTestsWritingToSpecialDirs(
      std::vector<int> blocked_dirs,
      FileWriteBlockedForTestingFunctionPtr failure_callback);
  BlockTestsWritingToSpecialDirs(
      const BlockTestsWritingToSpecialDirs& blocker) = delete;
  BlockTestsWritingToSpecialDirs& operator=(
      const BlockTestsWritingToSpecialDirs&) = delete;

  ~BlockTestsWritingToSpecialDirs();

 private:
  friend class BlockTestsWritingToSpecialDirsTest;
  friend class ScopedBlockTestsWritingToSpecialDirs;

  // This private method is used by `ScopedBlockTestsWritingToSpecialDirs` to
  // create an object of this class stored in a function static object.
  // `CanWriteToPath` above checks the paths stored in that object, if it is
  // set. Thus, only ScopedBlockTestsWritingToSpecialDirs should be able to
  // block tests writing to special dirs.
  static std::optional<BlockTestsWritingToSpecialDirs>& Get();

  // `blocked_paths_` will be initialized lazily, from `blocked_dirs_`.
  std::set<FilePath> blocked_paths_;
  std::vector<int> blocked_dirs_;
  FileWriteBlockedForTestingFunctionPtr failure_callback_ = nullptr;
};

}  // namespace base

#endif  // BASE_FILES_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_
