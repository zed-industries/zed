// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_BITSET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_BITSET_H_

#include <algorithm>
#include <array>
#include <bit>
#include <cstring>
#include <initializer_list>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"

namespace blink {

// A bitset designed for CSSPropertyIDs.
//
// It's different from std::bitset, in that it provides optimized traversal
// for situations where only a few bits are set (which is the common case for
// e.g. CSS declarations which apply to an element).
//
// The bitset can store a configurable amount of bits for testing purposes,
// (though not more than kNumCSSPropertyIDs).
template <size_t kBits>
class CORE_EXPORT CSSBitsetBase {
 public:
  static_assert(
      kBits <= kNumCSSPropertyIDs,
      "Bit count must not exceed kNumCSSPropertyIDs, as each bit position must "
      "be representable as a CSSPropertyID");
  static_assert(kBits > 0, "Iterator assumes at least one chunk.");

  static const size_t kChunks = (kBits + 63) / 64;

  CSSBitsetBase() : chunks_() {}
  CSSBitsetBase(const CSSBitsetBase<kBits>& o) { *this = o; }

  // This slightly weird construction helps Clang make an actual
  // compile-time static value, until we have constinit.
  template <int N>
  explicit constexpr CSSBitsetBase(const CSSPropertyID (&list)[N])
      : chunks_(CreateChunks(list)) {}

  CSSBitsetBase& operator=(const CSSBitsetBase& o) = default;

  bool operator==(const CSSBitsetBase& o) const { return chunks_ == o.chunks_; }
  bool operator!=(const CSSBitsetBase& o) const { return !(*this == o); }

  inline uint64_t HighPriorityBits() const {
    return chunks_.data()[0] & HighPriorityBitMask();
  }

  inline void Set(CSSPropertyID id) {
    size_t bit = static_cast<size_t>(static_cast<unsigned>(id));
    DCHECK_LT(bit, kBits);
    UNSAFE_TODO(chunks_.data()[bit / 64]) |= (1ull << (bit % 64));
  }

  inline void Or(CSSPropertyID id, bool v) {
    size_t bit = static_cast<size_t>(static_cast<unsigned>(id));
    DCHECK_LT(bit, kBits);
    UNSAFE_TODO(chunks_.data()[bit / 64]) |=
        (static_cast<uint64_t>(v) << (bit % 64));
  }

  inline bool Has(CSSPropertyID id) const {
    size_t bit = static_cast<size_t>(static_cast<unsigned>(id));
    DCHECK_LT(bit, kBits);
    return UNSAFE_TODO(chunks_.data()[bit / 64]) & (1ull << (bit % 64));
  }

  inline bool HasAny() const {
    for (uint64_t chunk : chunks_) {
      if (chunk) {
        return true;
      }
    }
    return false;
  }

  inline void Reset() {
    UNSAFE_TODO(std::memset(chunks_.data(), 0, sizeof(chunks_)));
  }

  // Yields the CSSPropertyIDs which are set.
  class Iterator {
   public:
    // Only meant for internal use (from begin() or end()).
    Iterator(const uint64_t* chunks, size_t chunk_index, size_t index)
        : chunks_(chunks),
          index_(index),
          chunk_index_(chunk_index),
          chunk_(chunks_[0]) {
      DCHECK(index == 0 || index == kBits);
      if (index < kBits) {
        ++*this;  // Go to the first set bit.
      }
    }

    // See BeginAfterHighPriority().
    struct FirstNonHighPriorityTag {};
    Iterator(const uint64_t* chunks, FirstNonHighPriorityTag)
        : chunks_(chunks), chunk_(chunks_[0] & ~HighPriorityBitMask()) {
      ++*this;  // Go to the first set bit.
    }

    inline void operator++() {
      // If there are no more bits set in this chunk,
      // skip to the next nonzero chunk (if any exists).
      while (!chunk_) {
        if (++chunk_index_ >= kChunks) {
          index_ = kBits;
          return;
        }
        chunk_ = UNSAFE_TODO(chunks_[chunk_index_]);
      }
      index_ = chunk_index_ * 64 + std::countr_zero(chunk_);
      chunk_ &= chunk_ - 1;  // Clear the lowest bit.
    }

    inline CSSPropertyID operator*() const {
      DCHECK_LE(index_, static_cast<size_t>(kLastCSSProperty));
      return static_cast<CSSPropertyID>(index_);
    }

    inline bool operator==(const Iterator& o) const {
      return index_ == o.index_;
    }
    inline bool operator!=(const Iterator& o) const {
      return index_ != o.index_;
    }

   private:
    const uint64_t* chunks_;
    // The current bit index this Iterator is pointing to. Note that this is
    // the "global" index, i.e. it has the range [0, kBits]. (It is not a local
    // index with range [0, 64]).
    //
    // Never exceeds kBits.
    size_t index_ = 0;
    // The current chunk index this Iterator is pointing to.
    // Points to kChunks if we are done.
    size_t chunk_index_ = 0;
    // The iterator works by "pre-fetching" the current chunk (corresponding
    // (to the current index), and removing its bits one by one.
    // This is not used (contains junk) for the end() iterator.
    uint64_t chunk_ = 0;
  };

  Iterator begin() const { return Iterator(chunks_.data(), 0, 0); }
  Iterator end() const { return Iterator(chunks_.data(), kChunks, kBits); }

  // Like begin(), except that it skips all high-priority properties
  // (so starts at the first set bit after kLastHighPriorityCSSProperty).
  Iterator BeginAfterHighPriority() const {
    return Iterator(chunks_.data(),
                    typename Iterator::FirstNonHighPriorityTag());
  }

 private:
  std::array<uint64_t, kChunks> chunks_;

  template <int N>
  static constexpr std::array<uint64_t, kChunks> CreateChunks(
      const CSSPropertyID (&list)[N]) {
    std::array<uint64_t, kChunks> chunks{};
    for (CSSPropertyID id : list) {
      unsigned bit = static_cast<unsigned>(id);
      chunks[bit / 64] |= uint64_t{1} << (bit % 64);
    }
    return chunks;
  }

  static constexpr uint64_t HighPriorityBitMask() {
    constexpr int from = static_cast<int>(kFirstHighPriorityCSSProperty);
    constexpr int to_exclusive =
        static_cast<int>(kLastHighPriorityCSSProperty) + 1;
    static_assert(
        from >= 0,
        "This function assumes all high-priority properties fit in 64 bits");
    static_assert(
        to_exclusive < 64,
        "This function assumes all high-priority properties fit in 64 bits");

    // NOTE: We need to_exclusive < 64 to have defined shifts.
    return ((uint64_t{1} << to_exclusive) - 1) & ~((uint64_t{1} << from) - 1);
  }
};

using CSSBitset = CSSBitsetBase<kNumCSSPropertyIDs>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_BITSET_H_
