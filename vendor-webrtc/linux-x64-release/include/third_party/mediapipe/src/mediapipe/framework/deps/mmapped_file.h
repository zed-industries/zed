// Copyright 2024 The MediaPipe Authors.
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

#include <cstddef>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "mediapipe/framework/port/logging.h"

#ifndef MEDIAPIPE_FRAMEWORK_DEPS_MMAPPED_FILE_H_
#define MEDIAPIPE_FRAMEWORK_DEPS_MMAPPED_FILE_H_
namespace mediapipe {
namespace file {
class MemoryMappedFile {
 public:
  MemoryMappedFile(std::string path, const void* base_address, size_t length)
      : path_(std::move(path)), base_address_(base_address), length_(length) {}

  virtual absl::Status Close() = 0;

  virtual ~MemoryMappedFile() = default;

  const std::string& Path() const { return path_; }
  const void* BaseAddress() const { return base_address_; }
  size_t Length() const { return length_; }

 private:
  std::string path_;
  const void* base_address_;
  size_t length_;
};
}  // namespace file
}  // namespace mediapipe
#endif  // MEDIAPIPE_FRAMEWORK_DEPS_MMAPPED_FILE_H_
