/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THREAD_SAFE_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THREAD_SAFE_DATA_H_

#include "base/containers/checked_iterators.h"
#include "base/containers/span.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

#if INSIDE_BLINK
#include "base/memory/scoped_refptr.h"
#else
#include <string>
#endif

namespace blink {

class RawData;

// A container for raw bytes. It is inexpensive to copy a WebThreadSafeData
// object.  It is safe to pass a WebThreadSafeData across threads.
class BLINK_PLATFORM_EXPORT WebThreadSafeData {
 public:
  // Ideally, instead of defining this, `begin()`/`end()` below would return
  // `auto` and just pass through the underlying storage types' iterators, so
  // that they could decide what to do. However, since those types are defined
  // outside /public/, that would be a layering violation. So be conservative
  // and use `CheckedContiguousIterator` directyly, regardless of what the
  // underlying type does.
  using iterator = base::CheckedContiguousIterator<const char>;

  WebThreadSafeData() = default;
  explicit WebThreadSafeData(base::span<const char> data);

  ~WebThreadSafeData() { Reset(); }

  void Assign(const WebThreadSafeData&);
  void Reset();

  size_t size() const;
  const char* data() const;

  // Iterators, required to satisfy the `std::ranges::contiguous_range` concept.
  iterator begin() const;
  iterator end() const;

  bool IsEmpty() const { return !size(); }

  WebThreadSafeData(const WebThreadSafeData&);
  WebThreadSafeData& operator=(const WebThreadSafeData&);

#if INSIDE_BLINK
  WebThreadSafeData(scoped_refptr<RawData>);
  WebThreadSafeData(scoped_refptr<RawData>&&);
  WebThreadSafeData& operator=(scoped_refptr<RawData>);
#else
  operator std::string() const {
    size_t len = size();
    return len ? std::string(data(), len) : std::string();
  }
#endif

 private:
  WebPrivatePtrForRefCounted<RawData> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_THREAD_SAFE_DATA_H_
