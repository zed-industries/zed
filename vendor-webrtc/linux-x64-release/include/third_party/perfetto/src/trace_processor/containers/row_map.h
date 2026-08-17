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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_H_

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <numeric>
#include <optional>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "src/trace_processor/containers/bit_vector.h"

namespace perfetto {
namespace trace_processor {

// Stores a list of row indices in a space efficient manner. One or more
// columns can refer to the same RowMap. The RowMap defines the access pattern
// to iterate on rows.
//
// Naming convention:
//
// As both the input and output of RowMap is a uint32_t, it can be quite
// confusing to reason about what parameters/return values of the functions
// of RowMap actually means. To help with this, we define a strict convention
// of naming.
//
// row:     input - that is, rows are what are passed into operator[]; named as
//          such because a "row" number in a table is converted to an index to
//          lookup in the backing vectors.
// index:   output - that is, indices are what are returned from operator[];
//          named as such because an "index" is what's used to lookup data
//          from the backing vectors.
//
// Implementation details:
//
// Behind the scenes, this class is implemented using one of three backing
// data-structures:
// 1. A start and end index (internally named 'range')
// 1. BitVector
// 2. std::vector<uint32_t> (internally named IndexVector).
//
// Generally the preference for data structures is range > BitVector >
// std::vector<uint32>; this ordering is based mainly on memory efficiency as we
// expect RowMaps to be large.
//
// However, BitVector and std::vector<uint32_t> allow things which are not
// possible with the data-structures preferred to them:
//  * a range (as the name suggests) can only store a compact set of indices
//  with no holes. A BitVector works around this limitation by storing a 1 at an
//  index where that row is part of the RowMap and 0 otherwise.
//  * as soon as ordering or duplicate rows come into play, we cannot use a
//   BitVector anymore as ordering/duplicate row information cannot be captured
//   by a BitVector.
//
// For small, sparse RowMaps, it is possible that a std::vector<uint32_t> is
// more efficient than a BitVector; in this case, we will make a best effort
// switch to it but the cases where this happens is not precisely defined.
class RowMap {
 public:
  using InputRow = uint32_t;
  using OutputIndex = uint32_t;
  using IndexVector = std::vector<OutputIndex>;

  struct Range {
    Range(OutputIndex start_index, OutputIndex end_index)
        : start(start_index), end(end_index) {
      PERFETTO_DCHECK(start_index <= end_index);
    }
    Range() : start(0), end(0) {}

    OutputIndex start;  // This is an inclusive index.
    OutputIndex end;    // This is an exclusive index.

    bool empty() const { return size() == 0; }
    uint32_t size() const { return end - start; }
    inline bool Contains(uint32_t val) const {
      return val >= start && val < end;
    }
  };

  // Allows efficient iteration over the rows of a RowMap.
  //
  // Note: you should usually prefer to use the methods on RowMap directly (if
  // they exist for the task being attempted) to avoid the lookup for the mode
  // of the RowMap on every method call.
  class Iterator {
   public:
    explicit Iterator(const RowMap* rm);

    Iterator(const Iterator&) = delete;
    Iterator& operator=(const Iterator&) = delete;

    Iterator(Iterator&&) noexcept = default;
    Iterator& operator=(Iterator&&) = default;

    // Forwards the iterator to the next row of the RowMap.
    void Next() { ++ordinal_; }

    // Returns if the iterator is still valid.
    explicit operator bool() const {
      if (const auto* range = std::get_if<Range>(&rm_->data_)) {
        return ordinal_ < range->end;
      }
      if (std::get_if<BitVector>(&rm_->data_)) {
        return ordinal_ < results_.size();
      }
      if (const auto* vec = std::get_if<IndexVector>(&rm_->data_)) {
        return ordinal_ < vec->size();
      }
      PERFETTO_FATAL("Didn't match any variant type.");
    }

    // Returns the index pointed to by this iterator.
    OutputIndex index() const {
      if (std::get_if<Range>(&rm_->data_)) {
        return ordinal_;
      }
      if (std::get_if<BitVector>(&rm_->data_)) {
        return results_[ordinal_];
      }
      if (const auto* vec = std::get_if<IndexVector>(&rm_->data_)) {
        return (*vec)[ordinal_];
      }
      PERFETTO_FATAL("Didn't match any variant type.");
    }

    // Returns the row of the index the iterator points to.
    InputRow row() const {
      if (const auto* range = std::get_if<Range>(&rm_->data_)) {
        return ordinal_ - range->start;
      }
      if (std::get_if<BitVector>(&rm_->data_) ||
          std::get_if<IndexVector>(&rm_->data_)) {
        return ordinal_;
      }
      PERFETTO_FATAL("Didn't match any variant type.");
    }

   private:
    // Ordinal will not be used for BitVector based RowMap.
    uint32_t ordinal_ = 0;
    // Not empty for BitVector based RowMap.
    std::vector<uint32_t> results_;

    const RowMap* rm_ = nullptr;
  };

