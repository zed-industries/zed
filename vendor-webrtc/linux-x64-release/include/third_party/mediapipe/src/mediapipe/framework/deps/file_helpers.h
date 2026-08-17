// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_DEPS_FILE_HELPERS_H_
#define MEDIAPIPE_FRAMEWORK_DEPS_FILE_HELPERS_H_

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "mediapipe/framework/deps/mmapped_file.h"

namespace mediapipe {
namespace file {
absl::Status GetContents(absl::string_view file_name, std::string* output,
                         bool read_as_binary = true);

absl::Status SetContents(absl::string_view file_name,
                         absl::string_view content);

absl::Status AppendStringToFile(absl::string_view file_name,
                                absl::string_view contents);

absl::StatusOr<std::unique_ptr<MemoryMappedFile>> MMapFile(
    absl::string_view path);

absl::Status LockMemory(const void* base_address, size_t length);
absl::Status UnlockMemory(const void* base_address, size_t length);

absl::Status MatchInTopSubdirectories(const std::string& parent_directory,
                                      const std::string& file_name,
                                      std::vector<std::string>* results);

absl::Status MatchFileTypeInDirectory(const std::string& directory,
                                      const std::string& file_suffix,
                                      std::vector<std::string>* results);

absl::Status Exists(absl::string_view file_name);

absl::Status IsDirectory(absl::string_view file_name);

absl::Status RecursivelyCreateDir(absl::string_view path);

}  // namespace file
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_DEPS_FILE_HELPERS_H_
