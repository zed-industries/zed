/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_OVERLAY_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_OVERLAY_H_

#include <stdint.h>

#include <optional>
#include <utility>
#include <vector>

#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/containers/row_map.h"

namespace perfetto::trace_processor {

// Contains indices which can be used to lookup data in one or more
// ColumnStorages.
//
// Implemented as a thin wrapper around RowMap so much of the documentation
// from RowMap also applies to this class.
class ColumnStorageOverlay {
 public:
  // Input type.
  using InputRow = uint32_t;
  using OutputIndex = uint32_t;

  // Allows efficient iteration over the rows of a ColumnStorageOverlay.
  class Iterator {
   public:
    explicit Iterator(RowMap::Iterator it) : it_(std::move(it)) {}

    Iterator(Iterator&&) noexcept = default;
    Iterator& operator=(Iterator&&) = default;

    // Forwards the iterator to the next row of the ColumnStorageOverlay.
    void Next() { return it_.Next(); }

    // Returns if the iterator is still valid.
    explicit operator bool() const { return bool(it_); }

    // Returns the index pointed to by this iterator.
    OutputIndex index() const { return it_.index(); }

    // Returns the row of the index the iterator points to.
    InputRow row() const { return it_.row(); }

   private:
    RowMap::Iterator it_;
  };

  // Creates an empty ColumnStorageOverlay.
  // By default this will be implemented using a range.
  ColumnStorageOverlay() : ColumnStorageOverlay(0) {}

  // Creates a |ColumnStorageOverlay| containing all rows between 0 and |size|.
  explicit ColumnStorageOverlay(uint32_t size)
      : ColumnStorageOverlay(0, size) {}

  // Creates a |ColumnStorageOverlay| containing all rows between |start| and
  // |end|.
  explicit ColumnStorageOverlay(uint32_t start, uint32_t end)
      : ColumnStorageOverlay(RowMap(start, end)) {}

  // Creates a |ColumnStorageOverlay| containing all rows corresponding to set
  // bits in |bv|.
  explicit ColumnStorageOverlay(BitVector bv)
      : ColumnStorageOverlay(RowMap(std::move(bv))) {}

  // Creates a |ColumnStorageOverlay| containing all rows in |rows|.
  explicit ColumnStorageOverlay(std::vector<uint32_t> rows)
      : ColumnStorageOverlay(RowMap(std::move(rows))) {}

  ColumnStorageOverlay(const ColumnStorageOverlay&) noexcept = delete;
  ColumnStorageOverlay& operator=(const ColumnStorageOverlay&) = delete;

  ColumnStorageOverlay(ColumnStorageOverlay&&) noexcept = default;
  ColumnStorageOverlay& operator=(ColumnStorageOverlay&&) = default;

  // Creates a copy of the ColumnStorageOverlay.
  // We have an explicit copy function because ColumnStorageOverlay can hold
  // onto large chunks of memory and we want to be very explicit when making a
  // copy to avoid accidental leaks and copies.
  ColumnStorageOverlay Copy() const {
    return ColumnStorageOverlay(row_map_.Copy());
  }

  // Returns the size of the ColumnStorageOverlay; that is the number of
  // indices in the ColumnStorageOverlay.
  uint32_t size() const { return row_map_.size(); }

  // Returns whether this ColumnStorageOverlay is empty.
  bool empty() const { return size() == 0; }

  // Returns the index at the given |row|.
  OutputIndex Get(uint32_t row) const { return row_map_.Get(row); }

  // Returns the first row of the given |index| in the ColumnStorageOverlay.
  std::optional<InputRow> RowOf(OutputIndex index) const {
    return row_map_.RowOf(index);
  }

  // Performs an ordered insert of the index into the current
  // ColumnStorageOverlay (precondition: this ColumnStorageOverlay is ordered
  // based on the indices it contains).
  //
  // See RowMap::Insert for more information on this function.
  void Insert(OutputIndex index) { return row_map_.Insert(index); }

  // Updates this ColumnStorageOverlay by 'picking' the indices given by
  // |picker|.
  //
  // See RowMap::SelectRows for more information on this function.
  ColumnStorageOverlay SelectRows(const RowMap& selector) const {
    return ColumnStorageOverlay(row_map_.SelectRows(selector));
  }

  // Clears this ColumnStorageOverlay by resetting it to a newly constructed
  // state.
  void Clear() { *this = ColumnStorageOverlay(); }

  // Returns the iterator over the rows in this ColumnStorageOverlay.
  Iterator IterateRows() const { return Iterator(row_map_.IterateRows()); }

  const RowMap& row_map() const { return row_map_; }

 private:
  explicit ColumnStorageOverlay(RowMap rm) : row_map_(std::move(rm)) {}

  RowMap row_map_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_OVERLAY_H_
