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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MEMORY_MAPPED_FILE_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MEMORY_MAPPED_FILE_H_

#include <cstddef>
#include <cstdint>
#include <memory>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/genai/inference/utils/llm_utils/scoped_file.h"

namespace mediapipe::tasks::genai::llm_utils {

// Represents a memory mapped file. All memory will be accessible while this
// object exists and will be cleaned up when it is destroyed.
class MemoryMappedFile {
 public:
  // Gets the required alignment for a file offset passed to Create().
  static size_t GetOffsetAlignment();

  // Creates a read-only MemoryMappedFile object.
  static absl::StatusOr<std::unique_ptr<MemoryMappedFile>> Create(
      absl::string_view path);
  // Creates a MemoryMappedFile object from the platform file handle. This does
  // not take ownership of the passed handle. The `key` passed here is an
  // optimization when mapping the same file with different offsets.
  static absl::StatusOr<std::unique_ptr<MemoryMappedFile>> Create(
      ScopedFile::PlatformFile file, uint64_t offset = 0u, uint64_t length = 0u,
      absl::string_view key = "");

  // Creates a mutable MemoryMappedFile object, any modification through data()
  // pointer will be carried over to the underlying path.
  static absl::StatusOr<std::unique_ptr<MemoryMappedFile>> CreateMutable(
      absl::string_view path);

  virtual ~MemoryMappedFile() = default;

  // Returns the file size in bytes.
  virtual uint64_t length() = 0;

  // Returns a pointer to the file data.
  virtual void* data() = 0;
};

}  // namespace mediapipe::tasks::genai::llm_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MEMORY_MAPPED_FILE_H_
