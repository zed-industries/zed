/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_BIT_VECTOR_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_BIT_VECTOR_H_

#include <algorithm>
#include <cstdint>
#include <initializer_list>
#include <iterator>
#include <utility>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"

namespace perfetto {
namespace protos::pbzero {
class SerializedColumn_BitVector;
class SerializedColumn_BitVector_Decoder;
}  // namespace protos::pbzero

namespace trace_processor {
namespace internal {

class BaseIterator;
class SetBitsIterator;

}  // namespace internal

// A BitVector which compactly stores a vector of bools using a single bit
// for each bool.
class BitVector {
 public:
  static constexpr uint32_t kBitsInWord = 64;

  // Builder class which allows efficiently creating a BitVector by appending
  // words. Using this class is generally far more efficient than trying to set
  // bits directly in a BitVector or even appending one bit at a time.
  class Builder {
   public:
    // Creates a Builder for building a BitVector of |size| bits.
    explicit Builder(uint32_t size, uint32_t skip = 0)
        : words_(BlockCount(size) * Block::kWords),
          global_bit_offset_(skip),
          size_(size),
          skipped_blocks_(skip / Block::kBits) {
      PERFETTO_CHECK(global_bit_offset_ <= size_);
    }

    // Appends a single bit to the builder.
    // Note: |AppendWord| is far more efficient than this method so should be
    // preferred.
    void Append(bool value) {
      PERFETTO_DCHECK(global_bit_offset_ < size_);

      words_[global_bit_offset_ / BitWord::kBits] |=
          static_cast<uint64_t>(value) << global_bit_offset_ % BitWord::kBits;
      global_bit_offset_++;
    }

    // Appends a whole word to the Builder. Builder has to end on a word
    // boundary before calling this function.
    void AppendWord(uint64_t word) {
      PERFETTO_DCHECK(global_bit_offset_ % BitWord::kBits == 0);
      PERFETTO_DCHECK(global_bit_offset_ + BitWord::kBits <= size_);

      words_[global_bit_offset_ / BitWord::kBits] = word;
      global_bit_offset_ += BitWord::kBits;
    }

    // Creates a BitVector from this Builder.
    BitVector Build() && {
      if (size_ == 0)
        return {};

      std::vector<uint32_t> counts(BlockCount(size_));
      PERFETTO_CHECK(skipped_blocks_ <= counts.size());
      for (uint32_t i = skipped_blocks_ + 1; i < counts.size(); ++i) {
        counts[i] = counts[i - 1] +
                    ConstBlock(&words_[Block::kWords * (i - 1)]).CountSetBits();
      }
      return {std::move(words_), std::move(counts), size_};
    }

    // Returns the number of bits which are in complete words which can be
    // appended to this builder before having to fallback to |Append| due to
    // being close to the end.
    uint32_t BitsInCompleteWordsUntilFull() const {
      uint32_t next_word = WordCount(global_bit_offset_);
      uint32_t end_word = WordFloor(size_);
      uint32_t complete_words = next_word < end_word ? end_word - next_word : 0;
      return complete_words * BitWord::kBits;
    }

    // Returns the number of bits which should be appended using |Append| either
    // hitting a word boundary (and thus able to use |AppendWord|) or until the
    // BitVector is full (i.e. no more Appends should happen), whichever would
    // happen first.
    uint32_t BitsUntilWordBoundaryOrFull() const {
      if (global_bit_offset_ == 0 && size_ < BitWord::kBits) {
        return size_;
      }
      uint8_t word_bit_offset = global_bit_offset_ % BitWord::kBits;
      return std::min(BitsUntilFull(),
                      (BitWord::kBits - word_bit_offset) % BitWord::kBits);
    }

    // Returns the number of bits which should be appended using |Append| before
    // hitting a word boundary (and thus able to use |AppendWord|) or until the
    // BitVector is full (i.e. no more Appends should happen).
    uint32_t BitsUntilFull() const { return size_ - global_bit_offset_; }

   private:
    std::vector<uint64_t> words_;
    uint32_t global_bit_offset_ = 0;
    uint32_t size_ = 0;
    uint32_t skipped_blocks_ = 0;
  };

  // Creates an empty BitVector.
  BitVector();

  BitVector(std::initializer_list<bool> init);

  // Creates a BitVector of |count| size filled with |value|.
  explicit BitVector(uint32_t count, bool value = false);

