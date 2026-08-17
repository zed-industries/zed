//===-- String utils --------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// Standalone string utility functions. Utilities requiring memory allocations
// should be placed in allocating_string_utils.h instead.
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STRING_STRING_UTILS_H
#define LLVM_LIBC_SRC_STRING_STRING_UTILS_H

#include "hdr/types/size_t.h"
#include "src/__support/CPP/bitset.h"
#include "src/__support/CPP/type_traits.h" // cpp::is_same_v
#include "src/__support/macros/config.h"
#include "src/__support/macros/optimization.h" // LIBC_UNLIKELY
#include "src/string/memory_utils/inline_bzero.h"
#include "src/string/memory_utils/inline_memcpy.h"

namespace LIBC_NAMESPACE_DECL {
namespace internal {

template <typename Word> LIBC_INLINE constexpr Word repeat_byte(Word byte) {
  constexpr size_t BITS_IN_BYTE = 8;
  constexpr size_t BYTE_MASK = 0xff;
  Word result = 0;
  byte = byte & BYTE_MASK;
  for (size_t i = 0; i < sizeof(Word); ++i)
    result = (result << BITS_IN_BYTE) | byte;
  return result;
}

// The goal of this function is to take in a block of arbitrary size and return
// if it has any bytes equal to zero without branching. This is done by
// transforming the block such that zero bytes become non-zero and non-zero
// bytes become zero.
// The first transformation relies on the properties of carrying in arithmetic
// subtraction. Specifically, if 0x01 is subtracted from a byte that is 0x00,
// then the result for that byte must be equal to 0xff (or 0xfe if the next byte
// needs a carry as well).
// The next transformation is a simple mask. All zero bytes will have the high
// bit set after the subtraction, so each byte is masked with 0x80. This narrows
// the set of bytes that result in a non-zero value to only zero bytes and bytes
// with the high bit and any other bit set.
// The final transformation masks the result of the previous transformations
// with the inverse of the original byte. This means that any byte that had the
// high bit set will no longer have it set, narrowing the list of bytes which
// result in non-zero values to just the zero byte.
template <typename Word> LIBC_INLINE constexpr bool has_zeroes(Word block) {
  constexpr Word LOW_BITS = repeat_byte<Word>(0x01);
  constexpr Word HIGH_BITS = repeat_byte<Word>(0x80);
  Word subtracted = block - LOW_BITS;
  Word inverted = ~block;
  return (subtracted & inverted & HIGH_BITS) != 0;
}

template <typename Word>
LIBC_INLINE size_t string_length_wide_read(const char *src) {
  const char *char_ptr = src;
  // Step 1: read 1 byte at a time to align to block size
  for (; reinterpret_cast<uintptr_t>(char_ptr) % sizeof(Word) != 0;
       ++char_ptr) {
    if (*char_ptr == '\0')
      return static_cast<size_t>(char_ptr - src);
  }
  // Step 2: read blocks
  for (const Word *block_ptr = reinterpret_cast<const Word *>(char_ptr);
       !has_zeroes<Word>(*block_ptr); ++block_ptr) {
    char_ptr = reinterpret_cast<const char *>(block_ptr);
  }
  // Step 3: find the zero in the block
  for (; *char_ptr != '\0'; ++char_ptr) {
    ;
  }
  return static_cast<size_t>(char_ptr - src);
}

// Returns the length of a string, denoted by the first occurrence
// of a null terminator.
template <typename T> LIBC_INLINE size_t string_length(const T *src) {
#ifdef LIBC_COPT_STRING_UNSAFE_WIDE_READ
  // Unsigned int is the default size for most processors, and on x86-64 it
  // performs better than larger sizes when the src pointer can't be assumed to
  // be aligned to a word boundary, so it's the size we use for reading the
  // string a block at a time.
  if constexpr (cpp::is_same_v<T, char>)
    return string_length_wide_read<unsigned int>(src);
#else
  size_t length;
  for (length = 0; *src; ++src, ++length)
    ;
  return length;
#endif
}

template <typename Word>
LIBC_INLINE void *find_first_character_wide_read(const unsigned char *src,
                                                 unsigned char ch, size_t n) {
  const unsigned char *char_ptr = src;
  size_t cur = 0;

  // Step 1: read 1 byte at a time to align to block size
  for (; reinterpret_cast<uintptr_t>(char_ptr) % sizeof(Word) != 0 && cur < n;
       ++char_ptr, ++cur) {
    if (*char_ptr == ch)
      return const_cast<unsigned char *>(char_ptr);
  }

  const Word ch_mask = repeat_byte<Word>(ch);

  // Step 2: read blocks
  for (const Word *block_ptr = reinterpret_cast<const Word *>(char_ptr);
       !has_zeroes<Word>((*block_ptr) ^ ch_mask) && cur < n;
       ++block_ptr, cur += sizeof(Word)) {
    char_ptr = reinterpret_cast<const unsigned char *>(block_ptr);
  }

  // Step 3: find the match in the block
  for (; *char_ptr != ch && cur < n; ++char_ptr, ++cur) {
    ;
  }

  if (*char_ptr != ch || cur >= n)
    return static_cast<void *>(nullptr);

  return const_cast<unsigned char *>(char_ptr);
}

LIBC_INLINE void *find_first_character_byte_read(const unsigned char *src,
                                                 unsigned char ch, size_t n) {
  for (; n && *src != ch; --n, ++src)
    ;
  return n ? const_cast<unsigned char *>(src) : nullptr;
}

// Returns the first occurrence of 'ch' within the first 'n' characters of
// 'src'. If 'ch' is not found, returns nullptr.
LIBC_INLINE void *find_first_character(const unsigned char *src,
                                       unsigned char ch, size_t max_strlen) {
#ifdef LIBC_COPT_STRING_UNSAFE_WIDE_READ
  // If the maximum size of the string is small, the overhead of aligning to a
  // word boundary and generating a bitmask of the appropriate size may be
  // greater than the gains from reading larger chunks. Based on some testing,
  // the crossover point between when it's faster to just read bytewise and read
  // blocks is somewhere between 16 and 32, so 4 times the size of the block
  // should be in that range.
  // Unsigned int is used for the same reason as in strlen.
  using BlockType = unsigned int;
  if (max_strlen > (sizeof(BlockType) * 4)) {
    return find_first_character_wide_read<BlockType>(src, ch, max_strlen);
  }
#endif
  return find_first_character_byte_read(src, ch, max_strlen);
}

// Returns the maximum length span that contains only characters not found in
// 'segment'. If no characters are found, returns the length of 'src'.
LIBC_INLINE size_t complementary_span(const char *src, const char *segment) {
  const char *initial = src;
  cpp::bitset<256> bitset;

  for (; *segment; ++segment)
    bitset.set(*reinterpret_cast<const unsigned char *>(segment));
  for (; *src && !bitset.test(*reinterpret_cast<const unsigned char *>(src));
       ++src)
    ;
  return static_cast<size_t>(src - initial);
}

// Given the similarities between strtok and strtok_r, we can implement both
// using a utility function. On the first call, 'src' is scanned for the
// first character not found in 'delimiter_string'. Once found, it scans until
// the first character in the 'delimiter_string' or the null terminator is
// found. We define this span as a token. The end of the token is appended with
// a null terminator, and the token is returned. The point where the last token
// is found is then stored within 'context' for subsequent calls. Subsequent
// calls will use 'context' when a nullptr is passed in for 'src'. Once the null
// terminating character is reached, returns a nullptr.
template <bool SkipDelim = true>
LIBC_INLINE char *string_token(char *__restrict src,
                               const char *__restrict delimiter_string,
                               char **__restrict saveptr) {
  // Return nullptr immediately if both src AND saveptr are nullptr
  if (LIBC_UNLIKELY(src == nullptr && ((src = *saveptr) == nullptr)))
    return nullptr;

  static_assert(sizeof(char) == sizeof(cpp::byte),
                "bitset of 256 assumes char is 8 bits");
  cpp::bitset<256> delimiter_set;
  for (; *delimiter_string != '\0'; ++delimiter_string)
    delimiter_set.set(static_cast<size_t>(*delimiter_string));

  if constexpr (SkipDelim)
    for (; *src != '\0' && delimiter_set.test(static_cast<size_t>(*src)); ++src)
      ;
  if (*src == '\0') {
    *saveptr = src;
    return nullptr;
  }
  char *token = src;
  for (; *src != '\0'; ++src) {
    if (delimiter_set.test(static_cast<size_t>(*src))) {
      *src = '\0';
      ++src;
      break;
    }
  }
  *saveptr = src;
  return token;
}

LIBC_INLINE size_t strlcpy(char *__restrict dst, const char *__restrict src,
                           size_t size) {
  size_t len = internal::string_length(src);
  if (!size)
    return len;
  size_t n = len < size - 1 ? len : size - 1;
  inline_memcpy(dst, src, n);
  dst[n] = '\0';
  return len;
}

template <bool ReturnNull = true>
LIBC_INLINE constexpr static char *strchr_implementation(const char *src,
                                                         int c) {
  char ch = static_cast<char>(c);
  for (; *src && *src != ch; ++src)
    ;
  char *ret = ReturnNull ? nullptr : const_cast<char *>(src);
  return *src == ch ? const_cast<char *>(src) : ret;
}

LIBC_INLINE constexpr static char *strrchr_implementation(const char *src,
                                                          int c) {
  char ch = static_cast<char>(c);
  char *last_occurrence = nullptr;
  while (true) {
    if (*src == ch)
      last_occurrence = const_cast<char *>(src);
    if (!*src)
      return last_occurrence;
    ++src;
  }
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif //  LLVM_LIBC_SRC_STRING_STRING_UTILS_H
