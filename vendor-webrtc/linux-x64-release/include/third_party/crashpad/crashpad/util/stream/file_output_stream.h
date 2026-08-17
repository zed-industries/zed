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

#ifndef CRASHPAD_UTIL_STREAM_FILE_OUTPUT_STREAM_H_
#define CRASHPAD_UTIL_STREAM_FILE_OUTPUT_STREAM_H_

#include "util/file/file_io.h"
#include "util/file/file_writer.h"
#include "util/stream/output_stream_interface.h"

namespace crashpad {

//! \brief The class is used to write data to a file.
class FileOutputStream : public OutputStreamInterface {
 public:
  //! \param[in] file_handle The file that this object writes to.
  explicit FileOutputStream(FileHandle file_handle);

  FileOutputStream(const FileOutputStream&) = delete;
  FileOutputStream& operator=(const FileOutputStream&) = delete;

  ~FileOutputStream();

  // OutputStream.
  bool Write(const uint8_t* data, size_t size) override;
  bool Flush() override;

 private:
  WeakFileHandleFileWriter writer_;
  bool flush_needed_;
  bool flushed_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_FILE_OUTPUT_STREAM_H_
