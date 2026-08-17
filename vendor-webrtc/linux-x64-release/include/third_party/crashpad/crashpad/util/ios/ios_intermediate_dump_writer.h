// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_WRITER_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_WRITER_H_

#include <sys/types.h>

#include "base/files/file_path.h"
#include "util/ios/ios_intermediate_dump_format.h"

namespace crashpad {
namespace internal {

//! \brief Wrapper class for writing intermediate dump file.
//!
//! Due to the limitations of in-process handling, an intermediate dump file is
//! written during exceptions. The data is streamed to a file using only
//! in-process safe methods.
//!
//! The file format is similar to binary JSON, supporting keyed properties, maps
//! and arrays.
//!  - Property [key:int, length:int, value:intarray]
//!  - StartMap [key:int], followed by repeating Properties until EndMap
//!  - StartArray [key:int], followed by repeating Maps until EndArray
//!  - EndMap, EndArray, EndDocument
//!
//!  Similar to JSON, maps can contain other maps, arrays and properties.
//!
//! Note: All methods are `RUNS-DURING-CRASH`.
class IOSIntermediateDumpWriter final {
 public:
  IOSIntermediateDumpWriter() : buffer_occupied_(0), fd_(-1) {}

  IOSIntermediateDumpWriter(const IOSIntermediateDumpWriter&) = delete;
  IOSIntermediateDumpWriter& operator=(const IOSIntermediateDumpWriter&) =
      delete;

  ~IOSIntermediateDumpWriter();

  //! \brief Command instructions for the intermediate dump reader.
  enum class CommandType : uint8_t {
    //! \brief Indicates a new map, followed by associated key.
    kMapStart = 0x01,

    //! \brief Indicates map is complete.
    kMapEnd = 0x02,

    //! \brief Indicates a new array, followed by associated key.
    kArrayStart = 0x03,

    //! \brief Indicates array is complete.
    kArrayEnd = 0x04,

    //! \brief Indicates a new property, followed by a key, length and value.
    kProperty = 0x05,

    //! \brief Indicates the start of the root map.
    kRootMapStart = 0x06,

    //! \brief Indicates the end of the root map, and that there is nothing left
    //!     to parse.
    kRootMapEnd = 0x07,
  };

  //! \brief Open and lock an intermediate dump file. This is the only method
  //!     in the writer class that is generally run outside of a crash.
  //!
  //! The client must invoke `Close()` before this object is destroyed.
  //!
  //! \param[in] path The path to the intermediate dump.
  //!
  //! \return On success, returns `true`, otherwise returns `false`.
  bool Open(const base::FilePath& path);

  //! \brief Completes writing the intermediate dump file and releases the
  //!     file handle.
  //!
  //! \return On success, returns `true`, otherwise returns `false`.
  bool Close();

  //! \brief A scoped wrapper for calls to RootMapStart and RootMapEnd.
  class ScopedRootMap {
   public:
    explicit ScopedRootMap(IOSIntermediateDumpWriter* writer)
        : writer_(writer) {
      writer->RootMapStart();
    }

    ScopedRootMap(const ScopedRootMap&) = delete;
    ScopedRootMap& operator=(const ScopedRootMap&) = delete;

    ~ScopedRootMap() { writer_->RootMapEnd(); }

   private:
    IOSIntermediateDumpWriter* writer_;
  };

  //! \brief A scoped wrapper for calls to MapStart and MapEnd.
  class ScopedMap {
   public:
    explicit ScopedMap(IOSIntermediateDumpWriter* writer,
                       IntermediateDumpKey key)
        : writer_(writer) {
      writer->MapStart(key);
    }

    ScopedMap(const ScopedMap&) = delete;
    ScopedMap& operator=(const ScopedMap&) = delete;

    ~ScopedMap() { writer_->MapEnd(); }

   private:
    IOSIntermediateDumpWriter* writer_;
  };

  //! \brief A scoped wrapper for calls to ArrayMapStart and MapEnd.
  class ScopedArrayMap {
   public:
    explicit ScopedArrayMap(IOSIntermediateDumpWriter* writer)
        : writer_(writer) {
      writer->ArrayMapStart();
    }

