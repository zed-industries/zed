#ifndef FUZZTEST_COMMON_TEMP_DIR_H_
#define FUZZTEST_COMMON_TEMP_DIR_H_

#include <filesystem>  // NOLINT

#include "absl/strings/string_view.h"

namespace fuzztest::internal {

// A helper class for creating a temporary directory. Removes the directory
// when it goes out of scope.
class TempDir {
 public:
  explicit TempDir(absl::string_view custom_prefix = "");
  ~TempDir();

  TempDir(const TempDir& other) = delete;
  TempDir& operator=(const TempDir& other) = delete;

  const std::filesystem::path& path() const { return path_; }

 private:
  std::filesystem::path path_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_TEMP_DIR_H_
