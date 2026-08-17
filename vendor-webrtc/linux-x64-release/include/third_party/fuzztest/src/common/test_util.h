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

#ifndef FUZZTEST_COMMON_TEST_UTIL_H_
#define FUZZTEST_COMMON_TEST_UTIL_H_

#include <cstddef>
#include <filesystem>  // NOLINT
#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "absl/log/check.h"
#include "absl/strings/str_format.h"
#include "./common/blob_file.h"
#include "./common/defs.h"

#include "./common/logging.h"

#define EXPECT_OK(status) EXPECT_TRUE((status).ok()) << VV(status)
#define ASSERT_OK(status) ASSERT_TRUE((status).ok()) << VV(status)

namespace fuzztest::internal {

// Returns a temp dir for use inside tests. The base dir is chosen in the
// following order of precedence:
// - $TEST_TMPDIR (highest)
// - $TMPDIR
// - /tmp
//
// An optional `subdir` can be appended to the base dir chosen as above. One
// useful value always available inside a TEST macro (and its variations) is
// `test_into_->name()`, which returns the name of the test case.
//
// If the final dir doesn't exist, it gets created.
std::filesystem::path GetTestTempDir(std::string_view subdir);

// Returns a path for i-th temporary file.
std::string GetTempFilePath(std::string_view subdir, size_t i);

// Returns the root directory filepath for a test's "runfiles".
std::filesystem::path GetTestRunfilesDir();

// Returns the filepath of a test's data dependency file.
std::filesystem::path GetDataDependencyFilepath(std::string_view rel_path);

// Returns a path to `llvm-symbolizer` binary.
std::string GetLLVMSymbolizerPath();

// Returns a path to `objdump` binary.
std::string GetObjDumpPath();

// Resets the PATH envvar to "`dir`:$PATH".
void PrependDirToPathEnvvar(std::string_view dir);

// Creates or clears a tmp dir in CTOR. The dir will end with `leaf` subdir.
//
// TODO(b/393384208): Merge this with TempDir in temp_dir.h.
class TempDir {
 public:
  explicit TempDir(std::string_view leaf1, std::string_view leaf2 = "")
      : path_{GetTestTempDir(leaf1) / leaf2} {
    std::filesystem::remove_all(path_);
    std::filesystem::create_directories(path_);
  }

  const std::filesystem::path& path() const { return path_; }

  std::string GetFilePath(std::string_view file_name) const {
    return path_ / file_name;
  }

  std::string CreateSubdir(std::string_view name) const {
    std::string path = GetFilePath(name);
    std::filesystem::remove_all(path);
    std::filesystem::create_directories(path);
    return path;
  }

 private:
  std::filesystem::path path_;
};

class TempCorpusDir : public TempDir {
 public:
  // Reuse the parent's ctor.
  using TempDir::TempDir;

  // Loads the corpus from the file `name_prefix``shard_index`
  // and returns it as a vector<ByteArray>.
  // Returns an empty vector if the file cannot be opened.
  std::vector<ByteArray> GetCorpus(size_t shard_index,
                                   std::string_view name_prefix = "corpus.") {
    // NOTE: The "6" in the "%06d" comes from kDigitsInShardIndex in
    // environment.cc.
    if (!reader_
             ->Open(GetFilePath(
                 absl::StrFormat("%s%06d", name_prefix, shard_index)))
             .ok()) {
      return {};
    }
    std::vector<ByteArray> corpus;
    ByteSpan blob;
    while (reader_->Read(blob).ok()) {
      corpus.emplace_back(blob.begin(), blob.end());
    }
    CHECK_OK(reader_->Close());
    return corpus;
  }

  // Returns the count of elements in the corpus file `path`/`file_name`.
  size_t CountElementsInCorpusFile(size_t shard_index,
                                   std::string_view name_prefix = "corpus.") {
    return GetCorpus(shard_index, name_prefix).size();
  }

 private:
  std::unique_ptr<BlobFileReader> reader_ = DefaultBlobFileReaderFactory();
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_TEST_UTIL_H_
