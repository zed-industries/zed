// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_BYTE_ARRAY_MUTATOR_H_
#define THIRD_PARTY_CENTIPEDE_BYTE_ARRAY_MUTATOR_H_

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "./centipede/execution_metadata.h"
#include "./centipede/knobs.h"
#include "./centipede/mutation_input.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// A simple class representing an array of up to kMaxEntrySize bytes.
class DictEntry {
 public:
  static constexpr uint8_t kMaxEntrySize = 16;

  explicit DictEntry(ByteSpan bytes)
      : bytes_{},  // initialize bytes_ to all zeros
        size_(bytes.size()) {
    if (size_ > kMaxEntrySize) __builtin_trap();
    memcpy(bytes_, bytes.data(), bytes.size());
  }
  const uint8_t *absl_nonnull begin() const { return bytes_; }
  const uint8_t *end() const { return bytes_ + size_; }
  size_t size() const { return size_; }
  bool operator<(const DictEntry &other) const {
    return memcmp(this, &other, sizeof(*this)) < 0;
  }

 private:
  // bytes_ must go first so that operator < is lexicographic.
  uint8_t bytes_[kMaxEntrySize];
  uint8_t size_;  // between kMinEntrySize and kMaxEntrySize.
};

// Dictionary of CMP args.
// Maintains an easy-to-query set of pairs {A,B}, such that
// an instruction `A CMP B` has been observed.
class CmpDictionary {
 public:
  static constexpr size_t kMinEntrySize = 2;  // 1-byte entries won't be added.

  CmpDictionary() = default;

  // Sets the dictionary from execution `metadata`.
  //
  // Returns false on bad metadata, true otherwise.
  bool SetFromMetadata(const ExecutionMetadata &metadata);

  // Clears `suggestions` on entry.
  // For every observed `A CMP B` such that `A` is a prefix of `bytes`,
  // adds `B` to `suggestions`.
  // `suggestions`, is filled up to capacity(), but not more.
  void SuggestReplacement(ByteSpan bytes,
                          std::vector<ByteSpan> &suggestions) const;

  // Returns the number of dictionary entries.
  size_t size() const { return dictionary_.size(); }

 private:
  using Pair = std::pair<DictEntry, DictEntry>;
  std::vector<Pair> dictionary_;
};

// This class allows to mutate a ByteArray in different ways.
// All mutations expect and guarantee that `data` remains non-empty
// since there is only one possible empty input and it's uninteresting.
//
// This class is thread-compatible.
// Typical usage is to have one such object per thread.
class ByteArrayMutator {
 public:
  // CTOR. Initializes the internal RNG with `seed` (`seed` != 0).
  // Keeps a const reference to `knobs` throughout the lifetime.
  ByteArrayMutator(const Knobs &knobs, uintptr_t seed)
      : rng_(seed), knobs_(knobs) {
    if (seed == 0) __builtin_trap();  // We don't include logging.h here.
  }

  // Adds `dict_entries` to an internal dictionary.
  void AddToDictionary(const std::vector<ByteArray> &dict_entries);

  // Populates the internal CmpDictionary using execution `metadata`.
  // Returns false on failure, true otherwise.
  bool SetMetadata(const ExecutionMetadata &metadata) {
    return cmp_dictionary_.SetFromMetadata(metadata);
  }

  // Takes non-empty `inputs` and produces `num_mutants` mutants.
  std::vector<ByteArray> MutateMany(const std::vector<MutationInputRef> &inputs,
                                    size_t num_mutants);

  using CrossOverFn = void (ByteArrayMutator::*)(ByteArray &,
                                                 const ByteArray &);

  // Mutates `data` by inserting a random part from `other`.
  void CrossOverInsert(ByteArray &data, const ByteArray &other);

  // Mutates `data` by overwriting some of it with a random part of `other`.
  void CrossOverOverwrite(ByteArray &data, const ByteArray &other);

  // Applies one of {CrossOverOverwrite, CrossOverInsert}.
  void CrossOver(ByteArray &data, const ByteArray &other);

  // Type for a Mutator member-function.
  // Every mutator function takes a ByteArray& as an input, mutates it in place
  // and returns true if mutation took place. In some cases mutation may fail
  // to happen, e.g. if EraseBytes() is called on a 1-byte input.
  // Fn is test-only public.
  using Fn = bool (ByteArrayMutator::*)(ByteArray &);

