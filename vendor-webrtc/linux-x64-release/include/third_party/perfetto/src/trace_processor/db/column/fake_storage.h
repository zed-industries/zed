/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_FAKE_STORAGE_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_FAKE_STORAGE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {

// Fake implementation of DataLayerChain which can be used in unittests.
class FakeStorageChain : public DataLayerChain {
 public:
  // Factory function for creating a DataLayerChain which matches all rows from
  // [0, size).
  static std::unique_ptr<DataLayerChain> SearchAll(uint32_t size) {
    return std::unique_ptr<DataLayerChain>(
        new FakeStorageChain(size, SearchStrategy::kAll, Range(), BitVector()));
  }

  // Factory function for creating a DataLayerChain which matches zero rows.
  static std::unique_ptr<DataLayerChain> SearchNone(uint32_t size) {
    return std::unique_ptr<DataLayerChain>(new FakeStorageChain(
        size, SearchStrategy::kNone, Range(), BitVector()));
  }

  // Factory function for creating a DataLayerChain which matches rows [r.start,
  // r.end).
  static std::unique_ptr<DataLayerChain> SearchSubset(uint32_t size, Range r) {
    return std::unique_ptr<DataLayerChain>(
        new FakeStorageChain(size, SearchStrategy::kRange, r, BitVector()));
  }

  // Factory function for creating a DataLayerChain which matches rows of the
  // set bit positions of |bv|.
  static std::unique_ptr<DataLayerChain> SearchSubset(uint32_t size,
                                                      BitVector bv) {
    return std::unique_ptr<DataLayerChain>(new FakeStorageChain(
        size, SearchStrategy::kBitVector, Range(), std::move(bv)));
  }

  // Factory function for creating a DataLayerChain which matches rows specified
  // by |index_vec|.
  static std::unique_ptr<DataLayerChain> SearchSubset(
      uint32_t size,
      const std::vector<uint32_t>& index_vec) {
    BitVector bv(size);
    for (uint32_t i : index_vec) {
      bv.Set(i);
    }
    return std::unique_ptr<DataLayerChain>(new FakeStorageChain(
        size, SearchStrategy::kBitVector, Range(), std::move(bv)));
  }

  // Implementation of DataLayerChain.
  SingleSearchResult SingleSearch(FilterOp, SqlValue, uint32_t) const override;

  SearchValidationResult ValidateSearchConstraints(FilterOp,
                                                   SqlValue) const override;

  RangeOrBitVector SearchValidated(FilterOp, SqlValue, Range) const override;

  void IndexSearchValidated(FilterOp, SqlValue, Indices&) const override;

  void StableSort(Token* start, Token* end, SortDirection) const override;

  void Distinct(Indices&) const override;

  std::optional<Token> MaxElement(Indices&) const override;

  std::optional<Token> MinElement(Indices&) const override;

  SqlValue Get_AvoidUsingBecauseSlow(uint32_t index) const override;

  uint32_t size() const override { return size_; }

  std::string DebugString() const override { return "FakeStorage"; }

 private:
  enum SearchStrategy { kNone, kAll, kRange, kBitVector };

  FakeStorageChain(uint32_t, SearchStrategy, Range, BitVector);

  uint32_t size_ = 0;
  SearchStrategy strategy_ = SearchStrategy::kNone;
  Range range_;
  BitVector bit_vector_;
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_FAKE_STORAGE_H_
