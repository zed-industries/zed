/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 * Copyright (C) Research In Motion Limited 2009-2010. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SHARED_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SHARED_BUFFER_H_

#include <algorithm>
#include <utility>
#include <vector>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/span.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// This class is designed to store and manage large amounts of data that may be
// split into multiple segments.
class WTF_EXPORT SegmentedBuffer {
 public:
  class Segment {
   public:
    Segment(size_t start_position, Vector<char>&& data)
        : start_position_(start_position), data_(std::move(data)) {}
    ~Segment() = default;
    Segment(const Segment&) = delete;
    Segment& operator=(const Segment&) = delete;
    Segment(Segment&&) = default;
    Segment& operator=(Segment&&) = default;

    size_t start_position() const { return start_position_; }
    const Vector<char>& data() const { return data_; }

   private:
    size_t start_position_;
    Vector<char> data_;
  };

  // Iterator for SegmentedBuffer contents. An Iterator will get invalid once
  // the associated SegmentedBuffer is modified (e.g., Append() is called). An
  // Iterator doesn't retain the associated container.
  class WTF_EXPORT Iterator final {
   public:
    ~Iterator() = default;

    Iterator& operator++();
    Iterator operator++(int) {
      auto temp = *this;
      ++*this;
      return temp;
    }
    bool operator==(const Iterator& that) const {
      return std::ranges::equal(value_, that.value_) && buffer_ == that.buffer_;
    }
    bool operator!=(const Iterator& that) const { return !(*this == that); }
    const base::span<const char>& operator*() const {
      DCHECK(!IsEnd());
      return value_;
    }
    const base::span<const char>* operator->() const {
      DCHECK(!IsEnd());
      return &value_;
    }

   private:
    friend class SegmentedBuffer;
    // for end()
    explicit Iterator(const SegmentedBuffer* buffer);
    // for the rest
    Iterator(Vector<Segment>::const_iterator segment_it,
             size_t offset,
             const SegmentedBuffer* buffer);

    void Init(size_t offset);
    bool IsEnd() const { return segment_it_ == buffer_->segments_.end(); }

    Vector<Segment>::const_iterator segment_it_;
    base::span<const char> value_;
    const SegmentedBuffer* buffer_;
  };

  SegmentedBuffer() = default;
  ~SegmentedBuffer() = default;
  SegmentedBuffer(const SegmentedBuffer&) = delete;
  SegmentedBuffer& operator=(const SegmentedBuffer&) = delete;
  SegmentedBuffer(SegmentedBuffer&&) = default;
  SegmentedBuffer& operator=(SegmentedBuffer&&) = default;

  size_t size() const { return size_; }

  bool empty() const { return !size(); }

  void Append(base::span<const char> data);
  void Append(base::span<const unsigned char> data) {
    Append(base::as_chars(data));
  }
  void Append(Vector<char>&& vector);

  void Clear();

  Iterator begin() const;
  Iterator cbegin() const { return begin(); }
  Iterator end() const;
  Iterator cend() const { return end(); }

  // Copies the segmented data into a contiguous buffer.  Use GetSomeData() or
  // iterators if a copy is not required, as they are cheaper.
  // Supported Ts: WTF::Vector<char>, std::vector<char>.
  template <typename T>
  T CopyAs() const;

  Vector<Vector<char>> TakeData() &&;

  // Returns an iterator for the given position of bytes. Returns |cend()| if
  // |position| is greater than or equal to |size()|.
  HAS_STRICTLY_TYPED_ARG
  Iterator GetIteratorAt(STRICTLY_TYPED_ARG(position)) const {
    STRICT_ARG_TYPE(size_t);
    return GetIteratorAtInternal(position);
  }

  // Copies `buffer.size()` bytes from the beginning of the content data into
  // `buffer` as a flat buffer. Returns true on success, otherwise the content
  // of `buffer` is not guaranteed.
  [[nodiscard]] bool GetBytes(base::span<uint8_t> buffer) const;

  void GetMemoryDumpNameAndSize(String& dump_name, size_t& dump_size) const;

