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
#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_STRING_STORAGE_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_STRING_STORAGE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/containers/bit_vector.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/storage_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor::column {

// Storage for String columns.
class StringStorage final : public StorageLayer {
 public:
  StringStorage(StringPool* string_pool,
                const std::vector<StringPool::Id>* data,
                bool is_sorted = false);
  ~StringStorage() override;

  std::unique_ptr<DataLayerChain> MakeChain();
  StoragePtr GetStoragePtr() override;

 private:
  class ChainImpl : public DataLayerChain {
   public:
    ChainImpl(StringPool* string_pool,
              const std::vector<StringPool::Id>* data,
              bool is_sorted);

    SingleSearchResult SingleSearch(FilterOp,
                                    SqlValue,
                                    uint32_t) const override;

    SearchValidationResult ValidateSearchConstraints(FilterOp,
                                                     SqlValue) const override;

    RangeOrBitVector SearchValidated(FilterOp, SqlValue, Range) const override;

    void IndexSearchValidated(FilterOp, SqlValue, Indices&) const override;

    void StableSort(Token* start,
                    Token* end,
                    SortDirection direction) const override;

    void Distinct(Indices&) const override;

    std::optional<Token> MaxElement(Indices&) const override;

    std::optional<Token> MinElement(Indices&) const override;

    SqlValue Get_AvoidUsingBecauseSlow(uint32_t index) const override;

    uint32_t size() const override {
      return static_cast<uint32_t>(data_->size());
    }

    std::string DebugString() const override { return "StringStorage"; }

   private:
    BitVector LinearSearch(FilterOp, SqlValue, Range) const;

    RangeOrBitVector IndexSearchInternal(FilterOp op,
                                         SqlValue sql_val,
                                         const uint32_t* indices,
                                         uint32_t indices_size) const;

    Range BinarySearchIntrinsic(FilterOp op,
                                SqlValue val,
                                Range search_range) const;

    inline bool LessForTokens(const Token& lhs, const Token& rhs) const {
      // If RHS is NULL, we know that LHS is not less than
      // NULL, as nothing is less then null. This check is
      // only required to keep the stability of the sort.
      if ((*data_)[rhs.index] == StringPool::Id::Null()) {
        return false;
      }

      // If LHS is NULL, it will always be smaller than any
      // RHS value.
      if ((*data_)[lhs.index] == StringPool::Id::Null()) {
        return true;
      }

      // If neither LHS or RHS are NULL, we have to simply
      // check which string is smaller.
      return string_pool_->Get((*data_)[lhs.index]) <
             string_pool_->Get((*data_)[rhs.index]);
    }

    // TODO(b/307482437): After the migration vectors should be owned by
    // storage, so change from pointer to value.
    const std::vector<StringPool::Id>* data_ = nullptr;
    StringPool* string_pool_ = nullptr;
    const bool is_sorted_ = false;
  };

  const std::vector<StringPool::Id>* data_ = nullptr;
  StringPool* string_pool_ = nullptr;
  const bool is_sorted_ = false;
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_STRING_STORAGE_H_