  // All public functions below are mutators.
  // They return true iff a mutation took place.

  // Applies some random mutation to data.
  bool Mutate(ByteArray &data);

  // Applies some random mutation that doesn't change size.
  bool MutateSameSize(ByteArray &data);

  // Applies some random mutation that decreases size.
  bool MutateDecreaseSize(ByteArray &data);

  // Applies some random mutation that increases size.
  bool MutateIncreaseSize(ByteArray &data);

  // Flips a random bit.
  bool FlipBit(ByteArray &data);

  // Swaps two bytes.
  bool SwapBytes(ByteArray &data);

  // Changes a random byte to a random value.
  bool ChangeByte(ByteArray &data);

  // Overwrites a random part of `data` with a random dictionary entry.
  bool OverwriteFromDictionary(ByteArray &data);

  // Overwrites a random part of `data` with an entry suggested by the internal
  // CmpDictionary.
  bool OverwriteFromCmpDictionary(ByteArray &data);

  // Inserts random bytes.
  bool InsertBytes(ByteArray &data);

  // Inserts a random dictionary entry at random position.
  bool InsertFromDictionary(ByteArray &data);

  // Erases random bytes.
  bool EraseBytes(ByteArray &data);

  // Set size alignment for mutants with modified sizes. Some mutators do not
  // change input size, but mutators that insert or erase bytes will produce
  // mutants with aligned sizes (if possible).
  //
  // Returns true if new size alignment was accepted. Returns false if max
  // length is not a multiple of the specified size alignment.
  bool set_size_alignment(size_t size_alignment) {
    if ((max_len_ != std::numeric_limits<size_t>::max()) &&
        (max_len_ % size_alignment != 0)) {
      return false;
    }
    size_alignment_ = size_alignment;
    return true;
  }

  // Set max length in bytes for mutants with modified sizes.
  //
  // Returns true if new max length was accepted. Returns false if specified max
  // length is not a multiple of size alignment.
  bool set_max_len(size_t max_len) {
    if ((max_len != std::numeric_limits<size_t>::max()) &&
        (max_len % size_alignment_ != 0)) {
      return false;
    }
    max_len_ = max_len;
    return true;
  }

 private:
  FRIEND_TEST(ByteArrayMutator, RoundUpToAddCorrectly);
  FRIEND_TEST(ByteArrayMutator, RoundDownToRemoveCorrectly);

  // Given a current size and a number of bytes to add, returns the number of
  // bytes that should be added for the resulting size to be properly aligned.
  //
  // If the original to_add would result in an unaligned input size, we round up
  // to the next larger aligned size.
  //
  // This function respects `max_len_` and will return 0 if curr_size is already
  // greater than or equal to `max_len_`.
  size_t RoundUpToAdd(size_t curr_size, size_t to_add);

  // Given a current size and a number of bytes to remove, returns the number of
  // bytes that should be removed for the resulting size to be property aligned.
  //
  // If the original to_remove would result in an unaligned input size, we
  // round down to the next smaller aligned size.
  //
  // However, we never return a number of bytes to remove that would result in a
  // 0 size. In this case, the resulting size will be the smaller of
  // curr_size and size_alignment_.
  //
  // This function respects `max_len_` and may return a larger number necessary
  // to get the mutant's size to below `max_len_`.
  size_t RoundDownToRemove(size_t curr_size, size_t to_remove);

  // Size alignment in bytes to generate mutants.
  //
  // For example, if size_alignment_ is 1, generated mutants can have any
  // number of bytes. If size_alignment_ is 4, generated mutants will have sizes
  // that are 4-byte aligned.
  size_t size_alignment_ = 1;

  // Max length of a generated mutant in bytes.
  size_t max_len_ = std::numeric_limits<size_t>::max();

  Rng rng_;
  const Knobs &knobs_;
  std::vector<DictEntry> dictionary_;
  CmpDictionary cmp_dictionary_;
};

// Controls how much crossover is used during mutations.
// https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)
// TODO(kcc): add tests with different values of knobs.
extern const KnobId knob_mutate_or_crossover;
// Controls how much crossver inserts data from the other input instead of
// overwriting.
extern const KnobId knob_cross_over_insert_or_overwrite;

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_BYTE_ARRAY_MUTATOR_H_