    ScopedArrayMap(const ScopedArrayMap&) = delete;
    ScopedArrayMap& operator=(const ScopedArrayMap&) = delete;

    ~ScopedArrayMap() { writer_->MapEnd(); }

   private:
    IOSIntermediateDumpWriter* writer_;
  };

  //! \brief A scoped wrapper for calls to ArrayStart and ArrayEnd.
  class ScopedArray {
   public:
    explicit ScopedArray(IOSIntermediateDumpWriter* writer,
                         IntermediateDumpKey key)
        : writer_(writer) {
      writer->ArrayStart(key);
    }

    ScopedArray(const ScopedArray&) = delete;
    ScopedArray& operator=(const ScopedArray&) = delete;

    ~ScopedArray() { writer_->ArrayEnd(); }

   private:
    IOSIntermediateDumpWriter* writer_;
  };

  //! \return The `true` if able to AddPropertyInternal the \a key \a value
  //!     \a count tuple.
  template <typename T>
  bool AddProperty(IntermediateDumpKey key, const T* value, size_t count = 1) {
    return AddPropertyInternal(
        key, reinterpret_cast<const char*>(value), count * sizeof(T));
  }

  //! \return The `true` if able to AddPropertyInternal the \a key \a value
  //!     \a count tuple.
  bool AddPropertyBytes(IntermediateDumpKey key,
                        const void* value,
                        size_t value_length) {
    return AddPropertyInternal(
        key, reinterpret_cast<const char*>(value), value_length);
  }

  //! \return `true` if able to vm_read a string of \a value  and write a
  //!     kProperty command with the \a key \a value up to a NUL byte.
  //!     The string cannot be longer than \a max_length with a maximum string
  //!     length of 1024.
  bool AddPropertyCString(IntermediateDumpKey key,
                          size_t max_length,
                          const char* value);

 private:
  //! \return `true` if able to vm_read_overwrite \a value into
  //!     \a buffer while only reading one page at a time up to a NUL byte.
  //!     Sets the final length of \a buffer to \a string_length.
  //!     Returns `false` if unable to vm_read \a value or when no NUL byte can
  //!     be found within /a max_length (unterminated).
  bool ReadCStringInternal(const char* value,
                           char* buffer,
                           size_t max_length,
                           size_t* string_length);

  //! \return `true` if able to vm_read \a value \a count and write a
  //!     kProperty command  with the \a key \a value \a count tuple.
  bool AddPropertyInternal(IntermediateDumpKey key,
                           const char* value,
                           size_t value_length);

  //! \return `true` if able to write a kArrayStart command  with the \a key.
  bool ArrayStart(IntermediateDumpKey key);

  //! \return `true` if able to write a kMapStart command with the \a key.
  bool MapStart(IntermediateDumpKey key);

  //! \return `true` if able to write a kMapStart command.
  bool ArrayMapStart();

  //! \return `true` if able to write a kArrayEnd command.
  bool ArrayEnd();

  //! \return `true` if able to write a kMapEnd command.
  bool MapEnd();

  //! \return `true` if able to write a kRootMapStart command.
  bool RootMapStart();

  //! \return `true` if able to write a kRootMapEnd command.
  bool RootMapEnd();

  //! \return `true` if able to write a kProperty command with the \a key
  //!     \a value \a value_length tuple.
  bool Property(IntermediateDumpKey key,
                const void* value,
                size_t value_length);

  //! \return `true` if able to write \a data up to \a size. The \a data might
  //!     not be written to fd_  until `buffer_` is full or the writer is
  //!     closed. All writes will be 4096 bytes (the size of your kBufferSize)
  //!     except for the final flush, which might be partial.
  bool BufferedWrite(const void* data, size_t size);

  //! \return `true` if able to write `buffer_` up to `buffer_occupied_`.
  bool FlushWriteBuffer();

  //! \brief The maximum size of the write buffer.
  static constexpr size_t kBufferSize = 4096;

  //! \brief The write data buffer and amount of that buffer occupied with data
  //!   to be written.
  char buffer_[kBufferSize];
  size_t buffer_occupied_;

  int fd_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_WRITER_H_
