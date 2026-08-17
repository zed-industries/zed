// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_MUTATION_HELPERS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_MUTATION_HELPERS_H_

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <optional>

#include "absl/random/bit_gen_ref.h"
#include "absl/random/distributions.h"
#include "./fuzztest/internal/table_of_recent_compares.h"

namespace fuzztest::internal {

// Trying to copy a segment from `from` to `to`, with given offsets.
// Invalid offset that cause boundary check failures will make this function
// return false. `is_self` tells the function whether `from` and `to` points
// to the same object. Returns `true` iff copying results in `to` being mutated.
template <bool is_self, typename ContainerT>
bool CopyPart(const ContainerT& from, ContainerT& to,
              size_t from_segment_start_offset, size_t from_segment_size,
              size_t to_segment_start_offset, size_t max_size) {
  bool mutated = false;
  if (from_segment_size == 0) return mutated;
  size_t from_segment_end_offset =
      from_segment_start_offset + from_segment_size;
  size_t to_segment_end_offset = to_segment_start_offset + from_segment_size;
  if (from_segment_start_offset >= from.size() ||
      to_segment_start_offset > to.size() ||
      from_segment_end_offset > from.size() || to_segment_end_offset > max_size)
    return mutated;
  if (to_segment_end_offset > to.size()) {
    mutated = true;
    to.resize(to_segment_end_offset);
  } else {
    if (!std::equal(std::next(to.begin(), to_segment_start_offset),
                    std::next(to.begin(), to_segment_end_offset),
                    std::next(from.begin(), from_segment_start_offset),
                    std::next(from.begin(), from_segment_end_offset)))
      mutated = true;
  }
  if (!mutated) return mutated;
  if constexpr (!is_self) {
    std::copy(std::next(from.begin(), from_segment_start_offset),
              std::next(from.begin(), from_segment_end_offset),
              std::next(to.begin(), to_segment_start_offset));
  } else {
    ContainerT tmp(std::next(from.begin(), from_segment_start_offset),
                   std::next(from.begin(), from_segment_end_offset));
    std::copy(tmp.begin(), tmp.end(),
              std::next(to.begin(), to_segment_start_offset));
  }
  return mutated;
}

// Trying to insert a segment from `from` to `to`, with given offsets.
// Invalid offset that cause boundary check failures will make this function
// return false. `is_self` tells the function whether `from` and `to` points
// to the same object. Returns `true` iff insertion results in `to` being
// mutated.
template <bool is_self, typename ContainerT>
bool InsertPart(const ContainerT& from, ContainerT& to,
                size_t from_segment_start_offset, size_t from_segment_size,
                size_t to_segment_start_offset, size_t max_size) {
  bool mutated = false;
  if (from_segment_size == 0) return mutated;
  size_t from_segment_end_offset =
      from_segment_start_offset + from_segment_size;
  if (from_segment_start_offset >= from.size() ||
      to_segment_start_offset > to.size() ||
      from_segment_end_offset > from.size() ||
      from_segment_size > max_size - to.size())
    return mutated;
  mutated = true;
  if constexpr (!is_self) {
    to.insert(std::next(to.begin(), to_segment_start_offset),
              std::next(from.begin(), from_segment_start_offset),
              std::next(from.begin(), from_segment_end_offset));
  } else {
    ContainerT tmp(std::next(from.begin(), from_segment_start_offset),
                   std::next(from.begin(), from_segment_end_offset));
    to.insert(std::next(to.begin(), to_segment_start_offset), tmp.begin(),
              tmp.end());
  }
  return mutated;
}

inline size_t GetOrGuessPositionHint(std::optional<size_t> position_hint,
                                     size_t max, absl::BitGenRef prng) {
  if (position_hint.has_value()) {
    return *position_hint;
  } else {
    return ChooseOffset(max + 1, prng);
  }
}

// Try to copy `dict_entry.value` to `val`:
// If `dict_entry` has a position hint, copy to that offset; otherwise,
// guess a position hint. Return the copied-to position if mutation succeed,
// otherwise std::nullopt. Return true iff `val` is successfully mutated.
template <bool is_self, typename ContainerT>
bool CopyFromDictionaryEntry(const DictionaryEntry<ContainerT>& dict_entry,
                             absl::BitGenRef prng, ContainerT& val,
                             size_t max_size) {
  if (dict_entry.value.size() > max_size) return false;
  size_t position_hint = GetOrGuessPositionHint(
      dict_entry.position_hint,
      std::min(val.size(), max_size - dict_entry.value.size()), prng);
  return CopyPart<is_self>(dict_entry.value, val, 0, dict_entry.value.size(),
                           position_hint, max_size);
}

// The same as above, but set `permanent_dict_candidate` iff successfully
// mutated.
template <bool is_self, typename ContainerT>
bool CopyFromDictionaryEntry(
    const DictionaryEntry<ContainerT>& dict_entry, absl::BitGenRef prng,
    ContainerT& val, size_t max_size,
    std::optional<DictionaryEntry<ContainerT>>& permanent_dict_candidate) {
  if (dict_entry.value.size() > max_size) return false;
  size_t position_hint = GetOrGuessPositionHint(
      dict_entry.position_hint,
      std::min(val.size(), max_size - dict_entry.value.size()), prng);
  bool mutated =
      CopyPart<is_self>(dict_entry.value, val, 0, dict_entry.value.size(),
                        position_hint, max_size);
  if (mutated) {
    permanent_dict_candidate = {position_hint, val};
  }
  return mutated;
}

// Try to insert `dict_entry.value` to `val`:
// If `dict_entry` has a position hint, copy to that offset; otherwise,
// guess a position hint. Return the inserted-to position if mutation succeed,
// otherwise std::nullopt. Return true iff successfully mutated.
template <bool is_self, typename ContainerT>
bool InsertFromDictionaryEntry(const DictionaryEntry<ContainerT>& dict_entry,
                               absl::BitGenRef prng, ContainerT& val,
                               size_t max_size) {
  if (val.size() + dict_entry.value.size() > max_size) return false;
  size_t position_hint =
      GetOrGuessPositionHint(dict_entry.position_hint, val.size(), prng);
  return InsertPart<is_self>(dict_entry.value, val, 0, dict_entry.value.size(),
                             position_hint, max_size);
}

// The same as above, but set `permanent_dict_candidate` iff successfully
// mutated.
template <bool is_self, typename ContainerT>
bool InsertFromDictionaryEntry(
    const DictionaryEntry<ContainerT>& dict_entry, absl::BitGenRef prng,
    ContainerT& val, size_t max_size,
    std::optional<DictionaryEntry<ContainerT>>& permanent_dict_candidate) {
  if (val.size() + dict_entry.value.size() > max_size) return false;
  size_t position_hint =
      GetOrGuessPositionHint(dict_entry.position_hint, val.size(), prng);
  bool mutated =
      InsertPart<is_self>(dict_entry.value, val, 0, dict_entry.value.size(),
                          position_hint, max_size);
  if (mutated) {
    permanent_dict_candidate = {position_hint, val};
  }
  return mutated;
}

template <typename ContainerT>
bool ApplyDictionaryMutationAndSavePermanentCandidate(
    ContainerT& val, const DictionaryEntry<ContainerT>& entry,
    absl::BitGenRef prng,
    std::optional<DictionaryEntry<ContainerT>>& permanent_dict_candidate,
    size_t max_size) {
  bool mutated = false;
  RunOne(
      prng,
      // Temporary dictionary replace contents from position hint.
      [&] {
        mutated = CopyFromDictionaryEntry<false>(entry, prng, val, max_size,
                                                 permanent_dict_candidate);
      },
      // Temporary dictionary insert into position hint.
      [&] {
        mutated = InsertFromDictionaryEntry<false>(entry, prng, val, max_size,
                                                   permanent_dict_candidate);
      });
  return mutated;
}

// Replace or insert the dictionary contents to position hints.
template <typename ContainerT>
bool MemoryDictionaryMutation(
    ContainerT& val, absl::BitGenRef prng,
    const internal::TablesOfRecentCompares* cmp_tables,
    ContainerDictionary<ContainerT>& temporary_dict,
    ContainerDictionary<ContainerT>& manual_dict,
    ContainerDictionary<ContainerT>& permanent_dict,
    std::optional<DictionaryEntry<ContainerT>>& permanent_dict_candidate,
    size_t max_size) {
  bool mutated = false;
  const bool can_use_manual_dictionary = !manual_dict.IsEmpty();
  const bool can_use_temporary_dictionary = !temporary_dict.IsEmpty();
  const bool can_use_permanent_dictionary = !permanent_dict.IsEmpty();
  const int dictionary_action_count =
      can_use_manual_dictionary + can_use_temporary_dictionary +
      can_use_permanent_dictionary + (cmp_tables == nullptr ? 0 : 1);
  int dictionary_action = absl::Uniform(prng, 0, dictionary_action_count);
  if (can_use_temporary_dictionary && dictionary_action-- == 0) {
    mutated = ApplyDictionaryMutationAndSavePermanentCandidate(
        val, temporary_dict.GetRandomSavedEntry(prng), prng,
        permanent_dict_candidate, max_size);
  }
  if (can_use_manual_dictionary && dictionary_action-- == 0) {
    mutated = ApplyDictionaryMutationAndSavePermanentCandidate(
        val, manual_dict.GetRandomSavedEntry(prng), prng,
        permanent_dict_candidate, max_size);
  }
  if (can_use_permanent_dictionary && dictionary_action-- == 0) {
    RunOne(
        prng,
        // Permanent dictionary replace contents from position hint.
        [&] {
          mutated = CopyFromDictionaryEntry<false>(
              permanent_dict.GetRandomSavedEntry(prng), prng, val, max_size);
        },
        // Permanent dictionary insert into position hint.
        [&] {
          mutated = InsertFromDictionaryEntry<false>(
              permanent_dict.GetRandomSavedEntry(prng), prng, val, max_size);
        });
  }
  // Pick entries from tables_of_recent_compares(TORC) directly.
  if (dictionary_action-- == 0) {
    auto dictionary_entry = ContainerDictionary<ContainerT>::GetRandomTORCEntry(
        val, prng, *cmp_tables);
    if (dictionary_entry.has_value()) {
      mutated = ApplyDictionaryMutationAndSavePermanentCandidate(
          val, *dictionary_entry, prng, permanent_dict_candidate, max_size);
    }
  }
  return mutated;
}

// Randomly erases a contiguous chunk of at least 1 and at most half the
// elements in `val`. The chunk's size is sampled from a distribution that makes
// smaller chunks more likely. The final size of `val` will be at least
// `min_size`.
template <typename ContainerT>
void EraseRandomChunk(ContainerT& val, absl::BitGenRef prng, size_t min_size) {
  if (val.size() <= min_size) return;
  const size_t min_final_size = std::max(min_size, val.size() / 2);
  const size_t chunk_size =
      min_final_size + 1 == val.size()
          ? 1
          : 1 + absl::Zipf(prng, val.size() - min_final_size - 1);
  const size_t chunk_offset = ChooseOffset(val.size() - chunk_size + 1, prng);
  auto it_start = std::next(val.begin(), chunk_offset);
  auto it_end = std::next(it_start, chunk_size);
  val.erase(it_start, it_end);
}

// Inserts a chunk consisting of `new_element_val` at a random position in
// `val`. The chunk's size is sampled from a distribution so that the
// final size of `val` is between `val.size() + 1` and `max_size`, with smaller
// chunks being more likely.
template <typename ContainerT, typename T>
void InsertRandomChunk(ContainerT& val, absl::BitGenRef prng, size_t max_size,
                       T new_element_val) {
  if (val.size() >= max_size) return;
  size_t chunk_size = val.size() + 1 == max_size
                          ? 1
                          : 1 + absl::Zipf(prng, max_size - val.size() - 1);
  const size_t chunk_offset = ChooseOffset(val.size() + 1, prng);
  while (chunk_size--) {
    val.insert(std::next(val.begin(), chunk_offset), new_element_val);
  }
}

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_CONTAINER_MUTATION_HELPERS_H_