  BitVector(const BitVector&) = delete;
  BitVector& operator=(const BitVector&) = delete;

  // Enable moving BitVectors as they have no unmovable state.
  BitVector(BitVector&&) noexcept = default;
  BitVector& operator=(BitVector&&) = default;

  // Create a copy of the BitVector.
  BitVector Copy() const;

  // Bitwise Not of the BitVector.
  void Not();

  // Bitwise Or of the BitVector.
  void Or(const BitVector&);

  // Bitwise And of the BitVector.
  void And(const BitVector&);

  // Returns the size of the BitVector.
  uint32_t size() const { return static_cast<uint32_t>(size_); }

  // Returns whether the bit at |idx| is set.
  bool IsSet(uint32_t idx) const {
    PERFETTO_DCHECK(idx < size());
    return ConstBitWord(&words_[WordFloor(idx)]).IsSet(idx % BitWord::kBits);
  }

  // Returns the number of set bits in the BitVector.
  uint32_t CountSetBits() const { return CountSetBits(size()); }

  // Returns the number of set bits between the start of the BitVector
  // (inclusive) and the index |end| (exclusive).
  uint32_t CountSetBits(uint32_t end) const {
    if (end == 0)
      return 0;

    // Although the external interface we present uses an exclusive |end|,
    // internally it's a lot nicer to work with an inclusive |end| (mainly
    // because we get block rollovers on exclusive ends which means we need
    // to have if checks to ensure we don't overflow the number of blocks).
    Address addr = IndexToAddress(end - 1);

    // Add the number of set bits until the start of the block to the number
    // of set bits until the end address inside the block.
    return counts_[addr.block_idx] +
           ConstBlockFromIndex(addr.block_idx).CountSetBits(addr.block_offset);
  }

  // Returns the index of the |n|th set bit. Should only be called with |n| <
  // CountSetBits().
  uint32_t IndexOfNthSet(uint32_t n) const {
    PERFETTO_DCHECK(n < CountSetBits());

    // First search for the block which, up until the start of it, has more than
    // n bits set. Note that this should never return |counts.begin()| as
    // that should always be 0.
    // TODO(lalitm): investigate whether we can make this faster with small
    // binary search followed by a linear search instead of binary searching the
    // full way.
    auto it = std::upper_bound(counts_.begin(), counts_.end(), n);
    PERFETTO_DCHECK(it != counts_.begin());

    // Go back one block to find the block which has the bit we are looking for.
    uint32_t block_idx =
        static_cast<uint32_t>(std::distance(counts_.begin(), it) - 1);

    // Figure out how many set bits forward we are looking inside the block
    // by taking away the number of bits at the start of the block from n.
    uint32_t set_in_block = n - counts_[block_idx];

    // Compute the address of the bit in the block then convert the full
    // address back to an index.
    BlockOffset block_offset =
        ConstBlockFromIndex(block_idx).IndexOfNthSet(set_in_block);
    return AddressToIndex(Address{block_idx, block_offset});
  }

  // Sets the bit at index |idx| to true and returns the previous value.
  bool Set(uint32_t idx) {
    // Set the bit to the correct value inside the block but store the old
    // bit to help fix the counts.
    auto addr = IndexToAddress(idx);
    bool old_value =
        ConstBlockFromIndex(addr.block_idx).IsSet(addr.block_offset);

    // If the old value was unset, set the bit and add one to the count.
    if (PERFETTO_LIKELY(!old_value)) {
      BlockFromIndex(addr.block_idx).Set(addr.block_offset);

      auto size = static_cast<uint32_t>(counts_.size());
      for (uint32_t i = addr.block_idx + 1; i < size; ++i) {
        counts_[i]++;
      }
    }
    return old_value;
  }

  // Sets the bit at index |idx| to false.
  void Clear(uint32_t idx) {
    // Set the bit to the correct value inside the block but store the old
    // bit to help fix the counts.
    auto addr = IndexToAddress(idx);
    bool old_value =
        ConstBlockFromIndex(addr.block_idx).IsSet(addr.block_offset);

    // If the old value was set, clear the bit and subtract one from all the
    // counts.
    if (PERFETTO_LIKELY(old_value)) {
      BlockFromIndex(addr.block_idx).Clear(addr.block_offset);

      auto size = static_cast<uint32_t>(counts_.size());
      for (uint32_t i = addr.block_idx + 1; i < size; ++i) {
        counts_[i]--;
      }
    }
  }

