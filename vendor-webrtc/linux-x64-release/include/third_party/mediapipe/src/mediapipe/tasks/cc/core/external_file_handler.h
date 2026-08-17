/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_CC_CORE_EXTERNAL_FILE_HANDLER_H_
#define MEDIAPIPE_TASKS_CC_CORE_EXTERNAL_FILE_HANDLER_H_

#include <cstdint>
#include <memory>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "mediapipe/tasks/cc/core/proto/external_file.pb.h"

namespace mediapipe {
namespace tasks {
namespace core {

// Handler providing easy access to the contents of a file specified by an
// ExternalFile proto [1]. Takes care (if needed, depending on the provided
// proto fields) of opening and/or mapping the file in memory at creation time,
// as well as closing and/or unmapping at destruction time.
//
// [1]: support/c/task/core/proto/external_file.proto
class ExternalFileHandler {
 public:
  // Creates an ExternalFileHandler from the input ExternalFile proto and
  // returns a pointer to the new object. Ownership is transferred to the
  // caller. Returns an error if the creation failed, which may happen if the
  // provided ExternalFile can't be opened or mapped into memory.
  //
  // Warning: Does not take ownership of `external_file`, which must refer to a
  // valid proto that outlives this object.
  static absl::StatusOr<std::unique_ptr<ExternalFileHandler>>
  CreateFromExternalFile(const proto::ExternalFile* external_file);

  ~ExternalFileHandler();

  // Returns the content of the ExternalFile as a string_view guaranteed to be
  // valid as long as the ExternalFileHandler is alive.
  absl::string_view GetFileContent();

 private:
  // Private constructor, called from CreateFromExternalFile().
  explicit ExternalFileHandler(const proto::ExternalFile* external_file)
      : external_file_(*external_file) {}

  // Opens (if provided by path) and maps (if provided by path or file
  // descriptor) the external file in memory. Does nothing otherwise, as file
  // contents are already loaded in memory.
  absl::Status MapExternalFile();

  // Reference to the input ExternalFile.
  const proto::ExternalFile& external_file_;

  // The file descriptor of the ExternalFile if provided by path, as it is
  // opened and owned by this class. Set to -1 otherwise.
  int owned_fd_{-1};

  // Points to the memory buffer mapped from the file descriptor of the
  // ExternalFile, if provided by path or file descriptor.
  void* buffer_{};

  // The mapped memory buffer offset, if any.
  int64_t buffer_offset_{};
  // The size in bytes of the mapped memory buffer, if any.
  int64_t buffer_size_{};

  // As mmap(2) requires the offset to be a multiple of sysconf(_SC_PAGE_SIZE):

  // The aligned mapped memory buffer offset, if any.
  int64_t buffer_aligned_offset_{};
  // The aligned mapped memory buffer size in bytes taking into account the
  // offset shift introduced by buffer_aligned_memory_offset_, if any.
  int64_t buffer_aligned_size_{};
};

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_EXTERNAL_FILE_HANDLER_H_
