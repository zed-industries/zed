// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_LITERAL_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_LITERAL_BUFFER_H_

#include <algorithm>
#include <bit>
#include <memory>
#include <type_traits>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_encoding.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

// For ASAN builds, disable inline buffers completely as they cause various
// issues.
#ifdef ANNOTATE_CONTIGUOUS_CONTAINER
#define BUFFER_INLINE_CAPACITY 0
#else
#define BUFFER_INLINE_CAPACITY kInlineSize
#endif

// LiteralBufferBase is an optimized version of Vector for LChar and UChar
// characters. In particular `AddChar` is faster than `push_back`, since
// it avoids unnecessary register spills. See https://crbug.com/1205338.
// Use one of the concrete implementations: LCharLiteralBuffer or
// UCharLiteralBuffer.
template <typename T, wtf_size_t kInlineSize>
class LiteralBufferBase {
  static_assert(std::is_same<LChar, T>::value || std::is_same<UChar, T>::value,
                "T must be a character type");

 public:
  using iterator = base::CheckedContiguousIterator<const T>;

  ~LiteralBufferBase() {
    if (!is_stored_inline())
      WTF::Partitions::BufferFree(begin_);
  }

  ALWAYS_INLINE const T* data() const { return begin_; }
  ALWAYS_INLINE wtf_size_t size() const {
    return base::checked_cast<wtf_size_t>(end_ - begin_);
  }

  // Iterators, so this type meets the requirements of
  // `std::ranges::contiguous_range`.
  ALWAYS_INLINE iterator begin() const {
    return UNSAFE_TODO(iterator(begin_, end_));
  }
  ALWAYS_INLINE iterator end() const {
    return UNSAFE_TODO(iterator(begin_, end_, end_));
  }

  ALWAYS_INLINE bool IsEmpty() const { return begin_ == end_; }

  ALWAYS_INLINE const T& operator[](wtf_size_t index) const {
    CHECK_GT(size(), index);
    // SAFETY: Check above.
    return UNSAFE_BUFFERS(begin_[index]);
  }

 protected:
  LiteralBufferBase() = default;

  // Clear without freeing any storage.
  ALWAYS_INLINE void ClearImpl() { end_ = begin_; }

  ALWAYS_INLINE void AddCharImpl(T val) {
    if (end_ == end_of_storage_) [[unlikely]] {
      end_ = Grow();
    }
    UNSAFE_TODO(*end_++) = val;
  }

  template <typename OtherT, wtf_size_t kOtherSize>
  void AppendLiteralImpl(const LiteralBufferBase<OtherT, kOtherSize>& val) {
    static_assert(sizeof(T) >= sizeof(OtherT),
                  "T is not big enough to contain OtherT");
    size_t count = val.size();
    size_t new_size = size() + count;
    if (capacity() < new_size)
      Grow(new_size);
    std::copy_n(val.data(), count, end_);
    UNSAFE_TODO(end_ += count);
  }

  template <wtf_size_t kOtherInlineSize>
  void Copy(const LiteralBufferBase<T, kOtherInlineSize>& other) {
    wtf_size_t other_size = other.size();
    if (capacity() < other_size) {
      // Create large-enough heap-allocated storage.
      if (!is_stored_inline())
        WTF::Partitions::BufferFree(begin_);
      begin_ = static_cast<T*>(WTF::Partitions::BufferMalloc(
          AllocationSize(other_size), "LiteralBufferBase"));
      end_of_storage_ = UNSAFE_TODO(begin_ + other_size);
    }
    std::copy_n(other.data(), other_size, begin_);
    end_ = UNSAFE_TODO(begin_ + other_size);
  }

  void Move(LiteralBufferBase&& other) {
    DCHECK_NE(this, &other);
    if (!other.is_stored_inline()) {
      if (!is_stored_inline())
        WTF::Partitions::BufferFree(begin_);
      begin_ = other.begin_;
      end_ = other.end_;
      end_of_storage_ = other.end_of_storage_;
      other.begin_ = &other.inline_storage[0];
      other.end_ = other.begin_;
      other.end_of_storage_ =
          UNSAFE_TODO(other.begin_ + BUFFER_INLINE_CAPACITY);
    } else {
      DCHECK_GE(capacity(), other.size());  // Sanity check.
      wtf_size_t other_size = other.size();
      std::copy_n(other.data(), other_size, begin_);
      end_ = UNSAFE_TODO(begin_ + other_size);
    }
  }

 private:
  size_t AllocationSize(size_t capacity) {
    return WTF::PartitionAllocator::QuantizedSize<T>(capacity);
  }

  ALWAYS_INLINE size_t capacity() const { return end_of_storage_ - begin_; }

  ALWAYS_INLINE bool is_stored_inline() const {
    return begin_ == &inline_storage[0];
  }

  size_t RoundUpToPowerOfTwo(size_t value) {
    constexpr size_t digits = 8 * sizeof(size_t);
    static_assert(digits == 32 || digits == 64,
                  "size_t must be either 32 or 64 bits");
    DCHECK_LE(value, size_t{1} << (digits - 1));
    if (value)
      --value;
    return size_t{1} << (digits - std::countl_zero(value));
  }

  // Grows the backing store by a factor of two. Returns the new end of the used
  // storage (this reduces binary size).
  NOINLINE T* Grow() { return Grow(0); }