  // Appends true to the BitVector.
  void AppendTrue() {
    AppendFalse();
    Address addr = IndexToAddress(size() - 1);
    BlockFromIndex(addr.block_idx).Set(addr.block_offset);
  }

  // Appends false to the BitVector.
  void AppendFalse() {
    Address addr = IndexToAddress(size_);
    uint32_t old_blocks_size = BlockCount();
    uint32_t new_blocks_size = addr.block_idx + 1;

    if (PERFETTO_UNLIKELY(new_blocks_size > old_blocks_size)) {
      uint32_t t = CountSetBits();
      words_.resize(words_.size() + Block::kWords);
      counts_.emplace_back(t);
    }

    size_++;
    // We don't need to clear the bit as we ensure that anything after
    // size_ is always set to false.
  }

  // Resizes the BitVector to the given |size|.
  // Truncates the BitVector if |size| < |size()| or fills the new space with
  // |filler| if |size| > |size()|. Calling this method is a noop if |size| ==
  // |size()|.
  void Resize(uint32_t new_size, bool filler = false);

  // Creates a BitVector of size |end| with the bits between |start| and |end|
  // filled by calling the filler function |f(index of bit)|.
  //
  // As an example, suppose RangeForTesting(3, 7, [](x) { return x < 5 }). This
  // would result in the following BitVector: [0 0 0 1 1 0 0]
  template <typename Filler = bool(uint32_t)>
  PERFETTO_WARN_UNUSED_RESULT static BitVector RangeForTesting(uint32_t start,
                                                               uint32_t end,
                                                               Filler f) {
    // Compute the block index and BitVector index where we start and end
    // working one block at a time.
    uint32_t start_fast_block = BlockCount(start);
    uint32_t start_fast_idx = BlockToIndex(start_fast_block);
    BitVector bv(start, false);

    // Minimum value of start_fast_idx is number of bits in block, so we need to
    // separate calculation for shorter ranges.
    if (start_fast_idx > end) {
      for (uint32_t i = start; i < end; ++i) {
        bv.Append(f(i));
      }
      return bv;
    }

    uint32_t end_fast_block = BlockFloor(end);
    uint32_t end_fast_idx = BlockToIndex(end_fast_block);

    // Fill up to |start_fast_index| with values from the filler.
    for (uint32_t i = start; i < start_fast_idx; ++i) {
      bv.Append(f(i));
    }

    // Assert words_ vector is full and size_ is properly calculated.
    PERFETTO_DCHECK(bv.words_.size() % Block::kWords == 0);
    PERFETTO_DCHECK(bv.words_.size() * BitWord::kBits == bv.size_);

    // At this point we can work one block at a time.
    bv.words_.resize(bv.words_.size() +
                     Block::kWords * (end_fast_block - start_fast_block));
    for (uint32_t i = start_fast_block; i < end_fast_block; ++i) {
      uint64_t* block_start_word = &bv.words_[i * Block::kWords];
      Block(block_start_word).FromFiller(bv.size_, f);
      bv.counts_.emplace_back(bv.CountSetBits());
      bv.size_ += Block::kBits;
    }

    // Add the last few elements to finish up to |end|.
    for (uint32_t i = end_fast_idx; i < end; ++i) {
      bv.Append(f(i));
    }

    return bv;
  }

  // Creates BitVector from a vector of sorted indices. Set bits in the
  // resulting BitVector are values from the index vector.
  // Note for callers - the passed index vector has to:
  // - be sorted
  // - have first element >= 0
  // - last value smaller than numeric limit of uint32_t.
  PERFETTO_WARN_UNUSED_RESULT static BitVector FromSortedIndexVector(
      const std::vector<int64_t>&);

  // Creates BitVector from a vector of unsorted indices. Set bits in the
  // resulting BitVector are values from the index vector.
  PERFETTO_WARN_UNUSED_RESULT static BitVector FromUnsortedIndexVector(
      const std::vector<uint32_t>&);

  // Creates a BitVector of size `min(range_end, size())` with bits between
  // |start| and |end| filled with corresponding bits from |this| BitVector.
  PERFETTO_WARN_UNUSED_RESULT BitVector
  IntersectRange(uint32_t range_start, uint32_t range_end) const;

