// Copyright 2019 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_STREAM_TEST_OUTPUT_STREAM_H_
#define CRASHPAD_UTIL_STREAM_TEST_OUTPUT_STREAM_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "util/stream/output_stream_interface.h"

namespace crashpad {
namespace test {

//! \brief The help class for \a OutputStreamInterface related tests.
class TestOutputStream : public OutputStreamInterface {
 public:
  TestOutputStream();

  TestOutputStream(const TestOutputStream&) = delete;
  TestOutputStream& operator=(const TestOutputStream&) = delete;

  ~TestOutputStream() override;

  // OutputStreamInterface:
  bool Write(const uint8_t* data, size_t size) override;
  bool Flush() override;

  //! \return the data that has been received by the last call of Write().
  const std::vector<uint8_t>& last_written_data() const {
    return last_written_data_;
  }

  //! \return all data that has been received.
  const std::vector<uint8_t>& all_data() const { return all_data_; }

  //! \return the number of times Write() has been called.
  size_t write_count() const { return write_count_; }

  //! \return the number of times Flush() has been called.
  size_t flush_count() const { return flush_count_; }

 private:
  std::vector<uint8_t> last_written_data_;
  std::vector<uint8_t> all_data_;
  size_t write_count_;
  size_t flush_count_;
  bool flush_needed_;
};

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_TEST_OUTPUT_STREAM_H_