  // Creates an empty RowMap.
  // By default this will be implemented using a range.
  RowMap();

  // Creates a RowMap containing the range of indices between |start| and |end|
  // i.e. all indices between |start| (inclusive) and |end| (exclusive).
  RowMap(OutputIndex start, OutputIndex end);

  // Creates a RowMap backed by a BitVector.
  explicit RowMap(BitVector);

  // Creates a RowMap backed by an std::vector<uint32_t>.
  explicit RowMap(IndexVector);

  RowMap(const RowMap&) noexcept = delete;
  RowMap& operator=(const RowMap&) = delete;

  RowMap(RowMap&&) noexcept = default;
  RowMap& operator=(RowMap&&) = default;

  // Creates a RowMap containing just |index|.
  // By default this will be implemented using a range.
  static RowMap SingleRow(OutputIndex index) {
    return RowMap(index, index + 1);
  }

  // Creates a copy of the RowMap.
  // We have an explicit copy function because RowMap can hold onto large chunks
  // of memory and we want to be very explicit when making a copy to avoid
  // accidental leaks and copies.
  RowMap Copy() const;

  // Returns the size of the RowMap; that is the number of indices in the
  // RowMap.
  uint32_t size() const {
    if (const auto* range = std::get_if<Range>(&data_)) {
      return range->size();
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return bv->CountSetBits();
    }
    if (const auto* vec = std::get_if<IndexVector>(&data_)) {
      return static_cast<uint32_t>(vec->size());
    }
    NoVariantMatched();
  }

  // Returns whether this rowmap is empty.
  bool empty() const { return size() == 0; }

  // Returns the index at the given |row|.
  OutputIndex Get(InputRow row) const {
    if (const auto* range = std::get_if<Range>(&data_)) {
      return GetRange(*range, row);
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return GetBitVector(*bv, row);
    }
    if (const auto* vec = std::get_if<IndexVector>(&data_)) {
      return GetIndexVector(*vec, row);
    }
    NoVariantMatched();
  }

  // Returns the vector of all indices in the RowMap.
  std::vector<OutputIndex> GetAllIndices() const {
    if (const auto* range = std::get_if<Range>(&data_)) {
      std::vector<uint32_t> res(range->size());
      std::iota(res.begin(), res.end(), range->start);
      return res;
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return bv->GetSetBitIndices();
    }
    if (const auto* vec = std::get_if<IndexVector>(&data_)) {
      return *vec;
    }
    NoVariantMatched();
  }

  // Returns maximum size of the output. Ie range.end or size of the BV.
  OutputIndex Max() const;

  // Returns whether the RowMap contains the given index.
  bool Contains(OutputIndex index) const {
    if (const auto* range = std::get_if<Range>(&data_)) {
      return index >= range->start && index < range->end;
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return index < bv->size() && bv->IsSet(index);
    }
    if (const auto* vec = std::get_if<IndexVector>(&data_)) {
      return std::find(vec->begin(), vec->end(), index) != vec->end();
    }
    NoVariantMatched();
  }

  // Returns the first row of the given |index| in the RowMap.
  std::optional<InputRow> RowOf(OutputIndex index) const {
    if (const auto* range = std::get_if<Range>(&data_)) {
      if (index < range->start || index >= range->end)
        return std::nullopt;
      return index - range->start;
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return index < bv->size() && bv->IsSet(index)
                 ? std::make_optional(bv->CountSetBits(index))
                 : std::nullopt;
    }
    if (const auto* vec = std::get_if<IndexVector>(&data_)) {
      auto it = std::find(vec->begin(), vec->end(), index);
      return it != vec->end() ? std::make_optional(static_cast<InputRow>(
                                    std::distance(vec->begin(), it)))
                              : std::nullopt;
    }
    NoVariantMatched();
  }

  // Performs an ordered insert of the index into the current RowMap
  // (precondition: this RowMap is ordered based on the indices it contains).
  //
  // Example:
  // this = [1, 5, 10, 11, 20]
  // Insert(10)  // this = [1, 5, 10, 11, 20]
  // Insert(12)  // this = [1, 5, 10, 11, 12, 20]
  // Insert(21)  // this = [1, 5, 10, 11, 12, 20, 21]
  // Insert(2)   // this = [1, 2, 5, 10, 11, 12, 20, 21]
  //
  // Speecifically, this means that it is only valid to call Insert on a RowMap
  // which is sorted by the indices it contains; this is automatically true when
  // the RowMap is in range or BitVector mode but is a required condition for
  // IndexVector mode.
  void Insert(OutputIndex index) {
    if (auto* range = std::get_if<Range>(&data_)) {
      if (index == range->end) {
        // Fast path: if we're just appending to the end
        // of the range, we can stay in range mode and
        // just bump the end index.
        range->end++;
        return;
      }

      // Slow path: the insert is somewhere else other
      // than the end. This means we need to switch to
      // using a BitVector instead.
      BitVector bv;
      bv.Resize(range->start, false);
      bv.Resize(range->end, true);
      InsertIntoBitVector(bv, index);
      data_ = std::move(bv);
      return;
    }
    if (auto* bv = std::get_if<BitVector>(&data_)) {
      InsertIntoBitVector(*bv, index);
      return;
    }
    if (auto* vec = std::get_if<IndexVector>(&data_)) {
      PERFETTO_DCHECK(std::is_sorted(vec->begin(), vec->end()));
      auto it = std::upper_bound(vec->begin(), vec->end(), index);
      vec->insert(it, index);
      return;
    }
    NoVariantMatched();
  }