  // Requests the removal of unused capacity.
  // Matches the semantics of std::vector::shrink_to_fit.
  void ShrinkToFit() {
    words_.shrink_to_fit();
    counts_.shrink_to_fit();
  }

  // Updates the ith set bit of this BitVector with the value of
  // |other.IsSet(i)|.
  //
  // This is the best way to batch update all the bits which are set; for
  // example when filtering rows, we want to filter all rows which are currently
  // included but ignore rows which have already been excluded.
  //
  // For example suppose the following:
  // this:  1 1 0 0 1 0 1
  // other: 0 1 1 0
  // This will change this to the following:
  // this:  0 1 0 0 1 0 0
  void UpdateSetBits(const BitVector& other);

  // For each set bit position  in |other|, Selects the value of each bit in
  // |this| and stores them contiguously in |this|.
  //
  // Precondition: |this.size()| <= |other.size()|.
  //
  // For example suppose the following:
  // this:  1 1 0 0 1 0 1
  // other: 0 1 0 1 0 1 0 0 1 0
  // |this| will change this to the following:
  // this:  1 0 0
  void SelectBits(const BitVector& other);

  // Returns the approximate cost (in bytes) of storing a BitVector with size
  // |n|. This can be used to make decisions about whether using a BitVector is
  // worthwhile.
  // This cost should not be treated as exact - it just gives an indication of
  // the memory needed.
  static constexpr uint32_t ApproxBytesCost(uint32_t n) {
    // The two main things making up a BitVector is the cost of the blocks of
    // bits and the cost of the counts vector.
    return BlockCount(n) * Block::kBits + BlockCount(n) * sizeof(uint32_t);
  }

  // Returns a vector<uint32_t> containing the indices of all the set bits
  // in the BitVector.
  std::vector<uint32_t> GetSetBitIndices() const;

  // Serialize internals of BitVector to proto.
  void Serialize(protos::pbzero::SerializedColumn_BitVector* msg) const;

  // Deserialize BitVector from proto.
  void Deserialize(
      const protos::pbzero::SerializedColumn_BitVector_Decoder& bv_msg);

 private:
  using SetBitsIterator = internal::SetBitsIterator;
  friend class internal::BaseIterator;
  friend class internal::SetBitsIterator;

  // Represents the offset of a bit within a block.
  struct BlockOffset {
    uint16_t word_idx;
    uint16_t bit_idx;
  };

  // Represents the address of a bit within the BitVector.
  struct Address {
    uint32_t block_idx;
    BlockOffset block_offset;
  };

  // Represents the smallest collection of bits we can refer to as
  // one unit.
  //
  // Currently, this is implemented as a 64 bit integer as this is the
  // largest type which we can assume to be present on all platforms.
  class BitWord {
   public:
    static constexpr uint32_t kBits = 64;

    explicit BitWord(uint64_t* word) : word_(word) {}

    // Bitwise ors the given |mask| to the current value.
    void Or(uint64_t mask) { *word_ |= mask; }

    // Bitwise ands the given |mask| to the current value.
    void And(uint64_t mask) { *word_ &= mask; }

    // Bitwise not.
    void Not() { *word_ = ~(*word_); }

    // Sets the bit at the given index to true.
    void Set(uint32_t idx) {
      PERFETTO_DCHECK(idx < kBits);

      // Or the value for the true shifted up to |idx| with the word.
      Or(1ull << idx);
    }

    // Sets the bit at the given index to false.
    void Clear(uint32_t idx) {
      PERFETTO_DCHECK(idx < kBits);

      // And the integer of all bits set apart from |idx| with the word.
      *word_ &= ~(1ull << idx);
    }

    // Clears all the bits (i.e. sets the atom to zero).
    void ClearAll() { *word_ = 0; }

    // Retains all bits up to and including the bit at |idx| and clears
    // all bits after this point.
    void ClearAfter(uint32_t idx) {
      PERFETTO_DCHECK(idx < kBits);
      *word_ = WordUntil(idx);
    }

    // Sets all bits between the bit at |start| and |end| (inclusive).
    void Set(uint32_t start, uint32_t end) {
      uint32_t diff = end - start;
      *word_ |= (MaskAllBitsSetUntil(diff) << static_cast<uint64_t>(start));
    }

