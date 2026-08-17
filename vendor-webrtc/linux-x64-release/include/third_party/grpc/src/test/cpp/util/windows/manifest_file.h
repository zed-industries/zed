// Copyright 2023 The gRPC Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_TEST_CPP_UTIL_WINDOWS_MANIFEST_FILE_H
#define GRPC_TEST_CPP_UTIL_WINDOWS_MANIFEST_FILE_H

#include <grpc/support/port_platform.h>

#ifdef GPR_WINDOWS

#include <fstream>
#include <string>
#include <unordered_map>

namespace grpc {
namespace testing {

std::string NormalizeFilePath(const std::string& filepath);

// This class is used for handling Runfiles for a Bazel target on Windows (e.g.
// the output of targets specified in the data attribute of the target). On
// Linux/macOS, Bazel creates a runfiles tree which contains symlinks to the
// actual runfiles. But on Windows, it only creates a MANIFEST file which
// contains a list of <symlink relative path, absolute symlink target path>.
// Thus one initializes a ManifestFile object with the filepath to a MANIFEST
// file and uses it as a key-value datastore by querying the absolute symlink
// target path with the imaginative symlink relative path. See
// https://github.com/bazelbuild/bazel/issues/4261#issuecomment-350723457 for
// more details.
class ManifestFile {
 public:
  explicit ManifestFile(const std::string& filepath);

  std::string Get(const std::string& key);

 private:
  std::fstream filestream_;
  std::unordered_map<std::string, std::string> cache_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GPR_WINDOWS

#endif  // GRPC_TEST_CPP_UTIL_WINDOWS_MANIFEST_FILE_H