  // Updates this RowMap by 'picking' the indices given by |picker|.
  // This is easiest to explain with an example; suppose we have the following
  // RowMaps:
  // this  : [0, 1, 4, 10, 11]
  // picker: [0, 3, 4, 4, 2]
  //
  // After calling Apply(picker), we now have the following:
  // this  : [0, 10, 11, 11, 4]
  //
  // Conceptually, we are performing the following algorithm:
  // RowMap rm = Copy()
  // for (p : picker)
  //   rm[i++] = this[p]
  // return rm;
  RowMap SelectRows(const RowMap& selector) const {
    uint32_t size = selector.size();

    // If the selector is empty, just return an empty RowMap.
    if (size == 0u)
      return {};

    // If the selector is just picking a single row, just return that row
    // without any additional overhead.
    if (size == 1u)
      return RowMap::SingleRow(Get(selector.Get(0)));

    // For all other cases, go into the slow-path.
    return SelectRowsSlow(selector);
  }

  // Intersects |this| with |second| independent of underlying structure of both
  // RowMaps. Modifies |this| to only contain indices present in |second|.
  void Intersect(const RowMap& second);

  // Intersects this RowMap with |index|. If this RowMap contained |index|, then
  // it will *only* contain |index|. Otherwise, it will be empty.
  void IntersectExact(OutputIndex index) {
    if (Contains(index)) {
      *this = RowMap(index, index + 1);
    } else {
      Clear();
    }
  }

  // Clears this RowMap by resetting it to a newly constructed state.
  void Clear() { *this = RowMap(); }

  // Converts this RowMap to an index vector in the most efficient way
  // possible.
  std::vector<uint32_t> TakeAsIndexVector() && {
    if (const auto* range = std::get_if<Range>(&data_)) {
      std::vector<uint32_t> rm(range->size());
      std::iota(rm.begin(), rm.end(), range->start);
      return rm;
    }
    if (const auto* bv = std::get_if<BitVector>(&data_)) {
      return bv->GetSetBitIndices();
    }
    if (auto* vec = std::get_if<IndexVector>(&data_)) {
      return std::move(*vec);
    }
    NoVariantMatched();
  }

  // Returns the data in RowMap BitVector, nullptr if RowMap is in a different
  // mode.
  const BitVector* GetIfBitVector() const {
    return std::get_if<BitVector>(&data_);
  }

  // Returns the data in RowMap IndexVector, nullptr if RowMap is in a different
  // mode.
  const std::vector<uint32_t>* GetIfIndexVector() const {
    return std::get_if<IndexVector>(&data_);
  }

  // Returns the data in RowMap Range, nullptr if RowMap is in a different
  // mode.
  const Range* GetIfIRange() const { return std::get_if<Range>(&data_); }

  // Returns the iterator over the rows in this RowMap.
  Iterator IterateRows() const { return Iterator(this); }

  // Returns if the RowMap is internally represented using a range.
  bool IsRange() const { return std::holds_alternative<Range>(data_); }

  // Returns if the RowMap is internally represented using a BitVector.
  bool IsBitVector() const { return std::holds_alternative<BitVector>(data_); }

  // Returns if the RowMap is internally represented using an index vector.
  bool IsIndexVector() const {
    return std::holds_alternative<IndexVector>(data_);
  }

 private:
  using Variant = std::variant<Range, BitVector, IndexVector>;

  explicit RowMap(Range);

  explicit RowMap(Variant);

  PERFETTO_ALWAYS_INLINE static OutputIndex GetRange(Range r, InputRow row) {
    return r.start + row;
  }
  PERFETTO_ALWAYS_INLINE static OutputIndex GetBitVector(const BitVector& bv,
                                                         uint32_t row) {
    return bv.IndexOfNthSet(row);
  }
  PERFETTO_ALWAYS_INLINE static OutputIndex GetIndexVector(
      const IndexVector& vec,
      uint32_t row) {
    return vec[row];
  }

  RowMap SelectRowsSlow(const RowMap& selector) const;

  static void InsertIntoBitVector(BitVector& bv, OutputIndex row) {
    if (row == bv.size()) {
      bv.AppendTrue();
      return;
    }
    if (row > bv.size())
      bv.Resize(row + 1, false);
    bv.Set(row);
  }

  PERFETTO_NORETURN static void NoVariantMatched() {
    PERFETTO_FATAL("Didn't match any variant type.");
  }

  Variant data_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_H_