    // Return a mask of all the bits up to and including bit at |idx|.
    static uint64_t MaskAllBitsSetUntil(uint32_t idx) {
      // Start with 1 and shift it up (idx + 1) bits we get:
      // top : 00000000010000000
      uint64_t top = 1ull << ((idx + 1ull) % kBits);

      // We need to handle the case where idx == 63. In this case |top| will be
      // zero because 1 << ((idx + 1) % 64) == 1 << (64 % 64) == 1.
      // In this case, we actually want top == 0. We can do this by shifting
      // down by (idx + 1) / kBits - this will be a noop for every index other
      // than idx == 63. This should also be free on x86 because of the mod
      // instruction above.
      top = top >> ((idx + 1) / kBits);

      // Then if we take away 1, we get precisely the mask we want.
      return top - 1u;
    }

   private:
    // Returns the bits up to and including the bit at |idx|.
    uint64_t WordUntil(uint32_t idx) const {
      PERFETTO_DCHECK(idx < kBits);

      // To understand what is happeninng here, consider an example.
      // Suppose we want to all the bits up to the 7th bit in the atom
      //               7th
      //                |
      //                v
      // atom: 01010101011111000
      //
      // The easiest way to do this would be if we had a mask with only
      // the bottom 7 bits set:
      // mask: 00000000001111111
      uint64_t mask = MaskAllBitsSetUntil(idx);

      // Finish up by and'ing the atom with the computed mask.
      return *word_ & mask;
    }

    uint64_t* word_;
  };

  class ConstBitWord {
   public:
    static constexpr uint32_t kBits = 64;

    explicit ConstBitWord(const uint64_t* word) : word_(word) {}

    // Returns whether the bit at the given index is set.
    bool IsSet(uint32_t idx) const {
      PERFETTO_DCHECK(idx < kBits);
      return (*word_ >> idx) & 1ull;
    }

    // Returns the index of the nth set bit.
    // Undefined if |n| >= |CountSetBits()|.
    uint16_t IndexOfNthSet(uint32_t n) const {
      PERFETTO_DCHECK(n < kBits);

      // The below code is very dense but essentially computes the nth set
      // bit inside |atom| in the "broadword" style of programming (sometimes
      // referred to as "SIMD within a register").
      //
      // Instead of treating a uint64 as an individual unit, broadword
      // algorithms treat them as a packed vector of uint8. By doing this, they
      // allow branchless algorithms when considering bits of a uint64.
      //
      // In benchmarks, this algorithm has found to be the fastest, portable
      // way of computing the nth set bit (if we were only targeting new
      // versions of x64, we could also use pdep + ctz but unfortunately
      // this would fail on WASM - this about 2.5-3x faster on x64).
      //
      // The code below was taken from the paper
      // http://vigna.di.unimi.it/ftp/papers/Broadword.pdf
      uint64_t s = *word_ - ((*word_ & 0xAAAAAAAAAAAAAAAA) >> 1);
      s = (s & 0x3333333333333333) + ((s >> 2) & 0x3333333333333333);
      s = ((s + (s >> 4)) & 0x0F0F0F0F0F0F0F0F) * L8;

      uint64_t b = (BwLessThan(s, n * L8) >> 7) * L8 >> 53 & ~7ull;
      uint64_t l = n - ((s << 8) >> b & 0xFF);
      s = (BwGtZero(((*word_ >> b & 0xFF) * L8) & 0x8040201008040201) >> 7) *
          L8;

      uint64_t ret = b + ((BwLessThan(s, l * L8) >> 7) * L8 >> 56);

      return static_cast<uint16_t>(ret);
    }

    // Returns the number of set bits.
    uint32_t CountSetBits() const {
      return static_cast<uint32_t>(PERFETTO_POPCOUNT(*word_));
    }

    // Returns the number of set bits up to and including the bit at |idx|.
    uint32_t CountSetBits(uint32_t idx) const {
      PERFETTO_DCHECK(idx < kBits);
      return static_cast<uint32_t>(PERFETTO_POPCOUNT(WordUntil(idx)));
    }

   private:
    // Constant with all the low bit of every byte set.
    static constexpr uint64_t L8 = 0x0101010101010101;

    // Constant with all the high bit of every byte set.
    static constexpr uint64_t H8 = 0x8080808080808080;

