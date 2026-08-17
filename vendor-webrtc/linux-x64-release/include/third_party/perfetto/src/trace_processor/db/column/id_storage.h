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
#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_ID_STORAGE_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_ID_STORAGE_H_

#include <cstdint>
#include <limits>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/storage_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {

// Base level storage for id columns.
//
// Note: This storage does not have any size instead spanning the entire
// uint32_t space (any such integer is a valid id). Overlays (e.g.
// RangeOverlay/SelectorOverlay) can be used to limit which ids are
// included in the column.
class IdStorage final : public StorageLayer {
 public:
  IdStorage();
  ~IdStorage() override;

  std::unique_ptr<DataLayerChain> MakeChain();
  StoragePtr GetStoragePtr() override;

 private:
  class ChainImpl : public DataLayerChain {
   public:
    SingleSearchResult SingleSearch(FilterOp,
                                    SqlValue,
                                    uint32_t) const override;

    SearchValidationResult ValidateSearchConstraints(FilterOp,
                                                     SqlValue) const override;

    RangeOrBitVector SearchValidated(FilterOp, SqlValue, Range) const override;

    void IndexSearchValidated(FilterOp, SqlValue, Indices&) const override;

    void StableSort(Token* start, Token* end, SortDirection) const override;

    void Distinct(Indices&) const override;

    std::optional<Token> MaxElement(Indices&) const override;

    std::optional<Token> MinElement(Indices&) const override;

    SqlValue Get_AvoidUsingBecauseSlow(uint32_t index) const override;

    uint32_t size() const override {
      return std::numeric_limits<uint32_t>::max();
    }

    std::string DebugString() const override { return "IdStorage"; }

   private:
    using Id = uint32_t;

    BitVector IndexSearch(FilterOp, Id, uint32_t*, uint32_t) const;
    static Range BinarySearchIntrinsic(FilterOp, Id, Range);
  };
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_ID_STORAGE_H_
