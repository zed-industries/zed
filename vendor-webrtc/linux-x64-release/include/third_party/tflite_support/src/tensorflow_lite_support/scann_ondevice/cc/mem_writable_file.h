/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_WRITABLE_FILE_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_WRITABLE_FILE_H_

#include <memory>
#include <string>

#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/cord.h"  // from @com_google_absl
#include "leveldb/env.h"  // from @com_google_leveldb
#include "leveldb/slice.h"  // from @com_google_leveldb
#include "leveldb/status.h"  // from @com_google_leveldb

namespace tflite {
namespace scann_ondevice {

// An implementation of LevelDB's WritableFile [1] that wraps an in-memory
// buffer.
//
// [1]: https://github.com/google/leveldb/blob/main/include/leveldb/env.h
class MemWritableFile : public leveldb::WritableFile {
 public:
  // Creates a MemWritableFile from a given buffer. Returns
  // InvalidArgumentError if pointer is null.
  static absl::StatusOr<std::unique_ptr<MemWritableFile>> Create(
      std::string* buffer);

  ~MemWritableFile() override = default;

  // Allow moves. Disallow copies.
  MemWritableFile(MemWritableFile&& rhs) = default;
  MemWritableFile& operator=(MemWritableFile&& rhs) = default;
  MemWritableFile(const MemWritableFile& rhs) = delete;
  MemWritableFile& operator=(const MemWritableFile& rhs) = delete;

  leveldb::Status Append(const leveldb::Slice& data) override;
  leveldb::Status Close() override;
  leveldb::Status Flush() override;
  leveldb::Status Sync() override;

 private:
  MemWritableFile(std::string* buffer);

  std::string* buffer_;
};

}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_WRITABLE_FILE_H_
