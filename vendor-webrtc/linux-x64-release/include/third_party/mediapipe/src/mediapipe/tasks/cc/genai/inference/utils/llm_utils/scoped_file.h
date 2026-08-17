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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_SCOPED_FILE_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_SCOPED_FILE_H_

#if defined(_WIN32)
#include <Windows.h>
#endif

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace mediapipe::tasks::genai::llm_utils {

// A file wrapper that will automatically close on deletion.
class ScopedFile {
 public:
#if defined(_WIN32)
  using PlatformFile = HANDLE;
  static const PlatformFile kInvalidPlatformFile;
#else
  using PlatformFile = int;
  static constexpr PlatformFile kInvalidPlatformFile = -1;
#endif

  static absl::StatusOr<ScopedFile> Open(absl::string_view path);
  static absl::StatusOr<ScopedFile> OpenWritable(absl::string_view path);

  ScopedFile() : file_(kInvalidPlatformFile) {}
  explicit ScopedFile(PlatformFile file) : file_(file) {}
  ~ScopedFile() {
    if (IsValid()) {
      CloseFile(file_);
    }
  }

  ScopedFile(ScopedFile&& other) { file_ = other.Release(); }
  ScopedFile& operator=(ScopedFile&& other) {
    file_ = other.Release();
    return *this;
  }

  ScopedFile(const ScopedFile&) = delete;
  ScopedFile& operator=(const ScopedFile&) = delete;

  PlatformFile file() const { return file_; }
  bool IsValid() const { return file_ != kInvalidPlatformFile; }

 private:
  PlatformFile Release() {
    PlatformFile temp = file_;
    file_ = kInvalidPlatformFile;
    return temp;
  }

  static void CloseFile(PlatformFile file);

  PlatformFile file_;
};

}  // namespace mediapipe::tasks::genai::llm_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_SCOPED_FILE_H_
