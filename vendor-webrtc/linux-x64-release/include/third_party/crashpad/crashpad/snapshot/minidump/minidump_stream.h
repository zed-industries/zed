// Copyright 2018 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STREAM_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STREAM_H_

#include <stdint.h>

#include <vector>


namespace crashpad {

//! \brief Stores a minidump stream along with its stream ID.
class MinidumpStream {
 public:
  MinidumpStream(uint32_t stream_type, std::vector<uint8_t> data)
      : stream_type_(stream_type), data_(data) {}

  MinidumpStream(const MinidumpStream&) = delete;
  MinidumpStream& operator=(const MinidumpStream&) = delete;

  uint32_t stream_type() const { return stream_type_; }
  const std::vector<uint8_t>& data() const { return data_; }

 private:
  uint32_t stream_type_;
  std::vector<uint8_t> data_;
};

}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STREAM_H_
