/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_BASE_TEST_TMP_DIR_TREE_H_
#define SRC_BASE_TEST_TMP_DIR_TREE_H_

#include <stack>
#include <string>

#include "perfetto/ext/base/temp_file.h"

namespace perfetto {
namespace base {

// Helper to construct and automatically destroy temporary file hierarchies in
// tests.
class TmpDirTree {
 public:
  TmpDirTree();
  virtual ~TmpDirTree();

  // Returns the absolute path where the temporary hierarchy is located.
  const std::string& path() const { return tmp_dir_.path(); }

  // Prepends `path()` to `relative_path` (making it an absolute path).
  std::string AbsolutePath(const std::string& relative_path) const;

  // Creates a directory at `relative_path`. All the parent directories should
  // have been created. PERFETTO_CHECK()s that the operation succeeds.
  void AddDir(const std::string& relative_path);

  // Creates a file at `relative_path` which contains `content`. All the parent
  // directories should have been created. PERFETTO_CHECK()s that the operation
  // succeeds.
  void AddFile(const std::string& relative_path, const std::string& content);

  // Tells this object to remove `relative_path` on destruction. This is used
  // for files created by the caller that still need to be removed on cleanup by
  // this object.
  void TrackFile(const std::string& relative_path);

 private:
  TmpDirTree(const TmpDirTree&) = delete;
  TmpDirTree& operator=(const TmpDirTree&) = delete;

  base::TempDir tmp_dir_;
  std::stack<std::string> dirs_to_remove_;
  std::stack<std::string> files_to_remove_;
};

}  // namespace base
}  // namespace perfetto

#endif  // SRC_BASE_TEST_TMP_DIR_TREE_H_
