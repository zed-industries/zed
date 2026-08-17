/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BIT_VECTOR_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BIT_VECTOR_H_

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/dataframe/impl/flex_vector.h"
#include "src/trace_processor/dataframe/impl/slab.h"

namespace perfetto::trace_processor::dataframe::impl {

// A space-efficient vector for storing and manipulating bit values.
//
// BitVector provides a memory-efficient alternative to vectors of boolean
// values, packing 64 boolean values into a single 64-bit word. This class
// offers efficient operations for common bit manipulation tasks including:
//
// - Setting, clearing, and testing individual bits
// - Efficient iteration over set/unset bits
// - Population counting within bit ranges
// - Filtering operations based on bit patterns
//
// Performance advantages over std::vector<bool> or other alternatives:
// - No branching in the hot path for most operations
// - Uses bitwise operations for maximum efficiency
// - Aligned storage for better memory access patterns
// - Specialized bit manipulation algorithms optimized for performance
struct BitVector {
 public:
  // Default constructor creates an empty bit vector.
  BitVector() = default;

  // Movable but not copyable.
  BitVector(BitVector&&) = default;
  BitVector& operator=(BitVector&&) = default;
  BitVector(const BitVector&) = delete;
  BitVector& operator=(const BitVector&) = delete;

  // Allocates a new BitVector with `size` unset bits.
  //
  // size: Size for how many bits to add to the BitVector.
  // Returns a BitVector with the given size and with all bits set to `value`.
  static BitVector CreateWithSize(uint64_t size, bool value = false) {
    if (size == 0) {
      return {};
    }
    auto words = FlexVector<uint64_t>::CreateWithSize((size + 63u) / 64u);
    memset(words.data(), value ? 0xFF : 0, words.size() * sizeof(uint64_t));
    // Clear any bits past `size`.
    if (size % 64u != 0u) {
      words.back() &= (1ull << (size % 64u)) - 1ull;
    }
    PERFETTO_DCHECK(size <= words.size() * 64u);
    return BitVector(std::move(words), size);
  }

  // Adds a bit to the end of the vector.
  //
  // bit: The boolean value to add to the end of the BitVector.
  PERFETTO_ALWAYS_INLINE void push_back(bool bit) {
    if (PERFETTO_UNLIKELY(size_ % 64ull == 0ull)) {
      words_.push_back(0ull);
    }
    words_[size_ / 64ull] |= static_cast<uint64_t>(bit) << (size_ % 64ull);
    ++size_;
  }

  // Changes the value of a bit at the specified index.
  //
  // i: The index of the bit to change.
  // bit: The new boolean value.
  PERFETTO_ALWAYS_INLINE void change(uint64_t i, bool bit) {
    PERFETTO_DCHECK(i < size_);
    uint64_t n = i % 64ull;
    words_[i / 64ull] =
        (words_[i / 64ull] & ~(1ull << n)) | (static_cast<uint64_t>(bit) << n);
  }

  // Changes the value of a bit that is known to be currently unset.
  // This is more efficient than change() when the current bit value is known to
  // be 0.
  //
  // i: The index of the bit to change.
  // bit: The new boolean value.
  PERFETTO_ALWAYS_INLINE void change_assume_unset(size_t i, bool bit) {
    PERFETTO_DCHECK(i < size_);
    uint64_t n = i % 64ull;
    words_[i / 64ull] |= static_cast<uint64_t>(bit) << n;
  }

  // Sets the bit at the specified position to true.
  //
  // i: The index of the bit to set.
  PERFETTO_ALWAYS_INLINE void set(uint64_t i) {
    PERFETTO_DCHECK(i < size_);
    words_[i / 64ull] |= 1ull << (i % 64ull);
  }

  // Sets the bit at the specified position to false.
  //
  // i: The index of the bit to clear.
  PERFETTO_ALWAYS_INLINE void clear(uint64_t i) {
    PERFETTO_DCHECK(i < size_);
    words_[i / 64ull] &= ~(1ull << (i % 64ull));
  }

  // Checks if the bit at the specified position is set.
  //
  // i: The index of the bit to check.
  // Returns true if the bit is set, false otherwise.
  PERFETTO_ALWAYS_INLINE bool is_set(uint64_t i) const {
    PERFETTO_DCHECK(i < size_);
    return (words_[i / 64ull] >> (i % 64ull)) & 1ull;
  }

  // Counts how many bits are set in the same word up to a given position.
  //
  // i: The index position to check up to.
  // Returns the number of set bits in the same 64-bit word as the bit at
  // position i up to position in word (i % 64).
  PERFETTO_ALWAYS_INLINE uint64_t
  count_set_bits_until_in_word(uint64_t i) const {
    PERFETTO_DCHECK(i < size_);
    return static_cast<uint64_t>(
        PERFETTO_POPCOUNT(words_[i / 64ull] & ((1ull << (i % 64ull)) - 1ull)));
  }

  // Filters a sequence by keeping only elements whose bit is set (or not set).
  //
  // This function takes a source array and copies elements to a target array
  // only if the corresponding bit in the BitVector matches the desired state.
  //
  // invert: If true, copies elements where the bit is NOT set.
  // source_begin: Pointer to the start of the source array.
  // source_end: Pointer to the end of the source array.
  // target: Pointer to the target array where filtered items will be stored.
  // Returns a pointer to the end of the written data in the target array.
  template <bool invert = false>
  [[nodiscard]] PERFETTO_ALWAYS_INLINE uint32_t* PackLeft(
      const uint32_t* source_begin,
      const uint32_t* source_end,
      uint32_t* target) const {
    uint32_t* out = target;
    for (const uint32_t* s = source_begin; s != source_end; ++s) {
      bool res = (words_[*s / 64ull] >> (*s % 64ull)) & 1ull;
      if constexpr (invert) {
        res = !res;
      }
      *out = *s;
      out += res;
    }
    return out;
  }

  // Computes the prefix sum of set bits for each 64-bit word.
  //
  // This creates an array where each element contains the count of set bits
  // in all preceding 64-bit words, useful for various bit-manipulation
  // algorithms.
  //
  // Returns a Slab containing the prefix sum counts.
  PERFETTO_ALWAYS_INLINE Slab<uint32_t> PrefixPopcount() const {
    Slab<uint32_t> res = Slab<uint32_t>::Alloc((size_ + 63ull) / 64ull);
    uint32_t accum = 0;
    for (uint32_t i = 0; i < (size_ + 63ull) / 64ull; ++i) {
      res[i] = accum;
      accum += static_cast<uint32_t>(PERFETTO_POPCOUNT(words_[i]));
    }
    return res;
  }

  // Returns the number of bits in the vector.
  PERFETTO_ALWAYS_INLINE uint64_t size() const { return size_; }

 private:
  // Constructor used by Alloc.
  explicit BitVector(FlexVector<uint64_t> data, uint64_t size)
      : words_(std::move(data)), size_(size) {}

  // The underlying storage as 64-bit words.
  FlexVector<uint64_t> words_;

  // Number of bits in the vector.
  uint64_t size_ = 0;
};

}  // namespace perfetto::trace_processor::dataframe::impl

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BIT_VECTOR_H_