    // Returns a packed uint64 encoding whether each byte of x is less
    // than each corresponding byte of y.
    // This is computed in the "broadword" style of programming; see
    // IndexOfNthSet for details on this.
    static uint64_t BwLessThan(uint64_t x, uint64_t y) {
      return (((y | H8) - (x & ~H8)) ^ x ^ y) & H8;
    }

    // Returns a packed uint64 encoding whether each byte of x is greater
    // than or equal zero.
    // This is computed in the "broadword" style of programming; see
    // IndexOfNthSet for details on this.
    static uint64_t BwGtZero(uint64_t x) { return (((x | H8) - L8) | x) & H8; }

    // Returns the bits up to and including the bit at |idx|.
    uint64_t WordUntil(uint32_t idx) const {
      PERFETTO_DCHECK(idx < kBits);

      // To understand what is happeninng here, consider an example.
      // Suppose we want to all the bits up to the 7th bit in the atom
      //               7th
      //                |
      //                v
      // atom: 01010101011111000
      //
      // The easiest way to do this would be if we had a mask with only
      // the bottom 7 bits set:
      // mask: 00000000001111111
      uint64_t mask = BitWord::MaskAllBitsSetUntil(idx);

      // Finish up by and'ing the atom with the computed mask.
      return *word_ & mask;
    }

    const uint64_t* word_;
  };

  // Represents a group of bits with a bitcount such that it is
  // efficient to work on these bits.
  //
  // On x86 architectures we generally target for trace processor, the
  // size of a cache line is 64 bytes (or 512 bits). For this reason,
  // we make the size of the block contain 8 atoms as 8 * 64 == 512.
  class Block {
   public:
    // See class documentation for how these constants are chosen.
    static constexpr uint16_t kWords = 8;
    static constexpr uint32_t kBits = kWords * BitWord::kBits;

    explicit Block(uint64_t* start_word) : start_word_(start_word) {}

    // Sets the bit at the given address to true.
    void Set(const BlockOffset& addr) {
      PERFETTO_DCHECK(addr.word_idx < kWords);
      BitWord(&start_word_[addr.word_idx]).Set(addr.bit_idx);
    }

    // Sets the bit at the given address to false.
    void Clear(const BlockOffset& addr) {
      PERFETTO_DCHECK(addr.word_idx < kWords);

      BitWord(&start_word_[addr.word_idx]).Clear(addr.bit_idx);
    }

    // Retains all bits up to and including the bit at |addr| and clears
    // all bits after this point.
    void ClearAfter(const BlockOffset& offset) {
      PERFETTO_DCHECK(offset.word_idx < kWords);

      // In the first atom, keep the bits until the address specified.
      BitWord(&start_word_[offset.word_idx]).ClearAfter(offset.bit_idx);

      // For all subsequent atoms, we just clear the whole atom.
      for (uint32_t i = offset.word_idx + 1; i < kWords; ++i) {
        BitWord(&start_word_[i]).ClearAll();
      }
    }

    // Set all the bits between the offsets given by |start| and |end|
    // (inclusive).
    void Set(const BlockOffset& start, const BlockOffset& end) {
      if (start.word_idx == end.word_idx) {
        // If there is only one word we will change, just set the range within
        // the word.
        BitWord(&start_word_[start.word_idx]).Set(start.bit_idx, end.bit_idx);
        return;
      }

      // Otherwise, we have more than one word to set. To do this, we will
      // do this in three steps.

      // First, we set the first word from the start to the end of the word.
      BitWord(&start_word_[start.word_idx])
          .Set(start.bit_idx, BitWord::kBits - 1);

      // Next, we set all words (except the last).
      for (uint32_t i = start.word_idx + 1; i < end.word_idx; ++i) {
        BitWord(&start_word_[i]).Set(0, BitWord::kBits - 1);
      }

      // Finally, we set the word block from the start to the end offset.
      BitWord(&start_word_[end.word_idx]).Set(0, end.bit_idx);
    }

    void Or(Block& sec) {
      for (uint32_t i = 0; i < kWords; ++i) {
        BitWord(&start_word_[i]).Or(sec.start_word_[i]);
      }
    }

