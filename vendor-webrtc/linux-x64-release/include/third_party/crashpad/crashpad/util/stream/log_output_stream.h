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

#ifndef CRASHPAD_UTIL_STREAM_LOG_OUTPUT_STREAM_H_
#define CRASHPAD_UTIL_STREAM_LOG_OUTPUT_STREAM_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <string>

#include "util/stream/output_stream_interface.h"

namespace crashpad {

//! \brief This class outputs a stream of data as a series of log messages.
class LogOutputStream : public OutputStreamInterface {
 public:
  //! \brief An interface to a log output sink.
  class Delegate {
   public:
    Delegate() = default;
    virtual ~Delegate() = default;

    //! \brief Logs |buf| to the output sink.
    //!
    //! \param buf the buffer to write to the log. More bytes than are in |buf|
    //!     may be written, e.g. to convey metadata.
    //! \return the number of bytes written, or a negative error code.
    virtual int Log(const char* buf) = 0;

    //! \brief Returns the maximum number of bytes to allow writing to this log.
    virtual size_t OutputCap() = 0;

    //! \brief Returns the maximum length of buffers allowed to be passed to
    //!     Log().
    virtual size_t LineWidth() = 0;
  };

  explicit LogOutputStream(std::unique_ptr<Delegate> delegate);

  LogOutputStream(const LogOutputStream&) = delete;
  LogOutputStream& operator=(const LogOutputStream&) = delete;

  ~LogOutputStream() override;

  // OutputStreamInterface:
  bool Write(const uint8_t* data, size_t size) override;
  bool Flush() override;

 private:
  // Flushes buffer_, returning false on failure.
  bool WriteBuffer();

  int WriteToLog(const char* buf);

  std::string buffer_;
  std::unique_ptr<Delegate> delegate_;
  size_t output_count_;
  bool flush_needed_;
  bool flushed_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_LOG_OUTPUT_STREAM_H_