  // Grows the backing store by a factor of two, and at least to `min_capacity`.
  NOINLINE T* Grow(size_t min_capacity) {
    DCHECK_GE(end_, begin_);
    size_t in_use = end_ - begin_;
    size_t new_capacity =
        RoundUpToPowerOfTwo(std::max(min_capacity, 2 * capacity()));
    T* new_storage = static_cast<T*>(WTF::Partitions::BufferMalloc(
        AllocationSize(new_capacity), "LiteralBufferBase"));
    std::copy_n(begin_, in_use, new_storage);
    if (!is_stored_inline())
      WTF::Partitions::BufferFree(begin_);
    begin_ = new_storage;
    end_ = UNSAFE_TODO(new_storage + in_use);
    end_of_storage_ = UNSAFE_TODO(new_storage + new_capacity);
    return end_;
  }

  // NOTE: we use pointers to the beginning and the end of the buffer, instead
  // of tuple (begin, size, capacity). This makes access of the next characters
  // faster when `AddChar` is inlined, since `end_` is readily available in a
  // register.
  T* begin_ = &inline_storage[0];
  T* end_ = begin_;
  T* end_of_storage_ = UNSAFE_TODO(begin_ + BUFFER_INLINE_CAPACITY);
  T inline_storage[BUFFER_INLINE_CAPACITY];
};

template <wtf_size_t kInlineSize>
class LCharLiteralBuffer : public LiteralBufferBase<LChar, kInlineSize> {
 public:
  LCharLiteralBuffer() = default;
  LCharLiteralBuffer(const LCharLiteralBuffer& other) { *this = other; }
  LCharLiteralBuffer(LCharLiteralBuffer&& other) { *this = std::move(other); }

  ~LCharLiteralBuffer() = default;

  template <wtf_size_t kOtherInlineSize>
  LCharLiteralBuffer& operator=(
      const LCharLiteralBuffer<kOtherInlineSize>& other) {
    if (this->data() != other.data())
      this->Copy(other);
    return *this;
  }

  LCharLiteralBuffer& operator=(LCharLiteralBuffer&& other) {
    if (this != &other)
      this->Move(std::move(other));
    return *this;
  }

  // Clear without freeing any storage.
  ALWAYS_INLINE void clear() { this->ClearImpl(); }

  ALWAYS_INLINE void AddChar(LChar val) { this->AddCharImpl(val); }

  String AsString() const { return String(*this); }
};

template <wtf_size_t kInlineSize>
class UCharLiteralBuffer : public LiteralBufferBase<UChar, kInlineSize> {
 public:
  UCharLiteralBuffer() = default;
  UCharLiteralBuffer(const UCharLiteralBuffer& other) { *this = other; }
  UCharLiteralBuffer(UCharLiteralBuffer&& other) { *this = std::move(other); }

  ~UCharLiteralBuffer() = default;

  template <wtf_size_t kOtherInlineSize>
  UCharLiteralBuffer& operator=(
      const UCharLiteralBuffer<kOtherInlineSize>& other) {
    if (this->data() == other.data())
      return *this;
    this->Copy(other);
    bitwise_or_all_chars_ = other.bitwise_or_all_chars_;
    return *this;
  }

  UCharLiteralBuffer& operator=(const UCharLiteralBuffer& other) {
    if (this == &other)
      return *this;
    this->Copy(other);
    bitwise_or_all_chars_ = other.bitwise_or_all_chars_;
    return *this;
  }

  UCharLiteralBuffer& operator=(UCharLiteralBuffer&& other) {
    if (this == &other)
      return *this;
    const UChar other_bitwise_or_all_chars = other.bitwise_or_all_chars_;
    this->Move(std::move(other));
    bitwise_or_all_chars_ = other_bitwise_or_all_chars;
    return *this;
  }

  // Clear without freeing any storage.
  ALWAYS_INLINE void clear() {
    this->ClearImpl();
    bitwise_or_all_chars_ = 0;
  }

  ALWAYS_INLINE void AddChar(UChar val) {
    this->AddCharImpl(val);
    bitwise_or_all_chars_ |= val;
  }

  template <wtf_size_t kOtherSize>
  void AppendLiteral(const LCharLiteralBuffer<kOtherSize>& val) {
    this->AppendLiteralImpl(val);
  }

  String AsString() const {
    if (Is8Bit()) {
      return String::Make8BitFrom16BitSource(base::span(*this));
    }
    return String(*this);
  }

  AtomicString AsAtomicString() const {
    return AtomicString(*this, Is8Bit()
                                   ? WTF::AtomicStringUCharEncoding::kIs8Bit
                                   : WTF::AtomicStringUCharEncoding::kIs16Bit);
  }

  ALWAYS_INLINE bool Is8Bit() const {
    return (bitwise_or_all_chars_ & ~0xff) == 0;
  }

 private:
  // Needed for operator=.
  template <wtf_size_t kOtherInlineSize>
  friend class UCharLiteralBuffer;

  // Bitwise OR of all characters in our buffer. We actually
  // only ever care if anyone of them have any high (>= 8) bits set,
  // but just checking that at the end is faster than branching
  // all the time.
  UChar bitwise_or_all_chars_ = 0;
};

#undef BUFFER_INLINE_CAPACITY

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_LITERAL_BUFFER_H_
