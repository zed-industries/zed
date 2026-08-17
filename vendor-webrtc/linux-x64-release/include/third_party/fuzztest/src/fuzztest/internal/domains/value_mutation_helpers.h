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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_VALUE_MUTATION_HELPERS_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_VALUE_MUTATION_HELPERS_H_

#include <cstddef>
#include <limits>
#include <optional>

#include "absl/numeric/bits.h"
#include "absl/numeric/int128.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/random/random.h"
#include "./fuzztest/internal/domains/mutation_metadata.h"
#include "./fuzztest/internal/meta.h"
#include "./fuzztest/internal/table_of_recent_compares.h"

namespace fuzztest::internal {

// Random bit flip: minimal mutation to a field, it will converge
// the hamming distance of a value to its target in constant steps.
template <typename T>
void RandomBitFlip(absl::BitGenRef prng, T& val, size_t range) {
  using U = MakeUnsignedT<T>;
  U u = static_cast<U>(val);
  u ^= U{1} << absl::Uniform(prng, 0u, range);
  val = static_cast<T>(u);
}

// BitWidth of the value of val, specially handle uint12 and int128, because
// they don't have overloads in absl::bit_width.
template <typename T>
size_t BitWidth(T val) {
  if constexpr (std::is_same_v<T, absl::int128> ||
                std::is_same_v<T, absl::uint128>) {
    auto val_unsigned = MakeUnsignedT<T>(val);
    size_t res = 0;
    while (val_unsigned >>= 1) ++res;
    return res;
  } else {
    return absl::bit_width(static_cast<MakeUnsignedT<T>>(val));
  }
}

// Given a parameter pack of functions `f`, run exactly one of the functions.
template <typename... F>
void RunOne(absl::BitGenRef prng, F... f) {
  ApplyIndex<sizeof...(F)>([&](auto... I) {
    int i = absl::Uniform<int>(prng, 0, sizeof...(F));
    ((i == I ? (void)f() : (void)0), ...);
  });
}

template <typename T, size_t N, typename F>
T ChooseOneOr(const T (&values)[N], absl::BitGenRef prng, F f) {
  const int i = absl::Uniform<int>(absl::IntervalClosedClosed, prng, 0, N);
  return i == N ? f() : values[i];
}

template <typename C, typename F>
auto ChooseOneOr(const C& values, absl::BitGenRef prng, F f) {
  const int i =
      absl::Uniform<int>(absl::IntervalClosedClosed, prng, 0, values.size());
  return i < values.size() ? values[i] : f();
}

template <typename T>
T SampleFromUniformRange(absl::BitGenRef prng, T min, T max) {
  return absl::Uniform(absl::IntervalClosedClosed, prng, min, max);
}

// Randomly apply Randomwalk or Uniform distribution or dictionary to mutate
// the val:
//  RandomWalk: converge the absolute distance of a value to its target
//  more efficiently.
//  Uniform Distribution: go across some non-linear boundary that cannot
//  be solved by bit flipping or randomwalk.
//  Dictionary: if applicable, choose randomly from the dictionary. if
//  dictionary fails to mutate, fall back to uniform.
template <unsigned char RANGE, typename T, typename IntegerDictionaryT>
void RandomWalkOrUniformOrDict(absl::BitGenRef prng, T& val, T min, T max,
                               domain_implementor::ConstCmpTablesPtr cmp_tables,
                               const IntegerDictionaryT& temporary_dict,
                               const IntegerDictionaryT& permanent_dict,
                               std::optional<T>& permanent_dict_candidate) {
  constexpr bool is_memory_dictionary_compatible_integer =
      std::numeric_limits<T>::is_integer &&
      (sizeof(T) == 1 || sizeof(T) == 2 || sizeof(T) == 4 || sizeof(T) == 8);
  const bool can_use_memory_dictionary =
      is_memory_dictionary_compatible_integer && cmp_tables != nullptr;
  const int action_count = 2 + can_use_memory_dictionary;
  int action = absl::Uniform(prng, 0, action_count);
  // Random walk.
  if (action-- == 0) {
    if (max / 2 - min / 2 <= RANGE) {
      val = SampleFromUniformRange(prng, min, max);
    } else {
      T lo = min;
      T hi = max;
      if (val > lo + RANGE) lo = val - RANGE;
      if (val < hi - RANGE) hi = val + RANGE;
      val = SampleFromUniformRange(prng, lo, hi);
    }
    return;
  }
  // Random choose.
  if (action-- == 0) {
    val = SampleFromUniformRange(prng, min, max);
    return;
  }
  // Dictionary
  if constexpr (is_memory_dictionary_compatible_integer) {
    if (can_use_memory_dictionary) {
      if (action-- == 0) {
        RunOne(
            prng,
            [&] {
              if (temporary_dict.IsEmpty()) {
                val = SampleFromUniformRange(prng, min, max);
              } else {
                val = temporary_dict.GetRandomSavedEntry(prng);
                permanent_dict_candidate = val;
              }
            },
            [&] {
              if (permanent_dict.IsEmpty()) {
                val = SampleFromUniformRange(prng, min, max);
              } else {
                val = permanent_dict.GetRandomSavedEntry(prng);
              }
            },
            [&] {
              auto entry = IntegerDictionary<T>::GetRandomTORCEntry(
                  val, prng, *cmp_tables, min, max);
              if (entry.has_value()) {
                val = *entry;
              } else {
                val = SampleFromUniformRange(prng, min, max);
              }
            });
      }
    }
  }
}

// REQUIRES: |target| < |val|
template <typename T>
T ShrinkTowards(absl::BitGenRef prng, T val, T target) {
  if (val < target) {
    return absl::Uniform(absl::IntervalOpenClosed, prng, val, target);
  } else {
    return absl::Uniform(absl::IntervalClosedOpen, prng, target, val);
  }
}

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_VALUE_MUTATION_HELPERS_H_
