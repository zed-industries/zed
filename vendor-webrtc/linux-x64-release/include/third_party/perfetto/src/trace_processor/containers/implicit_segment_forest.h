/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_IMPLICIT_SEGMENT_FOREST_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_IMPLICIT_SEGMENT_FOREST_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "perfetto/base/logging.h"

#if PERFETTO_BUILDFLAG(PERFETTO_X64_CPU_OPT)
#include <immintrin.h>
#endif

namespace perfetto::trace_processor {

// An implementation of a segment tree data structure [1] with:
// 1) parent-child relationships are implicit, saving memory.
// 2) the requirement for the number of values being a power of two, turning
//    the tree into a forest.
//
// Segment trees are a very powerful data structure allowing O(log(n)) aggregate
// queries to be performed on an arbitrary range of elements in an array.
// Specifically, for `T x[n]`, and an associative and commutative operation
// AggOp (e.g. +, *, min, max, etc.), segment trees can compute
// ```
//   T y = AggOp()(x[i], x[i + 1], x[i + 2], ..., x[j])
// ```
// in O(log(n)) time.
//
// Practically, in trace processor, this is useful for computing aggregations
// over events in a trace. For example:
// ```
// struct Slice { int64_t ts; int64_t dur; };
// struct MaxDurSlice {
//   Slice operator()(const Slice& a, const Slice& b) {
//     return a.dur < b.dur ? b : a;
//   }
// }
// using MipMap = ImplicitSegmentForest<Slice, MaxDurSlice>;
// ```
// allows building a "mipmap" [2] of a track in a trace in a UI. The UI can show
// a representation of the items in the track when very zoomed out while
// skipping the rendering slices which are smaller than one pixel.
//
// The design and implementation of this class takes heavy inspiration from
// Tristan Hume's "IForestIndex" data structure [3] as described in his blog
// post [4].
//
// [1] https://en.algorithmica.org/hpc/data-structures/segment-trees/
// [2] https://en.wikipedia.org/wiki/Mipmap
// [3]
// https://github.com/trishume/gigatrace/blob/dfde0d7244f356bdc9aeefb387d904dd8b09d94a/src/iforest.rs
// [4] https://thume.ca/2021/03/14/iforests/
template <typename T, typename AggOp>
class ImplicitSegmentForest {
 public:
  // Computes the aggregation (as specified by operator() in AggOp) over all
  // elements in the tree between the indices [start, end). Requires that
  // start < end.
  //
  // Complexity:
  // This function performs O(log(n)) operations (n = end - start).
  //
  // Returns:
  //  1) values[start]: if start + 1 == end
  //  2) AggOp()(values[start], ..., values[end - 1]) otherwise
  T Query(uint32_t start, uint32_t end) const {
    PERFETTO_DCHECK(start < end);

    const uint32_t in_start = start * 2;
    const uint32_t in_end = end * 2;

    uint32_t first_skip = LargestPrefixInsideSkip(in_start, in_end);
    T aggregated = values_[AggNode(in_start, first_skip)];
    for (uint32_t i = in_start + first_skip; i < in_end;) {
      uint32_t skip = LargestPrefixInsideSkip(i, in_end);
      aggregated = AggOp()(aggregated, values_[AggNode(i, skip)]);
      i += skip;
    }
    return aggregated;
  }

  // Pushes a new element to right-most part of the tree. This index of this
  // element can be used in future calls to |Query|.
  void Push(T v) {
    values_.emplace_back(std::move(v));

    size_t len = values_.size();
    auto levels_to_index =
        static_cast<uint32_t>(Tzcnt(static_cast<uint64_t>(~len))) - 1;

    size_t cur = len - 1;
    for (uint32_t level = 0; level < levels_to_index; ++level) {
      size_t prev_higher_level = cur - (1 << level);
      values_[prev_higher_level] =
          AggOp()(values_[prev_higher_level], values_[cur]);
      cur = prev_higher_level;
    }
    values_.emplace_back(values_[len - (1 << levels_to_index)]);
  }

  // Returns the value at |n| in the tree: this corresponds to the |n|th
  // element |Push|-ed into the tree.
  const T& operator[](uint32_t n) { return values_[n * 2]; }

  // Returns the number of elements pushed into the forest.
  uint32_t size() const { return static_cast<uint32_t>(values_.size() / 2); }

 private:
  static uint32_t Lsp(uint32_t x) { return x & ~(x - 1); }
  static uint32_t Msp(uint32_t x) {
    return (1u << (sizeof(x) * 8 - 1)) >> Lzcnt32(x);
  }
  static uint32_t LargestPrefixInsideSkip(uint32_t min, uint32_t max) {
    return Lsp(min | Msp(max - min));
  }
  static uint32_t AggNode(uint32_t i, uint32_t offset) {
    return i + (offset >> 1) - 1;
  }

  static PERFETTO_ALWAYS_INLINE uint32_t Lzcnt32(uint32_t value) {
#if defined(__GNUC__) || defined(__clang__)
    return value ? static_cast<uint32_t>(__builtin_clz(value)) : 32u;
#else
    unsigned long out;
    return _BitScanReverse(&out, value) ? 31 - out : 32u;
#endif
  }

  static PERFETTO_ALWAYS_INLINE uint32_t Tzcnt(uint64_t value) {
#if PERFETTO_BUILDFLAG(PERFETTO_X64_CPU_OPT)
    return static_cast<uint32_t>(_tzcnt_u64(value));
#elif defined(__GNUC__) || defined(__clang__)
    return value ? static_cast<uint32_t>(__builtin_ctzll(value)) : 64u;
#else
    unsigned long out;
    return _BitScanForward64(&out, value) ? static_cast<uint32_t>(out) : 64u;
#endif
  }

  std::vector<T> values_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_IMPLICIT_SEGMENT_FOREST_H_
