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

#ifndef CRASHPAD_UTIL_STREAM_ZLIB_OUTPUT_STREAM_H_
#define CRASHPAD_UTIL_STREAM_ZLIB_OUTPUT_STREAM_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "third_party/zlib/zlib_crashpad.h"
#include "util/misc/initialization_state.h"
#include "util/stream/output_stream_interface.h"

namespace crashpad {

//! \brief The class wraps zlib into \a OutputStreamInterface.
class ZlibOutputStream : public OutputStreamInterface {
 public:
  //! \brief Whether this object is configured to compress or decompress data.
  enum class Mode : bool {
    //! \brief Data passed through this object is compressed.
    kCompress = false,
    //! \brief Data passed through this object is decompressed.
    kDecompress = true
  };

  //! \param[in] mode The work mode of this object.
  //! \param[in] output_stream The output_stream that this object writes to.
  //!
  //! To construct an output pipeline, the output stream needs an output stream
  //! to write the result to. For example, the code below constructs a
  //! compress->base94-encoding->log output stream pipline.
  //!
  //! <code>
  //!   ZlibOutputStream zlib_output_stream(
  //!        ZlibOutputStream::Mode::kDeflate,
  //!        std::make_unique<Base94OutputStream>(
  //!            Base94OutputStream::Mode::kEncode,
  //!            std::make_unique<LogOutputStream>()));
  //! </code>
  //!
  //!
  ZlibOutputStream(Mode mode,
                   std::unique_ptr<OutputStreamInterface> output_stream);

  ZlibOutputStream(const ZlibOutputStream&) = delete;
  ZlibOutputStream& operator=(const ZlibOutputStream&) = delete;

  ~ZlibOutputStream() override;

  // OutputStreamInterface:
  bool Write(const uint8_t* data, size_t size) override;
  bool Flush() override;

 private:
  // Write compressed/decompressed data to |output_stream_| and empty the output
  // buffer in |zlib_stream_|.
  bool WriteOutputStream();

  uint8_t buffer_[4096];
  z_stream zlib_stream_;
  std::unique_ptr<OutputStreamInterface> output_stream_;
  Mode mode_;
  InitializationState initialized_;  // protects zlib_stream_
  bool flush_needed_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_ZLIB_OUTPUT_STREAM_H_