    template <typename Filler>
    void FromFiller(uint32_t offset, Filler f) {
      // We choose to iterate the bits as the outer loop as this allows us
      // to reuse the mask and the bit offset between iterations of the loop.
      // This makes a small (but noticeable) impact in the performance of this
      // function.
      for (uint32_t i = 0; i < BitWord::kBits; ++i) {
        uint64_t mask = 1ull << i;
        uint32_t offset_with_bit = offset + i;
        for (uint32_t j = 0; j < Block::kWords; ++j) {
          bool res = f(offset_with_bit + j * BitWord::kBits);
          BitWord(&start_word_[j]).Or(res ? mask : 0);
        }
      }
    }

    void ReplaceWith(Block block) {
      for (uint32_t i = 0; i < BitWord::kBits; ++i) {
        start_word_[i] = block.start_word()[i];
      }
    }

    uint64_t* start_word() { return start_word_; }

   private:
    uint64_t* start_word_;
  };

  class ConstBlock {
   public:
    // See class documentation for how these constants are chosen.
    static constexpr uint16_t kWords = Block::kWords;
    static constexpr uint32_t kBits = kWords * BitWord::kBits;

    explicit ConstBlock(const uint64_t* start_word) : start_word_(start_word) {}
    explicit ConstBlock(Block block) : start_word_(block.start_word()) {}

    // Returns whether the bit at the given address is set.
    bool IsSet(const BlockOffset& addr) const {
      PERFETTO_DCHECK(addr.word_idx < kWords);
      return ConstBitWord(start_word_ + addr.word_idx).IsSet(addr.bit_idx);
    }

    // Gets the offset of the nth set bit in this block.
    BlockOffset IndexOfNthSet(uint32_t n) const {
      uint32_t count = 0;
      for (uint16_t i = 0; i < kWords; ++i) {
        // Keep a running count of all the set bits in the atom.
        uint32_t value = count + ConstBitWord(start_word_ + i).CountSetBits();
        if (value <= n) {
          count = value;
          continue;
        }

        // The running count of set bits is more than |n|. That means this
        // atom contains the bit we are looking for.

        // Take away the number of set bits to the start of this atom from
        // |n|.
        uint32_t set_in_atom = n - count;

        // Figure out the index of the set bit inside the atom and create the
        // address of this bit from that.
        uint16_t bit_idx =
            ConstBitWord(start_word_ + i).IndexOfNthSet(set_in_atom);
        PERFETTO_DCHECK(bit_idx < 64);
        return BlockOffset{i, bit_idx};
      }
      PERFETTO_FATAL("Index out of bounds");
    }

    // Gets the number of set bits within a block up to and including the bit
    // at the given address.
    uint32_t CountSetBits(const BlockOffset& addr) const {
      PERFETTO_DCHECK(addr.word_idx < kWords);

      // Count all the set bits in the atom until we reach the last atom
      // index.
      uint32_t count = 0;
      for (uint32_t i = 0; i < addr.word_idx; ++i) {
        count += ConstBitWord(&start_word_[i]).CountSetBits();
      }

      // For the last atom, only count the bits upto and including the bit
      // index.
      return count + ConstBitWord(&start_word_[addr.word_idx])
                         .CountSetBits(addr.bit_idx);
    }

    // Gets the number of set bits within a block up.
    uint32_t CountSetBits() const {
      uint32_t count = 0;
      for (uint32_t i = 0; i < kWords; ++i) {
        count += ConstBitWord(&start_word_[i]).CountSetBits();
      }
      return count;
    }

   private:
    const uint64_t* start_word_;
  };

  BitVector(std::vector<uint64_t> words,
            std::vector<uint32_t> counts,
            uint32_t size);

  // Returns the number of 8 elements blocks in the BitVector.
  uint32_t BlockCount() {
    return static_cast<uint32_t>(words_.size()) / Block::kWords;
  }

  Block BlockFromIndex(uint32_t idx) {
    PERFETTO_DCHECK(Block::kWords * (idx + 1) <= words_.size());

    uint64_t* start_word = &words_[Block::kWords * idx];
    return Block(start_word);
  }

  ConstBlock ConstBlockFromIndex(uint32_t idx) const {
    PERFETTO_DCHECK(Block::kWords * (idx + 1) <= words_.size());

    return ConstBlock(&words_[Block::kWords * idx]);
  }

