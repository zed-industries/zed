/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_READER_H_

#include <stdint.h>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

#include "perfetto/ext/base/string_view.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/perf/perf_event.h"

namespace perfetto::trace_processor::perf_importer {

// Helper to read various types of data fields contained in a TraceBlobView.
// All methods return a boolean indicating whether the read was successful. A
// false value means there was not enough data in the underlying buffer to
// satisfy the read.
class Reader {
 public:
  explicit Reader(TraceBlobView tbv)
      : buffer_(tbv.blob()),
        current_(tbv.data()),
        end_(current_ + tbv.size()) {}

  // Data left to be read. The value returned here decrements as read or skip
  // methods are called.
  size_t size_left() const { return static_cast<size_t>(end_ - current_); }

  bool ReadStringView(base::StringView& str, size_t size) {
    if (size_left() < size) {
      return false;
    }
    str = base::StringView(reinterpret_cast<const char*>(current_), size);
    current_ += size;
    return true;
  }

  bool ReadPerfEventAttr(perf_event_attr& attr, size_t attr_size) {
    const size_t bytes_to_read = std::min(attr_size, sizeof(attr));
    const size_t bytes_to_skip = attr_size - bytes_to_read;
    static_assert(std::has_unique_object_representations_v<perf_event_attr>);

    if (size_left() < bytes_to_read + bytes_to_skip) {
      return false;
    }

    memset(&attr, 0, sizeof(attr));

    return Read(&attr, bytes_to_read) && Skip(bytes_to_skip);
  }

  bool ReadBlob(TraceBlobView& blob, uint32_t size) {
    if (size_left() < size) {
      return false;
    }
    blob = TraceBlobView(buffer_,
                         static_cast<size_t>(current_ - buffer_->data()), size);
    current_ += size;
    return true;
  }

  bool ReadStringUntilEndOrNull(std::string& out) {
    const uint8_t* ptr = current_;
    while (ptr != end_ && *ptr != 0) {
      ++ptr;
    }
    out = std::string(reinterpret_cast<const char*>(current_),
                      static_cast<size_t>(ptr - current_));
    current_ = ptr;
    return true;
  }

  template <typename T>
  bool Read(T& obj) {
    static_assert(std::has_unique_object_representations_v<T>);
    return Read(&obj, sizeof(T));
  }

  bool Read(void* dest, size_t size) {
    if (size_left() < size) {
      return false;
    }
    memcpy(dest, current_, size);
    current_ += size;
    return true;
  }

  bool Skip(size_t size) {
    if (size_left() < size) {
      return false;
    }
    current_ += size;
    return true;
  }

  template <typename T>
  bool Skip() {
    return Skip(sizeof(T));
  }

  // Reads consecutive values and stores them in the given vector. Reads as many
  // entries as the current vector size.
  template <typename T>
  bool ReadVector(std::vector<T>& vec) {
    static_assert(std::has_unique_object_representations_v<T>);
    size_t size = sizeof(T) * vec.size();
    if (size_left() < size) {
      return false;
    }
    memcpy(vec.data(), current_, size);
    current_ += size;
    return true;
  }

  // Convenience helper for reading values and storing them in an optional<>
  // wrapper.
  template <typename T>
  bool ReadOptional(std::optional<T>& obj) {
    T val;
    if (!Read(val)) {
      return false;
    }
    obj = val;
    return true;
  }

  // Reads a null terminated string.
  bool ReadCString(std::string& out) {
    for (const uint8_t* ptr = current_; ptr != end_; ++ptr) {
      if (*ptr == 0) {
        out = std::string(reinterpret_cast<const char*>(current_),
                          static_cast<size_t>(ptr - current_));
        current_ = ptr;
        return true;
      }
    }

    return false;
  }

 private:
  RefPtr<TraceBlob> buffer_;
  const uint8_t* current_;
  const uint8_t* end_;
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_READER_H_
