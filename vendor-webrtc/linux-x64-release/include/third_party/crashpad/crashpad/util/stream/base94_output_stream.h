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

#ifndef CRASHPAD_UTIL_STREAM_BASE94_OUTPUT_STREAM_H_
#define CRASHPAD_UTIL_STREAM_BASE94_OUTPUT_STREAM_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "util/stream/output_stream_interface.h"

namespace crashpad {

//! \brief This class implements Base94 encoding/decoding, it uses all
//! printable characters except space for encoding, and no padding is required.
//!
//! This implementation uses two base94 symbols to encoding 13 or 14 bit data,
//! To maximize encoding efficiency, 14-bit data is encoded into two base94
//! symbols if its low 13-bit is less than 644 ( = 94^2 - 2^13), otherwise
//! 13-bit data is encoded.
class Base94OutputStream : public OutputStreamInterface {
 public:
  //! \brief Whether this object is configured to encode or decode data.
  enum class Mode : bool {
    //! \brief Data passed through this object is encoded.
    kEncode = false,
    //! \brief Data passed through this object is decoded.
    kDecode = true
  };

  //! \param[in] mode The work mode of this object.
  //! \param[in] output_stream The output_stream that this object writes to.
  Base94OutputStream(Mode mode,
                     std::unique_ptr<OutputStreamInterface> output_stream);

  Base94OutputStream(const Base94OutputStream&) = delete;
  Base94OutputStream& operator=(const Base94OutputStream&) = delete;

  ~Base94OutputStream() override;

  // OutputStreamInterface:
  bool Write(const uint8_t* data, size_t size) override;
  bool Flush() override;

 private:
  bool Encode(const uint8_t* data, size_t size);
  bool Decode(const uint8_t* data, size_t size);
  bool FinishEncoding();
  bool FinishDecoding();
  // Write encoded/decoded data to |output_stream_| and empty the |buffer_|.
  bool WriteOutputStream();

  Mode mode_;
  std::unique_ptr<OutputStreamInterface> output_stream_;
  std::vector<uint8_t> buffer_;
  uint32_t bit_buf_;
  // The number of valid bit in bit_buf_.
  size_t bit_count_;
  char symbol_buffer_;
  bool flush_needed_;
  bool flushed_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_BASE94_OUTPUT_STREAM_H_
