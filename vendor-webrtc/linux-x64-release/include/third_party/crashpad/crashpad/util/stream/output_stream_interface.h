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

#ifndef CRASHPAD_UTIL_STREAM_OUTPUT_STREAM_INTERFACE_H_
#define CRASHPAD_UTIL_STREAM_OUTPUT_STREAM_INTERFACE_H_

#include <stddef.h>
#include <stdint.h>

namespace crashpad {

//! \brief The interface for an output stream pipeline.
//!
//! Example:
//! <code>
//!   class OutputStreamInterfaceImpl : public OutputStreamInterface {
//!     ...
//!   };
//!
//!   // Create a OutputStream.
//!   OutputStreamInterfaceImpl impl(...);
//!   // Write the data multiple times.
//!   while (has_data) {
//!     impl.Write(data, size);
//!     ...
//!   }
//!   // Flush internal buffer to indicate all data has been written.
//!   impl.Flush();
//! </code>
//!
class OutputStreamInterface {
 public:
  virtual ~OutputStreamInterface() = default;

  //! \brief Writes \a data to this stream. This method may be called multiple
  //! times for streaming.
  //!
  //! \param[in] data The data that should be written.
  //! \param[in] size The size of \a data.
  //!
  //! \return `true` on success.
  virtual bool Write(const uint8_t* data, size_t size) = 0;

  //! \brief Flush the internal buffer after all data has been written.
  //!
  //! Write() can't be called afterwards.
  //! \return `true` on success.
  virtual bool Flush() = 0;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_STREAM_OUTPUT_STREAM_INTERFACE_H_
