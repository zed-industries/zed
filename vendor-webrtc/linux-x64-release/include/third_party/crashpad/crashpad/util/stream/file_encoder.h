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

#ifndef CRASHPAD_UTIL_STREAM_FILE_ENCODER_H_
#define CRASHPAD_UTIL_STREAM_FILE_ENCODER_H_

#include "base/files/file_path.h"

namespace crashpad {

//! \brief The class is used to compress and base94-encode, or base94-decode
//! and decompress the given input file to the output file.
class FileEncoder {
 public:
  //! \brief Whether this object is configured to encode or decode data.
  enum class Mode : bool {
    //! \brief Data passed through this object is encoded.
    kEncode = false,
    //! \brief Data passed through this object is decoded.
    kDecode = true
  };

  //! \param[in] mode The work mode of this object.
  //! \param[in] input_path The input file that this object reads from.
  //! \param[in] output_path The output file that this object writes to.
  FileEncoder(Mode mode,
              const base::FilePath& input_path,
              const base::FilePath& output_path);

  FileEncoder(const FileEncoder&) = delete;
  FileEncoder& operator=(const FileEncoder&) = delete;

  ~FileEncoder();

  //! \brief Encode/decode the data from \a input_path_ file according work
  //! \a mode, and write the result to \a output_path_ on success.
  //!
  //! \return `true` on success.
  bool Process();

 private:
  Mode mode_;
  base::FilePath input_path_;
  base::FilePath output_path_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_FILE_ENCODER_H_