  // Helper for providing a contiguous view of the data.  If the SegmentedBuffer
  // is segmented, this will copy/merge all segments into a temporary buffer.
  // In general, clients should use the efficient/segmented accessors.
  class WTF_EXPORT DeprecatedFlatData {
    STACK_ALLOCATED();

   public:
    using iterator = base::CheckedContiguousIterator<const char>;

    explicit DeprecatedFlatData(const SegmentedBuffer*);

    const char* data() const { return data_; }
    size_t size() const { return buffer_->size(); }

    // Iterators, so this type meets the requirements of
    // `std::ranges::contiguous_range`.
    iterator begin() const {
      // SAFETY: The merged data has the same number of elements as the
      // underlying segmented data, so this points to at most just past the end
      // of the valid region.
      return UNSAFE_BUFFERS(iterator(data(), data() + size()));
    }
    iterator end() const {
      // SAFETY: As in `begin()` above.
      return UNSAFE_BUFFERS(iterator(data(), data() + size(), data() + size()));
    }

   private:
    const SegmentedBuffer* buffer_;
    Vector<char> flat_buffer_;
    const char* data_;
  };

 private:
  Iterator GetIteratorAtInternal(size_t position) const;

  size_t size_ = 0;
  Vector<Segment> segments_;
};

// Current CopyAs specializations.
template <>
inline Vector<char> SegmentedBuffer::CopyAs() const {
  Vector<char> buffer;
  buffer.ReserveInitialCapacity(base::checked_cast<wtf_size_t>(size_));
  for (const auto& span : *this) {
    buffer.AppendSpan(span);
  }
  DCHECK_EQ(buffer.size(), size_);
  return buffer;
}

template <>
inline Vector<uint8_t> SegmentedBuffer::CopyAs() const {
  Vector<uint8_t> buffer;
  buffer.ReserveInitialCapacity(base::checked_cast<wtf_size_t>(size_));
  for (const auto& span : *this) {
    buffer.AppendSpan(span);
  }
  DCHECK_EQ(buffer.size(), size_);
  return buffer;
}

template <>
inline std::vector<char> SegmentedBuffer::CopyAs() const {
  std::vector<char> buffer;
  buffer.reserve(size_);

  for (const auto& span : *this)
    buffer.insert(buffer.end(), span.data(), span.data() + span.size());

  DCHECK_EQ(buffer.size(), size_);
  return buffer;
}

template <>
inline std::vector<uint8_t> SegmentedBuffer::CopyAs() const {
  std::vector<uint8_t> buffer;
  buffer.reserve(size_);

  for (const auto& span : *this) {
    buffer.insert(buffer.end(), reinterpret_cast<const uint8_t*>(span.data()),
                  reinterpret_cast<const uint8_t*>(span.data() + span.size()));
  }
  DCHECK_EQ(buffer.size(), size_);
  return buffer;
}

// This is a RefCounted version of the SegmentedBuffer class.
class WTF_EXPORT SharedBuffer : public SegmentedBuffer,
                                public RefCounted<SharedBuffer> {
 public:
  static scoped_refptr<SharedBuffer> Create() {
    return base::AdoptRef(new SharedBuffer);
  }

  static scoped_refptr<SharedBuffer> Create(base::span<const char> data) {
    return base::AdoptRef(new SharedBuffer(data));
  }

  static scoped_refptr<SharedBuffer> Create(
      base::span<const unsigned char> data) {
    return base::AdoptRef(new SharedBuffer(data));
  }

  static scoped_refptr<SharedBuffer> Create(SegmentedBuffer&& data) {
    return base::AdoptRef(new SharedBuffer(std::move(data)));
  }

  static scoped_refptr<SharedBuffer> Create(Vector<char>&&);

 private:
  friend class RefCounted<SharedBuffer>;
  ~SharedBuffer();

  SharedBuffer();
  explicit SharedBuffer(wtf_size_t);
  explicit SharedBuffer(base::span<const char>);
  explicit SharedBuffer(base::span<const unsigned char>);
  explicit SharedBuffer(SegmentedBuffer&&);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SHARED_BUFFER_H_