  // Set all the bits between the addresses given by |start| and |end|
  // (inclusive).
  // Note: this method does not update the counts vector - that is the
  // responsibility of the caller.
  void Set(const Address& start, const Address& end) {
    static constexpr BlockOffset kFirstBlockOffset = BlockOffset{0, 0};
    static constexpr BlockOffset kLastBlockOffset =
        BlockOffset{Block::kWords - 1, BitWord::kBits - 1};

    if (start.block_idx == end.block_idx) {
      // If there is only one block we will change, just set the range within
      // the block.
      BlockFromIndex(start.block_idx).Set(start.block_offset, end.block_offset);
      return;
    }

    // Otherwise, we have more than one block to set. To do this, we will
    // do this in three steps.

    // First, we set the first block from the start to the end of the block.
    BlockFromIndex(start.block_idx).Set(start.block_offset, kLastBlockOffset);

    // Next, we set all blocks (except the last).
    for (uint32_t cur_block_idx = start.block_idx + 1;
         cur_block_idx < end.block_idx; ++cur_block_idx) {
      BlockFromIndex(cur_block_idx).Set(kFirstBlockOffset, kLastBlockOffset);
    }

    // Finally, we set the last block from the start to the end offset.
    BlockFromIndex(end.block_idx).Set(kFirstBlockOffset, end.block_offset);
  }

  // Helper function to append a bit. Generally, prefer to call AppendTrue
  // or AppendFalse instead of this function if you know the type - they will
  // be faster.
  void Append(bool value) {
    if (value) {
      AppendTrue();
    } else {
      AppendFalse();
    }
  }

  // Iterate all the set bits in the BitVector.
  //
  // Usage:
  // for (auto it = bv.IterateSetBits(); it; it.Next()) {
  //   ...
  // }
  SetBitsIterator IterateSetBits() const;

  // Returns the index of the word which would store |idx|.
  static constexpr uint32_t WordFloor(uint32_t idx) {
    return idx / BitWord::kBits;
  }

  // Returns number of words (int64_t) required to store |bit_count| bits.
  static uint32_t WordCount(uint32_t bit_count) {
    // See |BlockCount| for an explanation of this trick.
    return (bit_count + BitWord::kBits - 1) / BitWord::kBits;
  }

  static Address IndexToAddress(uint32_t idx) {
    Address a;
    a.block_idx = idx / Block::kBits;

    uint16_t bit_idx_inside_block = idx % Block::kBits;
    a.block_offset.word_idx = bit_idx_inside_block / BitWord::kBits;
    a.block_offset.bit_idx = bit_idx_inside_block % BitWord::kBits;
    return a;
  }

  static uint32_t AddressToIndex(Address addr) {
    return addr.block_idx * Block::kBits +
           addr.block_offset.word_idx * BitWord::kBits +
           addr.block_offset.bit_idx;
  }

  // Returns number of blocks (arrays of 8 int64_t) required to store
  // |bit_count| bits.
  //
  // This is useful to be able to find indices where "fast" algorithms can
  // start which work on entire blocks.
  static constexpr uint32_t BlockCount(uint32_t bit_count) {
    // Adding |Block::kBits - 1| gives us a quick way to get the count. We
    // do this instead of adding 1 at the end because that gives incorrect
    // answers for bit_count % Block::kBits == 0.
    return (bit_count + Block::kBits - 1) / Block::kBits;
  }

  // Returns the index of the block which would store |idx|.
  static constexpr uint32_t BlockFloor(uint32_t idx) {
    return idx / Block::kBits;
  }

  // Converts a block index to a index in the BitVector.
  static constexpr uint32_t BlockToIndex(uint32_t block_idx) {
    return block_idx * Block::kBits;
  }

  // Updates the counts in |counts| by counting the set bits in |words|.
  static void UpdateCounts(const std::vector<uint64_t>& words,
                           std::vector<uint32_t>& counts) {
    PERFETTO_CHECK(words.size() == counts.size() * Block::kWords);
    for (uint32_t i = 1; i < counts.size(); ++i) {
      counts[i] = counts[i - 1] +
                  ConstBlock(&words[Block::kWords * (i - 1)]).CountSetBits();
    }
  }

  uint32_t size_ = 0;
  // See class documentation for how these constants are chosen.
  static constexpr uint16_t kWordsInBlock = Block::kWords;
  static constexpr uint32_t kBitsInBlock = kWordsInBlock * BitWord::kBits;
  std::vector<uint32_t> counts_;
  std::vector<uint64_t> words_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_BIT_VECTOR_H_
