//===-- Utils to test conformance of mem functions ------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_STRING_MEMORY_UTILS_MEMORY_CHECK_UTILS_H
#define LIBC_TEST_SRC_STRING_MEMORY_UTILS_MEMORY_CHECK_UTILS_H

#include "src/__support/CPP/span.h"
#include "src/__support/libc_assert.h" // LIBC_ASSERT
#include "src/__support/macros/config.h"
#include "src/__support/macros/sanitizer.h"
#include "src/string/memory_utils/utils.h"
#include <stddef.h> // size_t
#include <stdint.h> // uintxx_t
#include <stdlib.h> // malloc/free

namespace LIBC_NAMESPACE_DECL {

// Simple structure to allocate a buffer of a particular size.
// When ASAN is present it also poisons the whole memory.
// This is a utility class to be used by Buffer below, do not use directly.
struct PoisonedBuffer {
  PoisonedBuffer(size_t size) : ptr((char *)malloc(size)) {
    ASAN_POISON_MEMORY_REGION(ptr, size);
  }
  ~PoisonedBuffer() { free(ptr); }

protected:
  char *ptr = nullptr;
};

// Simple structure to allocate a buffer (aligned or not) of a particular size.
// It is backed by a wider buffer that is marked poisoned when ASAN is present.
// The requested region is unpoisoned, this allows catching out of bounds
// accesses.
enum class Aligned : bool { NO = false, YES = true };
struct Buffer : private PoisonedBuffer {
  static constexpr size_t kAlign = 64;
  static constexpr size_t kLeeway = 2 * kAlign;
  Buffer(size_t size, Aligned aligned = Aligned::YES)
      : PoisonedBuffer(size + kLeeway), size(size) {
    offset_ptr = ptr;
    offset_ptr += distance_to_next_aligned<kAlign>(ptr);
    if (aligned == Aligned::NO)
      ++offset_ptr;
    ASAN_UNPOISON_MEMORY_REGION(offset_ptr, size);
  }
  cpp::span<char> span() { return cpp::span<char>(offset_ptr, size); }

private:
  size_t size = 0;
  char *offset_ptr = nullptr;
};

inline char GetRandomChar() {
  static constexpr const uint64_t a = 1103515245;
  static constexpr const uint64_t c = 12345;
  static constexpr const uint64_t m = 1ULL << 31;
  static uint64_t seed = 123456789;
  seed = (a * seed + c) % m;
  return static_cast<char>(seed);
}

// Randomize the content of the buffer.
inline void Randomize(cpp::span<char> buffer) {
  for (auto &current : buffer)
    current = GetRandomChar();
}

// Copy one span to another.
inline void ReferenceCopy(cpp::span<char> dst, const cpp::span<char> src) {
  for (size_t i = 0; i < dst.size(); ++i)
    dst[i] = src[i];
}

inline bool IsEqual(const cpp::span<char> a, const cpp::span<char> b) {
  LIBC_ASSERT(a.size() == b.size());
  for (size_t i = 0; i < a.size(); ++i)
    if (a[i] != b[i])
      return false;
  return true;
}

// Checks that FnImpl implements the memcpy semantic.
template <auto FnImpl>
inline bool CheckMemcpy(cpp::span<char> dst, cpp::span<char> src, size_t size) {
  Randomize(dst);
  FnImpl(dst, src, size);
  return IsEqual(dst, src);
}

// Checks that FnImpl implements the memset semantic.
template <auto FnImpl>
inline bool CheckMemset(cpp::span<char> dst, uint8_t value, size_t size) {
  Randomize(dst);
  FnImpl(dst, value, size);
  for (char c : dst)
    if (c != (char)value)
      return false;
  return true;
}

// Checks that FnImpl implements the bcmp semantic.
template <auto FnImpl>
inline bool CheckBcmp(cpp::span<char> span1, cpp::span<char> span2,
                      size_t size) {
  ReferenceCopy(span2, span1);
  // Compare equal
  if (int cmp = FnImpl(span1, span2, size); cmp != 0)
    return false;
  // Compare not equal if any byte differs
  for (size_t i = 0; i < size; ++i) {
    ++span2[i];
    if (int cmp = FnImpl(span1, span2, size); cmp == 0)
      return false;
    if (int cmp = FnImpl(span2, span1, size); cmp == 0)
      return false;
    --span2[i];
  }
  return true;
}

// Checks that FnImpl implements the memcmp semantic.
template <auto FnImpl>
inline bool CheckMemcmp(cpp::span<char> span1, cpp::span<char> span2,
                        size_t size) {
  ReferenceCopy(span2, span1);
  // Compare equal
  if (int cmp = FnImpl(span1, span2, size); cmp != 0)
    return false;
  // Compare not equal if any byte differs
  for (size_t i = 0; i < size; ++i) {
    ++span2[i];
    int ground_truth = __builtin_memcmp(span1.data(), span2.data(), size);
    if (ground_truth > 0) {
      if (int cmp = FnImpl(span1, span2, size); cmp <= 0)
        return false;
      if (int cmp = FnImpl(span2, span1, size); cmp >= 0)
        return false;
    } else {
      if (int cmp = FnImpl(span1, span2, size); cmp >= 0)
        return false;
      if (int cmp = FnImpl(span2, span1, size); cmp <= 0)
        return false;
    }
    --span2[i];
  }
  return true;
}

inline uint16_t Checksum(cpp::span<char> dst) {
  // We use Fletcher16 as it is trivial to implement.
  uint16_t sum1 = 0;
  uint16_t sum2 = 0;
  for (char c : dst) {
    sum1 = (sum1 + static_cast<uint16_t>(c)) % 255U;
    sum2 = (sum2 + sum1) % 255U;
  }
  return static_cast<uint16_t>((sum2 << 8) | sum1);
}

template <auto FnImpl>
inline bool CheckMemmove(cpp::span<char> dst, cpp::span<char> src) {
  LIBC_ASSERT(dst.size() == src.size());
  // Memmove can override the src buffer. Technically we should save it into a
  // temporary buffer so we can check that 'dst' is equal to what 'src' was
  // before we called the function. To save on allocation and copy we use a
  // checksum instead.
  const auto src_checksum = Checksum(src);
  FnImpl(dst, src, dst.size());
  return Checksum(dst) == src_checksum;
}

// Checks that FnImpl implements the memmove semantic.
//  - Buffer size should be greater than 2 * size + 1.
//  - Overlap refers to the number of bytes in common between the two buffers:
//    - Negative means buffers are disjoint
//    - zero mean they overlap exactly
//  - Caller is responsible for randomizing the buffer.
template <auto FnImpl>
inline bool CheckMemmove(cpp::span<char> buffer, size_t size, int overlap) {
  LIBC_ASSERT(buffer.size() > (2 * size + 1));
  const size_t half_size = buffer.size() / 2;
  LIBC_ASSERT(static_cast<size_t>(overlap >= 0 ? overlap : -overlap) <
              half_size);
  cpp::span<char> head =
      buffer.first(half_size + static_cast<size_t>(overlap)).last(size);
  cpp::span<char> tail = buffer.last(half_size).first(size);
  LIBC_ASSERT(head.size() == size);
  LIBC_ASSERT(tail.size() == size);
  // dst before src
  if (!CheckMemmove<FnImpl>(head, tail))
    return false;
  // dst after src
  if (!CheckMemmove<FnImpl>(tail, head))
    return false;
  return true;
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LIBC_TEST_SRC_STRING_MEMORY_UTILS_MEMORY_CHECK_UTILS_H
