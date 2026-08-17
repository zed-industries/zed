// Copyright 2020 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_FILE_OUTPUT_STREAM_FILE_WRITER_H_
#define CRASHPAD_UTIL_FILE_OUTPUT_STREAM_FILE_WRITER_H_

#include <memory>

#include "util/file/file_writer.h"

namespace crashpad {

class OutputStreamInterface;

//! \brief A file writer backed by a OutputSteamInterface.
//! \note The \a Seek related methods don't work and shouldn't be invoked.
class OutputStreamFileWriter : public FileWriterInterface {
 public:
  //! \param[in] output_stream The output stream that this object writes to.
  explicit OutputStreamFileWriter(
      std::unique_ptr<OutputStreamInterface> output_stream);

  OutputStreamFileWriter(const OutputStreamFileWriter&) = delete;
  OutputStreamFileWriter& operator=(const OutputStreamFileWriter&) = delete;

  ~OutputStreamFileWriter() override;

  // FileWriterInterface:
  bool Write(const void* data, size_t size) override;
  bool WriteIoVec(std::vector<WritableIoVec>* iovecs) override;

  // FileSeekerInterface:

  //! \copydoc FileWriterInterface::Seek()
  //!
  //! \note This method doesn't work and shouldn't be invoked.
  FileOffset Seek(FileOffset offset, int whence) override;

  //! \brief  Flush data to output_stream.
  //!
  //! Either \a Write() or \a WriteIoVec() can't be called afterwards.
  bool Flush();

 private:
  std::unique_ptr<OutputStreamInterface> output_stream_;
  bool flush_needed_;
  bool flushed_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_FILE_OUTPUT_STREAM_FILE_WRITER_H_
