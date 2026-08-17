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

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_RANDOM_ACCESS_FILE_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_RANDOM_ACCESS_FILE_H_

#include <cstddef>
#include <cstdint>

#include "leveldb/env.h"  // from @com_google_leveldb
#include "leveldb/slice.h"  // from @com_google_leveldb
#include "leveldb/status.h"  // from @com_google_leveldb

namespace tflite {
namespace scann_ondevice {

// An implementation of LevelDB's RandomAccessFile [1] that wraps an in-memory
// buffer.
//
// [1]: https://github.com/google/leveldb/blob/main/include/leveldb/env.h
class MemRandomAccessFile : public leveldb::RandomAccessFile {
 public:
  // Constructor does not take ownership of the provided buffer, which must
  // outlive this object.
  MemRandomAccessFile(const char* buffer_data, size_t buffer_size);
  ~MemRandomAccessFile() override;

  // Override of the `Read` function. Note that `scratch` is unused in the
  // implementation.
  leveldb::Status Read(uint64_t offset, size_t n, leveldb::Slice* result,
                       char* scratch) const override;

  // Class is movable and non-copyable.
  MemRandomAccessFile(MemRandomAccessFile&& rhs) = default;
  MemRandomAccessFile& operator=(MemRandomAccessFile&& rhs) = default;
  MemRandomAccessFile(const MemRandomAccessFile& rhs) = delete;
  MemRandomAccessFile& operator=(const MemRandomAccessFile& rhs) = delete;

 private:
  const char* buffer_data_;
  size_t buffer_size_;
};

}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_MEM_RANDOM_ACCESS_FILE_H_
