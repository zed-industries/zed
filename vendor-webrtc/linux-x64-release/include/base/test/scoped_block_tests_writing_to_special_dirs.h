// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_
#define BASE_TEST_SCOPED_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_

#include <vector>

#include "base/files/block_tests_writing_to_special_dirs.h"

namespace base {

// This is used by test harnesses to detect and prevent tests writing to
// special directories, with help from `BlockTestsWritingToSpecialDirs`.
class ScopedBlockTestsWritingToSpecialDirs {
 public:
  // `dirs_to_block` contains the PathService keys of the dirs to block.
  ScopedBlockTestsWritingToSpecialDirs(
      std::vector<int> dirs_to_block,
      FileWriteBlockedForTestingFunctionPtr failure_callback);

  ScopedBlockTestsWritingToSpecialDirs(
      const ScopedBlockTestsWritingToSpecialDirs&) = delete;
  ScopedBlockTestsWritingToSpecialDirs& operator=(
      const ScopedBlockTestsWritingToSpecialDirs&) = delete;

  ~ScopedBlockTestsWritingToSpecialDirs();
};

}  // namespace base

#endif  // BASE_TEST_SCOPED_BLOCK_TESTS_WRITING_TO_SPECIAL_DIRS_H_
