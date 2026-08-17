/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DATA_H_

#include <vector>

#include "base/containers/span.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

#if INSIDE_BLINK
#include "base/memory/scoped_refptr.h"
#endif

namespace blink {

// A container for raw bytes.  It is inexpensive to copy a WebData object.
//
// WARNING: It is not safe to pass a WebData across threads!!!
//
class BLINK_PLATFORM_EXPORT WebData {
 public:
  ~WebData() { Reset(); }

  WebData() = default;

  explicit WebData(base::span<const uint8_t> data) { Assign(data); }

  WebData(const WebData& d) { Assign(d); }

  WebData& operator=(const WebData& d) {
    Assign(d);
    return *this;
  }

  void Reset();
  void Assign(const WebData&);
  void Assign(base::span<const uint8_t> data);
  void Append(base::span<const uint8_t> data);

  size_t size() const;

  // Returns a span of the consecutive bytes after "position". Returns an empty
  // span when no more data is left.
  base::span<const uint8_t> GetSomeData(size_t position) const;

  // Same as SharedBuffer::CopyAs, copies the segmented data into a
  // contiguous buffer.  Use GetSomeData() or ForEachSegment() whenever
  // possible, if a copy can be avoided.
  std::vector<uint8_t> Copy() const;

  // Helper for applying a lambda to all data segments sequentially:
  //
  //   bool func(base::span<const uint8_t> segment, size_t segment_offset);
  //
  // The iterator stops early when the lambda returns false.
  template <typename Func>
  void ForEachSegment(Func&& func) const {
    size_t segment_offset = 0;
    for (base::span<const uint8_t> segment = GetSomeData(segment_offset);
         !segment.empty(); segment = GetSomeData(segment_offset)) {
      if (!func(segment, segment_offset)) {
        break;
      }
      segment_offset += segment.size();
    }
  }

  bool IsEmpty() const { return !size(); }
  bool IsNull() const { return private_.IsNull(); }

#if INSIDE_BLINK
  WebData(scoped_refptr<SharedBuffer>);
  WebData& operator=(scoped_refptr<SharedBuffer>);
  operator scoped_refptr<SharedBuffer>() const;
  operator const SharedBuffer&() const;
#endif

 private:
  WebPrivatePtrForRefCounted<SharedBuffer> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DATA_H_
