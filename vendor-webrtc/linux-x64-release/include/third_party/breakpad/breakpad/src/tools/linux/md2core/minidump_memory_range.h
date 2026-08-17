// Copyright 2011 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// minidump_memory_range.h: Define the google_breakpad::MinidumpMemoryRange
// class, which adds methods for handling minidump specific data structures
// on top of google_breakpad::MemoryRange. See common/memory_range.h for
// more details on MemoryRange.

#ifndef TOOLS_LINUX_MD2CORE_MINIDUMP_MEMORY_RANGE_H_
#define TOOLS_LINUX_MD2CORE_MINIDUMP_MEMORY_RANGE_H_

#include <string>

#include "common/memory_range.h"
#include "google_breakpad/common/minidump_format.h"

namespace google_breakpad {

// A derived class of MemoryRange with added methods for handling minidump
// specific data structures. To avoid virtual functions, it is not designed
// to be used polymorphically.
class MinidumpMemoryRange : public MemoryRange {
 public:
  MinidumpMemoryRange() {}

  MinidumpMemoryRange(const void* data, size_t length)
      : MemoryRange(data, length) {}

  // Returns a subrange of |length| bytes at |offset| bytes of this memory
  // range, or an empty range if the subrange is out of bounds.
  // This methods overrides the base implemementation in order to return
  // an instance of MinidumpMemoryRange instead of MemoryRange.
  MinidumpMemoryRange Subrange(size_t sub_offset, size_t sub_length) const {
    if (Covers(sub_offset, sub_length))
      return MinidumpMemoryRange(data() + sub_offset, sub_length);
    return MinidumpMemoryRange();
  }

  // Returns a subrange that covers the offset and length specified by
  // |location|, or an empty range if the subrange is out of bounds.
  MinidumpMemoryRange Subrange(const MDLocationDescriptor& location) const {
    return MinidumpMemoryRange::Subrange(location.rva, location.data_size);
  }

  // Gets a STL string from a MDString at |sub_offset| bytes of this memory
  // range. This method only works correctly for ASCII characters and does
  // not convert between UTF-16 and UTF-8.
  const std::string GetAsciiMDString(size_t sub_offset) const {
    std::string str;
    const MDString* md_str = GetData<MDString>(sub_offset);
    if (md_str) {
      const uint16_t* buffer = &md_str->buffer[0];
      for (uint32_t i = 0; i < md_str->length && buffer[i]; ++i) {
        str.push_back(buffer[i]);
      }
    }
    return str;
  }
};

}  // namespace google_breakpad

#endif  // TOOLS_LINUX_MD2CORE_MINIDUMP_MEMORY_RANGE_H_
